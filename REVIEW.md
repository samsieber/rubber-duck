# Named and Default Parameters in Rust

There have been numerous RFCs and dicussions around named and default parameters in Rust, 
eventually coalescing into a [wishlist issue in the RFCs repository.](https://github.com/rust-lang/rfcs/issues/323) 
I'd like to briefly consider how we should frame such considerations, 
review the discussion history and explored trade-offs and then propose a potential path forward. 

I say potential path because after writing this, I'm still not entirely certain we should try to get this into Rust.

### Motivations

__Explicitness__: Named Parameters make things more explicit. 
That's useful in large codebases, and for when an autocomplete isn't available.

__What about the builder pattern?__ The gold standard is the builder pattern. A brief overview:

__Pro: Power__
* Can use `Into<T>` or `AsRef<T>` in parameter types and convert them inside
* Can do other runtime checking
* Can even switch forms based on what arguments are provided
* If you're feeling like a pill, you can even enforce argument ordering *at compile time* 
* You can even make typesafe, requiring every field to be set exactly once *at compile time*

__Con: Clunk Factor__
* Need a separate builder for every function signature
* Hard to distinguish from other function calls
    * Imagine trying to use builders to build a DAG... or html with attributes
* But the clunk is only in using - one can easily derive builders now-days
* The API docs for advanced type builders (e.g. using it as a state machine or making it typesafe) is hard to read.

To be fair some of the pros and cons only surface when you using the consuming builder pattern that's actually a state machine.
But I'd argue that that's still the builder pattern, and rust just makes it more powerful with it's good type system.

### Weaker But Easier

How do named & default parameters line up with that evaluation? They appear to be strictly weaker than builders,
so adding them is only justified if they lead to less boiler-plate. 
We've special cased the happy path before in Rust - Struct Field Initialization Shorthand, the `try!` to `?` change, slice ranges, etc.

So ideally, implementing named & default arguments leads to some of the benefits of the builder pattern at a lower cost,
the benefits the builder being:
1) Naming arguments
2) Allow for default arguments
3) Being able to add new arguments to the API without breaking consumers.

These benefits are very important for certain types of API - consider for example the std::fs::OpenOptions. With 6 unique
settings, it's very suited to the builder pattern. Writing functions for individual each of the option combinations is impracticable
Using a struct with public fields would push boilerplate onto the consumers of the api, as well as freezing it. The boiler-plate
associated with writing and using the builder is definitely worth it in that case.

There are other API's for which the benefits are much less clear. Things that have only one or two separate arguments.

Implementing named and default arguments reduces the boiler-plate and maintence cost of gaining those benefits (well, attributes). 
That can be both good and bad, encouraging both more consise APIs (1 fn with a couple options vs a couple of specialized fns) and less precise APIs (a monster fn that takes 10+ arguments).

There is one other place that named arguments would shine: nesting builders. Building directed graphs in rust is tedious if builders are involved.
You can nest them or not. Either way, it's hard to get a good feel for what a built tree actually looks like because the 
the syntax for declaring a node vs setting an attribute is so similar.  
    
# Previous Attempts Timeline

With all of that said, 
here's an overview of various attempts to figure out how named & default parameters could be implemented in rust,
and whether they should be.

#### Rust PR: Default arguments and keyword arguments
`June 2013` 
| [Precursor Pull Request](https://github.com/rust-lang/rust/issues/6973) 
| [Continued Reddit Discussion](https://www.reddit.com/r/rust/comments/26ojst/named_optional_parameters/)

This was a general pull request to the rust-lang repository about creating functionality for named and default arguments.

There were conflicting views in the thread but it seemed the prevailing idea was struct sugar with FRU.

The conversation slowed because it wasn't a big priority pre 1.0, and eventually closed because it really should have been an RFC.

`July 2014` Postponed

#### RFC: Optional Paramters (*iopq*)
`July 2014`
| [Pull Request](https://github.com/rust-lang/rfcs/pull/152) 
| [Rendered](https://github.com/iopq/rfcs/blob/40fd9c156cf6202219a42551ae2d0f3e8123005a/active/0000-arity-parameter-overloading.md)

This suggested using optional parameters through writing different fn signatures as a more powerful alternative to just having default arguments.

It didn't receive a lot of discussion being low priority pre 1.0.

`July 2014` Postponed

#### RFC: Default and Named Arguments (*kowakiwi*)
`Sept 2014`
| [Pull Request](https://github.com/rust-lang/rfcs/pull/257)
| [Rendered](https://github.com/KokaKiwi/rfcs/blob/default_args/active/0000-default-arguments.md)
        
This RFC suggested making all arguments callable with named argument syntax, as well as proposing allowing default values for parameters.

It suggested making argument names and argument defaults as part of the function signature.

While it didn't receive much discussion at the time, it did spark a [wishlist issue in the RFCs repository](https://github.com/rust-lang/rfcs/issues/323) that started compiling all attempts at 
anything like this. That was immediately marked as postponed - and it was very contentious.       
        
`Sept 2014` Postponed

#### RFC: Struct Sugar (*bvssni*)
`Oct 2014` 
| [Internals](https://internals.rust-lang.org/t/struct-sugar/551/24)
| [Pull Request](https://github.com/rust-lang/rfcs/pull/343)
| [Rendered](https://github.com/bvssvni/rfcs/blob/master/active/0000-struct-sugar.md) 

This was a huge RFC. The crux of it was using a `Optional` trait to provide default values,
both for functions calls and struct initializers. It suggested adding sugar for creating structs that was
to be very similar to the syntax used for named arguments in function calls. The main purpose of the RFC seemed to be minimizing the
amount of textual changes in refactoring.

The author admitted that they were mostly hoping to spark discussion and get a ball rolling, not necessarily getting the RFC approved.

`Jan 2015` Postponed

        
#### RFC: Keyword Arguments (*iopq*)
`Feb 2015`
| [Pull Request](https://github.com/rust-lang/rfcs/pull/805)
| [Rendered](https://github.com/iopq/rfcs/blob/2e41311ffedacab38f80073deb29044f1ae20f7e/active/0000-keyword-arguments.md)

It suggested that named arguments be implemented as structural records (think tuples, but with named fields).
There were some suggestions that it should be named structs instead.

General concerns were that it's a heavy solution that encourages large functions that take many arguments, 
and that when necessary, builders were a viable alternative.

`Feb 2015` Postponed


#### Pre-RFC: make function arguments for Option-typed arguments (*djc*)
`July 2016` 
| [Internals Thread](https://internals.rust-lang.org/t/pre-rfc-make-function-arguments-for-option-typed-arguments/3741)

This suggested special casing the Option<T> type. This died pretty down pretty quick - it's likely most attention went to the next thread.

#### Pre-RFC Named Arguments (*Azerupi*):
`Aug 2016`
| [Internals Thread](https://internals.rust-lang.org/t/pre-rfc-named-arguments/3831)
| [Internals Thread - Summary](https://internals.rust-lang.org/t/pre-rfc-named-arguments/3831/196)

There was a lot in this thread, and it eventually died down. You should read the summary thread. 

It suggested a novel syntax of marking parameters in functions as named by using prefacing them with the `pub` keyword.


# Approaches

There seem to be 4 major approaches, with some variation on each of them being mentioned

1) Declare the named & default arguments in the function declaration & alter the type of the function
2) Declare the named & default arguments in the function declaration & let the compiler infer the ordering/default values.
3) Use anonymous structs for passing named parameters
4) Use named structs with sugar for building structs

#### Implementation Concerns:

These points come up in most of the discussions around named & default parameters

__Opt-in__: The option to omit values and to use the named syntax must be something the API opts into.
This rules out special casing the `Option` type as having a default value of `None` unless an additional marker or means
of marking as a default/omittable value is adopted

__Ordering of Arguments__: Allowing both named and positional calling style for the same arguments has a non-trivial chance
of leading to unintend behavior. It's possible that when removing labels from arguments that it switches what variable it
binds to. 

Example (adapted from the summar of Azurepi's internals thread):

> 
> [C]onsider this [declaration]:
> 
> ```fn position(lat: f32, lon: f32) { ... }```
>  
>  The user starts out with the named form.
>  
>  ```rust>
>  let lat = 50.85;
>  let lon = 4.35;
>  
>  position(lon: lon, lat: lat)
>  ```
>  The order is not the same as it would be in positional form, but this doesn’t cause any problems. However, if the user now decides to cut the named parameters because it feels redundant in this case, we obtain
>  ```rust
>  let lat = 50.85;
>  let lon = 4.35;
>  
>  position(lon, lat)
> ```
> 
>  Which is not the same! 

Note: this can only happen if keyword arguments can be passed in a different order than when they are called as positional arguments.

Ergo, we should either:
1) Require the same order whether using positional or named calling syntax **OR**
2) Not allow named arguments to be called with positional syntax

__Mixing of Arguments__: The general feel of the various threads suggests that being able to interleave positional args 
and named args in the same function call could be ambiguous and confusing. The general consensus appears to be putting all
the named args either at the beginning of a call, or at the end.

__Syntax__: The `var: Type` syntax would be nice for named arg syntax, but is already used for type ascription. There are several options for getting around
that:
1) Change the type ascription syntax, as type ascript is unstable (_I think_ this is getting less feasible with the stabilization work going on
2) Restrict where type ascription can be used (e.g. not in function calls; alternatively not in special {} syntax)
3) Require parens
4) Use alternative syntax, like `=>` or `:=`

__Type System__: If the changes affect the type system, closures and the Fn* traits could be a pain point. Do defaults 
become part of the type? Do the names become part of the type? The more we extend the type system in this regard, the
harder supporting equivalent support for closures and the Fn* types become.

__C FFI__: Although the named arguments are nice, it'd be really nice to be able to use them in rust and not break 
C FFI with wherever they are used.

__API Breakage & External Identifiers__: With named arguments, changing the name of a named function parameter in its
declaration can be a breaking change - do we want to adapt something that lets us have separate names for each parameter?

Example: (from Azurepi's internals thread summary):

> Like in Swift, we could allow for an internal name and an external name.
> 
> ```fn foo(ext int: i32) { ... }```
> 
> This addition would make the overloading of the pub keyword unnecessary. And allows some functions to read better while retaining a meaningful name in the function’s implementation.
> ```rust
> fn increment(_ value: i32, by increment: i32) -> i32 {
>     value + increment
> }
> 
> increment(value: 3, by: 5)
> ```
> But, like you can see in the example, it frequently occurs that you want the same external as internal name. In that case you would use an underscore _ to avoid the repetition of name name: i32

__Performance__: Will there be any performance hits from using named & default arguments? 
If it's easy to write non-static defaults, it might be. Will we need to automatically inline such functions?

__Boiler Plate__: How much boiler plate will we require the writer of the function to write? 
Will there be any for the user, like importing or naming the struct?
This is mostly a concern of struct based methods, like using named structs.

__Named Arg Shorthand__: Can we elide the labels on named arguments where the label is the same name of the variable 
be passed in (think field init shorthand for structs)?

# A potential plan

There has been two main road-blocks for getting something like this into Rust. 
The first is that the only actual RFCs happened before 1.0 and were postponed so 1.0 could ship (obviously not a problem anymore).
The second is that this is a huge feature - there are a fair number of contentious decisions to make, 
not to mention bikeshedding.

What if we addressed the large decisions by breaking it up into multiple RFCs? It would focus the decision making and 
reduce the likelyhood of stagnating discussion. It would add the constraint that we wouldn't really want to spread a real
feature across multiple RFCs, for the danger that only some RFCs make it in. They'd have to be coherent and stand on their own, at the very least, in a sequence.

With that in mind, I'd like to propose a sugary path getting named & default parameters into Rust by using Structs to hold the named arguments. (Note: This is stronly inspired by bvssni's RFC an other's comments - this is largly synthesis, not really anything new.)

Currently, that's not a very ergonomic solution for the following reasons:
1) It's verbose when calling the function: you still have to import the struct & name it during creation
2) It's obtuse - you could mimic default values with an FRU against the `StructName::default()` method but it's noisy and `default()` might be expensive
3) It's obtuse - matching on a struct in a function doesn't only partially shows up in the docs. 
It shows the struct name vs match names, but not the type. Too much information, and at the same time, not enough.
4) It's verbose when writing the function: you still need to define a struct, and then match the struct in the function
5) Even if structs did have defaults, they wouldn't be able to use values from function parameters

I listed them in that order for a reason - I think it's the order we should fix them in, 
that will allow us to stop safely and coherently at any point along the way. 
The details aren't fully fleshed out, but hopefully it's enough to see a practical path forward where decisions can be 
made in more constrained discussions.

#### 1) Struct Initialization Type Inference

We can solve pain point one by allow for struct initialization inference - we have a two general options here:

1) Add struct initializer type inference with special syntax as two RFCS
    1) Allow using an underscore (`_`) in place of the struct name when constructing a struct when it's type should be known.
    2) Allow one to use a sequence like `a => b, c` to desugar to the equivalent of `_ { a: b, c}` from step one. This has the major disadvantage that it doesn't work when using only matching names. How would you express `_ {a}` with this? You'd have be wordy and say `a => a`
2) Allow a curly braces block to be interpreted like it's a struct initializer, inferring the type. 
This might not be feasible for a couple reasons:
    * How does one parse `{ a }` - as `_ {a}` or as block with the return value of `a` ? If we can tell the type-checker it's either and let it figure it out, it won't be a problem. But I'm not sure that it's possible.
    * How does one parse `{ a: b }` - as `_ {a: b}` that type ascription or strut assignment? Again if we can tell the type-checker it's either, then it's fine.
    * If we can't let the type checker infer whether it's a block or a struct initializer, the we'll have to force a trailing comma OR make single value code blocks unallowed (e.g. always interpreting them as struct initialization).

Additionally, this'll probably play into how tuples are created as well.

Examples:

__1.i__: Using `_`
```rust
struct Demo {
    answer: u32
}
struct Tupled(u8, u16, u32);

fn the_answer() -> Demo {
    let t: Tupled = _(1,2,3);
    let answer = t.0+t.1+t.2;
    _ {answer}
}
```

__1.ii__: Using ` => `
```rust
struct Demo {
    answer: u32
}
struct Tupled(u8, u16, u32);

fn the_answer() -> Demo {
    let t: Tupled = _(1,2,3);
    let answer = t.0+t.1+t.2;
    answer => answer
}
```

__2__: Type inference for bare `{}` struct initializers 
```rust
struct Demo {
    answer: u32
}
struct Tupled(u8, u16, u32);

fn the_answer() -> Demo {
    let t: Tupled = Tupled(1,2,3);
    let answer = t.0+t.1+t.2;
    {answer}
}
```

I'll assume for syntax sake that we're going with option 2.

#### 2) Struct Defaults Sugar

Again, we probably have a couple of options here. 

One attempt I've made (with macros) that works pretty well involves converting any inputs for optional values into options, 
and thendoing an unwrap_or to get the default value. What would that look like? We'd need an attribute, something like
`#[optional(your_default_value_goes_here)]` along with the following in the Rust core:

```rust
pub trait ToOption<T> {
  fn to_option(self) -> T; 
}
impl <T> ToOption<T> for Option<T> {
  fn to_option(self) -> Option<T> {
    self
  }
}
impl <T> ToOption<T> for T {
  fn to_option(self) -> Option<T> {
    Some(self)
  }
}
```

Then whenever rust encounters struct initialization for a field marked with `#[optional(default_value_expr)]`, it 
calls `.to_option().unwrap_or(||default_value_expr)` on the value being assigned to that field when initialized.
If no value is passed in, the compiler inserts None, calling the `.to_option().unwrap_or(||default_value_expr)` on that.
This brings the benefit that it's easy to make wrapping functions that can pass along requests for default values or not.

Here's an example:

```rust
#[derive(Eq, PartialEq)]
struct Demo {
  #[optional(42)]
  answer: u32,
  question: String,
}

fn test() {
    let implicit_default: Demo = { 
        question: "What is the answer to life, the universe and everything".to_owned(), 
        .. // here the compiler inserts answer = None.to_option().unwrap_or(||42)
    }

    
    // This is useful if we are getting the override from else where and don't actually know
    // whether we should be using the default or not. In this case, we are using the default
    let explicit_default = { 
        question: "What is the answer to life, the universe and everything".to_owned(), 
        answer: None, // This gets desugared to 'answer = None.unwrap(||42) 
    }
    
    assert_eq!(implicit_default, explicit_default);
    
    let plain_override = Demo{
      question: "".to_owned(), 
      answer: 56 // This gets desugared to 'answer = 56.to_option().unwrap_or(|| 42)'
    };
    
    // This is useful if we are getting the override from else where and don't actually know
    // whether we should be using the default or not. In this case, we are not using the default
    let wrapped_override = {
      question: "".to_owned(),
      answer: Some(56), // This gets desugared to 'answer = Some(56).to_option().unwrap_or(|| 42)'
    };
    
    assert_eq!(plain_override, wrapped_override);
}
```

Again, this is one option. I expect a lot of debate, but the problem space for default field values in struct 
initialization seems a lot smaller than the problem space for named & default function parameters.

#### 3) Allow Inline Struct Parameter Documentation

Automatically inline the documentation for any value type that's only used in one function into that function.

Right now if you define a function like this:

```rust
struct StructName {
    field1: u32,
    field2: String,
}

fn (self, StructName { field1, field2 }: StructName) -> String {
  format!("{} - {}", field1, field2);
}
```

You end up with unhelpful documentation showing the matched names but not the types. 

We should instead remove the matched names from the struct documentation and instead show the types of the fields.

For example, with this and the previous two RFCs suggested, one could write:

```rust
#[repr(C)]
struct DoSomethingCoolArgs<T: IntoPath> {
    a: String,
    b: Optional<u32>,
    c: T,
}

fn do_something_cool(input: &str, args: DoSomethingCoolArgs<impl IntoPath>){
    let _ {
        a, b:__b, c
    } = args;
    
    let b = __b.unwrap_or(|| input.len());
    
    // Function body goes here
}
```

And one could call it the following ways (given the previous RFC suggestions passing):

```rust
// Use the default value for b
do_something_cool(input: "Hello world", {a: "Goodbye".to_owned(), b: None, "/"});
do_something_cool(input: "Hello world", {a: "Goodbye".to_owned(), "/"});

// Don't use the default value for b
do_something_cool(input: "Hello world", {a: "Goodbye".to_owned(), b: Some(11), "/"});
do_something_cool(input: "Hello world", {a: "Goodbye".to_owned(), b: 11, "/"});
```

And it would have great documents. But we could streamline it even further 


#### 4) Inline Struct Declartion w/Defaults in Functions

__Aside__: At this point, we'd have everything that function callers would need to conventiently call functions with struct args as named/default arguments. 
Function writers would still have to write extra structs, and do some default value setting in their function, but that's only for writers, not consumers.
And most of that could be offloaded to macros if the following RFC is contentious. I would in fact suggest that we attempt 
most of this RFC in macro form first as a way to experiment, since this RFC affects writing and not calling the functions.

__Proposal__:

1) Allow using *and declaring* a struct inline inside a function declaration with the named keyword.
2) Add an annotation for targeting the generated struct with derives & other statements.
3) Allow declaring of default value parameters with access to other parameters 

Example:

```rust
#[for_argument(named, repr(C))]
fn do_something_cool(input: &str, named : { a: String, b: u32 = input.len(), c: impl Into<Path>}){
    // Function body goes here
}
``` 

desugars to 

```rust
#[repr(C)]
struct DoSomethingCoolArgs<T: IntoPath> {
    a: String,
    b: Optional<u32>,
    c: T,
}

fn do_something_cool(input: &str, __args: DoSomethingCoolArgs<impl IntoPath>){
    let _ {
        a, b:__b, c
    } = __args;
    
    let b = __b.unwrap_or(|| input.len());
    
    // Function body goes here
}
```

This RFC has the most unknowns to decide - how to pass attributes to the struct and it's field, how to name the generated struct, 
and the various syntax bikeshed parts. And it's big.

Luckily, everything before this should still be usable, should this fail to get off the ground - this is still mostly for 
api _writers_ not consumers.

## Conclusion

While I present what I consider to be a very practical path to named and default arguments, basically sidestepping any
modifications to the type system, I'm still not certain that it's something worth doing.

I do think any conversations around it should be anchored in a comparison to the builder pattern - weaker, but easier to write.




