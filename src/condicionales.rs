
use std::io::{self, stdin};
use rand::Rng;
use std::io::{Write, BufReader, BufRead, ErrorKind};
use std::fs::File;
use std::cmp::Ordering;

pub fn condicionales_simples(){
    let edad = 24;
    if(edad >= 1 && edad <= 18){
        println!("Es un cumpleaños importante");
    } else if (edad == 21 || edad == 50) {
        println!("Es un cumpleaños importante");
    } else if (edad >= 65) {
        println!("Es un cumpleaños importante y cada vez te acercas mas a conocer a la muerte jaja");
    } else {
        println!("A nadie deberia importarle este cumpleaños. Ni siquiera a vos");
    }
}

pub fn operador_ternario(){
    let mut edad = 24;
    let puede_votar = if edad >= 18{
        true
    } else{
        false
    };
    println!("Puedo votar?: {}", puede_votar);
}

pub fn operador_match(){
    let edad = 24;
    match edad {
        1..=18 => println!("Es un cumpleaños importante"),
        21 | 50 => println!("Es un cumpleaños importante"),
        65..=i32::MAX => println!("Es un cumpleaños importante y cada vez te acercas mas a conocer a la muerte jaja"),
        _ => println!("A nadie deberia importarle este cumpleaños. Ni siquiera a vos"),
    };
}

pub fn operador_match_ordering(){
    let edad = 24;
    let edad_para_votar = 18;
    match edad.cmp(&edad_para_votar){
        Ordering::Less => println!("No puede votar"),
        Ordering::Greater => println!("Puede votar"),
        Ordering::Equal => println!("Puede votar por primera vez"),
    };
}

