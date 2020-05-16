pub trait Talk {
    fn talk(&self);
}

struct Zoo {
    pub animals: Vec<Box<dyn Talk>>,
}

impl Zoo {
    pub fn open(&self) {
        for animal in self.animals.iter() {
            animal.talk();
        }
    }
}

struct Dog {
    name: String,
}

struct Cat {
    name: String,
}

struct Person {
    name: String,
}

impl Talk for Dog {
    fn talk(&self){
        println!("Wang, my name is {}", self.name);
    }
}

impl Talk for Cat {
    fn talk(&self) {
        println!("Miao, my name is {}", self.name);
    }
}

impl Talk for Person {
    fn talk(&self){
        println!("Hello, my name is {}", self.name);
    }
}

pub fn trait_test(){
    let mut zoo = Zoo {
        animals: vec![],
    };

    zoo.animals.push(Box::new(Dog{name: String::from("bagong")}));
    zoo.animals.push(Box::new(Cat{name: String::from("tom")}));
    zoo.animals.push(Box::new(Person{name: String::from("Jack")}));

    zoo.open();
}