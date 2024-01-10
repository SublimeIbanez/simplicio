# Simplicio

Get rid of the annoying boilerplate in Rust and simplify creation of `String`s and `HashMap`s.

## Features
- `s!()` can be used to:
  - create a new `String`
  - convert any value that implements the `Display` trait into a string
  - concatinate values together that implement the `Display` trait
  - concatinate while inserting a space between each value with the `.` prefix (e.g. `s!(.a, b, c)`)
- `cnct!()` is a wrapper around `s!()` for preferential purposes
- `map()` creates a HashMap with initial values or from a Vec/array of tuple pairs
  - `map!(k1 v1, k1 v2)` | `map!(k1: v1, k2:v2)` | `map!(k1 => v1, k2 => v2)` | `map!(k1 -> v1, k2 -> v2)`

## Getting Started
To start using Simplicio, add the following to your `Cargo.toml`:
```toml
[dependencies]
simplicio = "0.1.2"
```
- Minimum supported Rust version: `1.56.1`

## Usage
String creation and concatination
```rust
use simplicio::{s, cnct};

// Creating an enum for example purposes
enum Enum { Value }  //Create the enum, Enum
impl std::fmt::Display for Enum {  // Implement the Display trait
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
         match self { Enum::Value => write!(f, "value"), }
    }
}

fn main() {
    assert_eq!(String::new(), s!()); // Make a new string
    assert_eq!(String::from("This is a String"), s!("This is a String")); // Stop using .to_string() or String::from()

    assert_eq!(true.to_string(), s!(true)); // Converts bools
    assert_eq!(123840.to_string(), s!(123840)); // Convert numbers fast boiiii
    assert_eq!(Enum::Value.to_string(), s!(Enum::Value)); // As long as it implements the ToString or Display traits, it will work

    let (a, b, c, d) = ("This", "is", "a", "String");
    assert_eq!(String::from("This is a String"), cnct!(.a, b, c, d)); // '.' prefix to automate spacing
    assert_eq!(s!(a, b, c, d), cnct!(a, b, c ,d)); // `cnct!()` is just a wrapper around `s!()`
}
```
Creating HashMaps
```rust
use simplicio::*;

fn main() {
    let mut tester = std::collections::HashMap::new();
    tester.insert("k1", "v1");
    tester.insert("k2", "v2");

    assert_eq!(map!("k1""v1", "k2" "v2"), tester);          // ' ' delimiter
    assert_eq!(map!("k1":"v1", "k2" : "v2"), tester);       // ':' delimiter
    assert_eq!(map!("k1"=>"v1", "k2" => "v2"), tester);     // '=>' delimiter
    assert_eq!(map!("k1"->"v1", "k2" -> "v2"), tester);     // '->' delimiter
    assert_eq!(map!("k1"["v1"], "k2" ["v2"]), tester);     // Key[Value]
    assert_eq!(map!([("k1", "v1"), ("k2", "v2")]), tester); // Similar to Hashmap::from(/*...*/)

    let vecmap = Vec::from([("k1", "v1"), ("k2", "v2")]);
    assert_eq!(map!(vecmap), tester); // Can convert a Vec<(_,_)>

    let arrmap = [("k1", "v1"), ("k2", "v2")];
    assert_eq!(map!(arrmap), tester); // Can convert an array [(_,_)]
}
```

## License
This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Contributing
._. why would you do this?
