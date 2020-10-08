# FucesaChallenge
Programa para almacenar y manejar la información sobre bichos.
Al iniciar el programa carga los valores de un archivo csv y al finalizar el programa guarda los valores en el mismo.
## Comandos:
    create --id <ID> --species <SPECIES> --description <DESCRIPTION>
    
        Función para añadir los elementos a la tabla
        Los valores de cada atributo deben estar entre comillas
        Ejemplo: create --id "XXXX" --species "Ejemplo" --description "Ejemplo de descripción"
    
    show <ID>
    
        Función para mostrar elementos específicos de la tabla
        El valor del identificador debe estar entre comillas
        Ejemplo: show "XXXX"
    
    update <ID> [--description <DESCRIPTION> || --species <SPECIES>]
    
        Función para actualizar valores de elementos específicos de la tabla
        Los valores de cada atributo deben estar entre comillas
    
        Ejemplos:
            update "XXXX" --description "Nueva Descripción"
            update "XXXX" --especie "Nueva especie"
            update "XXXX" --description "Nueva Descripción" --especie "Nueva especie"
            update "XXXX" --especie "Nueva especie" --description "Nueva Descripción"
    
    delete --id <ID>
    
        Función para borrar elementos específicos de la tabla
        El valor del identificador debe estar entre comillas
        Ejemplo: delete --id "XXXX"
    
    exit
    
        Función para terminar el programa, también se puede presionar "Enter" para salir.

## Instalación:

1.	Entrar a la carpeta “release” para descargar los archivos “bugwiki.exe” y “bugs.csv”.
2.	Una vez descargados dar doble click o ejecutar el archivo “bugwiki.exe”.
    - Se abrirá una línea de comandos en la que podrá hacer uso del programa como está explicado anteriormente
