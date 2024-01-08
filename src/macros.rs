/// Converts a string literal into a `String` object.
///
/// This macro simplifies the conversion of a string literal (`&str`)
/// into a `String` object. It's a shorthand for `String::from()`.
///
/// # Examples
///
/// ```no_run
/// use simplicio::s;
///
/// let my_string = s!("Hello, World!");
/// assert_eq!(my_string, String::from("Hello, World!"));
/// ```
#[macro_export]
macro_rules! s {
    ($str:expr) => { String::from($str) };
}

/// Concatenates multiple string slices and/or `String` objects.
///
/// This macro takes a series of string slices (`&str`) and `String` objects
/// and concatenates them into a single `String`. It's useful for building
/// strings from multiple parts.
///
/// # Examples
///
/// ```no_run
/// use simplicio::cnct;
///
/// let concatenated_string = cnct!("Hello", ", ", "World", "!");
/// assert_eq!(concatenated_string, String::from("Hello, World!"));
/// ```
#[macro_export]
macro_rules! cnct {
    ($e1:expr $(, $e2:expr)*) => {
        {
            let mut string: String = String::from($e1);
            $(
                let second: &str = &$e2;
                string.push_str(second);
            )*
            string
        }
    };
}

/// Creates a `HashMap` from a list of key-value pairs.
///
/// This macro simplifies the creation of a `HashMap` from a series
/// of key-value pairs. It's a convenient way to initialize a `HashMap`
/// without manually calling `insert` for each pair.
///
/// # Examples
///
/// ```no_run
/// use simplicio::map;
///
/// let my_map = map! {
///     "key1" => "value1",
///     "key2" => "value2"
/// };
/// assert_eq!(my_map.get("key1"), Some(&"value1"));
/// assert_eq!(my_map.get("key2"), Some(&"value2"));
/// ```
#[macro_export]
macro_rules! map {
    ($($key:expr => $value:expr),+) => {
        {
            let mut hashmap = std::collections::HashMap::new();
            $(
                hashmap.insert($key, $value);
             )+
            hashmap
        }
    };
}
