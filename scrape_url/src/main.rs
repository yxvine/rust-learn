// namespce
use std::fs;

fn do_scrape() {
    let url = "https://time.geekbang.org/column/article/411632";
    let output = "rust.md";

    println!("Fetching url: {}", url);
    // return is <T, E> unwrap just focus on success result
    // unwrap -> ? forward the return <T, E>
    let body = reqwest::blocking::get(url).unwrap().text().unwrap();

    println!("Converting html to markdown: {}", body);
    let md = html2md::parse_html(&body);

    fs::write(output, md.as_bytes()).unwrap();
    println!("Converted markdown has been generated saved in {}", output);
}

fn apply(value: i32, f: fn(i32) -> i32) -> i32 {
    f(value)
}

fn square(val: i32) -> i32 {
    val * val
}

fn cube(val: i32) -> i32 {
    // has must return a value of i32
    val * val * val
}

fn function_use_demo() {
    // no return value
    println!("apply use square:{}", apply(2, square));
    println!("apply use cube:{}", apply(2, cube))
}

fn pi() -> f64 {
    3.1415926
}

fn not_pi() {
    // return is a unit? () void
    3.1415926;
}

fn return_unit_demo() {
    let is_pi = pi();
    let is_unit1 = not_pi();

    let is_unit2 = {
        pi();
    };
    println!(
        "is_pi:{:?}, is_unit1:{:?}, is_unit2:{:?}",
        is_pi, is_unit1, is_unit2
    );
}

// data struct define
#[derive(Debug)]
enum Gender {
    Unspecified = 0,
    Female = 1,
    Male = 2,
}

#[derive(Debug, Copy, Clone)]
struct UserId(u64);

#[derive(Debug, Copy, Clone)]
struct TopicId(u64);

#[derive(Debug)]
struct User {
    id: UserId,
    name: String,
    gender: Gender,
}

#[derive(Debug)]
struct Topic {
    id: TopicId,
    name: String,
    owner: UserId,
}

// #[derive(Debug)] trait can use {:?} by println!
#[derive(Debug)]
enum Event {
    Join(UserId, TopicId),
    Leave(UserId, TopicId),
    Message(UserId, TopicId, String),
}

fn data_struct_demo() {
    let alice = User {
        id: UserId(1),
        name: "Alice".into(),
        gender: Gender::Female,
    };
    let bob = User {
        id: UserId(2),
        name: "Bob".into(),
        gender: Gender::Male,
    };

    let topic = Topic {
        id: TopicId(1),
        name: "rust".into(),
        owner: UserId(1),
    };

    let event1 = Event::Join(alice.id, topic.id);
    let event2 = Event::Join(bob.id, topic.id);
    let event3 = Event::Message(alice.id, topic.id, "hello".into());

    println!(
        "event1 :{:?}, event2:{:?}, evnet3:{:?}",
        event1, event2, event3
    );
}

fn fib_loop(n: u8) {
    let mut a = 1;
    let mut b = 1;
    let mut i = 2u8;

    loop {
        let c = a + b;
        a = b;
        b = c;
        i += 1;

        println!("next val is {}", b);

        if i >= n {
            break;
        }
    }
}

fn fib_while(n: u8) {
    let (mut a, mut b, mut i) = (1, 1, 2);

    while i < n {
        let c = a + b;
        a = b;
        b = c;
        i += 1;

        println!("next val {}", b);
    }
}

fn fib_for(n: u8) {
    let (mut a, mut b) = (1, 1);

    for _i in 2..n {
        let c = a + b;
        a = b;
        b = c;
        println!("next val {}", b);
    }
}

fn main() {
    //
    do_scrape();
    function_use_demo();
    return_unit_demo();
    data_struct_demo();

    let n = 10;
    fib_loop(n);
    fib_for(n);
    fib_while(n);
}
