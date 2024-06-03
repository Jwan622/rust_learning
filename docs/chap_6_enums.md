# Notes


In this chapter, we’ll look at enumerations, also referred to as enums. Enums allow you to define a type by enumerating its possible variants. First we’ll define and use an enum to show how an enum can encode meaning along with data. Next, we’ll explore a particularly useful enum, called Option, which expresses that a value can be either something or nothing. Then we’ll look at how pattern matching in the match expression makes it easy to run different code for different values of an enum. Finally, we’ll cover how the if let construct is another convenient and concise idiom available to handle enums in your code.

## Defining an Enum

Where structs give you a way of grouping together related fields and data, like a Rectangle with its width and height, enums give you a way of saying a value is one of a possible set of values. For example, we may want to say that Rectangle is one of a set of possible shapes that also includes Circle and Triangle. To do this, Rust allows us to encode these possibilities as an enum.

Let’s look at a situation we might want to express in code and see why enums are useful and more appropriate than structs in this case. Say we need to work with IP addresses. Currently, two major standards are used for IP addresses: version four and version six. Because these are the only possibilities for an IP address that our program will come across, we can enumerate all possible variants, which is where enumeration gets its name.

Any IP address can be either a version four or a version six address, but not both at the same time. That property of IP addresses makes the enum data structure appropriate because an enum value can only be one of its variants. Both version four and version six addresses are still fundamentally IP addresses, so they should be treated as the same type when the code is handling situations that apply to any kind of IP address.

We can express this concept in code by defining an IpAddrKind enumeration and listing the possible kinds an IP address can be, V4 and V6. These are the variants of the enum:

```rust
enum IpAddrKind {
    V4,
    V6,
}
```

`IpAddrKind` is now a custom data type that we can use elsewhere in our code.

## Enums values


We can create instances of each of the two variants of IpAddrKind like this:
```rust
let four = IpAddrKind::V4;
let six = IpAddrKind::V6;
```

Note that the variants of the enum are **`namespaced` under its identifier, and we use a double colon to separate the two**. This is useful because now both values IpAddrKind::V4 and IpAddrKind::V6 are of the same type: IpAddrKind. We can then, for instance, define a function that takes any IpAddrKind:

```rust
fn route(ip_kind: IpAddrKind) {...}
```

And we can call this function with either variant:
```rust
route(IpAddrKind::V4);
route(IpAddrKind::V6);
```

Using enums has even more advantages. Thinking more about our IP address type, at the moment we don’t have a way to store the actual IP address data; we only know what kind it is. Given that you just learned about structs in Chapter 5, you might be tempted to tackle this problem with structs as shown in Listing 6-1.
```rust
enum IpAddrKind {
    V4,
    V6,
}

struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

let home = IpAddr {
    kind: IpAddrKind::V4,
    address: String::from("127.0.0.1"),
};

let loopback = IpAddr {
    kind: IpAddrKind::V6,
    address: String::from("::1"),
};
// Listing 6-1: Storing the data and IpAddrKind variant of an IP address using a struct
```

Here, we’ve defined a struct IpAddr that has two fields: a kind field that is of type IpAddrKind (the enum we defined previously) and an address field of type String. We have two instances of this struct. The first is home, and it has the value IpAddrKind::V4 as its kind with associated address data of 127.0.0.1. The second instance is loopback. It has the other variant of IpAddrKind as its kind value, V6, and has address ::1 associated with it. We’ve used a struct to bundle the kind and address values together, so now the variant is associated with the value.

However, representing the same concept using just an enum is more concise: rather than an enum inside a struct, **we can put data directly into each enum variant**. This new definition of the IpAddr enum says that both V4 and V6 variants will have associated `String` values:

```rust
    enum IpAddr {
        V4(String),
        V6(String),
    }

    let home = IpAddr::V4(String::from("127.0.0.1"));

    let loopback = IpAddr::V6(String::from("::1"));
```

We **attach data to each variant of the enum directly**, so there is no need for an extra struct. Here, it’s also easier to see another detail of how enums work: the name of each enum variant that we define also becomes a function that **constructs an instance of the enum**. That is, **IpAddr::V4() is a function call that takes a String argument and returns an instance of the IpAddr type. We automatically get this constructor function defined as a result of defining the enum.**

Let’s look at how the standard library defines IpAddr: it has the exact enum and variants that we’ve defined and used, but it embeds the address data inside the variants in the form of two different structs, which are defined differently for each variant:

```rust
struct Ipv4Addr {
    // --snip--
}

struct Ipv6Addr {
    // --snip--
}

enum IpAddr {
    V4(Ipv4Addr),
    V6(Ipv6Addr),
}
```

This code illustrates that you can put any kind of data inside an enum variant: strings, numeric types, or structs, for example. You can even include another enum! Also, standard library types are often not much more complicated than what you might come up with.


### Why use Enum over struct?

this one has a wide variety of types embedded in its variants.

```rust
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}
// Listing 6-2: A Message enum whose variants each store different amounts and types of values
```

This enum has four variants with different types. Here, `Quit` is a specific type of enum Message.


- Quit has no data associated with it at all.
- Move has named fields, like a struct does.
- Write includes a single String.
- ChangeColor includes three i32 values.

- The Quit variant in the Message enum is not a struct; it’s an enum variant that does not contain any data. Let’s dive a bit deeper into this to clarify.
Quit:
Type: Message
Data: None (it’s a unit variant, similar to a unit type () in Rust)
Usage: Message::Quit


Defining an enum with variants such as the ones in Listing 6-2 is similar to defining different kinds of struct definitions, except the enum doesn’t use the struct keyword and all the variants are grouped together under the Message type. The following structs could hold the same data that the preceding enum variants hold:

```rust
struct QuitMessage; // unit struct
struct MoveMessage {
    x: i32,
    y: i32,
}

struct WriteMessage(String); // tuple struct
struct ChangeColorMessage(i32, i32, i32); // tuple struct
```

But if we used the different structs, each of which has its own type, we couldn’t as easily define a function to take any of these kinds of messages as we could with the Message enum defined in Listing 6-2, which is a single type.


There is one more similarity between enums and structs: just as we’re able to define methods on structs using impl, we’re also able to define methods on enums. Here’s a method named call that we could define on our Message enum:
```rust
impl Message {
    fn call(&self) {
        // method body would be defined here
    }
}

let m = Message::Write(String::from("hello"));
m.call();
```

The body of the method would use self to get the value that we called the method on. In this example, we’ve created a variable `m` that has the value `Message::Write(String::from("hello"))`, and that is what `self` will be in the body of the call method when `m.call()` runs.

### Example

a function using  Enums:
```rust
fn process_message(msg: Message) {
    match msg {
        Message::Quit => {
            println!("Quit message");
        }
        Message::Move { x, y } => {
            println!("Move message to coordinates: ({}, {})", x, y);
        }
        Message::Write(text) => {
            println!("Write message: {}", text);
        }
        Message::ChangeColor(r, g, b) => {
            println!("Change color to RGB: ({}, {}, {})", r, g, b);
        }
    }
}
```

If you used different structs instead, the function would have to handle each struct type separately, which is less convenient and more error-prone:

```rust
fn process_quit(msg: QuitMessage) {
    println!("Quit message");
}

fn process_move(msg: MoveMessage) {
    println!("Move message to coordinates: ({}, {})", msg.x, msg.y);
}

fn process_write(msg: WriteMessage) {
    println!("Write message: {}", msg.0);
}

fn process_change_color(msg: ChangeColorMessage) {
    println!("Change color to RGB: ({}, {}, {})", msg.0, msg.1, msg.2);
}
```
