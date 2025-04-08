use std::io;

fn main(){
    //变量与可变性
    let mut x= 3;
    println!("the x value is :{}",x);
    x = 11119;
    println!("the x value is :{}",x);
    //常量,使用const标识
    //Rust 说：“常量名按习惯得用大写字母，不然看着不规范。
    const Y:i64 = 60*60*60*10000;

    println!("the y value is :{}",Y);
    
    let spaces = "   ";
    let spaces = spaces.len();
    println!("the spaces lenght is:{}",spaces);

    //let mut s1 = "   ";
    //s1 = s1.len();
    //println!("the s1 length is: {}",s1);
    //上面两端代码的区别：spaces是重新声明遍历并覆盖，mut是对原始的变量
    //进行赋值，所以s1.len()会报错，因为原始的s1是str类型，s1.len是数字类型
    let num1 = 98_1000;
    println!("th num1 is :{}",num1);

    let num_10 = 1_0;
    println!("this is ten  decimal:{}",num_10);
    let num_16 = 0xa;
    println!("this is ten hex:{}",num_16);
    let num_8 = 0o12;
    println!("this is ten octal:{}",num_8);
    let num_2 = 0b1010;
    println!("this is ten binary:{}",num_2);
    let num_str = b"10";
    println!("this is ten byte u8 only:{:?}",num_str);
    //{}只能打印人看的懂的，对于数组，字符串那种，需要用{:?}来打印，和{}不同
    //的是，他表示debug模式，{:?}：随便啥都能打出来，但可能是“程序员格式”（需要 Debug）


    let sum = 31+2;
    println!("the sum is:{}",sum);
    let sub = 99.99-98.99;
    println!("the sub is :{}",sub);

    let mult = 31.0*10.1;
    println!("the mult is :{}",mult);
    let quot = 57.6/31.1;
    let trun = -6/3;
    println!("the quot is:{}",quot);
    println!("the trun is:{}",trun);

    let rema = 43%5;
    println!("the rema is :{}",rema);


    let tup1:(u8,f32,char) = (9,3.99,'😻');
    println!("the tup is :{:?}",tup1);

    // 故意不用，加下划线,rust比较严谨，认为定义不用的变量可能存在
    //隐患，因此会用warring来警告
    // 如果你就是想定义这个变量，但暂时不用（比如在学习字符类型），
    // 可以在变量名前加 _，告诉 Rust：“我知道不用，别警告我。”
    let tup2 = (500,6.4,10);
    let (x, _y, _z) = tup2;
    println!("the x value is :{}",x);

    let tup3:(i8,f32,char) = (9,3.99,'😻');
    println!("one value is :{}",tup3.0);
    println!("two value is :{}",tup3.1);
    println!("three value is :{}",tup3.2);
    println!("---------------------------------------------------------------");
    // 元组不能使用for循环遍历
    let items:[&dyn std::fmt::Display; 3]=[&tup3.0,&tup3.1,&tup3.2];
    for (index,item) in items.iter().enumerate(){     
        println!("the tup3's {} value is :{}",index,item);
    }

    println!("---------------------------------------------------------------");

    let months = ["January", "February", "March", "April", "May", "June", "July",
              "August", "September", "October", "November", "December"];
    //数组可以通过for循环来打印
    for (index,item) in months.iter().enumerate(){
        println!("the {} month is :{}",index+1,item);
    }

    // 数组的长度是固定的，不能动态改变，如果给的下标超出长度
    //会报错
    let list = [1,2,3,4,5];
    println!("please input the index of the list:");
    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("failed to read line");
    
    let index:usize = index.trim().parse().expect("please type a number");

    let list_member = list[index];
    println!("the value is :{}",list_member);
}
