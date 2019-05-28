// CITA
// Copyright 2016-2019 Cryptape Technologies LLC.

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

extern crate backtrace;
#[macro_use]
extern crate cita_logger as logger;

use backtrace::Backtrace;
use std::panic::{self, PanicInfo};
use std::process;
use std::thread;

static ABOUT_PANIC: &str = "
This is a bug. Please report it at:

    https://github.com/cryptape/cita/issues/new?labels=bug&template=bug_report.md
";

/// Set the panic hook
pub fn set_panic_handler() {
    panic::set_hook(Box::new(panic_hook));
}

fn panic_hook(info: &PanicInfo) {
    let location = info.location();
    let file = location.as_ref().map(|l| l.file()).unwrap_or("<unknown>");
    let line = location.as_ref().map(|l| l.line()).unwrap_or(0);
    let msg = match info.payload().downcast_ref::<&'static str>() {
        Some(s) => *s,
        None => match info.payload().downcast_ref::<String>() {
            Some(s) => &s[..],
            None => "Box<Any>",
        },
    };
    let thread = thread::current();
    let name = thread.name().unwrap_or("<unnamed>");
    let backtrace = Backtrace::new();
    let error = format!(
        "\n============================\n\
         {:?}\n\n\
         position:\n\
         Thread {} panicked at {}, {}:{}\n\
         {}\n\
         ============================\n\
         ",
        backtrace, name, msg, file, line, ABOUT_PANIC
    );
    error!("{}", error);
    process::exit(1);
}
