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


    const textdod:&str = "+";
    const textode:&str = "-";
    const textmn:&str = "*";
    const textdziel:&str = "/";

    match operacja.as_str().trim() {
        textdod => {

            let wynik = suma(inputa, inputb);

            println!("Wynik dodawania współczynników: {} oraz {} to {}", inputa, inputb, wynik);
        }
        
        textode => {

            let wynik = roznica(inputa, inputb);

            println!("Wynik odejmowania współczynników: {} oraz {} to {}", inputa, inputb, wynik);
        }
        textmn => {

            let wynik = iloczyn(inputa, inputb);

            println!("Wynik mnożenia współczynników: {} oraz {} to {}", inputa, inputb, wynik);
        }
        textdziel => {
            if inputb == 0{
                println!("Wynik dzielenia współczynników: {} oraz {} to 0", inputa, inputb);
            
            }else{
            let wynik = iloraz(inputa, inputb);

            println!("Wynik dzielenia współczynników: {} oraz {} to {}", inputa, inputb, wynik);
            }
        }
        _ =>{
                println!("No nie !!!");
        }
    }
}

