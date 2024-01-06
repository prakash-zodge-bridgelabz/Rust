#![allow(dead_code)]
use std::mem;
struct Point{
    x:f64,          // 8 bytes
    y:f64           // 8 bytes
}
fn origin() -> Point{
    Point{
        x:0.0,
        y:0.0
    }
}
pub fn stack_and_heap(){
    let p1 = origin();
    let p2 = Box::new(origin());
    println!("p1 takes size = {}bytes",mem::size_of_val(&p1));
    println!("p2 takes size = {}bytes",mem::size_of_val(&p2));
    let p3 = *p2;       // here p3 becomes box which contains two values i.e. x and y
    println!("p3.x = {},p3.y = {}",p3.x,p3.y);
    
}