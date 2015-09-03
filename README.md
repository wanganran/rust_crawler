# rust_crawler
a simple crawler written in rust to crawl English vocabulary and their definitions.

I initially intended to make a web-based English dictionary that can find the candidates according to a given edit distance, but I finally give up when I found that it so so difficult to implement a three-stage index for strings (English words) in Rust, while it needs maybe just <10 lines of codes in Scala.

Now that it became only a crawler that parses the HTML and stores the entries to files. 
This would be the first and the last little project I wrote in Rust. Rust is such a terrible language.

The creators of Rust is ambitious and proposed it to avoid any errors caused by data race or null reference or other undefined behaviours, but it is too ambitious that the current version of Rust can only limit its ability and flexibility in order to realize it. 
I can see the benefits of guaranteed safety that once the source codes are successfully compiled, it rarely goes wrong, but writing codes is such a terrible experience. For example, encountering unreasonably borrow or move or copy, you just don't know why a function returns a reference or non-reference, whether a structure can be copied, or only can be moved. You design a function that takes a non-reference parameter just want it to support pattern matching and varible binding, however in another function you call it with a parameter, then the parameter can never be used again.
Its another acclaimed advantages, utilizing stack rather than heap, is totally inferior than its predecessor C/C++. For example, the lambda expressions in C++11 can totally allocate their resources in stack using compile-time known templates and other tricks to form the closure, while in Rust, a lambda expression can only be "boxed" in order to be passed to or returned to another procedure.

The terrible points are commented in the source codes, started with "FUCK:".

1. You cannot use pattern matching to match a borrowed values to extract its parameters. For example, a &Tuple.

2. You won't know which to `use` before you Google the compiling result, for example, `use std::io::prelude::*;`. Same problem exists in Scala.

3. Builtin libraries are not adequate. Regex, Encodings, HTTP cannot be used directly. I even cannot find an implementation of HashSet or RB-Set in the builtin libraries!

4. Lack of flexibility in lambda expressions. You cannot point out which values are referenced and which are moved. All or none.

5. Thread_scoped is unsafe. This is such an important part of multithreading given the tough restrictions in Rust, but to use it you must import an extern crate and use it within an unsafe scope. Without it you even cannot write a parallel map function.

6. Strange language grammar. For example, I really cannot understand what the `::` in `collect::<Vec<_>>()` means until I Googled a lot for it. I'm also annoyed that "match" is placed before the names to be matched, given the custom that verbs should follow the nouns. I also cannot understand why there should be a `,` not a `;` after each pattern matching candidates. I also cannot understand to access a char in a string given an index I have to split the string to char array or byte array first, given that C#/Java/Scala/etc all don't have to be so troublesome. 

7. There are many `.unwrap()` in the codes. I know this is wrong but I also can't help myself writing so many error-handling codes for each `.unwrap()`. I have to admit that try-catch is more efficient, although Option/Result is more pretty.

8. I finally find it always safe to use `clone()` whenever possible. Oh this is a bad inclination forced by the strict restrictions.

All in all I won't use Rust again.
