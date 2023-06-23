use shell_tools::sh;
use std::process::Command;

pub fn commit(path: String) {
    sh!("git add .");
    sh!("git commit -m 'Update'");
    sh!("git push");
}
