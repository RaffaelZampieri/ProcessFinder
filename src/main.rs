use std::env;
use std::path::Path;
use sysinfo::System;

fn main() {
    // recebe o nome do processo como argumento
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Digite o nome do processo que deve ser finalizado");
        return;
    }
    let process_name = &args[1];
    // exclui o próprio processo
    let exclude = Path::new(&args[0]).file_name().unwrap().to_str().unwrap();
    // cria uma instância do sistema e itera sobre os processos existentes
    // finalizando os que contém o nome passado como argumento
    let s = System::new_all();
    for (_pid, process) in s.processes() {
        if process.cmd().contains(process_name) && !process.name().contains(exclude) {
            println!("{} - {}: {}",process.pid(), process.name(), process.cmd().join(" "));
            process.kill();
        }
    }
}
