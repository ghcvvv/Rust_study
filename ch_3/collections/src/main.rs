use std::collections::HashMap;
use std::mem::size_of_val;

fn main() {
    let mut v:Vec<i32>=Vec::new();
    let v1=vec![1,2,3];
    v.push(5);
    v.push(6);
    let third=v[1];
    println!("{}", v[1]);
    let mut v=vec!["DSAD","DASDA","DDDF"];
    let first=v[0];
    for i in v{
        println!("{}",i);
    }
    #[derive(Debug)]
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
    for i in row{
        println!("{:?}",i);
    }
    let mut s = String::from("foo");
    s.push_str("bar");
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // 注意 s1 被移动了，不能继续使用
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let team_name = String::from("Blue");
    let score = scores.get(&team_name).copied().unwrap_or(0);
    for (key,value) in &scores{
        println!("{key}:{value}");
    }
    //根据旧值插入新值
    //统计单词出现的次数
    let text="a a a n v re sda das fsa da a m ";
    let mut map=HashMap::new();
    for word in text.split_whitespace(){
        let  count=map.entry(word).or_insert(0);
        println!("{count}");
        *count+=1;
    }
    for(word,count) in &map{
        println!("{word},{count}");
    }
}
