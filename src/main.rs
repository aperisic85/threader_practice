use std::thread;
use std::time::Instant;

fn main() {
    println!("Hello, world!");
    let t1 = thread::spawn(f);
    let t2 = thread::spawn(f);

    t1.join().unwrap();
    t2.join().unwrap();

    //use closure
    let numbers = vec![12,3,4];

    thread::spawn( move|| {
        for n in numbers {
            println!("{n}");
        }
    }).join().unwrap();


    let new_vec =  Vec::from_iter(0..1_000_000);
    //let new_vec:Vec<usize> =  Vec::new(); divide by zero panick

    let now = Instant::now();
        let t3  = thread::Builder::new().name("average calc".into()).spawn( move ||
    // let t3 = thread::spawn(move ||
        {
            let len = new_vec.len();
            let sum:usize = new_vec.into_iter().sum();
            sum/len
        });
    let result: usize = t3.unwrap().join().unwrap();
    println!("Elapsed  : {} msec", now.elapsed().as_millis());
   // let result: usize = t3.join().unwrap();
    println!("{:?}",result);
}


fn f(){
    println!("Hello from another thread!");
    let id = thread::current().id();
    println!("This is my thread id: {id:?}")
}