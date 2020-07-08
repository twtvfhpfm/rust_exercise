use std::fmt;

#[derive(Debug)]
struct Complex {
    real: f64,
    imag: f64
}

struct List(Vec<i32>);

impl fmt::Display for Complex {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
    {
        write!(f, "{} + {}i", self.real, self.imag)
    }
}

impl fmt::Display for List {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
    {
        for (idx, value) in self.0.iter().enumerate() {
            if idx == 0 {
                write!(f, "[{}:{}", idx, value)?;
            }
            else{
                write!(f, " ,{}:{}", idx, value)?;
            }
        }
        write!(f, "]")
    }
}

struct Color {
    red: u8,
    green: u8,
    blue: u8
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
    {
        write!(f, "RGB ({0}, {1}, {2}) 0x{0:02X}{1:02X}{2:02X}",
                self.red, self.green, self.blue)
    }
}

pub fn display_test()
{
    let num = Complex {real:1.2,imag:3.4};
    println!("Debug: {:?}", num);
    println!("Display: {}", num);

    let l = List(vec![5,6,7]);
    println!("list: {}", l);

    for i in [
        Color{red: 128, green: 255, blue: 90},
        Color{red: 0, green: 3, blue: 254},
        Color{red: 0, green: 0, blue: 0}
    ].iter() {
        println!("{}", i);
    }
}