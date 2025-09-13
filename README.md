
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
