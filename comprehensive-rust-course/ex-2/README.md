# Types sizes

* `iN`, `uN`, `fN` - N bits
* `isize, usize` - size of a pointer (typically based on the architecture 64/32 bits)
* `char` - 32 bits
* `bool` - 8 bit

# Questions

## panic vs wrap


### problem
[exercise](https://google.github.io/comprehensive-rust/types-and-values/arithmetic.html) states following
> Change the i32’s to i16 to see an integer overflow, which panics (checked) in a debug build and wraps in a release build.

But following code fails on compilation even in a release build. Is it missleading exercise or I don't get something?

``` Rust
fn interproduct(a: i32, b: i32, c:i32) -> i16 {
  return a * b + b * c + c * a;
}
```


Following commented code also is not allowed and gives same results. Why?
``` Rust
fn interproduct_2(a: i32, b: i32, c:i32) -> i16 {
  // return (a * b).saturating_add(b * c).saturating_add(c * a);
  return (a * b).saturating_add(b * c).saturating_add(c * a).try_into().unwrap();
}
```

Compilation output

``` Rust
$ cargo run --release
   Compiling exercise-types v0.1.0 ()
error[E0308]: mismatched types
  --> src/main.rs:25:10
   |
24 | fn interproduct(a: i32, b: i32, c:i32) -> i16 {
   |                                           --- expected `i16` because of return type
25 |   return a * b + b * c + c * a;
   |          ^^^^^^^^^^^^^^^^^^^^^ expected `i16`, found `i32`
   |
help: you can convert an `i32` to an `i16` and panic if the converted value doesnt fit
   |
25 |   return (a * b + b * c + c * a).try_into().unwrap();
   |          +                     +++++++++++++++++++++

For more information about this error, try `rustc --explain E0308`.
error: could not compile `exercise-types` (bin "exercise-types") due to 1 previous error
```

### answer

```
?
```
