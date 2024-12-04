use std::{collections::HashMap, isize};
use util_aoc::aoc_io;

fn main() {
    aoc_io::lickballs();
    let mut wu: isize = 0;
    let mut map: HashMap<Vec<isize>, bool> = HashMap::new();
    let _ = aoc_io::line_vec("sema_ballz.gottem".to_string(), |arr| {
        let asc = arr.get(0).unwrap() < arr.get(1).unwrap();
        if abides_asc(&arr, asc) {
            if asc {
                if const_diff(&arr, |v1, v2| v2 - v1 <= 3 && v2 - v1 > 0) {
                    map.insert(arr, true);
                    wu = 1 + wu;
                } else {
                    map.insert(arr, false);
                }
            } else {
                if const_diff(&arr, |v2, v1| v2 - v1 <= 3 && v2 - v1 > 0) {
                    map.insert(arr, true);
                    wu = 1 + wu;
                } else {
                    map.insert(arr, false);
                }
            }
        } else {
            map.insert(arr, false);
        }
        println!("{wu}");
    });
    println!("{wu}");
    //rust so cool
    let redemption: Vec<Vec<isize>> = map
        .iter()
        .filter(|(_, &v)| !v)
        .map(|(k, _)| k.to_vec())
        .collect();
    //map.iter().for_each(|(k, v)| {
    //    if !v {
    //        redemption.push(k.to_vec());
    //    }
    //});
    //assert!(redemption.len() + wu , 1000);
    let mut eval = 0;
    let _ = redemption.iter().for_each(|v| {
        if reevaluation(v) {
            eval = eval + 1;
        }
    });
    println!("rhi {}", eval + wu);
}

fn reevaluation(arr: &Vec<isize>) -> bool {
    for i in 0..arr.len() {
        //to disregard, i in vec
        let arr1 = [&arr[..i], &arr[i + 1..]].concat();
        //let mut arr_new: Vec<isize> = arr.to_vec();
        //arr_new.remove(i);
        let asc = arr1.get(0).unwrap() < arr1.get(1).unwrap();
        if abides_asc(&arr1, asc) {
            if asc {
                if const_diff(&arr1, |v1, v2| v2 - v1 <= 3 && v2 - v1 > 0) {
                    return true;
                }
            } else {
                if const_diff(&arr1, |v2, v1| v2 - v1 <= 3 && v2 - v1 > 0) {
                    return true;
                }
            }
        }
    }

    false
}

//for (i, _) in arr.iter().enumerate() {
//    let asc = arr.get(0).unwrap() < arr.get(1).unwrap();
//    for _ in arr.iter().skip(i) {
//        if dsc_arr(&arr, asc) {
//            if asc {
//                if asc_const_diff(&arr) {
//                    return true;
//                }
//            } else {
//                if desc_const_diff(&arr) {
//                    return true;
//                }
//            }
//        }
//    }
//}

fn abides_asc(arr: &Vec<isize>, asc_init: bool) -> bool {
    let mut last_one = arr.get(0).unwrap();
    for v in arr.iter().skip(1) {
        let asc_curr = if *v > *last_one { true } else { false };
        if asc_init != asc_curr {
            return false;
        }
        last_one = v;
    }
    true
}
fn const_diff<F>(arr: &Vec<isize>, cmp: F) -> bool
where
    F: Fn(isize, isize) -> bool,
{
    arr.iter()
        .zip(arr.iter().skip(1))
        .all(|(a1, a2)| cmp(*a1, *a2))
}
//fn asc_const_diff(arr: &Vec<isize>) -> bool {
//    for (a1, a2) in arr.iter().zip(arr.iter().skip(1)) {
//        if a2 - a1 > 3 || a2 - a1 == 0 {
//            return false;
//        }
//    }
//    true
//}
//fn desc_const_diff(arr: &Vec<isize>) -> bool {
//    for (a1, a2) in arr.iter().zip(arr.iter().skip(1)) {
//        if a1 - a2 > 3 || a2 - a1 == 0 {
//            return false;
//        }
//    }
//    true
//}
