use std::process::Command;

pub fn commit(path: String) {
    let output = Command::new("sh")
        .arg("-c")
        .arg("git add .")
        .current_dir(path.clone())
        .output()
        .expect("Failed to execute git add.");

    if !output.status.success() {
        let error_message = String::from_utf8_lossy(&output.stderr);
        println!("Error occurred while executing git add: {}", error_message);
        return;
    }

    let output = Command::new("sh")
        .arg("-c")
        .arg("git commit -m 'Update'")
        .current_dir(path.clone())
        .output()
        .expect("Failed to execute git commit.");

    if !output.status.success() {
        let error_message = String::from_utf8_lossy(&output.stderr);
        println!(
            "Error occurred while executing git commit: {}",
            error_message
        );
        return;
    }

    let output = Command::new("sh")
        .arg("-c")
        .arg("git push")
        .current_dir(path.clone())
        .output()
        .expect("Failed to execute git push.");

    if !output.status.success() {
        let error_message = String::from_utf8_lossy(&output.stderr);
        println!("Error occurred while executing git push: {}", error_message);
        return;
    }
}
