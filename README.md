# Rubber Duck

Rubber duck is a library to facilitate writing functions that can be called with named value syntax (and with optional default values) - requires nightly.

See the rust doc of the rubber-duck crate for usage information. 
You'll need to checkout this repo and build the docs though, so here's a sample:


```rust
// Given this api declaration:
mod module {
   #[gen_struct_sugar(
        defaults(greeting = r#""Hello.""#),
        positionals(name),
    )]
   pub fn is_a_test(name: &'static str, greeting: &'static str, message: &'static str) -> String {
       format!("Dear {}, {}. {}", &name, &greeting, &message)
   }
}

// One can call the function in a variety of ways
mod stable{
    use crate::{n,module::is_a_test};
    // Named form requires a macro
    n!(is_a_test{"George", {greeting: "Hi.", message: "Rust is cool."}});   // Dear George, Hi. Rust is cool.
    // and lets you use defaults                                   
    n!(is_a_test{"George", {message: "Hi."}});                              // Dear George, Hello. Rust is cool.
    // and even lets you use sugar (you need a trailing comma if there's only one named arg
    let message = "Struct Sugar";                                           // Dear George, Hello. Struct Sugar
    n!(is_a_test{"George", {message,}}) 
 
    // Positional form doesn't need a macro, but args with defaults are wrapped in the option type
    // Override the default
    is_a_test("Bob", Some("Hi."), "Goodbye.");                              // Dear Bob, Hi. Goodbye.
    // Use the default
    is_a_test("Bob", None, "Goodbye.");                                     // Dear Bob, Hello. Goodbye. 
}

//There's also a slightly nicer way on nightly (behind the features=["nightly"] flag)
mod nightly_only {
    use crate::module::is_a_test;
     // Named form requires a macro
    is_a_test!("George", greeting=> "Hi.", message=> "Rust is cool");        // Dear George, Hi. Rust is cool.
     // and lets you use defaults                                   
    is_a_test!("George", message=> "Rust is cool");                          // Dear George, Hello. Rust is cool.  
}
// You don't even have to import it!
crate::module::is_a_test!("George", greeting=> "Hi.", message=> "Rust is cool");
```

See the REVIEW.md for general review about the previous named & default function arguments RFCs and discussions.

## Other Details

Not wanting to come up with a long descriptive name, or use a good name, I picked Rubber Duck. I wanted to explore options
for named & default arguments. I hope it furthers the progression towards an RFC for adding them to the language.

It adds two main calling methods to a function (doesn't work inside impls or with traits).

On stable and nightly, you can use the `n!` macro to wrap a function call and give yourself named/default argument calling capabilities.

On nightly, the named & default argument calling syntax are exported as decl 2.0 macros in the same place as the functions you write.

If you're interested in this, you might also want to check out the [namedarg](https://github.com/comex/namedarg) crate by comex.

