use std::fs::File;
use std::io::Write;
use std::path::Path;

use git2::{DescribeFormatOptions, DescribeOptions, ErrorCode, Repository};
use rustc_version;

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
        Err(ref e) if e.code() == ErrorCode::UnbornBranch || e.code() == ErrorCode::NotFound => None,
        Err(_) => return None,
    };
    head.as_ref()
        .and_then(|h| h.shorthand())
        .map(|v| v.to_owned())
}

/// [Command]: git describe --abbrev=0 --tags
fn get_latest_tag(repo: &Repository) -> Option<String> {
    let mut describe_opt = DescribeOptions::new();
    describe_opt.describe_tags();
    repo.describe(&describe_opt)
        .map(|describe| {
            let mut format_opt = DescribeFormatOptions::new();
            format_opt.abbreviated_size(0);
            describe.format(Some(&format_opt)).ok()
        })
        .ok()
        .unwrap_or(None)
}

/// Generate the build info functions (The file will be used by `include!` macro)
pub fn gen_build_info(out_dir: &str, dest_name: &str) {
    let dest_path = Path::new(&out_dir).join(dest_name);
    let mut f = File::create(&dest_path).unwrap();

    let (tag, branch, commit_id) = match Repository::discover(".") {
        Ok(repo) => (
            get_latest_tag(&repo),
            get_branch(&repo),
            get_commit_id(&repo),
        ),
        Err(_) => (None, None, None),
    };

    let (version, pre, commit_date) = {
        let ver_meta = rustc_version::version_meta().unwrap();
        let ver = &ver_meta.semver;
        let pre = ver.pre.get(0).map(|id| format!("{}", id));
        ((ver.major, ver.minor, ver.patch), pre, ver_meta.commit_date)
    };

    let info_str = format!(
        "{}-{:.7} (rustc {}.{}.{}-{}-{})",
        branch.as_ref().map(|x| &**x).unwrap_or("unknown"),
        commit_id.as_ref().map(|x| &**x).unwrap_or("unknown"),
        version.0,
        version.1,
        version.2,
        pre.as_ref().map(|x| &**x).unwrap_or("unknown"),
        commit_date.as_ref().map(|x| &**x).unwrap_or("unknown"),
    ).replace("\"", "\\\"");
    let code = format!(
        "
        pub fn get_build_info_str() -> &'static str {{
            \"{}\"
        }}

        pub fn get_build_info() -> (
           Option<&'static str>,  // git: tag
           Option<&'static str>,  // git: branch
           Option<&'static str>,  // git: commit id
           (u64, u64, u64),       // rustc: version(major, minor, patch)
           Option<&'static str>,  // rustc: pre-release
           Option<&'static str>,  // rustc: commit_date
        ) {{
           ({:?}, {:?}, {:?}, {:?}, {:?}, {:?})
        }}
    ",
        info_str, tag, branch, commit_id, version, pre, commit_date
    );
    f.write_all(code.as_bytes()).unwrap();
}
