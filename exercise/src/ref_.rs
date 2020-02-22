pub fn longest<'a>(a: &'a str, b: &'a str) -> &'a str
{
    if a.len() > b.len(){
        a
    }
    else{
        b
    }
}

pub fn ref_test()
{
    let a = String::from("你好");
    let b = String::from("hello");

    println!("{}", longest(a.as_str(), b.as_str()));
}