// build.rs
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
    println!("cargo:warning=To add the program to your PATH on Unix, run the following commands:");
    println!("cargo:warning=sudo cp target/release/ultimate_project_manager /usr/local/bin/upm");
}
