
use std::io::{self, stdin};
use rand::Rng;
use std::io::{Write, BufReader, BufRead, ErrorKind};
use std::fs::File;
use std::cmp::Ordering;

pub fn edad(){
    const MILLON: u32 = 1000000;
    const PI: f32 = 3.141592;
    //Dos variables con el mismo nombre pero con tipos de datos y valores distintos
    let edad = "23";
    let mut edad: u32 = edad.parse()
        .expect("No se ingreso un numero por edad"); //Sin este catch del error, no se compila
    edad = edad + 1;
    println!("Tengo {} años y quiero {} de dolares", edad, MILLON);
}

pub fn maximos(){
    println!("Maximo u8: {}", u8::MAX);
    println!("Maximo u16: {}", u16::MAX);
    println!("Maximo u32: {}", u32::MAX);
    println!("Maximo u64: {}", u64::MAX);
    println!("Maximo u128: {}", u128::MAX);
}

pub fn flotantes(){
    let num_1: f32 = 1.111111111111111;
    println!("f32: {}", num_1);

    let num_2: f64 = 1.111111111111111;
    println!("f64: {}", num_2);
}

pub fn operaciones(){
    let num_3: f32 = 5.0;
    let num_4: f32 = 4.0;
    println!("5 + 4 = {}", num_3 + num_4);
    println!("5 - 4 = {}", num_3 - num_4);
    println!("5 * 4 = {}", num_3 * num_4);
    println!("5 / 4 = {}", num_3 / num_4);
    println!("5 % 4 = {}", num_3 % num_4);

    let mut num_5: u32 = 3;
    num_5 += 1;
    println!("3 + 1 = {}", num_5);
}

pub fn random(){
    let random_num = rand::thread_rng().gen_range(1..101);
    println!("Random: {}", random_num);
}

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

pub fn arrays1(){
    let array1 = ['a', 'b', 'c', 'd', 'e', 'f', 'g'];
    println!("Primer elemento: {}", array1[0]);
    println!("Segundo elemento: {}", array1[1]);
    println!("Cantidad de elementos: {}", array1.len());
}

pub fn arrays2(){
    let array2 = [1, 2, 3, 4, 5, 6, 7, 8, 9];
    let mut indice = 0;
    loop {
        if array2[indice] % 2 == 0{
            indice += 1;
            continue; //Esto pasa a la siguiente iteracion del loop
        }
        if array2[indice] == 9{
            break; //Esto finaliza el loop
        }
        println!("Valor del array: {}", array2[indice]);
        indice += 1;
    }
}
