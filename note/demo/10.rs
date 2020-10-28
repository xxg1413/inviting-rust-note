struct A;

impl A{
    fn hello(&self){
        println!("in A");
    }
}

trait Hello {
    fn hello(&self);
}

impl Hello for A {
    fn hello(&self){
        println!("from Hello trait");
    }
}

fn main() {

    let a = A;
    a.hello();

    <A as Hello>::hello(&a);
    
}