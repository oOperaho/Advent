use std::fs;

fn main() {
    let file = fs::read_to_string("/home/operaho/Advent/D2/adv.txt").unwrap();
    let mut x = 0;
    let mut y = 0;
    let mut v = Vec::new();

    for i in file.lines() {
        let vec: Vec<&str> = i.split(" ").collect();
        v.push(vec);
    }

    for j in v {
        let inst = j[0];
        let num: i32 = j[1].parse().unwrap();
        if inst == "up" {
            y -= num;
        } else if inst == "down" {
            y += num;
        } else {
            x += num;
        }
    }

    let res = x * y;

    println!("{}", res);
}
