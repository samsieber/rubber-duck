pub trait AsOption<T> {
    fn as_option(self) -> Option<T>;
}

impl <T> AsOption<T> for Option<T> {
    fn as_option(self) -> Option<T> {
        self
    }
}
impl <T> AsOption<T> for T {
    fn as_option(self) -> Option<T>{
        Some(self)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
