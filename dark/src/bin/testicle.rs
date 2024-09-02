use dark::engines::kv::KvStore;

fn main(){
    let mut db = KvStore::new();
    let an = db.get(&"4".to_owned());
    println!("{an:?}");

}