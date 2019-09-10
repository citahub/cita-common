// Copyright Cryptape Technologies LLC.
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

use git2::{DescribeFormatOptions, DescribeOptions, ErrorCode, Reference, Repository};
use rustc_version;

const ASCII_LOGO: &str = r#"
                       ..-nnmmmmnn-..
                  .-nndNNNNNNmddddmmmho.
                .smNNMMMNnn-         :nnmhn.
              .dNMMMMMNs--:nosnno-.     ..dNn..   ....
            .hNMMMMMMMNmmNNNMMMMMNmn     ..nMNNmmmmmmmmdnn.
           .mMMMMMMMMMMMMMMMMMMMMMMN:  ..ohmNNNNNNmdhnsoonms
          nNMMMMMMMMMMMMMMMMMMMMMMMMh.                 .-:dN:
         :NMMMMMMMMMMMMMMMMMMMMMMMMMNdnnnhhddmmmNNNNNNNNNNNMn
        .mMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMNNMMMMMMMMMMMMm dMN:
        nNMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMN sMMMMMMMNNNNNNdNMNn
       .dMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMNNMMMMMNdn.--nohmNMmo.
       nNMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMNmhnn       .:smNmhn.
       .NMMMMMMMMMMMMMMMMMMMMMMMMMMMNmhhdddhsnn.             nmMNmn:
       nMMMMMMMMMMMMMMMMMMMMMMMMMMMMh..              .:n:-.   oNddNNn
       oMMMMMMMMMMMMMMMMMMMMMMMMMMMMh.             .:noshmdnndmn  -hm.
       sMMMMMMMMMMMMMMMMMMMMMMMMMMMMNdsn-... .:.         -onsn:    .Nn
       hMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMNNmmdmmNn                osssNn
      .mMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMs   .nnnmNNmdhnnnnssonNn
      oNMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMNsnsdNMMMMMMNhssnhdo.Ns
     nmMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMNnoN.
    .nNMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMNdm
   oNMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMmn
  :NMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMNNdnNMNn
 .mMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMNNds:" nMMNn
 nMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMd:.    nMMMNo.
nNMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMNdo:...sMMMMNo
nNMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMNNmmNMMMMMm
 NMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMd
  :mNNNMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMNNNms
     mMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMm
   ._____. ._____.  _. ._   ._____. ._____.   ._.   ._____. ._____.
   | .___| |___. | | | | |  |___. | |_____|   |_|   |___. | |_____|
   | |     ._. | | | |_| |  ._. | |   ._.   ._____. ._. | | ._____.
   | |     | | |_| \_____/  | | |_/   | |   | ,_, | | | |_/ |_____|
   | |___. | | ._.   ._.    | |       | |   | | | | | |     ._____.
   |_____| |_| |_|   |_|    |_|       |_|   |_| |_| |_|     |_____|
"#;

/// Get the commit ID of this repository
fn get_commit_id(repo: &Repository) -> Option<String> {
    repo.revparse("HEAD")
        .map(|revspec| revspec.from().map(|obj| format!("{}", obj.id())))
        .unwrap_or(None)
}

/// Get the branch name of this repository
fn get_branch(repo: &Repository) -> Option<String> {
    let head = match repo.head() {
        Ok(head) => Some(head),
        Err(ref e) if e.code() == ErrorCode::UnbornBranch || e.code() == ErrorCode::NotFound => {
            None
        }
        Err(_) => return None,
    };
    head.as_ref()
        .and_then(Reference::shorthand)
        .map(ToOwned::to_owned)
}

/// [Command]:
///   * git describe --abbrev=0 --tags
///   * git describe --dirty=-dev
fn get_describe(repo: &Repository, dirty: Option<&str>) -> Option<String> {
    let mut describe_opt = DescribeOptions::new();
    describe_opt.describe_tags();
    repo.describe(&describe_opt)
        .map(|describe| {
            let mut format_opt = DescribeFormatOptions::new();
            if let Some(dirty) = dirty {
                format_opt.dirty_suffix(dirty);
            } else {
                format_opt.abbreviated_size(0);
            }
            describe.format(Some(&format_opt)).ok()
        })
        .ok()
        .unwrap_or(None)
}

fn get_latest_tag(repo: &Repository) -> Option<String> {
    get_describe(repo, None)
}

/// Generate the build info functions (The file will be used by `include!` macro)
pub fn gen_build_info(out_dir: &str, dest_name: &str, version_str: String) {
    let dest_path = Path::new(&out_dir).join(dest_name);
    let mut f = File::create(&dest_path).unwrap();

    let (descr_dirty, tag, branch, commit_id) = match Repository::discover(".") {
        Ok(repo) => (
            get_describe(&repo, Some("-dev")),
            get_latest_tag(&repo),
            get_branch(&repo),
            get_commit_id(&repo),
        ),
        Err(_) => (None, None, None, None),
    };

    let (version, pre, commit_date) = {
        let ver_meta = rustc_version::version_meta().unwrap();
        let ver = &ver_meta.semver;
        let pre = ver.pre.get(0).map(|id| format!("{}", id));
        ((ver.major, ver.minor, ver.patch), pre, ver_meta.commit_date)
    };

    let version_string = {
        let commit_id_string = commit_id.clone().unwrap_or_else(|| "unknown".to_owned());
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
        "{version}\n({rustc})\n{logo}",
        version = version_string,
        rustc = rustc_str,
        logo = ASCII_LOGO,
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

        #[allow(unknown_lints)]
        #[allow(clippy::all)]
        pub fn get_build_info() -> (
           &'static str,          // ASCII Logo
           Option<&'static str>,  // git: describe --dirty=-dev
           Option<&'static str>,  // git: latest tag
           Option<&'static str>,  // git: current branch
           Option<&'static str>,  // git: current commit id
           (u64, u64, u64),       // rustc: version(major, minor, patch)
           Option<&'static str>,  // rustc: pre-release
           Option<&'static str>,  // rustc: commit_date
        ) {{
           ({:?}, {:?}, {:?}, {:?}, {:?}, {:?}, {:?}, {:?})
        }}
    ",
        version_string,
        info_str,
        ASCII_LOGO,
        descr_dirty,
        tag,
        branch,
        commit_id,
        version,
        pre,
        commit_date
    );
    f.write_all(code.as_bytes()).unwrap();
}
