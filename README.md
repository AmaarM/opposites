#### Write-Up

Reimplementing an old homework assignment in Rust. The original assignment was written in OCaml.
This was written with the intention of learning Rust, while also staying true to the functional
programming paradigm used in the original solution (by using folds, filters, reduce, iterators). This is also
the reason why there are custom Set implementations, instead of using the std::collections::HashSet.

### Problem Description:

Given a list of 6 letter words, can you change one word to another by changing a consonant to a vowel.
