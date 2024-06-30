// build.rs

use std::io;
use std::process;

fn main() -> io::Result<()> {
    process::Command::new("npx")
        .args([
            "tailwindcss",
            "-i",
            "./dashboard/styles/tailwind.css",
            "-o",
            "./dashboard/assets/main.css",
        ])
        .output()
        .expect("Failed building tailwindcss");

    Ok(())
}
