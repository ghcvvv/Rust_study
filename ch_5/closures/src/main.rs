
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

fn main() {
    let store = Inventory {
        shirts: vec![ShirtColor::Blue, ShirtColor::Red, ShirtColor::Blue, ShirtColor::Red, ShirtColor::Blue, ShirtColor::Blue]
    };
    let mut user_performances: Vec<Option<ShirtColor>> = Vec::new();
    for i in 0..2{
        let mut input=String::new();
        stdin().read_line(&mut input).expect("please enter a valid input");
        input.trim();
        match input.as_str() {
            "Red"=>user_performances.push(Some(ShirtColor::Red)),
            "Blue"=>user_performances.push(Some(ShirtColor::Blue)),
            _ => user_performances.push(None),
        };
    }
    for user_performance in user_performances{
        let giveaway=store.giveayay(user_performance);
        println!("giveaway:{:?}",giveaway);
    }
    let expensive_closure=|x:i32|->i32{
        x+x
    };
    println!("{}", expensive_closure(2));
}