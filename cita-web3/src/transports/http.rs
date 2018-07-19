// CITA
// Copyright 2016-2018 Cryptape Technologies LLC.

// This program is free software: you can redistribute it
// and/or modify it under the terms of the GNU General Public
// License as published by the Free Software Foundation,
// either version 3 of the License, or (at your option) any
// later version.

// This program is distributed in the hope that it will be
// useful, but WITHOUT ANY WARRANTY; without even the implied
// warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR
// PURPOSE. See the GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <http://www.gnu.org/licenses/>.

//! `Http` transport

use super::shared;
use hyper;
use jsonrpc_core as rpc;
use parking_lot::{Mutex, RwLock};
use serde_json;
use tokio_core::reactor;
use web3::{
    self, futures::{
        future::Either, sync::{mpsc, oneshot}, Future, Sink, Stream,
    },
};

pub type FetchTask<F> = shared::Response<F, hyper::Chunk>;

pub type Pending = oneshot::Sender<web3::transports::Result<hyper::Chunk>>;
pub type PendingRequest = (hyper::Request, Pending);

pub type HttpSender = mpsc::Sender<PendingRequest>;

#[derive(Debug)]
pub struct HttpCore {
    sender: Mutex<HttpSender>,
    url: RwLock<hyper::Uri>,
}

#[derive(Debug, Clone)]
pub struct Http {
    id: ::std::sync::Arc<::std::sync::atomic::AtomicUsize>,
    core: ::std::sync::Arc<HttpCore>,
}

#[derive(Debug, Clone)]
pub enum HttpError {
    TimeoutTooLow,
    ParseUrlFailed,
    SpawnFailed,
}

impl Http {
    pub fn new(server_uri: &str, buffer: usize, timeout_secs: u64) -> Result<Self, HttpError> {
        server_uri
            .parse::<hyper::Uri>()
            .map_err(|_| HttpError::ParseUrlFailed)
            .and_then(|url| {
                if timeout_secs == 0 {
                    Err(HttpError::TimeoutTooLow)
                } else {
                    Ok(url)
                }
            })
            .and_then(|url| HttpCore::new(url, buffer, timeout_secs))
            .map(|core| Http {
                id: Default::default(),
                core: ::std::sync::Arc::new(core),
            })
    }
}

impl HttpCore {
    pub fn new(url: hyper::Uri, buffer: usize, timeout_secs: u64) -> Result<Self, HttpError> {
        let tb = ::std::thread::Builder::new().name("HttpTransport".to_string());
        let (tx, rx) = mpsc::channel::<PendingRequest>(buffer);
        let timeout_dur = ::std::time::Duration::from_secs(timeout_secs);
        tb.spawn(move || {
            let mut core = reactor::Core::new().unwrap();
            let handle = core.handle();
            let client = hyper::Client::configure()
                .connector(hyper::client::HttpConnector::new(4, &handle))
                .keep_alive(false)
                .build(&handle);
            let messages = rx.for_each(|(req, sender)| {
                let timeout = reactor::Timeout::new(timeout_dur, &handle).unwrap();
                let post = client.request(req).and_then(|res| res.body().concat2());
                let work = post.select2(timeout).then(move |res| match res {
                    Ok(Either::A((got, _timeout))) => {
                        let _ = sender.send(Ok(got));
                        Ok(())
                    }
                    Ok(Either::B(_)) | Err(_) => {
                        let _ = sender
                            .send(Err(web3::ErrorKind::Transport("Timeout".to_owned()).into()));
                        Ok(())
                    }
                });
                handle.spawn(work);
                Ok(())
            });
            core.run(messages).unwrap();
        }).map_err(|_| HttpError::SpawnFailed)
            .map(|_| HttpCore {
                sender: Mutex::new(tx),
                url: RwLock::new(url),
            })
    }

    pub fn send_request<F, O>(
        &self,
        id: web3::RequestId,
        request: rpc::Request,
        extract: F,
    ) -> FetchTask<F>
    where
        F: Fn(hyper::Chunk) -> O,
    {
        let url = { self.url.read().clone() };
        let mut req = hyper::Request::new(hyper::Method::Post, url);
        let body = web3::helpers::to_string(&request);
        req.headers_mut().set(hyper::header::ContentType::json());
        req.headers_mut()
            .set(hyper::header::ContentLength(body.len() as u64));
        req.set_body(body);
        let (tx, rx) = oneshot::channel();
        let result = {
            let result = self
                .sender
                .lock()
                .start_send((req, tx))
                .map(|_| ())
                .map_err(|_| web3::ErrorKind::Io(::std::io::ErrorKind::BrokenPipe.into()).into());
            result
        };
        shared::Response::new(id, result, rx, extract)
    }
}

impl web3::Transport for Http {
    type Out = FetchTask<fn(hyper::Chunk) -> web3::transports::Result<rpc::Value>>;

    fn prepare(&self, method: &str, params: Vec<rpc::Value>) -> (web3::RequestId, rpc::Call) {
        let id = self.id.fetch_add(1, ::std::sync::atomic::Ordering::AcqRel);
        let request = web3::helpers::build_request(id, method, params);
        (id, request)
    }

    fn send(&self, id: web3::RequestId, request: rpc::Call) -> Self::Out {
        self.core
            .send_request(id, rpc::Request::Single(request), single_response)
    }
}

/// Parse bytes RPC response into `Result`.
fn single_response<T: ::std::ops::Deref<Target = [u8]>>(
    response: T,
) -> web3::transports::Result<rpc::Value> {
    let response = serde_json::from_slice(&*response)
        .map_err(|e| web3::Error::from(web3::ErrorKind::InvalidResponse(format!("{:?}", e))))?;
    match response {
        rpc::Response::Single(output) => web3::helpers::to_result_from_output(output),
        _ => Err(web3::ErrorKind::InvalidResponse("Expected single, got batch.".into()).into()),
    }
}
