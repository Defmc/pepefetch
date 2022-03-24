use std::fs;
use std::process::{Command, Stdio};
use systemstat::{platform::PlatformImpl, Platform};

pub fn hostname() -> String {
    let hostname = fs::read_to_string("/etc/hostname");
    let mut s = hostname.unwrap();
    s.truncate(s.len() - 1);
    s
}

pub fn kernel() -> String {
    let kernel = Command::new("/bin/uname")
        .arg("-r")
        .stdout(Stdio::piped())
        .output()
        .unwrap();
    let mut stdout = String::from_utf8(kernel.stdout).unwrap();
    stdout.truncate(stdout.len() - 1);
    stdout
}

pub fn time(sys: &PlatformImpl) -> (u64, u64) {
    match sys.uptime() {
        Ok(uptime) => (uptime.as_secs() / 3600, uptime.as_secs() % 3600 / 60),
        Err(_) => (0, 0),
    }
}

pub fn mem(sys: &PlatformImpl) -> (u64, u64) {
    match sys.memory() {
        Ok(mem) => (
            (mem.total.as_u64() - mem.free.as_u64()) / 1024 / 1024,
            mem.total.as_u64() / 1024 / 1024,
        ),
        Err(_) => (0, 0),
    }
}
