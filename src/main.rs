use sysinfo::System;
use owo_colors::OwoColorize;
use clap::Parser;

mod banners;
mod get_gpu;
mod get_disk_usage;
mod get_local_ip;
mod config;        // neu für TOML Config

// cli arguments e.g. ferrofetch --banner green
#[derive(Parser)]
#[command(name = "ferrofetch", about = "A system fetch tool written in Rust")]
struct Args {
    #[arg(short, long)]
    banner: Option<String>,

    #[arg(short, long)]
    color: Option<String>,

    #[arg(short, long)]
    list: bool,

    #[arg(long)]
    banner_path: Option<String>
}

// get system informations
fn get_infos() -> Vec<String> {
    let mut system = System::new_all();
    system.refresh_all();

    let username  = whoami::realname().unwrap_or_else(|_| "Unknown".to_string());
    let hostname  = whoami::hostname().unwrap_or_else(|_| "Unknown".to_string());
    let os_name   = System::name().unwrap_or("Unknown".to_string());
    let kernel    = System::kernel_version().unwrap_or("Unknown".to_string());
    let cpu_name  = system.cpus()[0].brand().to_string();
    let gpu_name = get_gpu::get_gpu();

    let ram_total = system.total_memory() / 1024 / 1024;
    let ram_used  = system.used_memory()  / 1024 / 1024;

    let uptime_secs = System::uptime();
    let uptime = format!("{}h {}m", uptime_secs / 3600, uptime_secs % 3600 / 60);

    let ip_adress = get_local_ip::get_local_ip();

    // Return all infos as a list
    let mut infos = vec![
        format!(""),
        format!(" [{}@{}]", username.yellow(), hostname.green()),
        format!("OS:         {}", os_name),
        format!("Kernel:     {}", kernel),
        format!("Uptime:     {}", uptime),
        format!("CPU:        {}", cpu_name),
        format!("GPU:        {}", gpu_name),
        format!("RAM:        {} MB / {} MB", ram_used, ram_total),
        format!("Local IP:   {}", ip_adress),
        format!("Disk usage: "),
    ];

    // Add disk lines under infos
    infos.extend(get_disk_usage::get_disk_usage());
    infos

}

// print banner and infos side by side
fn print_fetch(ascii: &[&str], infos: &[String], color: &str) {
    let empty_info = String::new();

    // find the longest line in the banner
    let banner_width = ascii.iter()
        .map(|line| line.chars().count())
        .max()
        .unwrap_or(0);

    let max = ascii.len().max(infos.len());

    for i in 0..max {
        let ascii_line = ascii.get(i).unwrap_or(&"");
        let info_line  = infos.get(i).unwrap_or(&empty_info);

        // fill the line with spaces until it reaches banner_width
        // now the infos are always at the same position
        let padding = banner_width - ascii_line.chars().count();
        let padded_line = format!("{}{}", ascii_line, " ".repeat(padding));

        let colored_line = match color {
            "red"            => padded_line.red().to_string(),
            "green"          => padded_line.green().to_string(),
            "yellow"         => padded_line.yellow().to_string(),
            "blue"           => padded_line.blue().to_string(),
            "magenta"        => padded_line.magenta().to_string(),
            "cyan"           => padded_line.cyan().to_string(),
            "white"          => padded_line.white().to_string(),
            "black"          => padded_line.black().to_string(),
            "bright_red"     => padded_line.bright_red().to_string(),
            "bright_green"   => padded_line.bright_green().to_string(),
            "bright_yellow"  => padded_line.bright_yellow().to_string(),
            "bright_blue"    => padded_line.bright_blue().to_string(),
            "bright_magenta" => padded_line.bright_magenta().to_string(),
            "bright_cyan"    => padded_line.bright_cyan().to_string(),
            "bright_white"   => padded_line.bright_white().to_string(),
            _                => padded_line.white().to_string(),
        };

        println!("{}    {}", colored_line, info_line);
    }
}

// main entry point
fn main() {
    let args = Args::parse();

    if args.list {
        println!("Available banners:");
        for name in banners::list_banners() {
            println!("  - {}", name);
        }
        return;
    }

    // Config aus TOML laden
    let cfg = config::load_config();

    // Reihenfolge: CLI Flag > config.toml > Hardcoded Fallback
    let banner = args.banner
        .or(cfg.banner)
        .unwrap_or_else(|| "batman".to_string());

    let color = args.color
        .or(cfg.color)
        .unwrap_or_else(|| "white".to_string());

    let banner_path = args.banner_path.or(cfg.banner_path);

    let ascii: Vec<String>;

    // check banner path and use the banner file
    if let Some(path) = banner_path {
        ascii = banners::get_banner_from_path(&path);
    } else {
        // logic for build in banners
        let vec_static = banners::get_banners(&banner);
        ascii = vec_static.iter().map(|&s| s.to_string()).collect();
    }

    let infos = get_infos();
    print_fetch(&ascii.iter().map(|s| s.as_str()).collect::<Vec<&str>>(), &infos, &color);
}