use std::{borrow::Borrow, cell::RefCell, rc::Rc, sync::Arc};

fn demo1() {
    let data = vec![1, 2, 3, 4, 5, 6, 7];
    let data1 = &data;
    println!(
        "addr of value:{:p} ({:p}), addr of data :{:p}, data1:{:p}",
        &data, data1, &&data, &data1
    );
    println!("sum of data1: {}", sum(data1));
    // println!("data1: {:?}", data1);
    println!(
        "addr of items: [{:p}, {:p}, {:p} ]",
        &data[0], &data[1], &data[2]
    );
}

fn sum(data: &Vec<u32>) -> u32 {
    println!("addr of valud:{:p}, addr of ref:{:p}", data, &data);
    data.iter().fold(0, |acc, x| acc + x)
}

fn is_copy<T: Copy>() {}

fn types_impl_copy_trait() {
    is_copy::<bool>();
    is_copy::<char>();
    // all iXX and uXX, usize/isize, fXX implement Copy trait
    is_copy::<i8>();
    is_copy::<u64>();
    is_copy::<i64>();
    is_copy::<usize>();
    // function (actually a pointer) is Copy
    is_copy::<fn()>();
    // raw pointer is Copy
    is_copy::<*const String>();
    is_copy::<*mut String>();
    // immutable reference is Copy
    is_copy::<&[Vec<u8>]>();
    is_copy::<&String>();
    // array/tuple with values which is Copy is Copy
    is_copy::<[u8; 4]>();
    is_copy::<(&str, &str)>();
}
// fn types_not_impl_copy_trait() {
//     // unsized or dynamic sized type is not Copy
//     is_copy::<str>();
//     is_copy::<[u8]>();
//     is_copy::<Vec<u8>>();
//     is_copy::<String>();
//     // mutable reference is not Copy
//     is_copy::<&mut String>();
//     // array / tuple with values that not Copy is not Copy
//     is_copy::<[Vec<u8>; 4]>();
//     is_copy::<(String, u32)>();
// }

// fn local_ref<'a>() -> &'a i32 {
//     let a = 42; //
//     &a
// }

fn push_local_ref(data: &mut Vec<i32>) {
    let v = 42;
    data.push(v);
}

fn demo2() {
    let mut data = vec![1, 2, 3, 4, 4];
    let data1 = vec![&data[0]];
    println!("data[0]:{:p}", &data[0]);
    for i in 1..100 {
        // data.push(i); // cannot borrow `data` as mutable because it is also borrowed as immutable
    }

    println!("data[0]:{:p}", &data[0]);
    println!("boxed:{:p}", &data1);
}

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
        self.downstream.as_ref().map(|v| v.clone())
    }
}

fn test_node() {
    let mut node = Node::new(1);
    let mut node1 = Node::new(2);
    let mut node2 = Node::new(3);
    let node3 = Node::new(4);

    node2.update_downstream(Rc::new(node3));

    node.update_downstream(Rc::new(node2));
    node1.update_downstream(node.get_downstream().unwrap());
    println!("node1: {:?}, node2:{:?}", node, node1);
}

fn test_refcell() {
    let data = RefCell::new(1);
    {
        let mut v = data.borrow_mut();
        *v = *v + 1;
    }
    println!("data:{:?}", data.borrow());
    println!("data1:{:?}", data.borrow());
}

fn main() {
    let arr = vec![1];
    std::thread::spawn(move || {
        println!("{:?}", arr);
    });

    let data_str = "rust";
    let arcstr = Arc::new(data_str);
    let narc = arcstr.clone();

    std::thread::spawn(move || println!("datastr:{:?}", narc.as_ref()));
    println!("main datastr:{:?}", arcstr.as_ref());

    test_node();
    test_refcell();
    demo2();
    // let r = local_ref();
    let mut data = Vec::new();
    push_local_ref(&mut data);
    println!("data: {:?}", data);
    demo1();
    types_impl_copy_trait();
    // types_not_impl_copy_trait();

    let a = Rc::new(1);
    let _b = a.clone();
    let _c = a.clone();
}
