extern crate rand;

use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {

    let y = rand::thread_rng().gen_range(1,11);

    loop {
        println!("Podaj swoją liczbę:");
        let mut x = String::new();
        io::stdin().read_line(&mut x)
            .expect("Nie podales stringa");

        let x: u32 = match x.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match  x.cmp(&y){
            Ordering::Less => println!("Za mała!"),
            Ordering::Greater => println!("Za duża!"),
            Ordering::Equal => {
                println!("Zgadłeś!");
                break;}

        }
        println!("Twoja liczba to {} a moja to {}", x,y);
    }
}
