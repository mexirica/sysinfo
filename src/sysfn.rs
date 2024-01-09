use lazy_static::lazy_static;
use std::sync::{Arc, RwLock};
use sysinfo::{Components, Disks, Networks, System};

lazy_static! {
    static ref SYS: Arc<RwLock<System>> = Arc::new(RwLock::new(System::new_all()));
}

pub struct DiskInfo{
    os: String,
    fs: String,
    disk_type: String,
    removable: bool,
    mount: String,
    size: u64
}

pub struct CpuInfo {
    number: u8,
    usage: f32
}
pub struct NetInfo {
    name: String,
    received: u64,
    transmitted: u64
}

pub struct RamInfo {
    total: u64,
    usage: u64
}

pub struct SysInfo {
    name: String,
    kernel: String,
    version : String,
    host: String,
    number_cpus: u8
}

pub fn cpu() -> Vec<CpuInfo> {
    let mut sys = SYS.write().unwrap();
    sys.refresh_cpu();
    let mut i: u8 = 1;
    let mut cpus: Vec<CpuInfo> = Vec::new();
    for cpu in sys.cpus() {
        cpus.push( CpuInfo{number: i, usage: cpu.cpu_usage()});
        i += 1;
    }
    std::thread::sleep(sysinfo::MINIMUM_CPU_UPDATE_INTERVAL);
    cpus
}

pub fn ram() -> RamInfo {
    let mut sys = SYS.write().unwrap();
    sys.refresh_memory();
    RamInfo{total:format_bytes(sys.total_memory()), usage:format_bytes(sys.used_memory())}
}

pub fn sys_info() -> SysInfo {
    let sys = SYS.write().unwrap();
    SysInfo {
        name: System::name().expect("Error to read System name"),
        kernel: System::kernel_version().expect("Error to read Kernel name"),
        version: System::os_version().expect("Error to read OS version"),
        host: System::host_name().expect("Error to read Host name"),
        number_cpus: sys.cpus().len() as u8
    }
}


pub fn disk_info() -> Vec<DiskInfo> {
    let mut sys = SYS.write().unwrap();
    sys.refresh_all();
    let disks = Disks::new_with_refreshed_list();
    let mut disk_vec:Vec<DiskInfo> = Vec::new();
    for disk in &disks {
        let name = disk.name().to_str().unwrap();
        let file_system = disk.file_system().to_str().unwrap();
        let disk_type = disk.kind().to_string();
        let is_removable = if disk.is_removable() { "yes" } else { "no" };
        let mount_point = disk.mount_point().to_str().unwrap();
        let total_space = format_bytes(disk.total_space());

        disk_vec.push(DiskInfo{os:name.to_string(), fs:file_system.to_string(), disk_type,removable: is_removable.parse::<bool>().unwrap(),mount: mount_point.to_string(),
            size: total_space})

    }
    disk_vec
}
pub fn network_info() -> Vec<NetInfo> {
    let networks = Networks::new_with_refreshed_list();
    let mut vec: Vec<NetInfo> = Vec::new();
    for (interface_name, data) in &networks {
        vec.push(NetInfo{name: interface_name.to_string(), received:format_bytes(data.received()), transmitted:format_bytes(data.transmitted())})
}
    vec
}


pub fn temp_info() {
    let components = Components::new_with_refreshed_list();
    println!("=> components:");
    for component in &components {
        println!("{component:?}");
    }
}
fn format_bytes(bytes: u64) -> u64 {
    const KB: u64 = 1024;
    const MB: u64 = KB * 1024;
    const GB: u64 = MB * 1024;

    if bytes >= GB {
        bytes / GB
    } else if bytes >= MB {
        bytes / MB
    } else if bytes >= KB {
        bytes / KB
    } else {
        bytes
    }
}

