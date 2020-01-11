use std::io::ErrorKind;
use std::fs::File;
use std::io::Write;
use std::io::Read;
pub fn file_test()
{
    let f = File::open("hello.txt");

    let mut f = match f{
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(file) => file,
                Err(error) => panic!("error creating hello.txt")
            },
            other => panic!("error opening hello.txt: {:?}", other)
        }
    };
    let bytes = f.write("hello".as_bytes()).expect("write failed\n");
    if bytes != 5 {
        println!("write {} bytes", bytes);
    }
    
    let mut s = String::new();
    File::open("hello.txt").and_then(|mut f|{f.read_to_string(&mut s)})
    .expect("read failed\n");
    read_file(&mut s).expect("read failed,,,,\n");
    println!("{}",s);
}

fn read_file(s: &mut String) -> Result<&mut String, std::io::Error>
{
    File::open("hello.txt")?.read_to_string(s)?;
    Ok(s)
}