use ansi_term::Colour::Red;
use pepefetch::{ascii, user};
use std::env;
use std::io::{self, BufWriter, Write};
use systemstat::{Platform, System};

fn format_env(msg: &str, info: String) -> String {
    format!(" {}: {}", Red.bold().paint(msg), info)
}

fn format_login() -> String {
    format!(
        " {}@{}",
        Red.bold().paint(get_env_var("USER")),
        Red.bold().paint(user::hostname())
    )
}

fn get_env_var(var: &str) -> String {
    env::var(var).unwrap_or_else(|_| format!("Variable {} not defined", var))
}

fn collect_info() -> Vec<String> {
    let ip = local_ipaddress::get().unwrap_or_else(|| "Not connected".to_owned());
    let sys = System::new();
    let (used_mem, total_mem) = user::mem(&sys);
    let (up_hours, up_mins) = user::time(&sys);

    vec![
        format_env("DE", get_env_var("DESKTOP_SESSION")),
        format_env("Term", get_env_var("TERM")),
        format_env("Local Ipv4", ip),
        format_env("Uptime", format!("{} hours, {} mins", up_hours, up_mins)),
        format_env("Kernel", user::kernel()),
        format_env("Memory", format!("{}MiB / {}MiB", used_mem, total_mem)),
    ]
}

fn print_fetch(source: impl Iterator<Item = String>) {
    let stdout = io::stdout().lock();
    let mut writer = BufWriter::new(stdout);
    source.for_each(|line| writeln!(writer, "{}", line).unwrap());
}

fn main() {
    let hide_info = env::args().any(|arg| arg == "--hide" || arg == "-h");

    let img_idx = rand::random::<usize>() % ascii::IMAGES.len();
    let mut ascii_img: Vec<String> = ascii::IMAGES[img_idx]
        .split('\n')
        .map(String::from)
        .collect();

    if !hide_info {
        ascii_img[1].push_str(&format_login());
        ascii_img
            .iter_mut()
            .skip(3)
            .zip(collect_info())
            .for_each(|(line, info)| line.push_str(&info));
    }

    print_fetch(ascii_img.into_iter())
}
