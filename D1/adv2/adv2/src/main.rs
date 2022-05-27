use std::fs;
use std::convert::TryInto;

fn main() {
    let file = fs::read_to_string("/home/operaho/Advent/D1/adv.txt").unwrap();
    let mut v = Vec::new();
    let mut count = 0;

    for i in file.lines() {
        v.push(i);
    }

    let v: Vec<u32> = v.iter().map(|k| k.parse::<u32>().unwrap()).collect();

    for x in 1..1998 {
        let n1 = &v[x..x + 3];
        let [a, b, c]: [u32; 3] = n1.try_into().ok().unwrap();
        let s1 = a + b + c;
        let n2 = &v[x - 1..x + 2];
        let [d, e, f]: [u32; 3] = n2.try_into().ok().unwrap();
        let s2 = d + e + f;

        if s1 > s2 {
            count += 1;
        }

    }

    println!("{}", count);
}
