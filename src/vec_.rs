pub fn test_vec()
{
    let mut v = vec![Vec::new(),Vec::new(),Vec::new()];
    let mut v1 = &mut v[1];
    v1.push(1);
    let mut v0 = &mut v[0];
    v0.push(0);
    println!("{:?}", v);
    println!("{}", v[0][0]);

    let mut s1 = String::from("hello");
    let mut s2 = String::from("world");
    let mut s3 = s1 + &s2;
    println!("{}", s3);
    let s4 = String::from("徐建南");
    //let s4 = "徐建南";
    let c = &s4[0..3];
    println!("{}", c);
}