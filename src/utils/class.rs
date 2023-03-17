use indextree::Arena;
use std::collections::HashMap;

pub trait Tree {
    fn id(&self) -> u32;
    fn pid(&self) -> u32;
    fn new(&self) -> u32;
}

#[derive(Debug)]
pub struct Node {
    id: u32,
    pid: u32,
    name: String,
}


pub fn createTree(arr: Vec<Node>) -> Arena<Node> {
    let mut hash = HashMap::new();
    let mut arena = Arena::new();
    arr.into_iter().for_each(|f| {
        let pid = f.pid;
        let id = f.id;
        let node = arena.new_node(f);
        hash.insert(id, node);

        match hash.get(&pid) {
            Some(pnode) => {
                pnode.append(node, &mut arena);
            }
            None => (),
        }
        // let a = arena.new_node(f);
    });
    arena
}




pub fn test() {
    let arr = vec![
        Node{id:1, pid: 0, name: String::from("1")},
        Node{id:2, pid: 0, name: String::from("2")},
        Node{id:3, pid: 1, name: String::from("2")}
    ];
    let tree = createTree(arr);
    let mut iter = tree.iter();
    iter.next().map(|node| {
        println!("{:?}", node);
    });
  
}