use std::io;
use rand::Rng;
use std::cmp::Ordering;

// 带有返回值的方法返回值
fn five()->i8{
   let a =3;
   let b =5;
   a+b

}

fn cat()->char{
    let a =3;
    let b=100;
    a+b;
    let c= '😀';
    c
}

fn main() {
    println!("Hello, world!");
    let secret_number = rand::thread_rng().gen_range(1..=5);
    guess_game(secret_number);
    let x = five();
    let b = cat();
    println!("the x value is :{}",x);
    println!("the b value is :{}",b);
}

// 额外带有参数的方法
fn guess_game(secret_number:i32){
    
    loop{
        println!("please input your number");

        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("file to read line");
        
        let input:i32 = input.trim().parse().expect("please input number");

        match input.cmp(&secret_number){
            Ordering::Less => println!("too small"),
            Ordering::Greater => println!("too big"),
            Ordering::Equal=>{
                println!("you win");
                break;
            }
        }
    }

}

