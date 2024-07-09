pub fn add(left: usize, right: usize) -> usize {
    left + right
}
pub fn add_one(x:i32)->i32{
    x+1
}
use rand;
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
    fn it_works2(){
        let result=add_one(3);
        assert_eq!(result,4);
    }
}
