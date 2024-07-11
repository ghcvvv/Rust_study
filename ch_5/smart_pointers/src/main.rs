//BOX<T>,用于在堆上分配值
//Rc<T>,一个引用计数类型，其数据可以有多个所有者
//Ref<T>和 RefMut<T>,通过RefCell<T>访问，




//BOX<T>box 允许你将一个值放在堆上而不是栈上。留在栈上的则是指向堆数据的指针。
//当有一个在编译时未知大小的类型，而又想要在需要确切大小的上下文中使用这个类型值的时候
//当有大量数据并希望在确保数据不被拷贝的情况下转移所有权的时候
//当希望拥有一个值并只关心它的类型是否实现了特定 trait 而不是其具体类型的时候

use std::marker::PhantomData;
use std::ops::Deref;
use std::rc::Rc;

struct MyBox<T>(T);
impl <T> MyBox<T>{
    fn new(x:T)->MyBox<T>{
        MyBox(x)
    }
}
impl <T> Deref for MyBox<T>{
    type Target =T;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
struct CustomSmartPointer{
    data:String
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPinter with data:{}",self.data);
    }
}//当实例离开作用域 Rust 会自动调用 drop，并调用我们指定的代码。变量以被创建时相反的顺序被丢弃，所以 d 在 c 之前被丢弃。
fn main() {
    let b=Box::new(5);
    println!("{:?}",b);
    let x = 5;
    let y = &x;

    assert_eq!(5, x);
    assert_eq!(5, *y);//引用与解引用

    let c=CustomSmartPointer{
        data:"Hello world!".to_string()
    };
    let d = CustomSmartPointer {
        data: String::from("other stuff"),
    };
    //Rc<T> 用于当我们希望在堆上分配一些内存供程序的多个部分读取，而且无法在编译时确定程序的哪一部分会最后结束使用它的时候。如果确实知道哪部分是最后一个结束使用的话
    // 就可以令其成为数据的所有者，正常的所有权规则就可以在编译时生效。
    let a=Rc::new(6);
    let b=Rc::clone(&a);
    {
        let c = Rc::clone(&a);
        println!("count after creating c = {}", Rc::strong_count(&a));
    }
    println!("a = {}", a);
    println!("b = {}", b);

    // 打印引用计数
    println!("Reference count: {}", Rc::strong_count(&a));
}
