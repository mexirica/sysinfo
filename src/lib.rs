use sysinfo::{Disks, Networks, System};

pub fn cpu(sys: &mut System) {
    sys.refresh_cpu();
    let mut i: u8 = 1;
    for cpu in sys.cpus() {
        println!("number: {}, usage: {:?}", i, cpu.cpu_usage());
        i += 1;
    }
    std::thread::sleep(sysinfo::MINIMUM_CPU_UPDATE_INTERVAL);
}

#[no_mangle]
pub fn ram(sys: &mut System) {
    sys.refresh_memory();
    println!("total: {:?} GB, usage: {:?} GB",format_bytes(sys.total_memory()), format_bytes(sys.used_memory()))
}

#[no_mangle]
pub fn sys_info(num: u8) {
        println!("OS: {:?}",System::name().unwrap());
        println!("Kernel: {:?}",System::kernel_version().unwrap());
        println!("Version: {:?}",System::os_version().unwrap());
        println!("Host: {:?}",System::host_name().unwrap());
        println!("Number CPUs: {:?}",{num});
}


#[no_mangle]
pub fn disk_info(sys: &mut System) {
    sys.refresh_all();
    let disks = Disks::new_with_refreshed_list();
    let mut i =1;
    for disk in &disks {
        println!("Disk: {}", &i);
        println!("Name: {:?}",disk.name());
        println!("File system: {:?}",disk.file_system());
        println!("Disk type: {:?}",disk.kind());
        println!("Removable: {:?}",disk.is_removable());
        println!("Mount: {:?}",disk.mount_point());
        println!("Total size: {:?} GB",format_bytes(disk.total_space()));
        i += 1;
    }
}
#[no_mangle]
pub fn network_info(){
    let networks = Networks::new_with_refreshed_list();
    for (interface_name, data) in &networks {
        println!("Name: {}",interface_name);
        println!("Data received: {:?}",format_bytes(data.received()));
        println!("Data transmitted: {:?}",format_bytes(data.transmitted()))
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
