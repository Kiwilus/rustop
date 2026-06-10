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
    let empty_ascii = "                                 ";
    let empty_info  = String::new();

    let max = ascii.len().max(infos.len());

    for i in 0..max {
        let ascii_line = ascii.get(i).unwrap_or(&empty_ascii);
        let info_line  = infos.get(i).unwrap_or(&empty_info);

        // color based on argument, default color is green
        let colored_line = match color {
            "red"            => ascii_line.red().to_string(),
            "green"          => ascii_line.green().to_string(),
            "yellow"         => ascii_line.yellow().to_string(),
            "blue"           => ascii_line.blue().to_string(),
            "magenta"        => ascii_line.magenta().to_string(),
            "cyan"           => ascii_line.cyan().to_string(),
            "white"          => ascii_line.white().to_string(),
            "black"          => ascii_line.black().to_string(),
            "bright_red"     => ascii_line.bright_red().to_string(),
            "bright_green"   => ascii_line.bright_green().to_string(),
            "bright_yellow"  => ascii_line.bright_yellow().to_string(),
            "bright_blue"    => ascii_line.bright_blue().to_string(),
            "bright_magenta" => ascii_line.bright_magenta().to_string(),
            "bright_cyan"    => ascii_line.bright_cyan().to_string(),
            "bright_white"   => ascii_line.bright_white().to_string(),
            _                => ascii_line.green().to_string(),
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