
fn main(){
    println!("Selecciona pastilla Roja u Azul");
    let mut pill:String = String::new();
    std::io::stdin().read_line(&mut pill).unwrap();
    pill = pill.trim().to_string();

    if pill == "Roja" {
        println!("Con la pastilla {} Sales de la Matrix", pill);
    } 
    else if pill == "Azul" {
        println!("Con la pastilla {} Sigues viviendo una fantasia",pill)
    }else {
        println!("Tomaste cualquier cosa .. se acaba el mundo.. la jodiste toda.")
    }

}

