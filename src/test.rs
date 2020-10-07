extern crate clap;
use std::io::{stdin,stdout,Write};
use std::collections::HashMap;

#[derive(Hash, Eq, PartialEq, Debug)]
struct Bug{
    species: String,
    description: String,
}

impl Bug{
    #[allow(dead_code)]
    fn new(species: &str, description: &str) -> Bug {
        Bug {
            species: species.to_string(),
            description: description.to_string(),
        }
    }

    #[allow(dead_code)]
    fn update(&mut self, value: String, opt: &i32){
        println!("Cambiar los valores dependiendo de la opcion {}",opt);
        match opt{
            1 => self.description = value,
            2 => self.species = value,
            _ => error("Opción desconocida")
        }
    }
}

fn create<'a>(cmd: &'a String,bugs: &mut HashMap<&'a str,Bug>){
    let values: Vec<&str> = cmd.trim().split(|c| c=='"'|| c=='"').collect();
    let id: &'a str = values[1];
    if values.len() != 7{
        error("número de parámetros no es valido.")
    } else {
        let tokens: Vec<&str> = values[0].split_ascii_whitespace().collect();
        if tokens[1].trim() != "--id" || tokens[2].trim() != "--species" || tokens[4].trim() != "--description" {
            error("los atributos del comando no son los correctos");
        } else {
            match bugs.get(tokens[1]){
                Some(_bg) =>{ 
                    bugs.insert(tokens[1],Bug::new(tokens[3], tokens[5]));},
                None => error("el insecto que intenta agregar ya existe") , //agregar que enseñe la info del insecto que ya existe
            }
            
        }
    }
}

// fn check(opt: &str){
//     match opt{
//         "id" => 
//     }
// }

fn error(desc: &str){
    println!("Error, {}",desc);
    println!("Escriba '--help' para obtener más información.")
}

fn help(){
    println!("Program to store information about bugs and manage such information\n");
    println!("\nOPTIONS:");
    println!("\tcreate --id <ID> --species <SPECIES> --description <DESCRIPTION>");
    println!("\tshow <ID>");
    println!("\tupdate <ID> [--description <DESCRIPTION> || --species <SPECIES>]");
    println!("\tdelete --id <ID>\n\n");
}

fn main() {
    let mut bugs: HashMap<&str,Bug> = HashMap::new();

    //Agregar funcionalida de carga de archivo
    let mut command = String::from("");
    loop {
        
        print!("> bugwiki ");
        stdout().flush().expect("flush fallido");
        
        
        
        match stdin().read_line(&mut command) {
        
            Err(error) => println!("Error leyendo la entrada: {}", error),
            _ => {
        
                let tokens: Vec<&str> = command.trim().split_ascii_whitespace().collect();
        
                if tokens.len() < 1{
                    continue;
                } else {
                    match tokens[0]{
                        "exit" => break,
                        "create" => {
                                let smth = command.clone();
                                create(&smth, &mut bugs);
                                println!("{:?}",bugs);
                            }
                        "--help" => help(),
                        _ => error("comando no reconocido.")
                    }
                }
            },
        }
    }
}

/*
create --id "2X3t" --species "Latrodectus mactans" --description "Latrodectus mactans, known as southern black widow or simply black widow, and the shoe-button spider, is a venomous species of spider in the genus Latrodectus."
*/