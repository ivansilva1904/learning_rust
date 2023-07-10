
//Son funciones almacenadas en una variable

pub fn clousure1(){
    let puede_votar = |edad: i32| {
        edad >= 18
    };
    println!("Puede votar con 24 años? {}", puede_votar(24));
}

pub fn clousure2(){
    let mut valor1 = 1;
    println!("valor 1: {}", valor1);

    valor1 = 4;
    let imprimir_valor1 = || println!("Valor 1: {}", valor1); //No es necesario el uso de brackets para una sola sentencia

    imprimir_valor1();

    valor1 = 3;

    let mut valor2 = valor1;
    println!("valor 2: {}", valor2);
    valor2 = 10;

    valor2 = 5;
    valor2 = 3;
    let mut imprimir_valor2 = || println!("Nuevo valor de valor 2: {}", valor2);

    imprimir_valor2();

    valor2 = 15;

    println!("Valor 2: {}", valor2);
}
//Ownership es bastante complicado

pub fn clousure3(){
    let mut valor1 = 5;
    let imprimir_valor1 = || println!("Valor 1: {}", valor1);

    imprimir_valor1();

    valor1 = 1;

    let mut modificar_valor1 = || valor1 += 1;
    
    modificar_valor1();

    println!("Valor 1: {}", valor1);

    valor1 = 3;

    println!("valor 1: {}", valor1);

}
