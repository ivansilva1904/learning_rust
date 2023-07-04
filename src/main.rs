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
mod funciones;

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
    //estructuras_datos::tuplas();
    //estructuras_datos::strings1();
    //estructuras_datos::strings2();
    //estructuras_datos::strings3();
    //estructuras_datos::strings_a_vector();
    //tipos_datos::casteo();
    //estructuras_datos::enums();
    //estructuras_datos::vectores();
    //funciones::hola();
    //hola_main();
    //funciones::suma(3, 8);
    //println!("La suma de 23 y 54 es: {}", funciones::suma2(23, 54));
    /*let (valor1, valor2) = funciones::devolver_multiples_valores(5);
    println!("5 + 1 = {} y 5 + 2 = {}", valor1, valor2);*/
    let lista = vec![2, 3, 4, 5, 6];
    println!("Suma del vector [2, 3, 4, 5, 6] = {}", funciones::suma_vector(lista));
}

fn hola_main(){
    println!("Hola desde el main");
}
