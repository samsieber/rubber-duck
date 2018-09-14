#![feature(proc_macro_gen)]
#![feature(decl_macro)]
use rubber_duck::macros::*;
#[doc(hidden)]
pub use rubber_duck::core::*;

use std::fs::File;

pub mod module {
    use super::*;
    use std::fs::File;
    use std::fs::OpenOptions;
    use std::path::Path;

    #[gen_struct_sugar(defaults(name = r#""Bob".to_owned()"#))]
    pub fn is_a_test(name: String, message: String) -> String {
        let i = 0;
        let i = i + 1;
        format!("{}) Hello {}, {} The end.", i, &name, &message)
    }

    #[gen_struct_sugar(
        defaults(
            read = "false",
            write = "false",
            append = "false",
            truncate = "false",
            create = "false",
            create_new = "false"
        ),
        positionals(path),
    )]
    pub fn open_file(
        path: impl AsRef<Path>,
        read: bool,
        write: bool,
        append: bool,
        truncate: bool,
        create: bool,
        create_new: bool,
    ) -> std::io::Result<File> {
        OpenOptions::new()
            .read(read)
            .write(write)
            .append(append)
            .truncate(truncate)
            .create(create)
            .create_new(create_new)
            .open(path)
    }
}

pub mod doc_test {
    pub struct S {
        name : String,
        greeting: String,
        answer: u32,
    }

    pub fn match_struct_S(S { name: test, greeting, answer} : S){

    }
}

//use crate::module::is_a_test;
use crate::module::open_file;

pub fn testing_call() -> std::io::Result<File> {
    open_file!("test.txt", read => true)
}

#[cfg(test)]
mod tests {
    use crate::module::is_a_test;
    use super::*;
    use std::error::Error;
    use std::io::Read;
    use std::result::Result::Ok;

    #[test]
    fn open_file_works() -> Result<(), Box<Error>> {
        let mut handle = crate::module::open_file!("test.txt", read => true)?;
        let mut contents = String::new();
        handle.read_to_string(&mut contents)?;

        assert_eq!("hello\n", contents);
        Ok(())
    }

    #[test]
    fn only_named_works() {
        assert_eq!(
            "1) Hello Bill, It's working! The end.",
        is_a_test!(name => Some("Bill".to_owned()), message => "It's working!".to_owned())
        );
        assert_eq!(
            "1) Hello Bill, It's working! The end.",
        crate::module::is_a_test!(name => "Bill".to_owned(), message => "It's working!".to_owned())
        );
        assert_eq!(
            "1) Hello Bob, It's working! The end.",
        crate::module::is_a_test!(message => "It's working!".to_owned())
        );
    }
}
