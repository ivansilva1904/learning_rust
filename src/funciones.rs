
pub fn hola(){
    println!("Hola");
}

pub fn suma(x: i32, y: i32){
    println!("{} + {} = {}", x, y, x + y);
}

pub fn suma2(x: i32, y: i32) -> i32 { //Devolves un tipo i32
    x + y //De esta forma se devuelve valores y no es necesario el ; al final. Tambien se puede usar el return
}

pub fn devolver_multiples_valores(x: i32) -> (i32, i32) { //Indicas que devolves un conjunto de valores i32
    return (x + 1, x + 2); //Tambien podes poner solo (x + 1, x + 2) para retornar
}

pub fn suma_vector(vector1: Vec<i32>) -> i32 {
    let mut acumulador = 0;
    for valor in vector1.iter(){
        acumulador += valor;
    }
    acumulador
}

use std::ops::Add; //Libreria necesaria para trabajar con genericos
/*Un generico basicamente es un tipo de dato creado por el usuario que puede amoldarse a cualquier
otro tipo de dato que le pasemos. En el main a esta funcion le pasamos tanto datos enteros como
flotantes sin problema*/
pub fn suma_generico<T: Add<Output = T>>(x: T, y: T) -> T {
    return x + y;
}