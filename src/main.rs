mod consts;

mod methods;
mod uptime;

use colored::Colorize;
use crate::consts::InfoPanel;

fn format_bytes(bytes: u64) -> String {
    const KIB: u64 = 1024;
    const MIB: u64 = KIB * 1024;
    const GIB: u64 = MIB * 1024;
    const TIB: u64 = GIB * 1024;

    match bytes {
        b if b >= TIB => format!("{:.2} TiB", b as f64 / TIB as f64),
        b if b >= GIB => format!("{:.2} GiB", b as f64 / GIB as f64),
        b if b >= MIB => format!("{:.2} MiB", b as f64 / MIB as f64),
        b if b >= KIB => format!("{:.2} KiB", b as f64 / KIB as f64),
        _ => format!("{} B", bytes),
    }
}

fn main() {
    let panel = InfoPanel::new();

    println!(
        r#"
{}
{}
{}
{}
"#,
        format!(
            "{}\n{}\n{}{}",
            "                            .oodMMMM".green(),
            "                   .oodMMMMMMMMMMMMM".green(),
            "       ..oodMMM".red(),"  MMMMMMMMMMMMMMMMMMM".green()
        ),
        format!(
            "{}{}\n{}{}{}\n{}{}{}",
            " oodMMMMMMMMMMM".red(),"  MMMMMMMMMMMMMMMMMMM".green(),
            " MMMMMMMMMMMMMM".red(),"  MMMMMMMMMMMMMMMMMMM    ".green(), panel.name.bold(),
            " MMMMMMMMMMMMMM".red(),"  MMMMMMMMMMMMMMMMMMM    ".green(),"----------------------------".bold(),
        ),
        format!(
            "{}{}{}\n{}{}{}\n{}{}{}\n{}{}{}\n{}{}{}\n{}{}{}",
            " MMMMMMMMMMMMMM".red(),"  MMMMMMMMMMMMMMMMMMM    ".green(),format!("OS: {}", panel.os).bold(),
            " MMMMMMMMMMMMMM".red(),"  MMMMMMMMMMMMMMMMMMM    ".green(),format!("Host IP Address: {}", panel.host_ipaddress).bold(),
            " MMMMMMMMMMMMMM".red(),"  MMMMMMMMMMMMMMMMMMM    ".green(),format!("Uptime: {}", panel.uptime).bold(),
            "               ","                         ",format!("Resolution: {}", panel.resolution).bold(),
            " MMMMMMMMMMMMMM".cyan(),"  MMMMMMMMMMMMMMMMMMM    ".yellow(),format!("CPU: {}", panel.cpu).bold(),
            " MMMMMMMMMMMMMM".cyan(),"  MMMMMMMMMMMMMMMMMMM    ".yellow(),format!("GPU: {}", panel.gpu).bold(),
        ),
        format!(
            "{}{}{}\n{}{}{}\n{}{}\n{}{}\n{}{}\n{}{}",
                " MMMMMMMMMMMMMM".cyan(),"  MMMMMMMMMMMMMMMMMMM".yellow(),format!("    RAM: {} / {}", format_bytes(panel.ram_used), format_bytes(panel.ram_total)).bold(),
            " MMMMMMMMMMMMMM".cyan(),"  MMMMMMMMMMMMMMMMMMM".yellow(),
            format!(
                    "    Memory: {} / {}",
                format_bytes(panel.memory_used),
                format_bytes(panel.memory_total)).bold(),
            " MMMMMMMMMMMMMM".cyan(),"  MMMMMMMMMMMMMMMMMMM".yellow(),
            " MMMMMMMMMMMMMM".cyan(),"  MMMMMMMMMMMMMMMMMMM".yellow(),
            " `^^^^^^MMMMMMM".cyan(),"  MMMMMMMMMMMMMMMMMMM".yellow(),
            "       ````^^^^".cyan(),"  ^^MMMMMMMMMMMMMMMMM\n                      ````^^^^^^MMMM".yellow(),
        ),
    );

}
