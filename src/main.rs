//Para evitar que variables sin utilizar sean tomadas como error
#![allow(unused)]

use std::io::{self, stdin};
use rand::Rng;
use std::io::{Write, BufReader, BufRead, ErrorKind};
use std::fs::File;
use std::cmp::Ordering;

//Importe de modulos del mismo directorio src
mod tipos_datos;
mod condicionales;
mod ciclos;
mod estructuras_datos;

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
    //tipos_datos::operaciones();
    //tipos_datos::random();
    //condicionales::condicionales_simples();
    //condicionales::operador_ternario();
    //condicionales::operador_match();
    //condicionales::operador_match_ordering();
    //estructuras_datos::arrays1();
    //ciclos::arrays2();
    //ciclos::ciclo_while();
    //ciclos::ciclo_for();
    estructuras_datos::tuplas();
}
