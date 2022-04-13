fn child_func_1() {
    println!("child func1");
}
fn child_func_2() {
    println!("child func2");
}
fn child_func_3() {
    println!("child func3");
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_func1() {
        child_func_1()
    }
    #[test]
    fn test_func2() {
        child_func_2()
    }
}
