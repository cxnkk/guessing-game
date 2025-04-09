fn main() {
    let x = 42;
    println!("Der Wert der Variable x ist {}", x);
    let x = 11; // Shadowing
    println!("Der Wert der Variable x ist {}", x);
    let mut y = 97; // Mutable
    println!("Der Wert der Variable y ist {}", y);
    y = 3456;
    println!("Der Wert der Variable y ist {}", y);
    
    const LEVEL:i32 = -88865; // Integer, positive sowie negative Zahlen möglich mit Type i
    println!("Das Level beträgt {}", LEVEL);
    const LEVEL2:u32 = 12345; // Unsigned Integer, nur positive Zahlen möglich mit Type u
    println!("Das Level beträgt {}", LEVEL2);
    let geschwindigkeit = 1.5; // Floating point numbers
    println!("Deine Geschwindigkeit beträgt {} km/h", geschwindigkeit);
    let booleans = true; // True oder false
    println!("Dieser Boolean ist {}", booleans);
    let character:char = 'a'; // Character, nur ein einzelnder Buchstabe möglich
    println!("Hier wird der Buchstabe {} angezeigt", character);

    // komplexe Datentypen
    let s:String = String::from("Hallo Welt!");
    println!("Hi: {}", s);

    let a:[char;4] = ['b','c','d','e'];
    println!("Array demo: {}", a[0]);
}