fn main() {
    let s1 = String::from("Lindsey");
    let s2 = String::from("Rosie");

    let result =get_max( &s1, &s2 );

    println!("bigger one:{}", result);
}

/*
fn max(s1: &str, s2: &str) -> &str { 
    if s1 > s2 { s1 } else { s2 }
}*/


fn get_max<'a>(s1: &'a str, s2: &'a str) ->&'a str { 

    if s1 > s2 { s1 } else { s2 }
}