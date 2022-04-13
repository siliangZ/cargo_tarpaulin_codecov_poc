mod child;
fn func1() {
    println!("function 1");
}

fn func2() {
    println!("function 2");
}

fn func3() {
    println!("untested function");
}

mod tests {
    use super::{func1, func2};
    #[test]
    fn test_func1() {
        func1();
    }

    #[test]
    fn test_func2() {
        func2();
    }
}
