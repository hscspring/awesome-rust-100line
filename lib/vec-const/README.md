## Disclaimer

Using this crate is undefined behavior always. Use at your own risk. It makes assumptions about various ABIs that are not guaranteed to match what this crate assumes them to be. Incorrect use can cause both heap corruption and access violations. Even correct use (if there even is such a thing) may put you at risk for these things. 

# vec-const

This is a Rust crate that performs a bit of black-magic to allow vectors to be declared as `const`. Contains the macro: `vec_const!`. This macro is meant to mimic the `vec!` macro as closely as possible, but it does require a type to be specified (TBD: Is there a way around that?). See the below example:

```rust
pub struct AThing(u8, &'static str);

const A_VEC_CONST: ManuallyDrop<Vec<AThing>> = vec_const!(AThing, AThing(5, "wow"), AThing(2, "cool"));

fn main()
{
    assert_eq!(*TEST, vec!(AThing(5, "wow"), AThing(2, "cool")));
}
```

### Danger Zone

This also comes with a `vec_const_unsafe!` macro that doesn't wrap the vec in a `ManuallyDrop`. That means you have to be the one to make sure nothing is ever dropped when it shouldn't be. This is hilariously unsafe and it doesn't even have to decency to require `unsafe` blocks anywhere. You're better off just leaving this one be.

### Why?

~~I created this to solve a niche problem: I had a struct with a mostly trivial fields but also a Vec field. I had to share that struct among quite a few contexts using `Rc<RefCell<Thing>>`s, and was using `RefCell::take` to eventually acquire a modified version of that original `Thing`. Thing is, `take` replaces the `RefCell` content with whatever `default` returns; and allocating a ton of redundant `Vec`s was slowing things down. This was my solution to make that problem disappear. That said, while there are some cases this crate could genuinely be used for good, it's most certainly an evil, rule-breaking crate. It laughs in the face of Rust's safety-guarantees, and as such should never be used by anyone that hasn't achieved ultimate enlightenment in the ways of all things memory-safety.~~

As I've been told, this can be safely solved using a `Cow<'static, [T]>`. `as_mut` is pretty convenient. This crate is not seriously necessary (or wise). That said, I still think it's an interesting experiement and still plan to make some [improvements](https://play.rust-lang.org/?version=nightly&mode=debug&edition=2018&gist=a063e5ff9e4e4762d00b0c32e4e27a5f) (thanks /u/SkiFire13) once 1.56 hits.