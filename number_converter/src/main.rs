fn römisch_zu_arabisch(s: &str) -> Option<i32> {
    let mut total = 0;
    let mut prev_value = 0;

    let römische_ziffern = vec![
        ('M', 1000),
        ('D', 500),
        ('C', 100),
        ('L', 50),
        ('X', 10),
        ('V', 5),
        ('I', 1),
    ];

    for c in s.chars().rev() {
        if let Some(&(_, value)) = römische_ziffern.iter().find(|&&(ch, _)| ch == c) {
            if value < prev_value {
                total -= value;
            } else {
                total += value;
            }
            prev_value = value;
        } else {
            return None; // Ungültige römische Ziffer
        }
    }

    Some(total)
}

fn arabisch_zu_roemisch(arabisch: u32) -> String {
    let roemisch = vec![
        (1000, "M"),
        (900, "CM"),
        (500, "D"),
        (400, "CD"),
        (100, "C"),
        (90, "XC"),
        (50, "L"),
        (40, "XL"),
        (10, "X"),
        (9, "IX"),
        (5, "V"),
        (4, "IV"),
        (1, "I"),
    ];

    let mut zahl = arabisch;
    let mut ergebnis = String::new();

    for (wert, symbol) in roemisch {
        while zahl >= wert {
            ergebnis.push_str(symbol);
            zahl -= wert;
        }
    }

    ergebnis
}

fn main() {
    let römische_zahl = "MCMXCIV";
    match römisch_zu_arabisch(römische_zahl) {
        Some(arabische_zahl) => println!("Die arabische Zahl zu {} ist: {}", römische_zahl, arabische_zahl),
        None => println!("Ungültige römische Zahl"),
    }

    let arabische_zahl = 2025;
    let roemische_zahl = arabisch_zu_roemisch(arabische_zahl);
    println!("Die römische Zahl für {} ist {}", arabische_zahl, roemische_zahl);
}