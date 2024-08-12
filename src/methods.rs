use sysinfo::{Disks, System};
use crate::consts::InfoPanel;
use systeminfo::{from_system_os, from_system_hardware};
use winreg::enums::HKEY_LOCAL_MACHINE;
use winreg::RegKey;
use regex::Regex;
use serde::de::Unexpected::Str;
use crate::uptime;

impl InfoPanel {
    fn get_sysinfo() -> System {
        let mut sys = System::new_all();
        sys.refresh_all();
        sys
    }
    fn get_reg() -> RegKey {
        RegKey::predef(HKEY_LOCAL_MACHINE)
    }
    fn get_name() -> String {
        from_system_os().hostname
    }
    fn get_os() -> String {
        from_system_os().os
    }

    fn get_host_ipaddress() -> String {
        from_system_os().ip_address
    }

    fn get_uptime() -> String {
        let duration = uptime::get().unwrap().as_secs();
        let seconds = duration % 60;
        let minutes = (duration / 60) % 60;
        let hours = (duration / 60) / 60;
        format!("{}:{}:{}", hours, minutes, seconds)
    }
    fn get_resolution() -> String {
        let video_key = Self::get_reg().open_subkey("SYSTEM\\CurrentControlSet\\Control\\Class\\{4d36e968-e325-11ce-bfc1-08002be10318}\\0000").unwrap();
        // Читаем значение из реестра
        let video_mode: String = video_key.get_value("Display1_Details").unwrap();
        video_mode
    }

    fn get_cpu() -> String {
        from_system_hardware().processor
    }

    fn get_gpu() -> String {
        let winsat_key = Self::get_reg().open_subkey("SOFTWARE\\Microsoft\\Windows NT\\CurrentVersion\\WinSAT").unwrap();
        let gpu_name: String = winsat_key.get_value("PrimaryAdapterString").unwrap();
        gpu_name
    }

    fn get_ram_used() -> u64 {
        Self::get_sysinfo().used_memory()
    }

    fn get_ram_total() -> u64 {
        // Ваша логика для получения максимального объема RAM
        Self::get_sysinfo().total_memory()
    }

    fn get_memory_used() -> String {
        let mut disk_info = Vec::new();
        let disk = Disks::new_with_refreshed_list();
        for d in &disk {
            disk_info.push(format!("{}: {}", d.available_space(), d.total_space()));
        }
        disk_info[0].to_string()
    }
    pub fn new() -> InfoPanel {
        InfoPanel {
                name: InfoPanel::get_name(),
                os: InfoPanel::get_os(),
                host_ipaddress: InfoPanel::get_host_ipaddress(),
                uptime: InfoPanel::get_uptime(),
                resolution: InfoPanel::get_resolution(),
                cpu: InfoPanel::get_cpu(),
                gpu: InfoPanel::get_gpu(),
                ram_used: InfoPanel::get_ram_used(),
                ram_total: InfoPanel::get_ram_total(),
                memory_used: InfoPanel::get_memory_used(),
        }
    }
}

    fn bytes_to_gb(bytes: u64) -> f64 {
        bytes as f64 / 1024.0 / 1024.0 / 1024.0
    }

