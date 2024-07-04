//所有程序都必须管理其运行时使用计算机内存的方式。一些语言中具有垃圾回收机制，在程序运行时有规律地寻找不再使用的内存；在另一些语言中，程序员必须亲自分配和释放内存。
//Rust 则选择了第三种方式：通过所有权系统管理内存，编译器在编译时会根据一系列的规则进行检查。如果违反了任何这些规则，程序都不能编译。在运行时，所有权系统的任何功能都不会减慢程序。

//变量的所有权总是遵循相同的模式：将值赋给另一个变量时移动它。当持有堆中数据值的变量离开作用域时，其值将通过 drop 被清理掉，除非数据被移动为另一个变量所有。
use std::arch::x86_64::_subborrow_u32;
fn takes_ownership(some_string: String) { // some_string 进入作用域
    println!("{}", some_string);
} // 这里，some_string 移出作用域并调用 `drop` 方法,函数被视为一个作用域
// 占用的内存被释放

fn makes_copy(some_integer: i32) { // some_integer 进入作用域
    println!("{}", some_integer);
} // 这里，some_integer 移出作用域。没有特殊之处
fn main() {
    let mut s=String::from("hello");//字符串字面值是不可变的
    s.push_str(",world");
    //变量与数据的交互 移动
    let x=5;
    let y=x;//实现了Copy特性,只在栈上的数据
    let s1="hello".to_string();
    let s2=s1;//只是复制了一份指针，指向同一块内存，为了防止两个指针同时释放内存，所以s1被舍弃
    //类似浅拷贝，但是Rust使s1失效了所以称为move
    //变量与数据的交互 克隆
    //如果我们真的需要深度复制堆上的数据，我们需要使用clone
    let s1=s2.clone();
    println!("{}{}",s1,s2);

    //所有权与函数
    let s = String::from("hello");  // s 进入作用域

    takes_ownership(s);             // s 的值移动到函数里 ...
    // ... 所以到这里不再有效

    let x = 5;                      // x 进入作用域

    makes_copy(x);                  // x 应该移动函数里，
    // 但 i32 是 Copy 的，
    // 所以在后面可继续使用 x
}
