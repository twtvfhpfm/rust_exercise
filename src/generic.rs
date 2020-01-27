use std::cmp::PartialOrd;

pub fn largest<T: PartialOrd + Copy>(list: &[T]) -> T
{
    let mut largest = list[0];
    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }
    largest
}

pub fn test()
{
    let list = [3,4,2,5,1];
    println!("largest: {}", largest(&list));
    let l = ["aa", "bb", "gg", "dd"];
    println!("largest: {}", largest(&l));

    test_share();
}

pub trait Shape{
    fn area(&self) -> i32;
}

pub struct Square{
    width: i32,
}

impl Shape for Square{
    fn area(&self) -> i32 {
        self.width * self.width
    }
}

pub struct Rect{
    width: i32,
    height: i32,
}

impl Shape for Rect{
    fn area(&self) -> i32 {
        self.width * self.height
    }
}

pub fn test_share()
{
    let squ = Square{
        width: 5
    };
    let rect = Rect{
        width: 3,
        height: 4,
    };
    print_shape(squ);
    print_shape(rect);
}

//pub fn print_shape(arg: impl Shape){
//pub fn print_shape<T: Shape>(arg: T){
pub fn print_shape<T>(arg: T) where T: Shape{
    println!("{}", arg.area());
}