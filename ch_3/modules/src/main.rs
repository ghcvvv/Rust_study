mod back_of_house{
    #[derive(Debug)]
    pub struct Breakfast{
        pub toast:String,
        pub seasonal_fruit:String,
    }

    impl Breakfast {
        pub fn summer(toast :&String)->Breakfast{
            Breakfast{
                toast:toast.clone(),
                seasonal_fruit:"peaches".to_string(),
            }
        }

    }
}
use crate::back_of_house::Breakfast;//crate是根
use std::collections::HashMap;
fn main() {
    let mut meal=back_of_house::Breakfast::summer(&"Rye".to_string());
    meal.toast="wheat".to_string();
    println!("{}",meal.seasonal_fruit);
    println!("{:?}",meal);
    let mut map=HashMap::new();
    map.insert(1,2);
}
