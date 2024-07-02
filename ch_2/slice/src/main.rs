use std::array::from_fn;

fn first_word(s:&String) ->&str{
    let bytes=s.as_bytes();//将string转化为了字节数组
    for(i,&item) in bytes.iter().enumerate(){
        if item==b' '{
            return &s[0..i];
        }
    }
    &s
}

fn main() {
    let s1="hello world".to_string();
    let s=first_word(&s1);
    println!("{s}");
}
