use std::io;

fn suma(a: i32, b: i32) -> i32 {
    a + b
}

fn roznica(a: i32, b: i32) -> i32 {
    a - b
}

fn iloczyn(a: i32, b: i32) -> i32 {
    a * b
}

fn iloraz(a: i32, b: i32) -> i32 {
    a / b
}

fn main() {

    println!("Podaj współczynnik a");
    let mut inputa = String::new();
    io::stdin().read_line(&mut inputa).expect("Nie udałos ię odczytać linii");

    println!("Podaj współczynnik b");
    let mut inputb = String::new();
    io::stdin().read_line(&mut inputb).expect("Nie udałos ię odczytać linii");
    
    let  inputa: i32 = inputa.trim().parse().expect("Proszę podać pierwszą liczbę: ");
    let  inputb: i32 = inputb.trim().parse().expect("Proszę podać drugą liczbę: ");

    println!("Jaką operację chcesz wykonać : dodawanie (+) , odejmowanie (-), mnożenie (*), dzielenie (/)");
    let mut operacja = String::new();
    io::stdin().read_line(&mut operacja).expect("Nie udało ci się odczytać linii");


    let textdod:&str = "+";
    let textode:&str = "-";
    let textmn:&str = "*";
    let textdziel:&str = "/";
    println!("{}", operacja);

    match operacja.as_str() {
    "textdod" => {
            println!("dodaj");
            let wynik = suma(inputa, inputb);

            println!("Wynik dodawania współczynników: {} oraz {} to {}", inputa, inputb, wynik);
            }
        
    "textode" => {
        println!("odejmij");
            let wynik = roznica(inputa, inputb);

            println!("Wynik odejmowania współczynników: {} oraz {} to {}", inputa, inputb, wynik);
        }
    "textmn" => {
            println!("iloczyn");
        //    println!("Wprowadziłeś: {} + {}", inputa.trim(), inputb.trim());
        }
        "textdziel" => {
            println!("textdziel");
            //    println!("Wprowadziłeś: {} + {}", inputa.trim(), inputb.trim());
        }
        _ =>{
                println!("Noichuj");
        }
    }
}

