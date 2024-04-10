use std::process::Command;

fn main() {
    #[cfg(target_os = "windows")]
    windows_instructions();

    #[cfg(target_os = "unix")]
    unix_instructions();

    #[cfg(target_os = "macos")]
    unix_instructions();
}

fn windows_instructions() {
    println!("cargo:warning=To add the program to your PATH on Windows, copy the executable to a directory that's already in your PATH, or run the following command with administrator privileges:");
    println!("cargo:warning=Set-ExecutionPolicy Bypass -Scope Process -Force; .\\install.ps1");
}

fn unix_instructions() {
    if cfg!(target_os = "macos") || cfg!(target_os = "unix") {
        // Attempt to copy the binary using `sudo`, which might prompt the user for their password.
        match Command::new("sudo")
            .arg("cp")
            .arg("target/release/ultimate_project_manager")
            .arg("/usr/local/bin/upm")
            .status()
        {
            Ok(status) if status.success() => println!("cargo:warning=Successfully copied the binary to /usr/local/bin."),
            _ => println!("cargo:warning=Failed to copy the binary. Please manually copy it to a directory in your PATH."),
        }
    }
}
