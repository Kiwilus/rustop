use sysinfo::System;


fn main() {
    let mut system = System::new_all();
    system.refresh_all();

    // get informations
    let os_name  = System::name().unwrap_or("Unknown".to_string());
    let kernel   = System::kernel_version().unwrap_or("Unknown".to_string());
    let cpu_name = system.cpus()[0].brand().to_string();
    let ram_total  = system.total_memory() / 1024 / 1024;
    let ram_used = system.used_memory()  / 1024 / 1024;

    // ASCII banner
    let ascii = vec![
        "  /\\_/\\  ",
        " ( o.o ) ",
        "  > ^ <  ",
        "         ",
    ];

    let infos = vec![
        format!("OS:     {}", os_name),
        format!("Kernel: {}", kernel),
        format!("CPU:    {}", cpu_name),
        format!("RAM:    {} MB / {} MB", ram_used, ram_total),
    ];

    // print informations on the right and banner on the left
    // zip() combines two lists: ascii with infos, ascii with infos, etc.
    for (ascii_line, info_line) in ascii.iter().zip(infos.iter()) {
        println!("{}    {}", ascii_line, info_line);
    }
}