use std::io;

struct EvenNumbers {
    current: u32,
    max: u32,
}

impl EvenNumbers {
    fn new(max: u32) -> Self {
        EvenNumbers { current: 0, max }
    }
}

impl Iterator for EvenNumbers {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        self.current += 2;
        if self.current <= self.max {
            Some(self.current)
        } else {
            None
        }
    }
}

fn main() {
    println!("Podaj liczbę do jakiej ma się dziać iteracja: ");
    let mut max = String::new();
    io::stdin()
        .read_line(&mut max)
        .expect("Wartość nie jest poprawna.");

    let max: u32 = max
        .trim()
        .parse()
        .expect("Nie wybrałeś liczby");

    let even_numbers = EvenNumbers::new(max);

    for item in even_numbers {
        println!("{}", item);
    }

    let sum: u32 = EvenNumbers::new(max).sum();
    println!("Suma liczb parzystych do podanej liczby wynosi: {}", sum);
}
