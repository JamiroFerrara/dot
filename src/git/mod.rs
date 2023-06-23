use shell_tools::sh;
use std::process::Command;

pub fn commit(path: String) {
    sh!("git add .", path);
    sh!("git commit -m 'Update'", path);
    sh!("git push", path);
}
