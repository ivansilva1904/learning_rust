
use std::io::{self, stdin};
use rand::Rng;
use std::io::{Write, BufReader, BufRead, ErrorKind};
use std::fs::File;
use std::cmp::Ordering;

pub fn arrays1(){
    let array1 = ['a', 'b', 'c', 'd', 'e', 'f', 'g'];
    println!("Primer elemento: {}", array1[0]);
    println!("Segundo elemento: {}", array1[1]);
    println!("Cantidad de elementos: {}", array1.len());
}

pub fn tuplas(){
    //let tupla1: (u8, String, f64) = (24, "Ivan".to_string(), 100000.00);
    let tupla1: (u8, &str, f64) = (24, "Ivan", 100000.00);
    
    println!("Nombre desde tupla: {}", tupla1.1);

    //Tambien podes mover los datos de una tupla a variables individuales
    let valor1 = tupla1.1;
    println!("Nombre desde variable: {}", valor1);

    //Y tambien se pueden crear grupos de variables
    let(elemento0, elemento1, elemento2) = tupla1;
    println!("Nombre desde otra variable: {}", elemento1);
}
