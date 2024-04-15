use std::io;

fn main() {
    //hier können alle fertigen Funktionen aufgerufen werden

    let eingabe = benutzerabfrage();
    println!("Deine Eingabe lautet: {}", eingabe);
}

fn benutzerabfrage() -> String {
    let mut input = String::new(); //ein neuer String in dem der Benutzerinput gespeichert** wird
    println!("Enter your text!"); //Anzeigetext bei Benutzereingabe. Dringend notwendig, sonst erscheint ein user input ohne Anweisungstext
    io::stdin()
        .read_line(&mut input) //readline speichert** die eingelesen Zeilen im String "input" !!
        .expect("You did not enter a valid string!"); //Fehlerbehandlung evtl. auch einfach .unwrap()
    let eingabe_string = input.trim().to_string(); // .trim() wird auf &str aufgerufen // .to_String  &str wird in einen String umgewandelt
    eingabe_string //der eingegebene Wert wird als String (nicht String slice &str) zurückgegeben
}

// fn vokal_abfrage() -> bool {}
