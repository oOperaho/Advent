use std::fs;

fn main() {
    let bingo = fs::read_to_string("/home/operaho/Advent/D4/adv.txt").unwrap();
    let f = fs::read_to_string("/home/operaho/Advent/D4/instructions.txt").unwrap();
    let inst: Vec<&str> = f.split(",").collect();
    let mut v = Vec::new();
    let mut c = 0;

    for i in bingo.lines() {
        if i != "" {
            v.push([i])
        }
    }

    println!("{:?}", v);
}
