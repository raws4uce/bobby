use util_aoc::aoc_io;

fn main() {
    //proccessing data to be rethought, Now parse this as 3d grid, 142x142 ithink
    aoc_io::lickballs();
    let mut ans = 0;
    let vec = aoc_io::mid_griddy("her.txt".to_string()).unwrap();
    for (i, y) in vec.iter().enumerate() {
        for (j, cell) in y.iter().enumerate() {
            if let Some(cell) = cell {
                if *cell == 'A' {
                    let snapshot = mid_snap(vec.clone(), (i, j));
                    let x = eval_x_mases(&snapshot);
                    ans += x;
                }
            }
        }
    }
    println!("{}", ans);
}

fn eval_x_mases(snap: &Vec<Vec<Option<char>>>) -> usize {
    for row in snap {
        for cell in row {
            match cell {
                Some(c) => print!(" {} ", c),
                None => print!(" . "),
            }
        }
        println!(); // Newline after each row
    }
    let mut flag1 = false;
    let mut flag2 = false;
    if let Some(q) = snap[2][0] {
        if let Some(w) = snap[0][2] {
            if let Some(e) = snap[0][0] {
                if let Some(r) = snap[2][2] {
                    if q != 'X'
                        && q != 'A'
                        && w != 'X'
                        && w != 'A'
                        && e != 'X'
                        && e != 'A'
                        && r != 'X'
                        && r != 'A'
                    {
                        if (q == 'M') ^ (w == 'M') {
                            flag1 = true;
                        }
                        if (e == 'M') ^ (r == 'M') {
                            flag2 = true;
                        }
                    }
                }
            }
        }
    }
    println!(); // Newline after each row
    println!(); // Newline after each row
    println!("f1 {}", flag1); // Newline after each row
    println!("f2 {}", flag2); // Newline after each row
    println!(); // Newline after each row
    println!(); // Newline after each row
    println!(); // Newline after each row
    if flag1 && flag2 {
        1
    } else {
        0
    }
}

fn eval_xmases(snap: &Vec<Vec<Option<char>>>) -> usize {
    //(3,3)(3,4)(3,5)(3,6) E Calm
    let mut east = String::new();
    let mut west = String::new();
    let mut north = String::new();
    let mut north_east = String::new();
    let mut north_west = String::new();
    let mut south = String::new();
    let mut south_west = String::new();
    let mut south_east = String::new();
    //(3,3)(4,3)(5,3)(6,3) S Calm
    //(3,3)(4,4)(5,5)(6,6) SE Calm
    //(3,3)(4,2)(5,1)(6,0) SW Calm
    //soufsVarients
    let y = 3;
    let mut y_se = 3;
    let mut y_sw = 3;
    for x in 3..=6 {
        //Eastside
        if let Some(c) = snap[3][x] {
            east.push(c);
        }
        if let Some(c) = snap[x][y] {
            south.push(c);
        }
        if let Some(c) = snap[x][y_se] {
            south_east.push(c);
        }
        if let Some(c) = snap[x][y_sw] {
            south_west.push(c);
        }
        y_se += 1;
        if x != 6 {
            y_sw -= 1;
        }
    }
    //(3,3)(3,2)(3,1)(3,0) W Backwards
    //Varients which decriments in x's
    //(3,3)(2,4)(1,5)(0,6) NE Backwards
    //(3,3)(2,2)(1,1)(0,0) NW Backwards
    //(3,3)(2,3)(1,3)(0,3) N will be backwards
    let mut y_ne = 3;
    let mut y_nw = 3;
    for x in (0..=3).rev() {
        //west
        if let Some(c) = snap[3][x] {
            west.push(c);
        }
        if let Some(c) = snap[x][y] {
            north.push(c);
        }
        if let Some(c) = snap[x][y_ne] {
            north_east.push(c);
        }
        if let Some(c) = snap[x][y_nw] {
            north_west.push(c);
        }
        y_ne += 1;
        if x != 0 {
            y_nw -= 1;
        }
    }

    println!(); // Newline after each row
    println!(); // Newline after each row
    println!("thturg{}", north);
    println!("n  : {}", north);
    println!("nw : {}", north_west);
    println!("ne : {}", north_east);

    println!("s  : {}", south);
    println!("sw : {}", south_west);
    println!("se : {}", south_east);

    println!("w  : {}", west);
    println!("e  : {}", east);
    println!(); // Newline after each row
    println!(); // Newline after each row
                //let north = revvin(north);
                //let north_west = revvin(north_west);
                //let north_east = revvin(north_east);
                //let west = revvin(west);
                //
    let mut ans = 0;

    if north == "XMAS".to_string() {
        ans += 1;
    }
    if north_west == "XMAS".to_string() {
        ans += 1;
    }
    if north_east == "XMAS".to_string() {
        ans += 1;
    }
    if south == "XMAS".to_string() {
        ans += 1;
    }
    if south_west == "XMAS".to_string() {
        ans += 1;
    }
    if south_east == "XMAS".to_string() {
        ans += 1;
    }
    if west == "XMAS".to_string() {
        ans += 1;
    }
    if east == "XMAS".to_string() {
        ans += 1;
    }
    ans
}

//fn eval_xmases(snap: &Vec<Vec<Option<char>>>) -> usize {
//    // print the grid
//    let mut N = String::new();
//    let mut NW = String::new();
//    let mut NE = String::new();
//    let mut S = String::new();
//    let mut SW = String::new();
//    let mut SE = String::new();
//    let mut E = String::new();
//    let mut W = String::new();
//    for (x, row) in snap.iter().enumerate() {
//        for (y, cell) in row.iter().enumerate() {
//            match cell {
//                Some(c) => print!(" {} ", c),
//                None => print!(" . "),
//            }
//            match (x, y) {
//                (0, 0) => {
//                    if let Some(c) = cell {
//                        NW.push(*c);
//                    }
//                }
//                (0, 3) => {
//                    if let Some(c) = cell {
//                        N.push(*c);
//                    }
//                }
//                (0, 6) => {
//                    if let Some(c) = cell {
//                        NE.push(*c);
//                    }
//                }
//
//                (1, 1) => {
//                    if let Some(c) = cell {
//                        NW.push(*c);
//                    }
//                }
//                (1, 3) => {
//                    if let Some(c) = cell {
//                        N.push(*c);
//                    }
//                }
//                (1, 5) => {
//                    if let Some(c) = cell {
//                        NE.push(*c);
//                    }
//                }
//
//                (2, 2) => {
//                    if let Some(c) = cell {
//                        NW.push(*c);
//                    }
//                }
//                (2, 3) => {
//                    if let Some(c) = cell {
//                        N.push(*c);
//                    }
//                }
//                (2, 4) => {
//                    if let Some(c) = cell {
//                        NE.push(*c);
//                    }
//                }
//                (3, 0) => {
//                    if let Some(c) = cell {
//                        W.push(*c);
//                    }
//                }
//                (3, 1) => {
//                    if let Some(c) = cell {
//                        W.push(*c);
//                    }
//                }
//                (3, 2) => {
//                    if let Some(c) = cell {
//                        W.push(*c);
//                    }
//                }
//                (3, 3) => {
//                    if let Some(c) = cell {
//                        NW.push(*c);
//                        SW.push(*c);
//                        SE.push(*c);
//                        NE.push(*c);
//                        W.push(*c);
//                        E.push(*c);
//                        S.push(*c);
//                        N.push(*c);
//                    }
//                }
//                (3, 4) => {
//                    if let Some(c) = cell {
//                        E.push(*c);
//                    }
//                }
//                (3, 5) => {
//                    if let Some(c) = cell {
//                        E.push(*c);
//                    }
//                }
//                (3, 6) => {
//                    if let Some(c) = cell {
//                        E.push(*c);
//                    }
//                }
//                (4, 2) => {
//                    if let Some(c) = cell {
//                        SW.push(*c);
//                    }
//                }
//                (4, 3) => {
//                    if let Some(c) = cell {
//                        S.push(*c);
//                    }
//                }
//
//                (4, 4) => {
//                    if let Some(c) = cell {
//                        SE.push(*c);
//                    }
//                }
//
//                (5, 1) => {
//                    if let Some(c) = cell {
//                        SW.push(*c);
//                    }
//                }
//                (5, 3) => {
//                    if let Some(c) = cell {
//                        S.push(*c);
//                    }
//                }
//                (5, 5) => {
//                    if let Some(c) = cell {
//                        SE.push(*c);
//                    }
//                }
//
//                (6, 0) => {
//                    if let Some(c) = cell {
//                        SW.push(*c);
//                    }
//                }
//                (6, 3) => {
//                    if let Some(c) = cell {
//                        S.push(*c);
//                    }
//                }
//                (6, 6) => {
//                    if let Some(c) = cell {
//                        SE.push(*c);
//                    }
//                }
//
//                (_, _) => {}
//            }
//        }
//        println!();
//    }
//
//    println!();
//    println!();
//
//    //(3,3)(4,3)(5,3)(6,3) S Calm
//    //(3,3)(2,3)(1,3)(0,3) N will be backwards
//    //let N_new = revvin(N)
//    //(3,3)(3,4)(3,5)(3,6) E Calm
//    //(3,3)(3,2)(3,1)(3,0) W Backwards
//    //let W_new = revvin(W);
//    //(3,3)(2,4)(1,5)(0,6) NE Backwards
//    //let NE_new = revvin(NE);
//    //(3,3)(4,4)(5,5)(6,6) SE Calm
//    //(3,3)(4,2)(5,1)(6,0) SW Calm
//    //(3,3)(2,2)(1,1)(0,0) NW Backwards
//    //let NW_new = revvin(NW);
//    let mut ans = 0;
//    for _ in 0..9 {
//        match "XMAS".to_string() {
//            mut N => {
//                println!("N");
//                ans += 1;
//                N.clear();
//            }
//            mut NW => {
//                println!("NW");
//                ans += 1;
//                NW.clear();
//            }
//            mut NE => {
//                println!("NE");
//                ans += 1;
//                NE.clear();
//            }
//            mut S => {
//                println!("S");
//                ans += 1;
//                S.clear();
//            }
//            mut SE => {
//                println!("SE");
//                ans += 1;
//                SE.clear();
//            }
//            mut SW => {
//                println!("SW");
//                ans += 1;
//                SW.clear();
//            }
//            mut E => {
//                println!("E");
//                ans += 1;
//                E.clear();
//            }
//            mut W => {
//                println!("W");
//                ans += 1;
//                W.clear();
//            }
//            _ => {
//                break;
//            }
//        }
//    }
//    println!("{}", ans);
//    println!();
//    println!();
//
//    ans
//}

fn mid_snap(vec: Vec<Vec<Option<char>>>, (i, j): (usize, usize)) -> Vec<Vec<Option<char>>> {
    let mut snapshot: Vec<Vec<Option<char>>> = Vec::new();
    for x in (i - 1)..=(i + 1) {
        let mut row: Vec<Option<char>> = Vec::new();
        if x < vec.len() {
            let grid_row = &vec[x];
            for y in (j - 1)..=(j + 1) {
                if y < grid_row.len() {
                    row.push(grid_row[y].clone());
                } else {
                    row.push(None);
                }
            }
        } else {
            row.extend(vec![None; 3]);
        }
        snapshot.push(row);
    }
    snapshot
}
fn snapshots(vec: Vec<Vec<Option<char>>>, (i, j): (usize, usize)) -> Vec<Vec<Option<char>>> {
    let mut snapshot: Vec<Vec<Option<char>>> = Vec::new();
    for x in (i - 3)..=(i + 3) {
        let mut row: Vec<Option<char>> = Vec::new();
        if x < vec.len() {
            let grid_row = &vec[x];
            for y in (j - 3)..=(j + 3) {
                if y < grid_row.len() {
                    row.push(grid_row[y].clone());
                } else {
                    row.push(None);
                }
            }
        } else {
            row.extend(vec![None; 7]);
        }
        snapshot.push(row);
    }
    snapshot
}

//let XMAS_pat: [u8; 4] = [88, 77, 65, 83];
//let SAMX_pat: [u8; 4] = [83, 65, 77, 88];
//let _ = aoc_io::process_file("pew.txt".to_string(), |line| {
//    line.as_bytes().windows(4).for_each(|win| {
//        if win == XMAS_pat || win == SAMX_pat {
//            ans += 1;
//        }
//    });
//});
