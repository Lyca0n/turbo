use std::path::Path;
use std::process::Command;

pub fn git_clone(parentdir: &Path, url: &str) -> bool {
    let urlstr = url;
    println!("cloning {:?}", &urlstr);
    run("git", &["clone", urlstr, parentdir.to_str().unwrap()])
}
pub fn run(exec: &str, args: &[&str]) -> bool {
    let mut cmd = Command::new(exec);
    cmd.args(args);
    let cmdstr = format!("{:?}", &cmd);
    println!("running {}", &cmdstr);
    let status = cmd.status().unwrap();
    println!("exit status {:?}", &status);
    if status.success() {
        true
    } else {
        false
    }
}
