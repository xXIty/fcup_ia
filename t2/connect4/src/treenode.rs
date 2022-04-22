use std::cell::RefCell;
use std::rc::{Rc, Weak};

pub type NodeLink = Rc<RefCell<Node>>;
type NodeWeakLink = Weak<RefCell<Node>>;

pub struct MCTSData {
    pub action: usize,
    pub t: f32, 
    pub n: f32,
    pub unexpanded: Vec<usize>,
}

pub struct Node {
    pub data: MCTSData,
    pub children: Vec<NodeLink>,
    pub parent: Option<NodeWeakLink>,
}

impl Node {
    pub fn is_leaf(&self) -> bool {
        return self.children.len() == 0 || self.data.unexpanded.len() > 0; 
    }

    pub fn new(action: usize, t: f32, n: f32, unexpanded: Vec<usize>) -> NodeLink {
            Rc::new(RefCell::new(Self {
                data: MCTSData { action, t, n, unexpanded},
                children: Vec::new(),
                parent: None,
            }))
    }

    pub fn new_with_parent(action: usize, t: f32, n: f32, unexpanded: Vec<usize>, parent: &NodeLink) -> NodeLink {
        Rc::new(RefCell::new(Self {
            data: MCTSData { action, t, n, unexpanded},
            children: Vec::new(),
            parent: Some(Rc::<RefCell<Node>>::downgrade(parent)),
        }))
    }


    pub fn add_child(current: &NodeLink, child: NodeLink) -> NodeLink {
        child.borrow_mut().parent = Some(Rc::<RefCell<Node>>::downgrade(current));
        let cochild = child.clone();
        let mut cur = current.borrow_mut();
        cur.children.push(child);
        return cochild;
    }


}
