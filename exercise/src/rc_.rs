use std::rc::Rc;

enum List {
    Cons(i32, Rc<List>),
    Nil,
}

use List::{Cons,Nil};

pub fn rc_test()
{
    let a = Rc::new(Cons(5, Rc::new(Cons(4, Rc::new(Nil)))));
    let b = Cons(3, Rc::clone(&a));
    let c = Cons(3, Rc::clone(&a));

    println!("strong_count: {}", Rc::strong_count(&a));
}