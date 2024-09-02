use std::{fs::{self, OpenOptions}, io::Read, path::Path};
use std::io::Write;

pub fn new (path: &str, overwrite: bool) {
    if !Path::new(path).exists(){
        match fs::write(path, "") {
            Ok(_) =>{println!("\"{path}.pva\" created.");}
            Err(err)=>{println!("Failed to create: \"{path}.pva\": {err}");}
            }
        
    } else {
        if overwrite{
            match fs::remove_file(path){
                Ok(_) => {
                    new(&path, true)
                }
                Err(err) => {
                    println!("\
                        Could not overwrite file.\n\
                        Reason: {err}")
                }
            }
        }else {
            println!("There is already a file in the same place.")
        }
    }
}

pub fn read_all (path: &str) -> Vec<Vec<String>> {
    let mut _res = Vec::new();
    
    match fs::read_to_string(path) {
            Ok(file_contents) => {
                _res = file_contents.lines().map(|line| line.to_string()).collect();
            }
            Err(err) => {
                println!("Failed to read file: {}", err);
                _res = Vec::new();
            }
    }
    let mut output: Vec<Vec<String>> = Vec::new();
    let mut temp_vec: Vec<String> = Vec::new();
    
    for i in _res {
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
    
    if output.is_empty(){
        println!("\"{path}\" is empty.");
    }
    return output;
}

pub fn read(path: &str, var: &str) -> String {
    for i in read_all(path){
        if i[0] == var{
            return i[1].clone();
        }
    }
    println!("Failed to locate \"{var}\"");
    return String::new();
}

pub fn write(path: &str, var: &str, value: &str){
    let mut exe = true;
    for i in read_all(path){
        if i[0] == var{
            println!("This var already exists.");
            exe = false;
        }
    }
    if exe{
        let mut file = OpenOptions::new()
            .write(true)
            .append(true)
            .create(false)
            .open(path)
            .expect("Falha ao abrir o arquivo");
        
        writeln!(file, "{var}:{value}").expect("Falha ao escrever no arquivo");
        println!("Created: \"{var}\" with the value \"{value}\"");
    }
}

pub fn remove(path: &str, var: &str){
    let mut value = String::new();
    for i in read_all(path){
        if i[0] == var{
            value = format!("{var}:{}", i[1]);
        }
    }
    
    if !value.is_empty(){
        let mut file = fs::File::open(path).expect("msg");
        let mut contents = String::new();
        file.read_to_string(&mut contents).expect("msg");
        
        let to_remove = format!("{value}\n");
        let removed = contents.replace(to_remove.as_str(), "");
        
        let mut output = fs::File::create(path)
            .expect("Não foi possível criar o arquivo de saída");
        output
            .write_all(removed.as_bytes())
            .expect("Não foi possível escrever no arquivo de saída");
        println!("Removed: {}", to_remove.trim());
    }
}