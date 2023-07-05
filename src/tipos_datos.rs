
use rand::Rng;

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

pub fn casteo(){
    let int1_u8: u8 = 5;
    let int2_u8: u8 = 4;
    let int3_u32: u32 = (int1_u8 as u32) + (int2_u8 as u32);

    println!("u8 + u8: {}", int3_u32);
}

