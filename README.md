# FucesaChallenge
Programa para almacenar y manejar la información sobre bichos.
Al iniciar el programa carga los valores de un archivo csv y al finalizar el programa guarda los valores en el mismo
OPCIONES:
create --id <ID> --species <SPECIES> --description <DESCRIPTION>
    println!("\t\tFunción para añadir los elementos a la tabla
    println!("\t\tLos valores de cada atributo deben estar entre comillas
    Ejemplo: create --id "XXXX" --species "Ejemplo" --description "Ejemplo de descripción"
    println!("\tshow <ID>\n");
    println!("\t\tFunción para mostar elementos específicos de la tabla\n");
    println!("\t\tEl valor del identificador debe estar entre comillas\n");
    println!("\t\tEjemplo: show \"XXXX\"\n");
    println!("\tupdate <ID> [--description <DESCRIPTION> || --species <SPECIES>]\n");
    println!("\t\tFunción para actualizar valores de elementos específicos de la tabla\n");
    println!("\t\tLos valores de cada atributo deben estar entre comillas\n");
    println!("\t\tEjemplos:\n");
    println!("\t\t           update \"XXXX\" --description \"Nueva Descripción\"\n");
    println!("\t\t           update \"XXXX\" --especie \"Nueva especie\"\n");
    println!("\t\t           update \"XXXX\" --description \"Nueva Descripción\" --especie \"Nueva especie\"\n");
    println!("\t\t           update \"XXXX\" --especie \"Nueva especie\" --description \"Nueva Descripción\"\n");
    println!("\tdelete --id <ID>\n");
    println!("\t\tFunción para borrar elementos específicos de la tabla\n");
    println!("\t\tEl valor del identificador debe estar entre comillas\n");
    println!("\t\tEjemplo: delete --id \"XXXX\"\n");
    println!("\texit\n");
    println!("\t\tFunción para terminar el programa, también se puede presionar \"Enter\" para salir.\n");
