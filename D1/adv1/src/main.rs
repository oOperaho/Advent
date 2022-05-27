use std::fs;

fn main() {
    let f = fs::read_to_string("/home/operaho/Advent/D1/adv.txt").unwrap();
    let mut v = Vec::new();
    let mut c = 0;

    for i in f.lines() {
        v.push(i);
    }

    for mut x in 1..v.len() {
        if v[x] > v[x - 1] {
            c += 1;
        } else {
            x += 1;
        }
    }

    println!("{}", c);
}
