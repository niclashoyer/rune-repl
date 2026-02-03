
A simple "REPL" for the [Rune Programming Language](https://github.com/rune-rs/rune).

This is not a true read eval print loop, as rune is a compiled language. It
still allows to experiment with rune, as it recompiles the input after each read
and allows for direct feedback.

```bash
~  cargo run
   Compiling rune-repl v0.1.0 (/home/niclas/Hacks/rust/rune-repl)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.80s
     Running `target/debug/rune-repl`
> 1+1
2
> what
error: No local variable `what`
  ┌─ <memory>:4:1
  │
4 │ what
  │ ^^^^ No local variable `what`

> let x = 4;
()
> 4
4
>  
```

## License

Licensed under either of

 * Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
 * MIT license
   ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
