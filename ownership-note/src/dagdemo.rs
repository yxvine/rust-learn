use std::rc::Rc;
// 假设 Node 就只包含 id 和指向下游（downstream）的指针，
// 因为 DAG 中的一个节点可能被多个其它节点指向，
// 所以我们使用  Rc<Node> 来表述它；
// 一个节点可能没有下游节点，
// 所以我们用  Option<Rc<Node>> 来表述它。

#[derive(Debug)]
struct Node {
    id: usize,
    downstream: Option<Rc<Node>>,
}

impl Node {
    pub fn new(id: usize) -> Self {
        Self {
            id,
            downstream: None,
        }
    }

    pub fn update_downstream(&mut self, downstream: Rc<Node>) {
        self.downstream = Some(downstream);
    }

    pub fn get_downstream(&self) -> Option<Rc<Node>> {
        self.downstream.as_ref().map(|v| v.clone)
    }
}

fn test_node() {
    let mut node = Node::new(1);
    let mut node1 = Node::new(2);
    let mut node2 = Node::new(3);
    let node3 = Node::new(4);

    node2.update_downstream(Rc::New(node3));

    node.update_downstream(Rc::New(node2));
    node1.update_downstream(node.get_downstream().unwrap());
    println!("node1: {:?}, node2:{:?}", node, node1);
}
