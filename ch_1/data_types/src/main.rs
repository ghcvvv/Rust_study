use std::str::FromStr;
use std::io;
fn main() {
    let guess:u32="32".parse().expect("not a number");
    //长度 8 16 32 64 128
    //有符号数i8 i16 i32 i64 i128 isize
    //无符号数u8 u16 u32 u64 u128 usize
    //其中usize和isize是与计算机架构有关的
    let x=6.0;
    let y=5;
    let z="sds";
    let heart_eyed_cat='🐱';
    let tup:(i32,f64,u8)=(500,6.4,1);
    let a=[1,2,3,4,5];
    let mut index=0;
    for i in 0..5{
        println!("{}",a[i]);
    }
    let a: [i32; 5] = [1, 2, 3, 4, 5];

}
