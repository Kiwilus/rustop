# ferrofetch
Neofetch alternative written in Rust.


Why ferroretch?
Ferro means iron on latin and iron get's Rusty and the project is written in Rust ;)
---

## Features

- System user and hostname
- OS name
- Kernel
- CPU name
- RAM (used/total)
- System language
- batman ASCII banner

---

## Installation
1. clone the repo
```bash
git clone https://github.com/Kiwilus/ferrofetch.git
```
2. change into the project directory
```bash
cd ferrofetch
```
3. and install the tool
```bash
cargo install --path .
```
4. then execute with:
```bash
ferrofetch
```

## Planned features

- Argument parsing with clap and other ASCII banners
- Argument parsing with clap for the color of the banner
- configuration in a .config/rustop/rustop.conf file where you can set color/banner manually and forever
- this structure: user@hostname, OS, Kernel, Uptime, CPU name, GPU, RAM (used/total), Disk usage, local IP adress  
