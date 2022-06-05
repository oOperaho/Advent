use std::fs;

fn main() {
    let bingo = fs::read_to_string("/home/operaho/Advent/D4/adv.txt").unwrap();
    let inst = fs::read_to_string("/home/operaho/Advent/D4/instructions.txt").unwrap();
    let mut v = Vec::new();

    for i in bingo.lines() {
        v.push([i])
    }

    println!("{:?}", v);
}
