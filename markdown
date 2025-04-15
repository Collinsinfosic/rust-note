用户键盘监控
use std::io::{self,Read};

fn main(){
    for b in io::stdin().bytes(){
        let c = b.unwrap() as char;
// 读取输入：
// io::stdin() 创建一个标准输入的句柄（可以理解为连接到键盘输入的工具）。
// .bytes() 将输入流按字节（byte）逐个读取，返回一个迭代器，每次给一个字节。
// 循环处理字节：
// for b in io::stdin().bytes() 遍历每个字节，b 是每个字节的值（类型是 Result<u8, std::io::Error>）。
// b.unwrap() 取出字节的值（u8 类型），忽略可能的错误（如果输入有问题，程序会崩溃）。
// as char 把字节转换为字符（比如字节 65 对应字符 'A'）。
        if c == 'q'{
            break;
        }
        println!("{}",c);
    }
}



