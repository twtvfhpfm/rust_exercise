use rand::Rng;
use std::cmp::Ordering;
use std::io::Write;
pub fn guess_number()
{
  let num = rand::thread_rng().gen_range(1, 1001);
  loop{
    print!("请输入一个整数: ");
    std::io::stdout().flush().expect("flush失败");
    let mut guess = String::new();
    std::io::stdin().read_line(&mut guess).expect("读取失败");
    let guess: u32 = match guess.trim().parse(){
      Ok(n) => n,
      Err(_) => continue,
    };

    match guess.cmp(&num) {
      Ordering::Less => println!("太小了"),
      Ordering::Greater => println!("太大了"),
      Ordering::Equal => {
        println!("猜中啦！");
        break;
      }
    }
  }
}