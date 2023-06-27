use rand::Rng;
use std::io;
use std::cmp::Ordering;


fn main() {
    println!("請猜一個數字!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    // println!("秘密數字為: {secret_number}");

    loop {
        println!("請輸入你猜的數字。");
        
        let mut guess = String::new();
    
        io::stdin()
            .read_line(&mut guess)
            .expect("讀取該行失敗");
    
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        // .expect("請輸入一個數字!");
    
        println!("你猜的數字: {guess}");
    
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("太小了!"),
            Ordering::Greater => println!("太大了!"),
            Ordering::Equal => {
                println!("獲勝!");
                break;
            },
        }
    }
}
