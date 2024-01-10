use sysinfo::{System};
use std::env;
mod lib;
fn main() {
    let mut sys = System::new_all();
    let args: Vec<String> = env::args().collect();

    let args_len = args.len();
    let first_argument: &String;
    let second_argument: &String;

    match args_len {
        2 => {
            first_argument = &args[1];
        },
        3 => {
            first_argument = &args[1];
            second_argument = &args[2];
        }
        _ => {
            eprintln!("Use: {} <argument>\nTo get help use help as flag", &args[0]);
            std::process::exit(1);
        },
    }

    match first_argument.as_str() {
        "cpu" => lib::cpu(&mut sys),
        "ram" => lib::ram(&mut sys),
        "sysinfo" => lib::sys_info(sys.cpus().len() as u8),
        "disk" => lib::disk_info(&mut sys),
        "net" => lib::network_info(),
        "help" => {
            println!("To get information about your pc use the followed flags:");
            println!("cpu --> the percent of use of each cpu");
            println!("ram --> the total RAM and the usage");
            println!("sysinfo --> general informations about your pc");
            println!("disk --> general informations about the disk");
            println!("net --> name, data received and transmitted")
        }
        _ => {
            eprintln!("Use: {} <argument>", args[0]);
            std::process::exit(1);
        }
    }

}