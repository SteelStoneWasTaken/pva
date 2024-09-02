use std::{fs, path::Path};

pub fn new (name: &str, overwrite: bool) {
    let name_pva = format!("{name}.pva");
    if !Path::new(&name_pva).exists(){
        println!("Criando \"{name}.pva\".");
        fs::write(&name_pva, "").expect("Não pôde criar o arquivo");
        println!("\"{name}.pva\" criado.");
    } else {
        if overwrite{
            match fs::remove_file(&name_pva){
                Ok(_) => {
                    new(&name, true)
                }
                Err(err) => {
                    println!("\
                        Não pôde sobrescrever o arquivo.\n\
                        Motivo: {err}")
                }
            }
        }else {
            println!("Já existe um arquivo com o mesmo nome.")
        }
    }
}

pub fn read_all (name: &str) -> Vec<Vec<String>> {
    let mut res = Vec::new();
    
    let name_pva = format!("{name}.pva");
    match fs::read_to_string(&name_pva) {
            Ok(file_contents) => {
                res = file_contents.lines().map(|line| line.to_string()).collect();
            }
            Err(err) => {
                println!("Erro ao ler o arquivo: {}", err);
                res = Vec::new();
            }
    }
    let mut output: Vec<Vec<String>> = Vec::new();
    let mut temp_vec: Vec<String> = Vec::new();
    
    for i in res {
        let mut temp_string = String::new();
        let mut found = false;
        for a in i.chars() {
            if a == ':' 
            && !found{
                temp_vec.push(temp_string.clone());
                temp_string.clear();
                found = true
            } else {
                temp_string.push(a)
            }
        }
        temp_vec.push(temp_string.clone());
        temp_string.clear();
        
        output.push(temp_vec.clone());
            temp_vec.clear()
    }
    
    return output;
}