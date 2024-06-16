use std::collections::BTreeMap;


pub fn main() {
    let mut btree : BTreeMap<i32, i32> = BTreeMap::new();
    for i in 0..10 {
        btree.insert(i, i);
    }
    println!("{:?}", btree);
    let v = btree.get(&5).unwrap() as *const i32;
    println!("{:?}", v);
    for i in 0..1000 {
        btree.insert(i, i);
    }
    let v = btree.get(&5).unwrap() as *const i32;
    println!("{:?}", v);
    for i in 6..1000 {
        btree.remove(&i);
    }
    let v = btree.get(&5).unwrap() as *const i32;
    println!("{:?}", v);
    for i in 0..4 {
        btree.remove(&i);
    }
    let v = btree.get(&5).unwrap() as *const i32;
    println!("{:?}", v);
}
