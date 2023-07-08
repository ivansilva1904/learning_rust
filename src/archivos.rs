
use std::io::{Write, BufReader, BufRead, ErrorKind};
use std::fs::File;

pub fn error(){
    panic!();
}

pub fn archivo_errores(){
    let ruta = "archivo.txt";
    let salida = File::create(ruta);
    let mut salida = match salida{
        Ok(archivo) => archivo,
        Err(error) => {
            panic!("Problema al crear archivo: {:?}", error);
        }
    };
    write!(salida, "Holaaaaaaa \no adios?").expect("Error al escribir archivo");
    let entrada = File::open(ruta).unwrap();
    let buffer = BufReader::new(entrada);

    for linea in buffer.lines(){
        println!("{}", linea.unwrap());
    }
}

pub fn archivo_errores2(){
    let salida = File::create("random.txt");
    let salida = match salida {
        Ok(archivo) => archivo,
        Err(error) => match error.kind(){
            ErrorKind::NotFound => match File::create("random.txt"){
                Ok(archivo_creado) => archivo_creado,
                Err(error) => panic!("No se puede crear el archivo: {:?}", error)
            },
            _otro_error => panic!("Problema al abrir archivo: {:?}", error),
        },
    };
}
