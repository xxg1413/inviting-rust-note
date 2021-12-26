use std::rc::Rc;

#[derive(Debug)]
struct Node {
    id: u32,
    downsteam: Option<Rc<Node>>,
}

impl Node {

    pub fn new(id:u32) -> Self {
        Self { id, downsteam: None, }

    }

    pub fn add_downstream(&mut self, downsteam: Rc<Node>) {
        self.downsteam = Some(downsteam);
    }


    pub fn get_downstream(&self) -> Option<Rc<Node>> {
        self.downsteam.as_ref().map(|x| x.clone())
    }
}

fn main() {

    let mut node1 = Node::new(1);
    let mut node2 = Node::new(2);
    let mut node3 = Node::new(3);

    let node4 = Node::new(4);
    node3.add_downstream(Rc::new(node4));

    node1.add_downstream(Rc::new(node3));
    node2.add_downstream(node1.get_downstream().unwrap());

    println!("node1: {:?},node2: {:?}", node1, node2)

}