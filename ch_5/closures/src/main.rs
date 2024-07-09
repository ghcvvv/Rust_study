use std::arch::x86_64::_mm256_i32gather_epi64;
use std::io::stdin;

#[derive(Debug)]
enum ShirtColor{
    Red,
    Blue,
}
struct Inventory{
    shirts:Vec<ShirtColor>,
}



impl Inventory{
    fn giveayay(&self,user_performance:Option<ShirtColor>)->ShirtColor{
        user_performance.unwrap_or_else(|| self.most_stocked())
    }
    fn most_stocked(&self) ->ShirtColor{
        let mut mut_red =0;
        let mut mut_blue =0;
        for shirt in &self.shirts{
           match shirt {
               ShirtColor::Red=>mut_red+=1,
               ShirtColor::Blue=>mut_blue+=1
           };
        }
        if mut_red>mut_blue{
            return ShirtColor::Red;
        }else {
            return ShirtColor::Blue;
        }
    }
}
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}
fn main() {
    let store = Inventory {
        shirts: vec![ShirtColor::Blue, ShirtColor::Red, ShirtColor::Blue, ShirtColor::Red, ShirtColor::Blue, ShirtColor::Blue]
    };
    // let mut user_performances: Vec<Option<ShirtColor>> = Vec::new();
    // for i in 0..2{
    //     let mut input=String::new();
    //     stdin().read_line(&mut input).expect("please enter a valid input");
    //     input.trim();
    //     match input.as_str() {
    //         "Red"=>user_performances.push(Some(ShirtColor::Red)),
    //         "Blue"=>user_performances.push(Some(ShirtColor::Blue)),
    //         _ => user_performances.push(None),
    //     };
    // }
    // for user_performance in user_performances{
    //     let giveaway=store.giveayay(user_performance);
    //     println!("giveaway:{:?}",giveaway);
    // }
    let expensive_closure=|x:i32|->i32{
        x+x
    };
    println!("{}", expensive_closure(2));
    let mut list = [
        Rectangle { width: 10, height: 1 },
        Rectangle { width: 3, height: 5 },
        Rectangle { width: 7, height: 12 },
    ];
    list.sort_by_key(|r| r.width);
    println!("{:?}",list);
    let v1=vec![1,2,3];
    let toral:i32=v1.iter().sum();//这些调用 next 方法的方法被称为 消费适配器（consuming adaptors），因为调用它们会消耗迭代器。一个消费适配器的例子是 sum 方法。这个方法获取迭代器的所有权并反复调用 next 来遍历迭代器
    let new_vec:Vec<i32>=v1.iter().map(|x| x+1).collect();//collect方法消耗迭代器生成了一个集合
    println!("{new_vec:?}");
    for val in v1.iter(){
        println!("{val}");
    }
}
#[test]
fn iterator_demonstration() {
    let v1 = vec![1, 2, 3];

    let mut v1_iter = v1.iter();

    assert_eq!(v1_iter.next(), Some(&1));
    assert_eq!(v1_iter.next(), Some(&2));
    assert_eq!(v1_iter.next(), Some(&3));
    assert_eq!(v1_iter.next(), None);
}
