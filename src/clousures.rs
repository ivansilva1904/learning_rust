
//Son funciones almacenadas en una variable

pub fn clousure1(){
    let puede_votar = |edad: i32| {
        edad >= 18
    };
    println!("Puede votar con 24 años? {}", puede_votar(24));
}

pub fn clousure2(){
    let mut valor1 = 1; //La variable valor1 y la funcion imprimir_valor estan dentro del mismo scope
    let imprimir_valor = || println!("Valor 1: {}", valor1); //No es necesario el uso de brackets para una sola sentencia

    imprimir_valor();

    //Tengo que clonar a valor1 porque tan solo usarlo en el closure anterior libera su espacio en memoria
    //En el curso (de hace 11 meses) no era necesario esto
    let mut valor2 = valor1.clone();
    valor2 = 10;
    let mut modificar_valor = || println!("{}", valor2);

    imprimir_valor();

    valor2 = 15;
}
