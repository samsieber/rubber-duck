# Rubber Duck

Rubber duck is a library to facilitate writing functions that can be called with named value syntax (and with optional default values) - requires nightly.

See the rust doc of the rubber-duck crate for usage information. 
You'll need to checkout this repo and build the docs though, so here's a sample:

> With this give api declaration:
> 
> ```
> mod module {
>    #[gen_struct_sugar(defaults(name = r#""Bob".to_owned()"#))]
>    pub fn is_a_test(name: String, message: String) -> String {
>        let i = 0;
>        let i = i + 1;
>        format!("{}) Hello {}, {} The end.", i, &name, &message)
>    }
> }
> ```
>
> One can call the function in a variety of ways
>
> ```
> {
>     use crate::module::is_a_test
>      // Named form requires a macro
>     is_a_test!(message=> "Hi.".to_owned(), name=>"George".to_owned());  // 1) Hello George. Hi. The end.
>      // and lets you use defaults                                   
>     is_a_test!(message=> "Hi.".to_owned());                             // 1) Hello Bob. Hi. The end.  
>      // Positional form doesn't (need a macro or let you do defaults)
>     is_a_test("bob".to_owned(), "Hi.".to_owned());                      // 1) Hello Bob. Hi. The end. 
> }
> // You don't even have to import it!
> crate::module::is_a_test!(message=> "there".to_owned(), name=>"hi".to_owned());
> ```

See the REVIEW.md for general review about the previous named & default function arguments RFCs and discussions.

## Other Details

Not wanting to come up with a long descriptive name, or use a good name, I picked Rubber Duck. It's suppose to help
Rustaceans talk through their desire for named and default function arguments.

Right now it needs nightly (and 2018 edition) and a couple of features in order to write APIs. Consumers of said API will only need nightly (on 2018 edition).

The named & default argument calling syntax are exported as decl 2.0 macros in the same place as the functions you write.

If you're interested in this, you might also want to check out the [namedarg](https://github.com/comex/namedarg) crate by comex.

