# Generic Types, Traits, Lifetimes


## Example

Let's see what's going on here:

```rust
fn print_value<T: Printable>(value: &T) {
    value.print();
}
// Function Definition
```


- `fn print_value<T: Printable>(value: &T) { ... }:` This line defines a function named print_value. The function takes a single parameter value of type &T, which is a reference to a value of any type that implements the Printable trait.
- `<T: Printable>`: This is a generic type parameter declaration. It specifies that `T` can be any type that implements the `Printable` trait. In other words, **T must satisfy the constraint of implementing the `Printable` trait.** T is also in the parameter.


`value.print();:` This line calls the print method on the value parameter. Since value is of type &T, and T implements the Printable trait, this line is valid if T implements a method named print as defined by the Printable trait.

Now, let's say we have a type MyType that implements the Printable trait:

```rust
trait Printable {
    fn print(&self);
}


struct MyType {
    data: i32,
}

// Implement the Printable trait for MyType
impl Printable for MyType {
    fn print(&self) {
        println!("Printing MyType: {}", self.data);
    }
}
```

We can then use the `print_value` function with MyType objects:

```rust
fn main() {
    let my_value = MyType { data: 123 }; // this is an instance of a Struct
    print_value(&my_value); // Calls print method for MyType
}
```
In this usage:

- MyType implements the Printable trait by providing an implementation for the print method.
- We create an instance my_value of MyType.
- We call the print_value function with a reference to my_value, which is of type &MyType.
- Inside print_value, the print method of MyType is called on my_value, printing "Printing MyType: 123" to the console.

In summary, the print_value function is a generic function that accepts any type T that implements the Printable trait and calls its print method. 