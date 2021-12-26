
fn find_pos(data: Vec<u32>, v: u32) -> Option<usize> {

    for(i,item) in  data.iter().enumerate() {
        if *item == v {
            return Some(i);
        }
    }
    None
}

fn main() {

    let data =vec![1,0,42,99,90,11];

    let v = 42;

    if let Some(pos) = find_pos(data,v) {
        println!("found {} at position {}",v,pos);
    } else {
        println!("{} not found",v);
    }
}


