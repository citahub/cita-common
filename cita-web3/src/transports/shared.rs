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

use web3::{
    self, futures::{self, sync::oneshot},
};

type PendingResult<O> = oneshot::Receiver<web3::transports::Result<O>>;

enum RequestState<O> {
    Sending(Option<web3::transports::Result<()>>, PendingResult<O>),
    WaitingForResponse(PendingResult<O>),
    Done,
}

/// A future representing a response to a pending request.
pub struct Response<T, O> {
    id: web3::RequestId,
    state: RequestState<O>,
    extract: T,
}

impl<T, O> Response<T, O> {
    /// Creates a new `Response`
    pub fn new(
        id: web3::RequestId,
        result: web3::transports::Result<()>,
        rx: PendingResult<O>,
        extract: T,
    ) -> Self {
        Response {
            id,
            extract,
            state: RequestState::Sending(Some(result), rx),
        }
    }

    pub fn get_id(&self) -> web3::RequestId {
        self.id
    }
}

impl<T, O, Out> futures::Future for Response<T, O>
where
    T: Fn(O) -> web3::transports::Result<Out>,
    Out: ::std::fmt::Debug,
{
    type Item = Out;
    type Error = web3::Error;

    fn poll(&mut self) -> futures::Poll<Self::Item, Self::Error> {
        loop {
            let extract = &self.extract;
            match self.state {
                RequestState::Sending(ref mut result, _) => {
                    if let Some(Err(e)) = result.take() {
                        return Err(e);
                    }
                }
                RequestState::WaitingForResponse(ref mut rx) => {
                    let result = try_ready!(rx.poll().map_err(|_| web3::Error::from(
                        web3::ErrorKind::Io(::std::io::ErrorKind::TimedOut.into())
                    )));
                    return result.and_then(|x| extract(x)).map(futures::Async::Ready);
                }
                RequestState::Done => {
                    return Err(web3::ErrorKind::Unreachable.into());
                }
            }
            // Proceed to the next state
            let state = ::std::mem::replace(&mut self.state, RequestState::Done);
            self.state = if let RequestState::Sending(_, rx) = state {
                RequestState::WaitingForResponse(rx)
            } else {
                state
            }
        }
    }
}
