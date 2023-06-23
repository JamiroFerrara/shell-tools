#[macro_export()]
macro_rules! sh {
    ($var:expr) => {
        let err = format!("Failed to execute {}", $var);
        let output = Command::new("sh").arg("-c").arg($var).output().expect(&err);

        if output.status.success() {
            // Print the stdout if the command executed successfully
            let stdout = String::from_utf8_lossy(&output.stdout);
            println!("{}", stdout);
        } else {
            // Print the stderr if the command failed
            let stderr = String::from_utf8_lossy(&output.stderr);
            println!("Command failed:\n{}", stderr);
        }
    };
}
