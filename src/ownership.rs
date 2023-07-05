
//Acerca de la pertenencia u ownership
/* Reglas
1. Cada valor se asigna a una variable y se dice que su dueño es el scope donde se encuentra
2. Una variable puede tener solo un dueño a la vez
3. Cuando se pierde el dueño (se sale del scope), entonces se pierde el valor almacenado en la variable */

//Rust constantemente libera recursos que no se utilizan

//Esto es un ejemplo de como rust libera espacio en memoria cuando lo cree necesario
pub fn copiar_strings(){ //No se supone que ejecutes esto
    let string1 = String::from("Hola");
    //Al hacer esta copia, directamente estamos moviendo el valor de string1 a string 2
    let string2 = string1;
    //println!("{}", string1); Lanza un error que dice que el valor se movio
    println!("{}", string2);
}

//Esto es un ejemplo de como copiar (clonar) un valor correctamente
pub fn clonar_strings(){
    let string1 = String::from("Hola de nuevo");
    let string2 = string1.clone(); //Metodo para clonar valores

    println!("{}", string1);
    println!("{}", string2);
} //Nota: No aplica para tipos de datos: enteros, flotantes, booleanos, tuplas, caracteres
//Si aplica para strings, arrays, etc

pub fn imprimir_string(x: String){
    println!("{}", x);
}

pub fn imprimir_devolver_string(x: String) -> String{
    println!("{}", x);
    x
}

pub fn modificar_string(x: &mut String){
    x.push_str(", como estas?");
    println!("{}", x);
}

pub fn implementar_strings(){ //Funcion que utiliza las 3 funciones anteriores
    let string1 = String::from("Hola");

    //Funcion imprimir_string
    imprimir_string(string1.clone()); //Mira que son todos clones de string1 en esta linea, la 50, y 54

    //Funcion imprimir_devolver_string
    let string2 = imprimir_devolver_string(string1.clone());
    println!("{}", string2);

    //Funcion modificar_string
    let mut string3 = string1.clone(); //Mira que ocurre cuando intentas copiarlo en lugar de clonarlo
    modificar_string(&mut string3);
}
