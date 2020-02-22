pub fn string_test()
{
    let mut s = String::new();
    s.push_str("hello徐建南");
    //let mut s2 = s;
    //s2.push_str("_____");
    println!("{}", s);
    println!("{}", s.capacity());
    println!("{}", s.chars().count());
    let a = [1,2,3,4,5];
    let mut a1 = a;
    a1[0] = 6;
    println!("{:?}", a);
    println!("{:?}", a1);
    let v = vec![1,2,3,4,5];
    let mut v1 = v.clone();
    v1.push(7);
    println!("{}", v1.capacity());
    println!("{}", v1.len());
    println!("{:?}", v);
    println!("{:?}", v1);

    let person1 = Person{ age: 30, name: "张三"};
    let mut person2 = person1;
    println!("{:?}", person2);
    if !person2.isChild(){
        println!("adult");
    }
    person2.set_age(16);
    if person2.isChild(){
        println!("child");
    }

    let tup = (5, "a");
    let mut tup2 = tup;
    tup2.0 = 6;
    tup2.1 = "b";
    println!("{:?}", tup);
}

#[derive(Debug)]
struct Person<'a>{
    age: u32,
    name: &'a str,
}

impl Person<'static>{
    fn isChild(&self) -> bool
    {
        if self.age < 18 {
            true
        }else{
            false
        }
    }
    fn set_age(&mut self, new_age: u32) 
    {
        self.age = new_age;
    }
}