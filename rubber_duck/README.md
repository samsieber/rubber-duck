# Overview
This crate provides a psuedo-implementation of one of many named argument RFCs.
It's not exactly like the real thing, given that macros are used instead of real compiler support.
This crate provides an attribute-like proc macro to annotate a function with to generate named argument support.

That annotation does the following:
 * Generates a struct with the same arguments as the function
    * along with an associated builder() function
    * and this struct is hidden from docs by default
 * Generates a builder for said struct method
    * It's a type-safe builder, and requires that each method without a default value must be called
    * The build function converts the builder back into the argument struct
    * This is strcut is also doc hidden
 * Default values can be specified in the annotation for the fields, for each
    * The field type is wrapped in an Option (e.g. T changes from T to Option<T>)
    * A default value of None is specified on the builder
    * And with type of Option<T>, the builder setter for the field accepts T or Option<T>
 * The positional arguments can be specified as well (no defaults allowed for those)
 * A macro is generated with the same name as the function, which accepts named parameters (nightly only)
    * creates the builder
    * sets whatever arguments are passed in
    * calls build (which will be a compile error if not all named values are provided if the lack defaults)
 * Another macro exits as well - `n!` which you wrap a function call with to enable named/default arg syntax. 

 So how this all works out is that we overload the method name in several namespaces (fn, struct and macro),
 such that the consumer can just us the plain fn or the macro

## Limitations

This implementation punts on the declaration side - e.g. how to declare the parameters as being named, and how to specify
parameter defaults. It instead specifies all those in the attribute to avoid parsing issues. A future version could switch
from an attribute-like proc macro to a function-like proc macro to actually experiment with delcaration syntax

This version doesn't deal well with generics. That seems like a solvable limitation in this approach

This version only works with stand alone functions, not functions in impl blocks. That should be solveable as well.

This version requires nightly (for decl. macro 2.0 and proc_macro_gen) and the 2018 edition (due to macro paths).
While proc_macro_gen might be stabalized soon, decl. macros 2.0 won't (I think). Decl. Macros 2.0 were used instead of
macro_rules to get proper module namespacing support for the generated macros

### Probably bug imposed

The macros exported from rubber_duck don't appear in the rubber_duck docs

The macros generated for the api declarer don't get shown in the correct module in the generated docs
[Github issue #54112](https://github.com/rust-lang/rust/issues/54112)

# Using
## Prerequisites:
* Use edition 2018 (macro paths between editions interact very poorly with what I'm doing)

### Nightly vs Not-Nighlty
This crate has some features that are only available on nightly.

At this point, there is not feature detection of nighly vs not-nightly. Instead, in the crate
declaring the functions that you want to call with named/default arg syntax, you must opt into
the nighly-only features by using `features=["nightly"]`

### Writing an API

To setup a crate to publish methods that can be called with named/default arg syntax,
you'll need some initial setup work of modifying your Cargo.toml and lib.rs. Then you'll be able to
annotate the methods. Any consuming crates won't need to do anything different.

#### Setup

Add this to your Cargo.toml if you want the nightly features
```toml
[dependencies]
rubber_duck = { version="0.2", features=["nightly"]}
```
Otherwise add this if you're on stable
```toml
[dependencies]
rubber_duck = "0.2"
```

Then add this at the top of your lib.rs

```
// The following two lines (well, 4 if you count comments) are only needed when using features=["nightly"]
// Allows us to generate macros from macros
#![feature(proc_macro_hygiene)]
// Allows the use of declarative macros 2.0 (which are generated from the proc macro
#![feature(decl_macro)]

// These lines are needed on both nightly and stable
// Hide the base items the macro internals use
#[doc(hidden)]
// Import the base items the macro internals use
// This must be at the base of the crate
pub use rubber_duck::core::*;
```

#### Annotating Functions
To make a function callable with named syntax, in the same mod as the function, import the macros:

```
use ::rubber_duck::macros::*;
```

Then annotate a function with `#[gen_struct_sugar]` - that will be enough to generate a macro with
that can be called with named syntax.

Furthermore, you can declare a) positional parameters and b) default values for named parameters

```
#[gen_struct_sugar(
       defaults( // You don't have to set defaults for all named parameters, but here we do
           read = "false",
           write = "false",
           append = "false",
           truncate = "false",
           create = "false",
           create_new = "false",
       ),
       positionals(path), // Here we list the positional parameters in the order they appear
   )]
   pub fn open_file(
       path: PathBuf,
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
```

### Consuming an API
To consume an API, one can
 a) call the method, like normal - any methods that have default values of type T will actually
     be of type Option<T>
 b) wrap the method call in the `n!` macro. Change the `()` of the method call to `{}` and
    put any named arguments in another `{}` as an argument in the form of `{name: value}` or `{name}` (like the struct construction sugar).
    A single named argument needs a trailing comma.
  ) call the method with a bang `!` - any positional parameters must go first in proper order without names,
    the named parameters go next in the form of `name => value`, where name is the publish arg name in the docs

Furthermore, consumer is in a separate crate from the writer, they don't need any feature flags enabled

They still need to be on nightly though and probably 2018 (because of paths...)

## Full Example:

Given api declaration:

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
mod stable {
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

Also, take a look at the example_api and example_consumer directories in the github repository.