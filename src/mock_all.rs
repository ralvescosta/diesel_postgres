pub trait MyTrait {
    fn do_something(&self) -> bool;
}
#[cfg(test)]
mod tests {
    use super::MyTrait;
    use mockall::predicate::*;
    use mockall::*;

    struct MyStruct;
    #[automock]
    impl MyTrait for MyStruct {
        fn do_something(&self) -> bool {
            todo!()
        }
    }
    #[test]
    fn test() {
        let mut mock = MockMyStruct::new();
        mock.expect_do_something().returning(|| false);
        let result = mock.do_something();
        println!("{}", result)
    }
}
