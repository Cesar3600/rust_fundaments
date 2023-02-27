
fn main(){
    let numero_1:String = "123".to_string();
    let numero_2:String = "321".to_string();


   let num1:i32 = numero_1.trim().parse().unwrap();
   let num2:i32  = numero_2.trim().parse().unwrap();

   let suma = num1 + num2;

    // mostrar los dos numeros en pantalla

    loop{
        println!("por favor escribir la suma de {} y {}",numero_1,numero_2); 
    
        let mut resultado:String = String::new();
        std::io::stdin().read_line(&mut resultado).unwrap();
        let resultado_int:i32 = resultado.trim().parse().unwrap();
    
        if resultado_int == suma {
            println!("el resultado: {} es correcto",resultado_int);
            break;
        }else{
            println!("El resultado {} no es correcto",resultado_int)
        }
    }
}