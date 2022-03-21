use ansi_term::Colour::{Blue, Green, Red};
use local_ipaddress;
use std::{
    env, fs,
    process::{Command, Stdio},
};
use systemstat::{Platform, System};
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

fn main() {
    kernel();
    let sys = System::new();
    let (used, total): (u64, u64) = match sys.memory() {
        Ok(mem) => (
            (mem.total.as_u64() - mem.free.as_u64()) / 1024 / 1024,
            mem.total.as_u64() / 1024 / 1024,
        ),
        Err(_) => (0, 0),
    };
    let ip = match local_ipaddress::get() {
        Some(foo) => foo,
        None => "Not connected".to_owned(),
    };
    let (up_hours, up_minutes): (u64, u64) = match sys.uptime() {
        Ok(uptime) => (uptime.as_secs() / 3600, uptime.as_secs() / 600),
        Err(_) => (0, 0),
    };
    println!("{}", Green.bold().paint("⣿⣿⣿⣿⣿⣿⠿⢋⣥⣴⣶⣶⣶⣬⣙⠻⠟⣋⣭⣭⣭⣭⡙⠻⣿⣿⣿⣿⣿"));
    println!(
        "{} {}@{}",
        Green.bold().paint("⣿⣿⣿⣿⡿⢋⣴⣿⣿⠿⢟⣛⣛⣛⠿⢷⡹⣿⣿⣿⣿⣿⣿⣆⠹⣿⣿⣿⣿"),
        Red.bold()
            .paint(env::var("USER").unwrap_or("Variable USER not defined.".to_owned())),
        Red.bold().paint(hostname())
    );
    println!("{}", Green.bold().paint("⣿⣿⣿⡿⢁⣾⣿⣿⣴⣿⣿⣿⣿⠿⠿⠷⠥⠱⣶⣶⣶⣶⡶⠮⠤⣌⡙⢿⣿"));
    println!(
        "{} {}: {}",
        Green.bold().paint("⣿⡿⢛⡁⣾⣿⣿⣿⡿⢟⡫⢕⣪⡭⠥⢭⣭⣉⡂⣉⡒⣤⡭⡉⠩⣥⣰⠂⠹"),
        Red.bold().paint("DE"),
        env::var("DESKTOP_SESSION").unwrap_or("Variable DESKTOP_SESSION not defined.".to_owned())
    );
    println!(
        "{} {}: {}",
        Green.bold().paint("⡟⢠⣿⣱⣿⣿⣿⣏⣛⢲⣾⣿⠃⠄⠐⠈⣿⣿⣿⣿⣿⣿⠄⠁⠃⢸⣿⣿⡧"),
        Red.bold().paint("Term"),
        env::var("TERM").unwrap_or("Variable TERM not defined.".to_owned())
    );
    println!(
        "{} {}: {}",
        Green.bold().paint("⢠⣿⣿⣿⣿⣿⣿⣿⣿⣇⣊⠙⠳⠤⠤⠾⣟⠛⠍⣹⣛⣛⣢⣀⣠⣛⡯⢉⣰"),
        Red.bold().paint("Local Ipv4"),
        ip
    );
    println!(
        "{} {}: {} hours, {} mins",
        Green.bold().paint("⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣷⡶⠶⢒⣠⣼⣿⣿⣛⠻⠛⢛⣛⠉⣴⣿⣿"),
        Red.bold().paint("Uptime"),
        up_hours,
        up_minutes
    );
    println!(
        "{} {}: {}",
        Green.bold().paint("⣿⣿⣿⣿⣿⣿⣿⡿⢛⡛⢿⣿⣿⣶⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣷⡈⢿⣿"),
        Red.bold().paint("Kernel"),
        kernel()
    );
    println!(
        "{} {}: {}MiB / {}MiB",
        Green.bold().paint("⣿⣿⣿⣿⣿⣿⣿⠸⣿⡻⢷⣍⣛⠻⠿⠿⣿⣿⣿⣿⣿⣿⣿⣿⣿⠿⢇⡘⣿"),
        Red.bold().paint("Memory"),
        used,
        total
    );
    println!("{}", Green.bold().paint("⣿⣿⣿⣿⣿⣿⣿⣷⣝⠻⠶⣬⣍⣛⣛⠓⠶⠶⠶⠤⠬⠭⠤⠶⠶⠞⠛⣡⣿"));
    println!("{}", Green.bold().paint("⢿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣷⣶⣬⣭⣍⣙⣛⣛⣛⠛⠛⠛⠿⠿⠿⠛⣠⣿⣿"));
    println!("{}", Green.bold().paint("⣦⣈⠉⢛⠻⠿⠿⢿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⡿⠿⠛⣁⣴⣾⣿⣿⣿⣿"));
    println!(
        "{}{}",
        Blue.bold().paint("⣿⣿⣿⣶⣮⣭⣁⣒⣒⣒⠂⠠⠬⠭⠭⠭⢀⣀⣠⣄"),
        Green.bold().paint("⣿⡘⠿⣿⣿⣿⣿⣿⣿")
    );
    println!(
        "{}{}",
        Blue.bold().paint("⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣦⡈"),
        Green.bold().paint("⢿⣿⣿⣿⣿⣿")
    );
}
