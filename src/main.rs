use std::io;

fn main() {
    //hier können alle fertigen Funktionen aufgerufen werden

    let eingabe = benutzerabfrage("your Text");
    println!("Deine Eingabe lautet: {}", eingabe);

    vokal_abfrage();
}

fn benutzerabfrage(prompt: &str) -> String {
    //enthält Übergabeparameter, dass je nach Anforderung anderes (Tag, Stunde, etc.) abgefragt werdn kann
    let mut input = String::new(); //ein neuer String in dem der Benutzerinput gespeichert** wird
    println!("Enter {}!", prompt); //Anzeigetext bei Benutzereingabe. Dringend notwendig, sonst erscheint ein user input ohne Anweisungstext
    io::stdin() //aus io::stdin
        .read_line(&mut input) //readline speichert** die eingelesen Zeilen im String "input", stnd als &str!! könnte auch andere Variablen nehmen
        .expect("You did not enter a valid string!"); //Fehlerbehandlung evtl. auch einfach .unwrap()
    let eingabe_string = input.trim().to_string(); // .trim() wird auf &str aufgerufen // .to_String  &str wird in einen String umgewandelt
    eingabe_string //der eingegebene Wert wird als String (nicht String slice &str) zurückgegeben
}

fn vokal_abfrage() -> bool {
    let name = benutzerabfrage("your name"); //als Ausgabe erhalte ich jetzt einen Namen, mit dem ich weiter arbeiten kann
    let first_character = name.chars().nth(0); //somit wird der erste char in name als neue Variable gespeichert

    /*  if first_character == Some('A') {
        println!("Your name starts with A!");
        true
    } else if first_character == Some('a') {
        println!("Your name starts with a!");
        true
    } else {
        println!("Your name doesnt start with A or a! ");
        false
    }
    */

    match first_character {
        //die {} braucht man hier nicht unbedingt, nur sobald man mehr als eine Aktion ausgeführt haben will
        //statt if kann auch ein match konstrukt verwendet werden
        Some('A') => {
            //Vergleicht Muster1 (A) mit dem Wert (first_character), wenn true, wird die Aktion ausgeführt, sonst nächsten case abfragen
            println!("Starts with A!");
            true
        }
        Some('a') => {
            println!("Starts with a!");
            true
        }
        Some('B') => {
            println!("Starts with B!");
            true
        }
        _ => {
            println!("Doesnt start with A,a,B");
            false // Ein Platzhalter für den Fall, dass das erste Zeichen weder 'A' noch 'a' ist
        }
    }
}
