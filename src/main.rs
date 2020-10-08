extern crate csv;
extern crate serde;

#[macro_use]
extern crate serde_derive;
use std::io::{stdin,stdout,Write};
use std::collections::HashMap;
use std::error::Error;

/**
 * Estructura para almacenar la información de los bichos:
 *  species
 *  description
 */
#[derive(Hash, Eq, PartialEq, Debug)]
struct Bug{
    species: String,
    description: String,
}

/**
 * Estructura para deserializar la información del archivo en forma de strings:
 *  id
 *  species
 *  description
 */
#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct Record {
    id: String,
    species: String,
    description: String,
}

/**
 * 
 */
impl Bug{
    fn new(species: &str, description: &str) -> Bug {
        Bug {
            species: species.to_string(),
            description: description.to_string(),
        }
    }

    fn new2(species: String, description: String) -> Bug {
        Bug {
            species: species,
            description: description,
        }
    }
}

fn create(command: Vec<&str>, whites_vector: Vec<&str>, bugs: &mut HashMap<String, Bug>){
    if command.len() !=7 {
        error("",1);
        return
    }
    if whites_vector[1] != "--id" || command[2].trim() != "--species" || command[4].trim() != "--description" {
        error("",2);
        return
    }
    let id = String::from(command[1]);
    if bugs.contains_key(&id){
        error("",5);
        show(id, 3, bugs);
        return
    } 
    bugs.insert(id,Bug::new(command[3], command[5]));
}

fn show(id: String, len: usize, bugs: &mut HashMap<String, Bug>){
    if len !=3 {
        error("",1);
        return
    }
    if !bugs.contains_key(&id){
        error("",4);
        return
    }
    match bugs.get(&id){
        Some(bug) => {
            println!("\n\tSpecies: {}",bug.species);
            println!("\n\tDescription: {}",bug.description);
        },
        _ => error("falla desconocida", 999),
    }
}

fn update(command: Vec<&str>, bugs: &mut HashMap<String, Bug>){
    if command.len() !=5{
        if command.len() !=7 {
            error("",1);
            return
        }
    }
    /*
     * opt = 1 cambiar especie
     * opt = 2 cambiar descripcion
     * opt = 3 cambiar especies y descripcion
     * opt = 4 cambiar descripcion y especies
     */
    let mut _opt = 1;
    if command[2].trim() != "--species" && command.len() == 5{
        _opt = 2;
        if command[2].trim() != "--description"{
            error("",2);
            return
        }
    } else {
        if command.len() == 7{
            _opt =3;
            if command[2].trim() != "--species" || command[4].trim() != "--description"{
                _opt = 4;
                if command[4].trim() != "--species" || command[2].trim() != "--description"{
                    error("",2);
                    return;
                }
            }
        }
    }
    let id = String::from(command[1]);
    let species: &str;
    let description: &str;
    let new_bug: Bug;

    match bugs.get(&id){
        Some(bug) => {
            species = &bug.species;
            description = &bug.description;
            new_bug = match _opt{
                1 => Bug::new(command[3], description),
                2 => Bug::new(species, command[3]),
                3 => Bug::new(command[3], command[5]),
                4 => Bug::new(command[5], command[3]),
                _ => Bug::new(species, description),
            }
        },
        None => {
            error("",4);
            return;
        },
    }

    bugs.insert(id, new_bug);
}

/**
 * Función para borrar elementos de la tabla
 * Recibe 3 parámetros: 
 *  @command Vector de strings, con el input del usuario separado por comillas
 *  @whites_vector Vector de strings, con el input del usuario separado por espacios en blanco
 *  @bugs HashMap, con las llaves de tipo String y contiene un elemento de tipo Bug
 */
fn delete(command: Vec<&str>, whites_vector: Vec<&str>, bugs: &mut HashMap<String, Bug>){
    if command.len() != 3{
        error("",1);
        return
    }
    if whites_vector[1] != "--id" {
        error("",2);
        return
    }
    let id = String::from(command[1]);
    if !bugs.contains_key(&id){
        error("",4);
        return
    } 
    bugs.remove(&id);
}

fn charge_csv_file(bugs: &mut HashMap<String, Bug>) -> Result<(), Box<dyn Error>>{
    let mut rdr = csv::Reader::from_path("bugs.csv")?;
    for result in rdr.deserialize() {
        let record: Record = result?;
        bugs.insert(record.id, Bug::new2(record.species, record.description));
    }
    Ok(())
}

fn write_csv_file(bugs: &mut HashMap<String, Bug>) -> Result<(), Box<dyn Error>> {
    let mut wtr = csv::Writer::from_path("bugs.csv")?;
    wtr.write_record(&["Id", "Species", "Description"])?;
    for (key, bug) in bugs{
        wtr.write_record(&[key,&bug.species,&bug.description])?;
    }
    wtr.flush()?;
    Ok(())
}

/**
 * Función para imprimir los errores más comunes a la terminal
 * Recibe como parametros:
 *  @desc es el mensaje que se va a imprimir
 *  @opt es la opción de error a imprimir
 */
fn error(desc: &str, opt: i32){
    let error_str: String = "\nError, ".to_owned();
    let new_str: &str;
    match opt {
        1 =>  new_str = "número incorrecto de parametros o sintaxis incorrecta",
        2 => new_str = "los atributos del comando no son los correctos",
        3 => new_str = "opción desconocida",
        4 => new_str = "el ID no existe",
        5 => new_str = "el ID ya existe",
        _ => new_str = desc,
    }
    println!("{}",error_str+new_str);
    println!("Escriba '--help' para obtener más información.\n")
}

//completar descripciones de cada opción
fn help(){
    println!("Program to store information about bugs and manage such information\n");
    println!("\nOPTIONS:");
    println!("\tcreate --id <ID> --species <SPECIES> --description <DESCRIPTION>");
    println!("\tshow <ID>");
    println!("\tupdate <ID> [--description <DESCRIPTION> || --species <SPECIES>]");
    println!("\tdelete --id <ID>\n\n");
}

fn main() {
    let mut bugs: HashMap<String,Bug> = HashMap::new();
    
    if let Err(err) = charge_csv_file(&mut bugs) {
        println!("{}", err);
    }
    //Agregar funcionalida de carga de archivo
    let mut command = String::new();
    loop {
        print!("> bugwiki ");
        stdout().flush().expect("flush fallido");
        stdin().read_line(&mut command).expect("lectura fallida");

        let whites_vector: Vec<&str> = command.trim().split(" ").collect();
        let commas_vector: Vec<&str> = command.trim().split('"').collect();
        
        if whites_vector[0] == "exit" || command.trim().is_empty(){
            if let Err(err) = write_csv_file(&mut bugs) {
                println!("{}", err);
            }
            break;
        }
        if whites_vector.len() < 2 && whites_vector[0] != "--help"{
            error("", 1);
            command.clear();
            continue;
        }
        match whites_vector[0]{
            "create" => create(commas_vector, whites_vector, &mut bugs),
            "show" => show(String::from(commas_vector[1]), commas_vector.len(), &mut bugs),
            "update" => update(commas_vector, &mut bugs),
            "delete" => delete(commas_vector, whites_vector, &mut bugs),
            "--help" => help(),
            _ => error("",3),
        }

        command.clear();
    }
}

/*
create --id "2X3t" --species "Latrodectus mactans" --description "Latrodectus mactans, known as southern black widow or simply black widow, and the shoe-button spider, is a venomous species of spider in the genus Latrodectus."
show "2X3t"
update "2X3t" --description "Latrodectus mactans was first described by Johan Christian Fabricius in 1775, placing it in the genus Aranea."
update "2X3t" --species "lalala" --description "Latrodectus mactans was first described by Johan Christian Fabricius in 1775, placing it in the genus Aranea."
update "2X3t" --description "Latrodectus mactans was first described by Johan Christian Fabricius in 1775, placing it in the genus Aranea. Holi" --species "lalala2"
delete --id "2X3t"
--help
exit
*/