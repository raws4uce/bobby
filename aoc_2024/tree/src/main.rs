use util_aoc::aoc_io;
fn main() {
    aoc_io::lickballs();
    let mul_patt: [u8; 4] = [109, 117, 108, 40]; //mul(
    let do_patt: [u8; 4] = [100, 111, 40, 41];
    let dont_patt: [u8; 7] = [100, 111, 110, 39, 116, 40, 41];
    let mut ans = 0;
    let mut will_do = true;
    let _ = aoc_io::process_file("mule.txt".to_string(), |line| {
        //ts about parsing/maybe deserialisation which is why im thinking about using serde, in the
        //same breath, that may be over complicating things
        line.as_bytes().windows(7).enumerate().for_each(|win| {
            println!("{:?}", win.1 == dont_patt);
            //println!("{:?}", win);
            if win.1 == dont_patt {
                will_do = false;
            }
            let mordo_patt = line[win.0..win.0 + 4].as_bytes();
            if mordo_patt == do_patt {
                will_do = true;
            } else if mordo_patt == mul_patt && will_do {
                let num = parse_tuple(&line[win.0 + 4..win.0 + 12]);
                ans = ans + num;
            }
        });
    });
    println!("{ans}");
}
//got lazy
fn parse_tuple(s: &str) -> usize {
    let mut n = s.split(",");
    //println!(".                 {:?}", s);
    let first = n.nth(0);
    let second = n.next();

    let mut n1 = String::new();
    let mut n2 = String::new();

    if let Some(first) = first {
        for c in first.chars() {
            if c == ',' {
                break;
            } else if c.is_numeric() {
                n1.push(c);
            } else {
                return 0;
            }
        }
    }

    if let Some(second) = second {
        for c in second.chars() {
            if c == ')' {
                break;
            } else if c.is_numeric() {
                n2.push(c);
            } else {
                return 0;
            }
        }
    }
    //println!("{}, {}", n1, n2);
    let n1: usize = n1.parse().unwrap();
    let n2: usize = n2.parse().unwrap();
    //println!("{} * {} = {}", n1, n2, n1 * n2);
    n1 * n2
}
//pt1
//197793119
//pt2
//95786593 >
//86830302 < //was being stupid
