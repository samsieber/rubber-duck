# Rubber Duck

Rubber duck is a library to facilitate writing functions that can be called with named value syntax (and with optional default values) - requires nightly.

See the rust doc of the rubber-duck crate for usage information.

See the REVIEW.md for general review about the previous named & default function arguments RFCs and discussions.

## Other Details

Not wanting to come up with a long descriptive name, or use a good name, I picked Rubber Duck. It's suppose to help
Rustaceans talk through their desire for named and default function arguments.

Right now it needs nightly (and 2018 edition) and a couple of features in order to write APIs. Consumers of said API will only need nightly (on 2018 edition).

The named & default argument calling syntax are exported as decl 2.0 macros in the same place as the functions you write.

If you're interested in this, you might also want to check out the [namedarg](https://github.com/comex/namedarg) crate by comex.

