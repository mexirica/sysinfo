use std::fs;
use std::fs::File;
use std::io::Write;
use std::process::{Command, exit};

fn main() {
    let user_profile = std::env::var("USERPROFILE").unwrap_or_else(|_| ".".to_string());

    let sys_info_exe = format!("{}/AppData/Local/Microsoft/WindowsApps/sys_info_cli.exe", user_profile);

    match fs::rename("sys_info_cli.exe", &sys_info_exe) {
        Ok(_) => {},
        Err(e) => {
            eprintln!("Erro ao mover o arquivo: {}", e);
            exit(1);
        }
    }

    let contents = format!("@echo off\nstart \"\" \"{}\" %*", &sys_info_exe);

    let mut file = File::create("C:\\Windows\\System32\\sys-info.bat").expect("Não foi possível criar o arquivo");

    file.write_all(contents.as_bytes()).expect("Não foi possível escrever no arquivo");

    println!("Arquivo .bat criado com sucesso em: C:\\Windows\\System32\\sys-info.bat");

    println!("Executando o script...");

    let output = Command::new("cmd")
        .args(&["/C", "C:\\Windows\\System32\\sys-info.bat"])
        .output()
        .expect("Falha ao executar o script");

    // Verifique se a execução foi bem-sucedida
    if output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        println!("{}", stdout);
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        eprintln!("{}", stderr);
        exit(1);
    }
}
