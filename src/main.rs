use sysinfo::System;
use owo_colors::OwoColorize;

mod calculate_language;

fn main() {
    let mut system = System::new_all();
    system.refresh_all();

    // get informations
    let username = whoami::realname().unwrap_or_else(|_| "Unknown".to_string());
    let hostname = whoami::hostname().unwrap_or_else(|_| "Unknown".to_string());

    let os_name  = System::name().unwrap_or("Unknown".to_string());
    let kernel   = System::kernel_version().unwrap_or("Unknown".to_string());

    let uptime_secs = System::uptime();
    let uptime = format!("{}h {}m", uptime_secs / 3600, uptime_secs % 3600 / 60);

    let cpu_name = system.cpus()[0].brand().to_string();
    let ram_total = system.total_memory() / 1024 / 1024;
    let ram_used  = system.used_memory()  / 1024 / 1024;
    let language = whoami::lang_prefs().unwrap_or_default();
    let lang = calculate_language::extract_country(&language.to_string());

    // ASCII banner
    let ascii = vec![
        "           |\\_|\\               ",
        "           | a_a\\              ",
        "           | | \"]              ",
        "       ____| '-\\___            ",
        "      /.----.___.-'\\           ",
        "     //        _    \\          ",
        "    //   .-. (~v~) /|          ",
        "   |'|  /\\:  .--  / \\          ",
        "  // |-/  \\_/____/\\/~|        ",
        " |/  \\ |  []_|_|_] \\ |        ",
        " | \\  | \\ |___   _\\ ]_}      ",
        " | |  '-' /   '.'  |            ",
        " | |     /    /|:  |            ",
        " | |     |   / |:  /\\          ",
        " | |     /  /  |  /  \\         ",
        " | |    |  /  /  |    \\        ",
        " \\ |    |/\\/  |/|/\\    \\    ",
        "  \\|\\ |\\|  |  | / /\\/\\__\\ ",
        "   \\ \\| | /   | |__           ",
        "        / |   |____)            ",
        "        |_/                     ",
    ];

    let infos = vec![
        format!(""),
        format!(" [{}@{}]", username.yellow(), hostname.green()),
        format!("OS:        {}", os_name),
        format!("Kernel:    {}", kernel),
        format!("Uptime:    {}", uptime),
        format!("CPU:       {}", cpu_name),
        format!("RAM:       {} MB / {} MB", ram_used, ram_total),
        format!("Language:  {}", lang)
    ];

    // Fallback values for missing rows
    let empty_ascii = "         ";
    let empty_info  = "".to_string();

    let max = ascii.len().max(infos.len());

    for i in 0..max {
        let ascii_line = ascii.get(i).unwrap_or(&empty_ascii);
        let info_line  = infos.get(i).unwrap_or(&empty_info);
        println!("{}    {}", ascii_line.green(), info_line);
    }
}
