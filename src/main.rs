
fn main(){
    println!("ingresa tu nombre:");
    let mut nombre:String = String::new();
    std::io::stdin().read_line(&mut nombre).unwrap();
    nombre = nombre.trim().to_string();

    println!("ingresa tu edad:");
    let mut edad:String = String::new();
    edad = edad.trim().to_string();
    std::io::stdin().read_line(&mut edad).unwrap();

    let edad_int:u8 = edad.trim().parse().unwrap();
    
    println!("Buenos dias {}, ya tienes {} de edad",nombre,edad_int);
    

}

