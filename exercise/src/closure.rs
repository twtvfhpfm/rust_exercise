use std::collections::HashMap;

pub fn closure_test()
{
    let c = |num|{
        println!("executing closure");
        num * 2
    };

    println!("{}",c(1));
    println!("{}",c(2));

    let mut casher = Casher::new(c);
    println!("{}",casher.value(3));
    println!("{}",casher.value(3));
    println!("{}",casher.value(4));
    println!("{}",casher.value(4));
    println!("{}",casher.value(4));
    println!("{}",casher.value(4));
}

struct Casher<T> where T: Fn(u32)->u32
{
    calculation: T,
    value: HashMap<u32, u32>,
}

impl<T> Casher<T> where T: Fn(u32)->u32{

    fn new(calculation: T) -> Casher<T> {
        Casher {
            calculation,
            value: HashMap::new(),
        }
    }

    fn value(&mut self, arg: u32) -> u32 {
        if (self.value.contains_key(&arg)){
            *self.value.get(&arg).unwrap()
        }
        else{
            let v = (self.calculation)(arg);
            self.value.insert(arg, v);
            println!("insert: {}->{}", arg, v);
            v
        }


    }
}