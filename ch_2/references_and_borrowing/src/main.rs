

//引用像是一个指针，我们可以由此访问存储该地址的属于其他变量的数据。
fn main() {
    let s1="hello world".to_string();
    println!("{}",calculate_length(&s1));
    println!("{}",s1);//引用不会获取所有权
    //尝试修改引用的值
    let mut s1="hello world".to_string();
    change(&mut s1);
    println!("{}",s1);
    //可变引用有一个很大的限制：如果你有一个对该变量的可变引用，你就不能再创建对该变量的引用。

}
fn calculate_length(s:&String)->usize{
    s.len()
}
fn change(str:& mut String){
    str.push_str("1111");
}

//悬垂引用
fn dangle<'a>() ->  & 'a String { // dangle 返回一个字符串的引用

    let s = String::from("hello"); // s 是一个新字符串

    &s // 返回字符串 s 的引用
} // 这里 s 离开作用域并被丢弃。其内存被释放。
// 危险！