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

use std::fs::File;
use std::io::Write;
use std::path::Path;

use git2::Repository;
use rustc_version;

/// Get the commit ID of this repository
fn get_commit_id(repo: &Repository) -> Option<String> {
    repo.revparse("HEAD")
        .map(|revspec| revspec.from().map(|obj| format!("{}", obj.id())))
        .unwrap_or(None)
}

/// Generate the build info functions (The file will be used by `include!` macro)
pub fn gen_build_info(out_dir: &str, dest_name: &str, version_str: String) {
    let dest_path = Path::new(&out_dir).join(dest_name);
    let mut f = File::create(&dest_path).unwrap();

    let commit_id = match Repository::discover(".") {
        Ok(repo) => get_commit_id(&repo),
        Err(_) => None,
    };

    let (version, pre, commit_date) = {
        let ver_meta = rustc_version::version_meta().unwrap();
        let ver = &ver_meta.semver;
        let pre = ver.pre.get(0).map(|id| format!("{}", id));
        ((ver.major, ver.minor, ver.patch), pre, ver_meta.commit_date)
    };

    let version_string = {
        let commit_id_string = commit_id.unwrap_or_else(|| "unknown".to_owned());
        format!("{}-{:.8}", version_str, commit_id_string)
    };
    let pre_str = pre.as_ref().map(|x| &**x).unwrap_or("unknown");
    let commit_date_str = commit_date.as_ref().map(|x| &**x).unwrap_or("unknown");
    let rustc_str = format!(
        "rustc {major}.{minor}.{patch}-{pre}-{commit_date}",
        major = version.0,
        minor = version.1,
        patch = version.2,
        pre = pre_str,
        commit_date = commit_date_str,
    );
    let info_str = format!(
        "{version}\n({rustc})",
        version = version_string,
        rustc = rustc_str,
    )
    .replace("\\", "\\\\")
    .replace("\"", "\\\"")
    .replace("\n", "\\n");
    let code = format!(
        "
        #[allow(unknown_lints)]
        #[allow(clippy::all)]
        pub fn get_build_info_str(short: bool) -> &'static str {{
           if short {{ \"{}\" }} else {{ \"{}\" }}
        }}
    ",
        version_string, info_str
    );
    f.write_all(code.as_bytes()).unwrap();
}
