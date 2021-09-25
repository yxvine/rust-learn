fn main() {
    println!("Hello, world! rust demo");
    let data = vec![1, 2, 3, 4];
    let v = 3;
    if let Some(pos) = find_pos(data, v) {
        println!("Found {} at {}", v, pos);
    }

}

fn find_pos(data: Vec<u32>, v: u32) -> Option<usize> {
    for (pos, item) in data.iter().enumerate() {
        if *item == v {
            return Some(pos);
        }
    }
    None
}
