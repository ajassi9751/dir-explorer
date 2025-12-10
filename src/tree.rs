// Is Tree even needed?
pub struct Tree <T> {
    value: T, // Value will most likley be "."
    children: Vec<Node<T>>
}

pub struct Node <T> {
    value: T,
    children: Vec<Node<T>> // Make use of .if_empty() to see if it is a file or folder
}

impl <T> Tree <T> {
    pub fn new (data: T) -> Tree <T> {
        Tree {
            value: data,
            children: Vec::new()
        }
    }
    pub fn add_node (&mut self, data: T) {
        self.children.push(Node::new(data));
    }
    pub fn get_node_mut (&mut self, index: usize) -> Option<&mut Node<T>> {
        self.children.get_mut(index)
    }
    pub fn get_node (&self, index: usize) -> Option<&Node<T>> {
        self.children.get(index)
    }
    pub fn get_children_mut (&mut self) -> &mut Vec<Node<T>> {
        &mut self.children
    }
    pub fn get_children (&self) -> &Vec<Node<T>> {
        &self.children
    }
    // Make this an option or not
    pub fn get_val (&self) -> &T {
        &self.value
    }
    pub fn get_val_mut (&mut self) -> &mut T {
        &mut self.value
    }
}

impl <T> Node <T> {
    pub fn new (data: T) -> Node<T> {
        Node {
            value: data,
            children: Vec::new()
        }
    }
    pub fn add_node (&mut self, data: T) {
        self.children.push(Node::new(data));
    }
    pub fn get_node_mut (&mut self, index: usize) -> Option<&mut Node<T>> {
        self.children.get_mut(index)
    }
    pub fn get_node (&self, index: usize) -> Option<&Node<T>> {
        self.children.get(index)
    }
    pub fn get_children_mut (&mut self) -> &mut Vec<Node<T>> {
        &mut self.children
    }
    pub fn get_children (&self) -> &Vec<Node<T>> {
        &self.children
    }
    pub fn get_val (&self) -> &T {
        &self.value
    }
    pub fn get_val_mut (&mut self) -> &mut T {
        &mut self.value
    }
}