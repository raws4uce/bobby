use std::collections::HashMap;
use util_aoc::aoc_io;

fn main() {
    aoc_io::lickballs();
    //0-4/
    let mut v1: Vec<i32> = Vec::new();
    let mut h1: HashMap<i32, i32> = HashMap::new();
    //7-12
    let mut v2: Vec<i32> = Vec::new();
    let mut h2: HashMap<i32, i32> = HashMap::new();
    let _ = aoc_io::process_file("headieone.txt".to_string(), |line| {
        let mut nom = String::new();
        for i in 0..5 {
            nom.push(line.chars().nth(i).unwrap());
        }
        let nom: i32 = nom.trim().parse().unwrap();
        if let Some(c) = h1.get(&nom) {
            h1.insert(nom, c + 1);
        } else {
            h1.insert(nom, 1);
        }
        //v1.push(nom);
    });
    let _ = aoc_io::process_file("headieone.txt".to_string(), |line| {
        let mut nom = String::new();
        for i in 8..13 {
            nom.push(line.trim_start().chars().nth(i).unwrap());
        }
        let nom: i32 = nom.trim().parse().unwrap();
        if let Some(c) = h2.get(&nom) {
            h2.insert(nom, c + 1);
        } else {
            h2.insert(nom, 1);
        }
        //v2.push(nom);
    });

    println!("{:?} \n\n {:?}", h1, h2);
    //v1.sort();
    //v2.sort();
    assert_eq!(v1.len(), v2.len());
    //let answer: i32 = v2.iter().zip(v1.iter()).map(|(l, r)| (l - r).abs()).sum();
    //1st issue, we do not iterate far enough
    //let ans1: i32 = h2
    //    .iter()
    //    .map(|(k2, v2)| {
    //        let mut ans: i32 = 0;
    //        if let Some(v) = h1.get(k2) {
    //            ans = v * k2;
    //        }
    //        //println!("{}", ans1);
    //        ans
    //    })
    //    .sum();
    //
    let ans2: i32 = h1
        .iter()
        .map(|(k1, v1)| {
            let mut ans: i32 = 0;
            if let Some(v) = h2.get(k1) {
                ans= v * k1;
            println!("before i develop space aids, I have a list of statements, if and only if they are true, we would have cured space aids...\n 
                {} (the key from the HM1) appears in HM2 {} times, therefore is {}",k1,v,ans);
            }else {
                println!("{} (the key from HM1) does not appear in HM2, so contributes 0",k1);
            }
            //println!("{}", ans1);
            ans * v1
        })
        .sum();
    println!("ans{} ", ans2);
}
//16962237

//21559164
//19694173
//2181649
//1931479
//1791065
//1730436
//1375018
