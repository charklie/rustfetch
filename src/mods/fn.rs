use colored::Colorize;
use libmacchina::{traits::MemoryReadout as _, MemoryReadout};
use std::{env, fs::File, io::Read, process::Command, process::Output};

pub fn help() {
    println!("{}", "Rsftch".bold());
    println!("A lightning fast hardware fetch written in rust,");
    println!("{}", "Written by charklie.".italic());
    println!("\nUsage: rsftch [OPTION...]\n");
    println!("  -h,  --help       Bring up this menu");
    println!("  -o,  --override   Override distribution, changes ASCII. (not implemented yet)");
    println!("  -nc, --no-color   Removes all colors and formatting.");
    println!("  -t,  --tree       Enables tree mode.");
}

pub fn whoami() -> String {
    let output = Command::new("whoami").output().expect("whoami failed");
    String::from_utf8_lossy(&output.stdout).trim().to_string()
}

pub fn get_os_release_pretty_name() -> Option<String> {
    let output: Output = match Command::new("bash")
        .arg("-c")
        .arg("awk -F'=' '/^ID=/ {print tolower($2)}' /etc/*-release 2> /dev/null")
        .output()
    {
        Ok(output) => output,
        Err(_) => return None,
    };

    if output.status.success() {
        let stdout = String::from_utf8(output.stdout).ok()?;
        Some(stdout.trim().to_string())
    } else {
        None
    }
}

pub fn format_bytes(kbytes: u64) -> String {
    const MIB: u64 = 1048576;
    format!("{:.2} GiB", kbytes as f64 / MIB as f64)
}

pub fn get_wm() -> String {
    if env::var("DISPLAY").is_err() {
        return String::new();
    }

    for env_var in &[
        "XDG_SESSION_DESKTOP",
        "XDG_CURRENT_DESKTOP",
        "DESKTOP_SESSION",
    ] {
        if let Ok(de) = env::var(env_var) {
            return de;
        }
    }

    let path = format!("{}/.xinitrc", env::var("HOME").unwrap_or_default());
    if let Ok(mut file) = File::open(&path) {
        let mut buf = String::new();
        if file.read_to_string(&mut buf).is_ok() {
            if let Some(last_line) = buf.lines().last() {
                let last_word = last_line.split(' ').last().unwrap_or("");
                return last_word.to_string();
            }
        }
    }
    String::new()
}

pub fn get_mem() -> String {
    let mem_readout = MemoryReadout::new();
    let total_mem = mem_readout.total().unwrap_or(0);
    let used_mem = mem_readout.used().unwrap_or(0);

    let total_mem_str = format_bytes(total_mem);
    let used_mem_str = format_bytes(used_mem);

    format!("{} / {}", used_mem_str, total_mem_str)
}

pub fn uname_r() -> String {
    let output = Command::new("uname")
        .arg("-r")
        .output()
        .expect("uname failed -r");
    String::from_utf8_lossy(&output.stdout).trim().to_string()
}

pub fn uname_s() -> String {
    let output = Command::new("uname")
        .arg("-s")
        .output()
        .expect("uname failed -s");
    String::from_utf8_lossy(&output.stdout).trim().to_string()
}

pub fn uname_n() -> String {
    let output = Command::new("uname")
        .arg("-n")
        .output()
        .expect("uname failed -n");
    String::from_utf8_lossy(&output.stdout).trim().to_string()
}

pub fn shell_name() -> String {
    let shell = env::var("SHELL").expect("SHELL not set");
    let parts: Vec<&str> = shell.split('/').collect();
    parts.last().unwrap().to_string()
}

pub fn get_terminal() -> String {
    let term = env::var("TERM").unwrap_or("".to_string());
    return term;
}