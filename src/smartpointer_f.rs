use crate::smartpointer_f::List::Cons;
//or use List::{Cons, Nil};
enum List {
    Cons(i32, Box<List>),
    Nil,
}


pub fn smartpointer_file() {
    let b = Box::new(5);
    println!("box ={b}");

    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
}