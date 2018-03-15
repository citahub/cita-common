use serde::de;
use toml;

#[macro_export]
macro_rules! micro_service_init {
    ($x:expr, $y:expr) => {
        dotenv::dotenv().ok();
        // Always print backtrace on panic.
        ::std::env::set_var("RUST_BACKTRACE", "full");

        //exit process when panic
        set_panic_handler();

        // log4rs config
        logger::init_config($x);
        info!($y);
    };
}

pub fn parse_config_from_buffer<'de, T>(s: &'de str) -> Result<T, toml::de::Error>
where
    T: de::Deserialize<'de>,
{
    Ok(toml::from_str::<T>(s)?)
}

#[macro_export]
macro_rules! parse_config {
    ($type:ty,$path:expr) => {{
        use std::fs::File;
        use std::io::Read;
        use util::init::parse_config_from_buffer;

        let mut buffer = String::new();
        File::open($path).and_then(|mut f| {
        f.read_to_string(&mut buffer)
        }).unwrap_or_else(|err| panic!("Error while loading config: [{}]", err));
        parse_config_from_buffer::<$type>(&buffer).unwrap_or_else(|err| panic!("Error while parsing config: [{}]", err))
    }};
}
