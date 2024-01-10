mod lib;
use std::{env, io};
use sysinfo::System;
fn main() {
    let mut sys = System::new_all();
    let args: Vec<String> = env::args().collect();

    let args_len = args.len();
    let first_argument: &String;

    if args_len < 2 {
            eprintln!("Use: {} <argument>\nTo get help use help as flag", &args[0]);
            std::process::exit(1);
    }

    first_argument = &args[1];

    match first_argument.as_str() {
        "cpu" => lib::cpu(&mut sys),
        "ram" => lib::ram(&mut sys),
        "sysinfo" => lib::sys_info(sys.cpus().len() as u8),
        "disk" => lib::disk_info(&mut sys),
        "net" => lib::network_info(),
        "help" => print_help(&args[0]),
        _ => {
            eprintln!("Error: Invalid argument '{}'", first_argument);
            print_help(&args[0]);
            std::process::exit(1);
        }
    }
    println!("Press Enter to exit...");
    io::stdin().read_line(&mut String::new()).is_ok();

}

fn print_help(program_name: &str) {
    println!("To get information about your pc use the following flags:");
    println!("cpu --> the percent of use of each cpu");
    println!("ram --> the total RAM and the usage");
    println!("sysinfo --> general informations about your pc");
    println!("disk --> general informations about the disk");
    println!("net --> name, data received and transmitted");
    eprintln!("Usage: {} <argument>\nTo get help use 'help' as flag", program_name);
}
