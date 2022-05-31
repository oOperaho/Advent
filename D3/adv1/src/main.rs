use std::fs;

fn main() {
    let file = fs::read_to_string("/home/operaho/Advent/D3/adv.txt").unwrap();
    let mut c = 1;
    let mut v0 = Vec::new();
    let mut v1 = Vec::new();
    let mut v2 = Vec::new();

    for i in file.lines() {
        v0.push(i);
    }

    while c <= 12 {
        v1.clear();

        for j in &v0 {
            v1.push(&j[c-1..c]);
        }

        let f0 = v1.iter().filter(|&n| *n == "0").count();
        let f1 = v1.iter().filter(|&n| *n == "1").count();

        if f0 > f1 { // change the order to < in order to get the other binary's count
            v2.push(0);
        } else {
            v2.push(1);
        }
        
        c += 1;
    }

    println!("{:?}", v2);
}
