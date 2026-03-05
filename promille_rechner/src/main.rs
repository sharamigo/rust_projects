use std::io;

fn main() {
    let mut geschlecht = String::new();
    let mut koerpergewicht = String::new();
    let mut anzahl_bier = String::new();   let mut anzahl_wein = String::new();
    let mut anzahl_longdrink = String::new();
    let mut anzahl_schnaps = String::new();

    println!("Bitte geben Sie Ihr Geschlecht ein (m/w):");
    io::stdin().read_line(&mut geschlecht).expect("Fehler beim Lesen der Eingabe");
    
    println!("Bitte geben Sie Ihr Körpergewicht in kg ein:");
    io::stdin().read_line(&mut koerpergewicht).expect("Fehler beim Lesen der Eingabe");
    
    println!("Bitte geben Sie die Anzahl an BIER (0,5L) an die Sie getrunken haben:");
    io::stdin().read_line(&mut anzahl_bier).expect("Fehler beim Lesen der Eingabe");
    
    println!("Bitte geben Sie die Anzahl an WEIN (0,2L) an die Sie getrunken haben:");
    io::stdin().read_line(&mut anzahl_wein).expect("Fehler beim Lesen der Eingabe");
    
    println!("Bitte geben Sie die Anzahl an LONGDRINKS (0,2L) an die Sie getrunken haben:");
    io::stdin().read_line(&mut anzahl_longdrink).expect("Fehler beim Lesen der Eingabe");
    
    println!("Bitte geben Sie die Anzahl an SCHNAPPES (0,02L) an die Sie getrunken haben:");
    io::stdin().read_line(&mut anzahl_schnaps).expect("Fehler beim Lesen der Eingabe");

    let geschlecht = geschlecht.trim();
    let koerpergewicht: f32 = koerpergewicht.trim().parse().expect("Bitte eine gültige Zahl eingeben");
    let anzahl_bier: f32 = anzahl_bier.trim().parse().expect("Bitte eine gültige Zahl eingeben");
    let anzahl_wein: f32 = anzahl_wein.trim().parse().expect("Bitte eine gültige Zahl eingeben");
    let anzahl_longdrink: f32 = anzahl_longdrink.trim().parse().expect("Bitte eine gültige Zahl eingeben");
    let anzahl_schnaps: f32 = anzahl_schnaps.trim().parse().expect("Bitte eine gültige Zahl eingeben");
    
    let anzahl_getraenke_gesamt: f32 = (anzahl_bier * 20.0) + (anzahl_wein * 18.0) + (anzahl_longdrink * 16.0) + (anzahl_schnaps * 5.0);

    let r = if geschlecht == "m" { 0.68 } else { 0.55 };
    let alkoholgehalt = (anzahl_getraenke_gesamt) / (koerpergewicht * r) - 0.15; //  Alkohol pro Getränk sollte aus den Variablen errechnet sein, 0.15 Abbau pro Stunde

    println!("Ihr geschätzter Blutalkoholgehalt beträgt: {:.2} ‰", alkoholgehalt);
    println!("ZUSAMMENFASSUNG: -------------");
    println!("Anzahl Bier: {}", anzahl_bier);
    println!("Anzahl Wein: {}", anzahl_wein);
    println!("Anzahl Longdrinks: {}", anzahl_longdrink);
    println!("Anzahl Schnaps: {}", anzahl_schnaps);
}
