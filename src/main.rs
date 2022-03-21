use ansi_term::Colour::Red;
use std::{env, fs};
use systemstat::{Platform, System};

fn hostname() -> String {
    let hostname = fs::read_to_string("/etc/hostname");
    let mut s = hostname.unwrap();
    s.truncate(s.len() - 1);
    s
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
    println!("⣿⣿⣿⣿⣿⣿⠿⢋⣥⣴⣶⣶⣶⣬⣙⠻⠟⣋⣭⣭⣭⣭⡙⠻⣿⣿⣿⣿⣿");
    println!(
        "⣿⣿⣿⣿⡿⢋⣴⣿⣿⠿⢟⣛⣛⣛⠿⢷⡹⣿⣿⣿⣿⣿⣿⣆⠹⣿⣿⣿⣿ {}@{}",
        Red.bold()
            .paint(env::var("USER").unwrap_or("Variable USER not defined.".to_owned())),
        Red.bold().paint(hostname())
    );
    println!("⣿⣿⣿⡿⢁⣾⣿⣿⣴⣿⣿⣿⣿⠿⠿⠷⠥⠱⣶⣶⣶⣶⡶⠮⠤⣌⡙⢿⣿");
    println!(
        "⣿⡿⢛⡁⣾⣿⣿⣿⡿⢟⡫⢕⣪⡭⠥⢭⣭⣉⡂⣉⡒⣤⡭⡉⠩⣥⣰⠂⠹ {}: {}",
        Red.bold().paint("DE"),
        env::var("DESKTOP_SESSION").unwrap_or("Variable DESKTOP_SESSION not defined.".to_owned())
    );
    println!(
        "⡟⢠⣿⣱⣿⣿⣿⣏⣛⢲⣾⣿⠃⠄⠐⠈⣿⣿⣿⣿⣿⣿⠄⠁⠃⢸⣿⣿⡧ {}: {}",
        Red.bold().paint("Term"),
        env::var("TERM").unwrap_or("Variable TERM not defined.".to_owned())
    );
    println!(
        "⢠⣿⣿⣿⣿⣿⣿⣿⣿⣇⣊⠙⠳⠤⠤⠾⣟⠛⠍⣹⣛⣛⣢⣀⣠⣛⡯⢉⣰ {}: {}",
        Red.bold().paint("Editor"),
        env::var("EDITOR").unwrap_or("Variable EDITOR not defined.".to_owned())
    );
    println!(
        "⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣷⡶⠶⢒⣠⣼⣿⣿⣛⠻⠛⢛⣛⠉⣴⣿⣿ {}: {}",
        Red.bold().paint("Actual Path"),
        env::var("PWD").unwrap_or("Variable PWD not defined.".to_owned())
    );
    println!(
        "⣿⣿⣿⣿⣿⣿⣿⡿⢛⡛⢿⣿⣿⣶⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣷⡈⢿⣿ {}: {}MiB / {}MiB",
        Red.bold().paint("Memory"),
        used,
        total
    );
    println!("⣿⣿⣿⣿⣿⣿⣿⠸⣿⡻⢷⣍⣛⠻⠿⠿⣿⣿⣿⣿⣿⣿⣿⣿⣿⠿⢇⡘⣿");
    println!("⣿⣿⣿⣿⣿⣿⣿⣷⣝⠻⠶⣬⣍⣛⣛⠓⠶⠶⠶⠤⠬⠭⠤⠶⠶⠞⠛⣡⣿");
    println!("⢿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣷⣶⣬⣭⣍⣙⣛⣛⣛⠛⠛⠛⠿⠿⠿⠛⣠⣿⣿");
    println!("⣦⣈⠉⢛⠻⠿⠿⢿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⡿⠿⠛⣁⣴⣾⣿⣿⣿⣿");
    println!("⣿⣿⣿⣶⣮⣭⣁⣒⣒⣒⠂⠠⠬⠭⠭⠭⢀⣀⣠⣄⡘⠿⣿⣿⣿⣿⣿⣿⣿");
    println!("⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣦⡈⢿⣿⣿⣿⣿⣿");
}
