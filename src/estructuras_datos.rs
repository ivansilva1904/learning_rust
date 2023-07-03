
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

pub fn strings1(){
    //String es un vector de bits
    //&str es un puntero al string como se tiene en otros lenguajes (linea 16 usa String)
    
    let mut string1 = String::new();
    string1.push('a');
    string1.push('b');
    for letra in string1.split_whitespace(){
        println!("{}", letra);
    }

    let mut string2 = String::new();
    string2.push_str("hola");
    string2.push_str("chau");
    for palabra in string2.split_whitespace(){
        println!("{}", palabra);
    }

    let mut string3 = String::new();
    string3.push('a');
    string3.push_str("chau");
    for coso in string3.split_whitespace(){
        println!("{}", coso);
    }
}

pub fn strings2(){
    let mut string4 = String::new();
    string4.push('a');
    string4.push('b');
    for letra in string4.split_whitespace(){
        println!("{}", letra);
    }

    let string5 = string4.replace('a', "c");
    println!("{}", string5);
}

pub fn strings3(){
    let string6 = "Hola soy un string";
    let mut string7: String = string6.to_string();
    println!("&str: {}", string6);
    println!("String: {}", string7);

    println!();

    let byte_array: &[u8] = string7.as_bytes();
    let string8 = &string7[0..6]; //Recorta el string con el parametro recibido dentro de []
    println!("Largo de string: {}", string8.len());
    println!("String acortado: {}", string8);
}

pub fn strings_a_vector(){
    let string6 = String::from("a b c z x y m o p p");
    let mut vector1: Vec<char> = string6.chars().collect();

    vector1.sort(); //Ordena y borra duplicados
    vector1.dedup();

    //Vista del vector sin modificar
    for letra in vector1{
        print!("{}", letra);
    }
    println!(); //Solo imprime salto de linea
}
