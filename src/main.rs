
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
    
    if edad_int >= 18 && edad_int != 30 {
        println!("{}, tienes {} puedes ingresar a la discoteca", nombre, edad_int)
    } else if edad_int < 18 {
        println!("Aun eres menor de edad, no puedes ingresar")
    }else if edad_int ==30 {
        println!("no me caen los de 30, no entras!! ")
    }

    //println!("Buenos dias {}, ya tienes {} de edad",nombre,edad_int);
    

}

