

use std::cmp::PartialOrd;
fn largest<T:PartialOrd> (list:&[T])->&T{
    let mut largest=&list[0];
    for item in list.iter(){
        if item>largest{
            largest=item;
        }
    }
    largest
}
struct Point<T, U> {
    x: T,
    y: U,
}

impl <T,U> Point<T,U> {
    fn mixup<X,Y>(self,other:Point<X,Y>)->Point<T,Y>{
        Point{
            x:self.x,
            y:other.y,
        }
    }
}

fn main() {
    let number_List=vec![1,2,3,4,5];
    let result=largest(&number_List);
    println!("{}",result);
    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest(&char_list);
    println!("The largest char is {result}");
    let both_integer = Point { x: 5, y: 10 };
    let both_float = Point { x: 1.0, y: 4.0 };
    let integer_and_float = Point { x: 5, y: 4.0 };//可以是不同的数据了
    let p1 = Point { x: 5, y: 10.4 };
    let p2 = Point { x: "Hello", y: 'c' };

    let p3 = p1.mixup(p2);

    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);

}
