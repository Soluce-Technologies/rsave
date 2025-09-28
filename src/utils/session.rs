use chrono::{DateTime, Duration, Utc};
use std::env;
use std::fs;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::PathBuf;
use std::process::Command;

pub fn session_file() -> PathBuf {
    dirs::home_dir().unwrap().join(".rsave/session")
}

pub fn load_session() -> Option<String> {
    let path = session_file();
    if !path.exists() {
        return None;
    }

    let data = fs::read_to_string(&path).ok()?;
    let parts: Vec<&str> = data.split("::").collect();
    if parts.len() != 2 {
        return None;
    }
    let timestamp: DateTime<Utc> = parts[0].parse().ok()?;
    if Utc::now() - timestamp > Duration::minutes(10) {
        return None;
    }
    let password_enc = parts[1];
    Some(password_enc.to_string())
}

pub fn save_session(password: &str) {
    let path = session_file();
    let timestamp = Utc::now();
    let data = format!("{}::{}", timestamp, password); // encrypt password if needed
    fs::write(path, data).unwrap();
}

/// The "beacon" cleaner
pub(crate) fn session_cleaner() {
    let log_file = dirs::home_dir().unwrap().join(".rsave/session_cleaner.log");

    loop {
        std::thread::sleep(std::time::Duration::from_secs(60));

        let path = session_file();
        let log_message = if load_session().is_none() && path.exists() {
            let _ = fs::remove_file(&path);
            format!("Deleted expired session at {:?}\n", Utc::now())
        } else {
            format!("Checked session at {:?}\n", Utc::now())
        };

        if let Ok(mut file) = OpenOptions::new().create(true).append(true).open(&log_file) {
            let _ = file.write_all(log_message.as_bytes());
        }
    }
}

fn is_beacon_running() -> bool {
    let pid_file = dirs::home_dir().unwrap().join(".rsave/beacon.pid");
    if let Ok(pid_str) = std::fs::read_to_string(&pid_file) {
        if let Ok(pid) = pid_str.trim().parse::<i32>() {
            return nix::sys::signal::kill(nix::unistd::Pid::from_raw(pid), None).is_ok();
        }
    }
    false
}

pub(crate) fn spawn_beacon() {
    if is_beacon_running() {
        return;
    }
    let exe = env::current_exe().unwrap();
    #[cfg(unix)]
    unsafe {
        use std::os::unix::process::CommandExt;

        let child = Command::new(exe)
            .arg("--beacon")
            .pre_exec(|| {
                libc::setsid(); // detach from terminal
                Ok(())
            })
            .spawn()
            .expect("Failed to spawn beacon");

        let pid_file = dirs::home_dir().unwrap().join(".rsave/beacon.pid");
        fs::write(pid_file, child.id().to_string()).expect("Failed to write PID file");
    }
}
