use rand::Rand;

pub mod rand;

fn main() {
    let mut ran = Rand::new(3);

    for i in 0..9 {
        println!("{}: {}", i, ran.gen())
    }
}
