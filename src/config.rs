use std::{process::Command, fs};



pub fn reload() {
    let cwd_buf = std::env::current_dir().unwrap();
    let cwd = cwd_buf.to_str().unwrap();

    let _ = fs::write("/tmp/config.ts", format!("\
import config from \"{cwd}/config.ts\";\
console.write(JSON.stringify(config))\
")).unwrap();

    let handle = Command::new("bun")
        .arg("run")
        .arg(format!("/tmp/config.ts"))
        .spawn()
        .unwrap();

    let stdout = handle.wait_with_output().unwrap().stdout;
    let data = String::from_utf8(stdout).unwrap();

    println!("{data}");
}


