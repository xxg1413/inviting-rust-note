use std::collections::BTreeMap;

fn main() {

    let mut map = BTreeMap::new();
    map.insert("blue", 10);

    println!("map: {:?}", map);
}