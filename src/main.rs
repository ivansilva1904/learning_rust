//Para evitar que variables sin utilizar sean tomadas como error
#![allow(unused)]

use std::io::{self, stdin};
use rand::Rng;
use std::io::{Write, BufReader, BufRead, ErrorKind};
use std::fs::File;
use std::cmp::Ordering;

//Importe de modulos del mismo directorio src
mod tipos_datos;

fn main() {
    /*let mut nombre = String::new();
    let saludo = "Hola";

    println!("Como te llamas?");
    io::stdin().read_line(&mut nombre)
        .expect("No ingreso un nombre");

    println!("{}!, {}", saludo, nombre);*/

    /*Tipos de datos*/
    //tipos_datos::edad();
    //tipos_datos::maximos();
    //tipos_datos::flotantes();
    tipos_datos::operaciones();
}
