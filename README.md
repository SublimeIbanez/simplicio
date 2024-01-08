# Simplicio

Get rid of the annoying boilerplate with Rust.

## Features
- String::from() becomes `s!()`
- String concatination with `cnct!()`
- HashMap instantialization with initial values with `map!()`

## Getting Started
To start using Simplicio, add the following to your `Cargo.toml`:
```toml
[dependencies]
simplicio = "0.1.1"
```
- Minimum supported Rust version: `1.56.1`

## Usage
String creation and concatination
```rust
use simplicio::*;

fn main() {
    let string = s!("This is a String");
    assert_eq!(String::from("This is a String"), string);

    let (a, b, c, d) = (s!("This "), s!("is "), s!("a "), s!("String"));
    assert_eq!(String::from("This is a String"), cnct!(a, b, c, d));
}
```

```rust
use simplicio::*;

fn main() {
    let hashmap = map!("a" => "b", "c" => "d", "e" => "f");
}
```

## License
This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Contributing
._. why would you do this?
