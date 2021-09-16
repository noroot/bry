extern crate chrono;
use std::f64::consts::PI;
use std::env;
use chrono::{DateTime, NaiveDate, NaiveDateTime, NaiveTime, Utc, Timelike};
use chrono::format::ParseError;

// Wikipedia article as reference https://en.wikipedia.org/wiki/Biorhythm_(pseudoscience)
fn getBio(daysPassed: &f64) -> (f64, f64, f64) {
    let physical = ((2.0 * PI * daysPassed / 23.0) as f64).sin();
    let emo = ((2.0 * PI * daysPassed / 28.0) as f64).sin();
    let mind = ((2.0 * PI * daysPassed / 33.0) as f64).sin();
    return (physical, emo, mind);
}

fn calc(datePoint: &String) -> (f64, f64, f64) {

    let now = Utc::now();
    let birthDate = NaiveDateTime::parse_from_str(&datePoint, "%Y-%m-%d %H:%M:%S").unwrap();
    let bdu = DateTime::<Utc>::from_utc(birthDate, Utc);
    let diff = now.signed_duration_since(bdu);
    let daysPassed = diff.num_days() as f64;

    // Wikipedia article as reference https://en.wikipedia.org/wiki/Biorhythm_(pseudoscience)
    return getBio(&daysPassed);
}

fn main() -> Result<(), ParseError> {
    println!("BRY: biorythm calculator for terminal");

    let now = Utc::now();
    let args : Vec<String> = env::args().collect();
    let date = format!("{} 00:00:00", &args[1]);
    let birthDate = NaiveDateTime::parse_from_str(&date, "%Y-%m-%d %H:%M:%S")?;
    let bdu = DateTime::<Utc>::from_utc(birthDate, Utc);
    let diff = now.signed_duration_since(bdu);
    let daysPassed = diff.num_days() as f64;

    // Wikipedia article as reference https://en.wikipedia.org/wiki/Biorhythm_(pseudoscience)
    let physical = ((2.0 * PI * daysPassed / 23.0) as f64).sin();
    let emo = ((2.0 * PI * daysPassed / 28.0) as f64).sin();
    let mind = ((2.0 * PI * daysPassed / 33.0) as f64).sin();

    println!("{:?}", birthDate);
    println!("Days: {}", daysPassed);
    println!("PHY {:?}", physical);
    println!("EMO {:?}", emo);
    println!("MIND {:?}", mind);

    let v = calc(&date);
    println!("{:?}", v);

    let mut i = 0;
    let mut d;
    let mut cpd = daysPassed;
    while i < 10 {
        cpd += 1.0;
        d = getBio(&cpd);
        println!("{:?}", d);
        i += 1;
    }

    Ok(())
}
