use std::io;

fn main() {
    let num = loop{
        println!("Input Nth Data");
        let mut num = String::new();
        io::stdin().read_line(&mut num)
        .expect("Please Input Data");

        let num: u32 = match num.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Input integer Data");
                continue
            }
        };
        break num;
    };
    println!("Result is {}",fibanacci(num));
}
fn fibanacci(x: u32) -> u32 {
    if x==1 || x==2 {
        1
    }
    else{
    fibanacci(x-1) + fibanacci(x-2)
    }
}
