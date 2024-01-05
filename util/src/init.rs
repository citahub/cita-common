// Copyright Rivtower Technologies LLC.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use serde::de;
use std::io::Write;
use toml;

#[macro_export]
macro_rules! micro_service_init {
    ($x:expr, $y:expr, $s:expr) => {
        dotenv::dotenv().ok();
        // Always print backtrace on panic.
        ::std::env::set_var("RUST_BACKTRACE", "full");

        //exit process when panic
        set_panic_handler();

        // log4rs config
        if $s {
            logger::init_config(&logger::LogFavour::Stdout($x));
        } else {
            logger::init_config(&logger::LogFavour::File($x));
        }
        info!($y);
    };
}

pub fn parse_config_from_buffer<T>(s: &'_ str) -> Result<T, toml::de::Error>
where
    T: de::DeserializeOwned,
{
    toml::from_str::<T>(s)
}

#[macro_export]
macro_rules! parse_config {
    ($type:ty, $path:expr) => {{
        use std::fs::File;
        use std::io::Read;
        use util::init::parse_config_from_buffer;

        let mut buffer = String::new();
        File::open($path)
            .and_then(|mut f| f.read_to_string(&mut buffer))
            .unwrap_or_else(|err| panic!("Error while loading config: [{}]", err));
        parse_config_from_buffer::<$type>(&buffer)
            .unwrap_or_else(|err| panic!("Error while parsing config: [{}]", err))
    }};
}

pub fn write_toml<T: serde::Serialize>(content: T, path: impl AsRef<std::path::Path>) {
    let toml = toml::Value::try_from(content).unwrap();

    let mut file = std::fs::OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(path.as_ref())
        .unwrap_or_else(|_| panic!("open file({:?}) failed.", path.as_ref().to_str()));
    file.write_all(toml::to_string_pretty(&toml).unwrap().as_bytes())
        .unwrap();
    file.write_all(b"\n").unwrap();
}
