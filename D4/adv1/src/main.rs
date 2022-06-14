use std::fs;

fn main() {
    let bingo = fs::read_to_string("/home/operaho/Advent/D4/adv.txt").unwrap();
    let f = fs::read_to_string("/home/operaho/Advent/D4/instructions.txt").unwrap();
    let inst: Vec<&str> = f.split(",").collect();
    let mut v0 = Vec::new();
    let mut v1 = Vec::new();
    let (mut c0, mut c1, mut c2): (usize, usize, usize) = (0, 0, 0);

    for i in bingo.lines() {
        if i != "" {
            v0.push(i)
        }
    }

    for j in 0..v0.len() {
        if j == c0 {
            let slice = &v0[c0..c0+5];
            v1.push(slice);
            c0 += 5
        }
    }

    for x in &v1 {
        // snip
    }

    // println!("{:?}", v1;
}
