
mod ordenar_pizza{
    pub struct Pizza{
        pub masa: String,
        pub queso: String,
        pub topping: String
    }

    impl Pizza{
        pub fn constructor(topping: &str) -> Pizza {
            Pizza{
                masa: String::from("Masa simple"),
                queso: String::from("Muzzarela"),
                topping: String::from(topping)
            }
        }
    }

    pub mod ayuda_al_cliente{
        fn sentar_cliente(){
            println!("Cliente sentado en la mesa");
        }
        fn servir_cliente(pizza_cliente: super::Pizza){
            println!("Se sirvio al cliente una pizza de muzzarela con {} de topping", pizza_cliente.topping);
        }
        pub fn tomar_orden(){
            sentar_cliente();
            let pizza_cliente: super::Pizza = super::Pizza::constructor("Cebolla");
            servir_cliente(pizza_cliente);
        }
    }
}

pub fn ordenar_comida(){
    crate::restaurante::ordenar_pizza::ayuda_al_cliente::tomar_orden();
}
