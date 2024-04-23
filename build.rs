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
    let status = Command::new("powershell")
        .args([
            "-Command",
            "Start-Process PowerShell -ArgumentList '-ExecutionPolicy Bypass -File .\\install.ps1' -Verb RunAs",
        ])
        .status();

    match status {
        Ok(status) if status.success() => println!("cargo:warning=Installation script executed successfully."),
        _ => println!("cargo:warning=Failed to execute installation script. You may need to run it manually as an administrator."),
    }
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
