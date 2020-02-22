use std::collections::HashMap;
pub fn test_hash_map()
{
    let mut map = HashMap::new();
    map.insert("a", 1);
    map.insert("b", 2);
    println!("{:?}", map);
    map.entry("a").or_insert(3);
    map.entry("c").or_insert(3);
    println!("{:?}", map);
    for (key, value) in &map{
        println!("{}:{}", key, value);
    }
}

pub fn character_statistics()
{
    let s = String::from("The mutable reference goes out of scope at the end of the for loop");
    let mut map = HashMap::new();
    for c in s.chars(){
        if map.contains_key(&c) {
            map.insert(c, map.get(&c).unwrap() + 1);
        }else{
            map.insert(c, 1);
        }
    }
    println!("{:?}", map);
    
    map.clear();
    for c in s.chars(){
        let count = map.entry(c).or_insert(0);
        *count += 1;
    }
    println!("{:?}", map);
}