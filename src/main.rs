use ansi_term::Colour::Red;
use systemstat::{Platform, System};
use std::{
    env, fs,
    process::{Command, Stdio},
};

fn hostname() -> String {
    let hostname = fs::read_to_string("/etc/hostname");
    let mut s = hostname.unwrap();
    s.truncate(s.len() - 1);
    s
}

fn kernel() -> String {
    let kernel = Command::new("/bin/uname")
        .arg("-r")
        .stdout(Stdio::piped())
        .output()
        .unwrap();
    let mut stdout = String::from_utf8(kernel.stdout).unwrap();
    stdout.truncate(stdout.len() - 1);
    stdout
}

fn format_env(hide_infos: bool, msg: &str, info: String) -> String {
    if hide_infos { return String::new() }
    format!(" {}: {info}", Red.bold().paint(msg), info=info)
}

fn format_login(hide_infos: bool, usr: &str, host: &str) -> String {
    if hide_infos { return String::new() }
    format!(" {}@{}",
            Red.bold().paint(usr),
            Red.bold().paint(host)
    )
}

fn main() {
    let sys = System::new();
    let (used, total): (u64, u64) = match sys.memory() {
        Ok(mem) => (
            (mem.total.as_u64() - mem.free.as_u64()) / 1024 / 1024,
            mem.total.as_u64() / 1024 / 1024,
        ),
        Err(_) => (0, 0),
    };

    let hide_infos = env::args()
        .any(|arg| arg == "--hide" || arg == "-h");

    let ip = local_ipaddress::get()
        .unwrap_or_else(|| "Not connected".to_owned());

    let (up_hours, up_minutes): (u64, u64) = match sys.uptime() {
        Ok(uptime) => (uptime.as_secs() / 3600, uptime.as_secs() / 60),
        Err(_) => (0, 0),
    };
    let get_env_var = |var: &str| env::var(var)
        .unwrap_or_else(|_| format!("Variable {var} not defined", var=var));

    let mut ascii_img: Vec<String> = include!("../assets/ascii").split('\n')
        .map(|s| s.to_owned())
        .collect();

    let hi = hide_infos;

    ascii_img[1].push_str(&format_login(hi, &get_env_var("USER"), &hostname()));
    let infos = vec![
        format_env(hi, "DE", get_env_var("DESKTOP_SESSION")),
        format_env(hi, "Term", get_env_var("TERM")),
        format_env(hi, "Local Ipv4", ip),
        format_env(hi, "Uptime", format!("{up_hours} hours, {up_minutes} mins", up_hours=up_hours, up_minutes=up_minutes)),
        format_env(hi, "Kernel", kernel()),
        format_env(hi, "Memory", format!("{used}MiB / {total}MiB", used=used, total=total)),
    ];

    ascii_img.iter_mut()
        .skip(3)
        .zip(infos)
        .for_each(|(line, info)| line.push_str(&info));

    ascii_img.iter()
        .for_each(|line| println!("{line}", line=line));
}
