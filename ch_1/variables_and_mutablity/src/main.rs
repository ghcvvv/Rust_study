

const THREE_HOURS_IN_SECONDS:u32=60*60*3;

fn main() {
    let x=5;
    println!("{x}");
    // x=6;变量默认不可以再修改
    let mut x=6;
    x=5;//使用mut定义后就可以修改
    println!("{x}");
    let spaces="     ";
    let spaces=spaces.len();
    println!("{spaces}");

}