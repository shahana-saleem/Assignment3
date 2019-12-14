//Q1
// #![allow(unused)]
// fn main() {
// fn consume_with_relish<F>(func: F)
//      where F: FnOnce() -> String
// {
//     println!("consumed: {}", func());

//     println!("Stay happy!");
// }

// let x = String::from("s");
// let consume_and_return_s = || x;
// consume_with_relish(consume_and_return_s);
// }
///////////////////////////////////////////////////////////////////////////////////////
//Q2
// #![allow(unused)]
// fn main() {
// fn do_twice<F:FnMut()>(mut func: F)
//  {
//      func();
//      func();
// }
// let mut value: usize = 1;
// {
//      let add_five_to_value = || value += 5;
//      do_twice(add_five_to_value);
// }
// println!("{}",value);
// }
////////////////////////////////////////////////////////////////////////////////////
//Q3
// use std::thread;
// use std::time::Duration;

// fn main() {
//     let handle = thread::spawn(|| {
//         for s in 1..16 {
//             println!("hi number {} from the spawned thread!", s);
//             thread::sleep(Duration::from_millis(1));
//         }
//     });

//     for a in 1..10 {
//         println!("hi number {} from the main thread!", a);
//         thread::sleep(Duration::from_millis(1));
//     }

//     handle.join().unwrap();
// }
