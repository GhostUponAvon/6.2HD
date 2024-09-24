use std;

fn main() {

   let args: Vec<String> = std::env::args().collect(); 
    
    let a: i32 = args[1].clone().parse().unwrap();
    let b: i32 = args[2].clone().parse().unwrap();

    let sum: i32 = add(a, b);

    println!("The sum is: {}", sum);
}

fn add(a: i32, b: i32) -> i32 {
    return a+b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn addition_test() {
        assert_eq!(add(10,10), 20)
    }
}