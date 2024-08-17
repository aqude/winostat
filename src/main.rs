mod consts;

mod methods;
mod uptime;

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

    println!(r#"
                            .oodMMMM
                   .oodMMMMMMMMMMMMM
       ..oodMMM  MMMMMMMMMMMMMMMMMMM
 oodMMMMMMMMMMM  MMMMMMMMMMMMMMMMMMM
 MMMMMMMMMMMMMM  MMMMMMMMMMMMMMMMMMM   {}
 MMMMMMMMMMMMMM  MMMMMMMMMMMMMMMMMMM    ------
 MMMMMMMMMMMMMM  MMMMMMMMMMMMMMMMMMM    OS: {}
 MMMMMMMMMMMMMM  MMMMMMMMMMMMMMMMMMM    Host IP Address: {}
 MMMMMMMMMMMMMM  MMMMMMMMMMMMMMMMMMM    Uptime: {}
                                        Resolution: {}
 MMMMMMMMMMMMMM  MMMMMMMMMMMMMMMMMMM    CPU: {}
 MMMMMMMMMMMMMM  MMMMMMMMMMMMMMMMMMM    GPU: {}
 MMMMMMMMMMMMMM  MMMMMMMMMMMMMMMMMMM    RAM: {} / {}
 MMMMMMMMMMMMMM  MMMMMMMMMMMMMMMMMMM    Memory: {} / {}
 MMMMMMMMMMMMMM  MMMMMMMMMMMMMMMMMMM
 `^^^^^^MMMMMMM  MMMMMMMMMMMMMMMMMMM
       ````^^^^  ^^MMMMMMMMMMMMMMMMM
                      ````^^^^^^MMMM
"#,
        panel.name,
        panel.os,
        panel.host_ipaddress,
        panel.uptime,
        panel.resolution,
        panel.cpu,
        panel.gpu,
        format_bytes(panel.ram_used),
        format_bytes(panel.ram_total),
        format_bytes(panel.memory_used),
        format_bytes(panel.memory_total),
    );
}
