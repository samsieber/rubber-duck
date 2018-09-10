#![feature(decl_macro)]


pub mod macros {
    pub use rubber_duck_builder::*;
    pub use rubber_duck_params::*;
}

pub mod core{
    pub trait AsOption<T> {
        fn as_option(self) -> ::std::option::Option<T>;
    }
    impl<T> AsOption<T> for ::std::option::Option<T> {
        fn as_option(self) -> ::std::option::Option<T> {
            self
        }
    }
    impl<T> AsOption<T> for T {
        fn as_option(self) -> ::std::option::Option<T> {
            Some(self)
        }
    }

    pub struct Unset;
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
