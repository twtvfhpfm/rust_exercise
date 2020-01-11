pub mod mod1;
enum Activity{
    Eat(String),
    Drink(u32,u32),
    Play([u32;5]),
    Happy(u32),
}

impl Activity{
    fn call(&self){
        
    }
}

pub fn enum_test()
{
    let eat = Activity::Eat(String::from("hello"));
    match eat {
        Activity::Eat(s) => {
            println!("{}", s);
        },
        Activity::Drink(t1,t2) => {
            println!("{} {}", t1,t2);
        },
        Activity::Play(a) => {
            println!("{:?}", a);
        },
        Activity::Happy(u) =>{
            println!("{}", u);
        }
    }
    let i = Some(4);
    if let Some(5) = i {

    }

}

pub mod mod_test{
    pub fn test(){
        super::enum_test();
    }
}