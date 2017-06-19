
extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    /*
    let x = rand::random::<u8>();
println!("{}", x);

let y = rand::random::<f64>();
println!("{}", y);

if rand::random() { // generates a boolean
    println!("Better lucky than good!");
}
*/

    println!("Fibonacci-Zahlen");
    //let geheime_zahl = rand::thread_rng().gen_range(1, 101);

   //println!("Die geheime Zahl ist: {}", geheime_zahl);

/*loop {
    println!("Bitte gib deine Vermutung ein.");

    let mut vermutung = String::new();


    io::stdin().read_line(&mut vermutung)
        .ok()
        .expect("Fehler beim Lesen der Zeile");

        let vermutung: u32 = vermutung.trim().parse()
         .ok()
         .expect("Bitte eine Zahl eintippen!");*/


         println!("Bitte gib die Anzahl der zu berechnenden Zahlen ein:");

         let mut anzahl = String::new();
    io::stdin().read_line(&mut anzahl)
        .ok()
        .expect("Fehler beim Lesen der Zeile");
        let anzahl: u32 = anzahl.trim().parse()
         .ok()
         .expect("Bitte eine Zahl eintippen!");

    println!("Die Anzahl: {}", anzahl);
    let mut i = 1;
    let mut fst = 0;
    let mut sec = 1;
    let mut res;
    loop {
        res = fst + sec;
        println!("Die {}. Zahl ist:{}",i,res);i+=1;
        fst=sec; sec=res;
        if i>anzahl {break};
    }


    /*match vermutung.cmp(&geheime_zahl) {
           Ordering::Less    => println!("Zu klein!"),
           Ordering::Greater => println!("Zu groß!"),
           Ordering::Equal   => {println!("Gewonnen!");break;}
       } }*/

}
