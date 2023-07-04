
use std::fmt::Display;
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
    let string9 = String::from("a b c z x y m o p p");
    let mut vector1: Vec<char> = string9.chars().collect();

    vector1.sort(); //Ordena y borra duplicados
    vector1.dedup();

    //Vista del vector sin modificar
    for letra in vector1{
        print!("{}", letra);
    }
    println!(); //Solo imprime salto de linea
}

pub fn enums(){ //Enumerated types
    //Esto define un enum (digamos objeto JSON) con los dias de la semana
    enum Dia {
        Lunes,
        Martes,
        Miercoles,
        Jueves,
        Viernes,
        Sabado,
        Domingo
    }

    //Usas impl que permite implementar funciones segun el elemento de un enum
    //En este caso tiene una funcion que devuelve true si el dia que se le pasa es sabado o domingo
    impl Dia{
        fn fin_de_semana(&self) -> bool {
            match self {
                Dia::Sabado | Dia::Domingo => true,
                _ => false
            }
        }
    }

    //Definis una variable de tipo Dia (objeto JSON) al cual le asignas el elemento lunes
    let hoy: Dia = Dia::Lunes;

    //Solo estableces un match que imprime un mensaje segun el dia que hayas asignado a la variable hoy
    match hoy {
        Dia::Lunes => println!("Odio los lunes"),
        Dia::Martes => println!("Odio los martes"),
        Dia::Miercoles => println!("Odio los miercoles"),
        Dia::Jueves => println!("Odio los jueves"),
        Dia::Viernes => println!("Odio los viernes"),
        Dia::Sabado => println!("Odio los sabados"),
        Dia::Domingo => println!("Me gustan los domingos")
    }

    //Recien aca se prueba la funcion de la linea 111 usando de nuevo la variable hoy
    println!("Es finde?: {}", hoy.fin_de_semana());

}

pub fn vectores(){
    let vector1: Vec<i32> = Vec::new();
    let mut vector2 = vec![1, 2, 3, 4];
    vector2.push(5); //Empuja un nuevo elemento al final del vector

    println!("Primer elemento del vector 2: {}", vector2[0]);

    //let segundo = vector2[1]; //No es necesario inicializar esta variable
    let segundo: i32;
    match vector2.get(1){
        //Este match solo evalua si existe un segundo elemento en el vector recibido de parametro
        //Podes cambiar vector2.get(1) por vector1.get(1) para ver las diferencias
        Some(segundo) => println!("Segundo elemento del vector 2: {}", segundo),
        None => println!("No hay un segundo elemento en el vector 2")
    }

    for i in &mut vector2{ //Se coloca &mut para indicar que la variable indice es mutable
        *i *= 2; //Esto lo que hace es el producto de cada elemento del vector * 2
    }
    for i in vector2{
        println!("{}", i);
    }
}
