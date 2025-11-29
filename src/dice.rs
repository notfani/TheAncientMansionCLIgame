use rand::Rng;

pub fn roll_1d20() -> u8 {

    let mut rng = rand::rng();
    let roll: u8 = rng.random_range(1..=20);
    println!("Вы бросаете кость и получаете: {}", roll);
    roll
}