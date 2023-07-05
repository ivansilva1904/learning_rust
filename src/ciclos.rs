
pub fn arrays2(){
    let array2 = [1, 2, 3, 4, 5, 6, 7, 8, 9];
    let mut indice = 0;
    loop {
        if array2[indice] % 2 == 0{ //Mira que pasa cuando cambias el == por !=
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

pub fn ciclo_while(){
    let array3 = ['a', 'b', 'c', 'd', 'e'];
    let mut indice = 0;
    while (indice < array3.len()){
        println!("Elemento de la posicion {}: {}", indice, array3[indice]);
        indice += 1;
    }
}

pub fn ciclo_for(){
    let array4 = ['a', 'b', 'c', 'd', 'e'];
    for valor in array4.iter(){ //Este iter permite pasar al siguiente elemento del array,
        println!("Elemento: {}", valor); // mientras que for itera hasta que se acaben los valores dentro de este
    }
}
