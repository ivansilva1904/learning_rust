
use std::collections::HashMap;

//Pares clave-valor
pub fn hashmap(){
    let mut heroes = HashMap::new();

    heroes.insert("Superman", "Clark kent");
    heroes.insert("Batman", "Bruce Wayne");
    heroes.insert("Spiderman", "Peter Parker");

    for (clave, valor) in heroes.iter(){
        println!("{} = {}", clave, valor);
    }

    println!("Elementos del hashmap: {}", heroes.len());
}

pub fn hashmap2(){
    let mut heroes = HashMap::new();

    heroes.insert("Superman", "Clark kent");
    heroes.insert("Batman", "Bruce Wayne");
    heroes.insert("Spiderman", "Peter Parker");

    if(heroes.contains_key("Batman")){
        let heroe_batman = heroes.get("Batman");

        match heroe_batman{ //Este match es algo redundante. De por si al entrar a la condicional se da por seguro que Batman se encuentra en el hashmap heroes
            Some(x) => println!("Batman es un heroe"),
            None => println!("Batman no es un heroe")
        }
    }
}

//Estructura de datos similar a un documento JSON
pub fn struct1(){
    struct Cliente{ //Se define la estructura
        nombre: String,
        direccion: String,
        saldo: f32
    }

    let mut ivan = Cliente{
        nombre: String::from("Ivan Silva"),
        direccion: String::from("Calle falsa 123"),
        saldo: 1000.00
    };

    println!("Nombre: {}; Direccion: {}; Saldo: {}", ivan.nombre, ivan.direccion, ivan.saldo);

    ivan.direccion = String::from("Calle Wallaby 42, Sydney");

    println!("Nombre: {}; Direccion: {}; Saldo: {}", ivan.nombre, ivan.direccion, ivan.saldo);
}

pub fn struct2(){ //Esta vez usando genericos
    struct Rectangulo<T, U>{
        largo: T,
        ancho: U
    }

    let rectangulo1 = Rectangulo{
        largo: 5,
        ancho: 1.5
    };

    println!("Largo del rectangulo: {} \nAncho del rectangulo: {}", rectangulo1.largo, rectangulo1.ancho);
}

pub fn traits1(){ //Aca tambien se podria aplicar genericos
    const PI: f32 = 3.141592;
    //Esto viene a ser como una clase padre en objetos. Define contrato de comportamiento y caracteristicas
    trait Figura {
        fn new(largo: f32, ancho: f32) -> Self; //Esto viene a ser un constructor
        fn area(&self) -> f32; //Esto es como un metodo que devuelve el area de la figura
    }

    //Estas vienen a ser las clases que heredan de la clase padre Figura
    struct Rectangulo{
        largo: f32,
        ancho: f32
    }
    struct Circulo{
        largo: f32,
        ancho: f32
    }

    //Aca es donde definis que comportamiento especifico va a tener cada metodo de las figuras
    impl Figura for Rectangulo{
        fn new(largo: f32, ancho: f32) -> Rectangulo{
            return Rectangulo{largo, ancho};
        }
        fn area(&self) -> f32{
            return self.ancho * self.largo;
        }
    }
    impl Figura for Circulo{
        fn new(largo: f32, ancho: f32) -> Circulo{
            return Circulo{largo, ancho};
        }
        fn area(&self) -> f32{
            return (self.largo / 2.0).powf(2.0) * PI;
        }
    }

    //Luego "instancias" un objeto de cada clase
    let rectangulo1: Rectangulo = Figura::new(5.0, 1.5);
    let circulo1: Circulo = Figura::new(12.0, 5.3);

    //Por ultimo, mostras el funcionamiento del metodo area
    println!("Area rectangulo: {}", rectangulo1.area());
    println!("Area circulo: {}", circulo1.area());
}
