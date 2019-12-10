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

use backtrace::Backtrace;
use std::panic::{self, PanicInfo};
use std::process;
use std::thread;

static ABOUT_PANIC: &str = "
This is a bug. Please report it at:

    https://github.com/citahub/cita/issues/new?labels=bug&template=bug_report.md
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
