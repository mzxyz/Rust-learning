# Appendix

## Keywords Currently in Use

- `as`: *perform primitive casting, [disambiguate the specific trait containing an item](https://doc.rust-lang.org/book/ch19-03-advanced-traits.html), or rename items in `use` and `extern crate` statements*

- `break`: *exit a loop immediately*

- `continue`: *continue to the next loop iteration*

- `const`: *define constant items or constant raw pointers*

- `static`: *global variable or lifetime lasting the entire program execution*

- `crate`: *link an external crate or a macro variable representing the crate in which the macro is defined*

- `extern`: *link an external crate, function or variable*

- `dyn`: *dynamic dispatch to a trait object*
  
- `if`: *branch based on the result of a conditional expression*
  
- `else`: *fallback for `if` and `if let` control flow constructs*

- `for`: *loop over items from an iterator, implement a trait, or specify a higher-ranked lifetime*

- `in`: *part of `for` loop syntax*

- `loop`: *loop unconditionally*

- `while`: *loop conditionally based on the result of an expression*

- `where`: *denote clauses that `constrain a type`*

- `struct`: *define a stucture*

- `enum`: *define an enumeration*

- `type`: *define a type alias or associated type*

- `unsafe`: *denote unsafe code, functions, traits, or implementations*

- `pub`: *denote public visiblity in struct fields, `impl` blocks(`fn`), or modules*

- `fn`: *define a function or the function pointer type*

- `impl`: *implement inherent or trait functionality*

- `false`: *Boolean false literal*

- `true`: *Boolean true literal*

- `let`: *bind a variable*

- `mut`: *denote mutability in references, raw pointers, or pattern bindings*

- `ref`: *bind by reference*

- `return`: *return from function*

- `Self`: *a type alias for the type implementation*

- `self`: *method subject or current module*

- `super`: *parent module of the current module*

- `match`: *match a value to patterns*

- `mod`: *define a module*

- `move`: *make a closure*

- `trait`: *define a trait*

- `use`: *bring symbols into scope*

## Keywords Reserved for Furture Use

- `abstract`, `async`, `become`, `box`, `do`, `final`, `macro`, `override`, `priv`, `try`, `typeof`, `unsized`, `virtual`, `yield`

## Raw Indentifiers

Use a raw identifier by prefixing a keyword with `r#`.

```rust
fn r#match() -> bool {
  true
}

fn main() {
  // To use match as a function name, you need to use the raw identifier syntax `r#`
  assert!(r#match());
}
```
