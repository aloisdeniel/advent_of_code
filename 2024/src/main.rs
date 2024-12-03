mod d1;
mod d2;

fn main() {
    let (d1a, d1b) = d1::run();
    println!("Day 01: {}, {}", d1a, d1b);

    let (d2a, d2b) = d2::run();
    println!("Day 02: {}, {} ", d2a, d2b);
}
