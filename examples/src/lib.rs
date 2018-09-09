#![feature(proc_macro_gen)]
#![feature(decl_macro)]

extern crate derive_builder_sugar;
extern crate derive_params_sugar;
extern crate base;

use base::*;

use derive_builder_sugar::gen_typesafe_builder;
use derive_params_sugar::gen_struct_sugar;

//#[gen_typesafe_builder]
//pub struct Ex {
//    #[default = "None"]
//    a: Option<usize>,
//    b: u32,
//    #[default = r#""hello".to_owned()"#]
//    c: String,
//    d: Vec<u8>,
//}

#[macro_use]
pub mod ex {
    use super::*;

    #[gen_struct_sugar(defaults(name = r#""Bob".to_owned()"#,))]
    pub fn is_a_test(name: String, message: String) -> String {
        let i = 0;
        let i = i + 1;
        format!("{}) Dear {}, {} The end.", i, &name, &message)
    }

    pub macro testing {
        ($($all:tt)*) => { $($all)* }
    }
}

use ex::{is_a_test, Is_a_test };

pub fn main() -> String {
//    panic!()
    is_a_test!(name => Some("Bill".to_owned()), message => "It's working!".to_owned())
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(
            "1) Dear Bill, It's working! The end.",
            is_a_test!(name => Some("Bill".to_owned()), message => "It's working!".to_owned())
        );
        assert_eq!(
            "1) Dear Bill, It's working! The end.",
            main()
        );
        assert_eq!(
            "1) Dear Bill, It's working! The end.",
            is_a_test!(name => "Bill".to_owned(), message => "It's working!".to_owned())
        );
        assert_eq!(
            "1) Dear Bob, It's working! The end.",
            is_a_test!(message => "It's working!".to_owned())
        );
    }
}
