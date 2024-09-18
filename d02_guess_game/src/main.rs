use rand::Rng;
use std::cmp::Ordering;
use std::io; // 导入随机数生成器

/**
 * 猜数小游戏
 * 从控制台读取用户输入：判断用户输入是否是数字
 * 把用户输入数字和随机的数字作比较，并给出提示
 * 游戏可以循环执行知道猜对
 */
fn main() {
    println!("猜数");
    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("猜测一个数");
        // 定义一个可变的变量绑定到空字符串上
        let mut guess = String::new();
        // 用户输入是可变的，用一个可变的引用；引用默认也是不可变的
        io::stdin()
            .read_line(&mut guess)
            .expect("无法读取输入的数字");

        // 容许用同名的新变量隐藏就变量
        // let guess: u32 = guess.trim().parse().expect("请输入数字");
        let guess: u32 = match guess.trim().parse(){
            Ok(num) => num,
            Err(_) => continue
        };
        // io::Result Ok, Err
        println!("你猜测的数是{}", guess);
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("太小了"),
            Ordering::Greater => println!("太大了"),
            Ordering::Equal => {
                println!("猜对了");
                break;
            }
        }
    }
}
