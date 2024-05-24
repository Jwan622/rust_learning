# Notes


## Macros vs functions

```rust
fn main() {
    println!("Hello, world!");
}
```

println! calls a Rust macro. If it had called a function instead, it would be entered as println (without the !). We’ll discuss Rust macros in more detail in Chapter 19. For now, you just need to know that using a ! means that you’re calling a macro instead of a normal function and that macros don’t always follow the same rules as functions.

end the line with a semicolon (;), which indicates that this expression is over and the next one is ready to begin. Most lines of Rust code end with a semicolon.


## Compiling and running

Compiling and Running Are Separate Steps

Before running a Rust program, you must compile it using the Rust compiler by entering the rustc command and passing it the name of your source file, like this:

```rust
$ rustc main.rs
```

If you’re more familiar with a dynamic language, such as Ruby, Python, or JavaScript, you might not be used to compiling and running a program as separate steps. Rust is an ahead-of-time compiled language, meaning you can compile a program and give the executable to someone else, and they can run it even without having Rust installed. If you give someone a .rb, .py, or .js file, they need to have a Ruby, Python, or JavaScript implementation installed (respectively). But in those languages, you only need one command to compile and run your program. Everything is a trade-off in language design.


Just compiling with rustc is fine for simple programs, but as your project grows, you’ll want to manage all the options and make it easy to share your code. Next, we’ll introduce you to the Cargo tool, which will help you write real-world Rust programs.