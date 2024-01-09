use sysinfo::Signal::Sys;
use sysinfo::System;

mod sysfn;

fn main() {
     let mut sys: System = System::new_all();

     let sys_ram = sys.clone();
     let sys_cpu = sys.clone();

     let ram_thread = std::thread::spawn(move || {
          sysfn::ram(&mut sys_ram);
     });

     let cpu_thread = std::thread::spawn(move || {
          sysfn::cpu(&mut sys_cpu);
     });

     let temp_info_thread = std::thread::spawn(|| {
          sysfn::temp_info();
     });

     ram_thread.join().unwrap();
     cpu_thread.join().unwrap();
     temp_info_thread.join().unwrap();
}