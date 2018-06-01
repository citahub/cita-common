// CITA
// Copyright 2016-2017 Cryptape Technologies LLC.

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

#![feature(use_extern_macros)]

extern crate chan_signal;
extern crate chrono;
extern crate log4rs;
extern crate log;

pub use log::{debug, error, info, log, log_enabled, trace, warn};

use chan_signal::Signal;
use chrono::Local;
use log::LevelFilter;
use log4rs::append::console::ConsoleAppender;
use log4rs::append::file::FileAppender;
use log4rs::config::{Appender, Config, Logger, Root};
use log4rs::encode::pattern::PatternEncoder;
use std::env;
use std::fs;
use std::str::FromStr;
use std::sync::{Once, ONCE_INIT};
use std::thread;
use std::vec::Vec;

#[derive(Debug, Clone)]
struct Directive {
    // module name
    name: String,
    // log level
    level: LevelFilter,
}

static INIT_LOG: Once = ONCE_INIT;

pub fn init_config(service_name: &str) {
    INIT_LOG.call_once(|| {
        // parse RUST_LOG
        let directives: Vec<Directive> = match env::var("RUST_LOG") {
            Ok(s) => parse_env(&s),
            Err(_) => Vec::new(),
        };

        // log4rs config
        let log_name = format!("logs/{}.log", service_name);
        let directives_clone = directives.clone();
        let config = config_file_appender(&log_name, directives_clone);
        let handle = log4rs::init_config(config).unwrap();

        // log rotate via signal(USR1)
        let signal = chan_signal::notify(&[Signal::USR1]);

        // Any and all threads spawned must come after the first call to notify (or notify_on).
        // This is so all spawned threads inherit the blocked status of signals.
        // If a thread starts before notify is called, it will not have the correct signal mask.
        // When a signal is delivered, the result is indeterminate.
        let service_name_clone = service_name.to_string();
        thread::spawn(move || {
            loop {
                //Blocks until this process is sent an USR1 signal.
                signal.recv().unwrap();

                //rotate current log file
                let time_stamp = Local::now().format("_%Y-%m-%d_%H-%M-%S");
                let log_rotate_name = format!("logs/{}{}.log", &service_name_clone, time_stamp);
                if let Err(e) = fs::rename(&log_name, log_rotate_name) {
                    warn!("logrotate failed because of {:?}", e.kind());
                    continue;
                }

                // reconfig
                let directives_clone = directives.clone();
                let new_config = config_file_appender(&log_name, directives_clone);
                handle.set_config(new_config);
            }
        });
    });
}

// use in tests
pub fn init() {
    INIT_LOG.call_once(|| {
        // parse RUST_LOG
        let directives: Vec<Directive> = match env::var("RUST_LOG") {
            Ok(s) => parse_env(&s),
            Err(_) => Vec::new(),
        };
        let config = config_console_appender(directives);
        log4rs::init_config(config).unwrap();
    });
}

// use in unit case
pub fn silent() {
    INIT_LOG.call_once(|| {
        let config = Config::builder()
            .build(Root::builder().build(LevelFilter::Off))
            .unwrap();
        log4rs::init_config(config).unwrap();
    });
}

// simple parse env (e.g: crate1,crate2::mod=debug,crate3::mod=trace)
fn parse_env(env: &str) -> Vec<Directive> {
    let mut directives = Vec::new();

    for s in env.split(',') {
        if s.len() == 0 {
            continue;
        }
        let mut parts = s.split('=');
        let (log_level, name) = match (parts.next(), parts.next().map(|s| s.trim()), parts.next()) {
            (Some(part0), None, None) => match LevelFilter::from_str(part0) {
                Ok(num) => {
                    println!(
                        "warning: log level '{}' need explicit crate or module name.",
                        num
                    );
                    continue;
                }
                Err(_) => (LevelFilter::Info, part0),
            },
            (Some(part0), Some(""), None) => (LevelFilter::Info, part0),
            (Some(part0), Some(part1), None) => match LevelFilter::from_str(part1) {
                Ok(num) => (num, part0),
                _ => {
                    println!(
                        "warning: invalid logging spec '{}', \
                         ignoring it",
                        part1
                    );
                    continue;
                }
            },
            _ => {
                println!(
                    "warning: invalid logging spec '{}', \
                     ignoring it",
                    s
                );
                continue;
            }
        };

        if name.len() != 0 {
            directives.push(Directive {
                name: name.to_string(),
                level: log_level,
            });
        }
    }

    directives
}

fn create_loggers(directives: Vec<Directive>, appender: String) -> Vec<Logger> {
    let mut loggers = Vec::new();

    if directives.is_empty() {
        return loggers;
    }

    //creat loggers via module/crate and log level
    for directive in directives {
        let appender_clone = appender.clone();
        let logger = Logger::builder()
            .appender(appender_clone)
            .additive(false)
            .build(directive.name, directive.level);
        loggers.push(logger);
    }

    loggers
}

// FileAppender config
fn config_file_appender(file_path: &str, directives: Vec<Directive>) -> Config {
    let requests = FileAppender::builder()
        .encoder(Box::new(PatternEncoder::new("{d} - {l} - {m}{n}")))
        .build(file_path)
        .unwrap();

    let mut config_builder = Config::builder().appender(Appender::builder().build("requests", Box::new(requests)));

    let loggers = create_loggers(directives, "requests".to_string());

    // config crate or module log level
    if loggers.len() != 0 {
        config_builder = config_builder.loggers(loggers.into_iter());
    }

    //config global log level
    let config = config_builder
        .build(
            Root::builder()
                .appender("requests")
                .build(LevelFilter::Info),
        )
        .unwrap();

    config
}

// ConsoleAppender config
fn config_console_appender(directives: Vec<Directive>) -> Config {
    let stdout = ConsoleAppender::builder()
        .encoder(Box::new(PatternEncoder::new("{d} - {l} - {m}{n}")))
        .build();

    let mut config_builder = Config::builder().appender(Appender::builder().build("stdout", Box::new(stdout)));

    let loggers = create_loggers(directives, "stdout".to_string());

    // config crate or module log level
    if loggers.len() != 0 {
        config_builder = config_builder.loggers(loggers.into_iter());
    }

    //config global log level
    let config = config_builder
        .build(Root::builder().appender("stdout").build(LevelFilter::Info))
        .unwrap();

    config
}

#[cfg(test)]
mod tests {

    use super::parse_env;
    use log::LevelFilter;

    #[test]
    fn parse_env_valid() {
        let directives = parse_env("crate1::mod1,crate1::mod2=debug,crate2=trace");
        assert_eq!(directives.len(), 3);
        assert_eq!(directives[0].name, "crate1::mod1".to_string());
        assert_eq!(directives[0].level, LevelFilter::Info);

        assert_eq!(directives[1].name, "crate1::mod2".to_string());
        assert_eq!(directives[1].level, LevelFilter::Debug);

        assert_eq!(directives[2].name, "crate2".to_string());
        assert_eq!(directives[2].level, LevelFilter::Trace);
    }

    #[test]
    fn parse_env_invalid_crate() {
        let directives = parse_env("crate1::mod=warn=info,crate2=warn");
        assert_eq!(directives.len(), 1);
        assert_eq!(directives[0].name, "crate2".to_string());
        assert_eq!(directives[0].level, LevelFilter::Warn);
    }

    #[test]
    fn parse_env_invalid_level() {
        let directives = parse_env("crate1::mod=wrong,crate2=error");
        assert_eq!(directives.len(), 1);
        assert_eq!(directives[0].name, "crate2".to_string());
        assert_eq!(directives[0].level, LevelFilter::Error);
    }

    #[test]
    fn parse_env_empty() {
        let directives = parse_env("crate1::mod=,=trace");
        assert_eq!(directives.len(), 1);
        assert_eq!(directives[0].name, "crate1::mod".to_string());
        assert_eq!(directives[0].level, LevelFilter::Info);
    }

}
