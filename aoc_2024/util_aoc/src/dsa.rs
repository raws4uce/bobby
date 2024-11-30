use std::collections::HashMap;

//little dump of dsa's real short n sweet

fn memoise<F, A, T>(mut f: F) -> impl FnMut(A) -> T
where
    F: FnMut(A) -> T,
    A: std::hash::Hash + Eq + Clone,
    T: Clone,
{
    let mut cache: HashMap<A, T> = HashMap::new();
    move |x: A| {
        if let Some(result) = cache.get(&x) {
            result.clone()
        } else {
            let result = f(x.clone());
            cache.insert(x, result.clone());
            result
        }
    }
}
//this is how it would look to use memoisation, as a base case kakaww
//if let Some(&result) = cache.get(&n) {
//    return result;
//}
