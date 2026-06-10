use sysinfo::System;
use owo_colors::OwoColorize;
use clap::Parser;
use nvml_wrapper::Nvml;

mod calculate_language;
mod banners;

// cli arguments e.g. ferrofetch --banner green
#[derive(Parser)]
#[command(name = "ferrofetch", about = "A system fetch tool written in Rust")]
struct Args {
    #[arg(short, long, default_value = "batman")]
    banner: String,

    #[arg(short, long, default_value = "green")]
    color: String,

    #[arg(short, long)]
    list: bool
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

    let nvml = Nvml::init().unwrap_or_else(|_| panic!("NVML not found"));
    let device = nvml.device_by_index(0).unwrap();
    let gpu_name = device.name().unwrap_or("Unknown".to_string());

    let ram_total = system.total_memory() / 1024 / 1024;
    let ram_used  = system.used_memory()  / 1024 / 1024;
    let language  = whoami::lang_prefs().unwrap_or_default();
    let lang      = calculate_language::extract_country(&language.to_string());

    let uptime_secs = System::uptime();
    let uptime = format!("{}h {}m", uptime_secs / 3600, uptime_secs % 3600 / 60);

    // Return all infos as a list
    vec![
        format!(""),
        format!(" [{}@{}]", username.yellow(), hostname.green()),
        format!("OS:        {}", os_name),
        format!("Kernel:    {}", kernel),
        format!("Uptime:    {}", uptime),
        format!("CPU:       {}", cpu_name),
        format!("GPU:       {}", gpu_name),
        format!("RAM:       {} MB / {} MB", ram_used, ram_total),
        format!("Language:  {}", lang),
    ]
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
            _                => padded_line.green().to_string(),
        };

        println!("{}    {}", colored_line, info_line);
    }
}

// main entry point
fn main() {
    let args = Args::parse();

    // If --list flag is set, print all banners and exit
    if args.list {
        println!("Available banners:");
        for name in banners::list_banners() {
            println!("  - {}", name);
        }
        return;
    }

    let ascii = banners::get_banners(&args.banner);
    let infos = get_infos();
    print_fetch(&ascii, &infos, &args.color);
}