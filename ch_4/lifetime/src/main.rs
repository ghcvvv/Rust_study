fn longest<'a>(x:&'a str,y:&'a str)->&'a str{
    if x.len()>y.len(){
        x
    }else {
        y
    }
}
struct ImportantExcerpt<'a> {
    part: &'a str,
}//结构体的生命周期


fn main() {
    fn main() {
        let string1 = String::from("long string is long");

        {
            let string2 = String::from("xyz");
            let result = longest(string1.as_str(), string2.as_str());
            println!("The longest string is {result}");
        }
    }
    // let s: &'static str = "I have a static lifetime.";
    // println!("Hello, world!");
}
