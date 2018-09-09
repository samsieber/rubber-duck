extern crate derive_builder_sugar;
extern crate examples;

use derive_builder_sugar::gen_typesafe_builder;
use examples::ex::{is_a_test, Is_a_test};

mod foo;

pub struct Unset;

pub struct Example {
    a: String,
    b: Box<u32>,
    c: Option<Box<Example>>,
    d: usize,
    e: Vec<String>,
}

pub struct ExampleBuilder<A, B, C, D, E> {
    a: A,
    b: B,
    c: C,
    d: D,
    e: E,
}

impl ExampleBuilder<Unset, Unset, Unset, Unset, Unset> {
    pub fn default() -> ExampleBuilder<Unset, Unset, Option<Box<Example>>, Unset, Unset> {
        ExampleBuilder {
            a: Unset,
            b: Unset,
            c: None,
            d: Unset,
            e: Unset,
        }
    }
}

impl<T2, T3, T4, T5> ExampleBuilder<Unset, T2, T3, T4, T5> {
    pub fn a(self, value: String) -> ExampleBuilder<String, T2, T3, T4, T5> {
        ExampleBuilder {
            a: value,
            b: self.b,
            c: self.c,
            d: self.d,
            e: self.e,
        }
    }
}

impl<T1, T3, T4, T5> ExampleBuilder<T1, Unset, T3, T4, T5> {
    pub fn b(self, value: Box<u32>) -> ExampleBuilder<T1, Box<u32>, T3, T4, T5> {
        ExampleBuilder {
            a: self.a,
            b: value,
            c: self.c,
            d: self.d,
            e: self.e,
        }
    }
}

impl<T1, T2, T3, T4, T5> ExampleBuilder<T1, T2, T3, T4, T5> {
    pub fn c(
        self,
        value: Option<Box<Example>>,
    ) -> ExampleBuilder<T1, T2, Option<Box<Example>>, T4, T5> {
        ExampleBuilder {
            a: self.a,
            b: self.b,
            c: value,
            d: self.d,
            e: self.e,
        }
    }
}

impl<T1, T2, T3, T5> ExampleBuilder<T1, T2, T3, Unset, T5> {
    pub fn d(self, value: usize) -> ExampleBuilder<T1, T2, T3, usize, T5> {
        ExampleBuilder {
            a: self.a,
            b: self.b,
            c: self.c,
            d: value,
            e: self.e,
        }
    }
}

impl<T1, T2, T3, T4> ExampleBuilder<T1, T2, T3, T4, Unset> {
    pub fn e(self, value: Vec<String>) -> ExampleBuilder<T1, T2, T3, T4, Vec<String>> {
        ExampleBuilder {
            a: self.a,
            b: self.b,
            c: self.c,
            d: self.d,
            e: value,
        }
    }
}

impl ExampleBuilder<String, Box<u32>, Option<Box<Example>>, usize, Vec<String>> {
    pub fn build(self) -> Example {
        Example {
            a: self.a,
            b: self.b,
            c: self.c,
            d: self.d,
            e: self.e,
        }
    }
}

//impl <T1, T2, T3, T4, T5> ExampleBuilder<T1, T2, T3, T4, T5> {
//    pub fn method(self, value: Value) -> ExampleBuilder<T1, T2, T3, T4, T5>{
//        ExampleBuilder {
//            a: self.a,
//            b: self.b,
//            c: self.c,
//            d: self.d,
//            e: self.e,
//        }
//    }
//}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mod_macros_work() {assert_eq!(
        "1) Dear Bob, Good day :) The end.",
        is_a_test!(message => "Good day :)".to_owned())
    );
    }

    #[test]
    fn it_works() {
        ExampleBuilder::default()
            .a("Hello".to_owned())
            .b(Box::new(32))
            .c(None)
            .c(None)
            .d(11)
            .e(vec!["Hllo".to_owned()])
            .build();
    }
}
