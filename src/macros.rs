/// Converts a string literal into a `String` object.
///
/// This macro simplifies the conversion of a passed value or string literal into a `String` object.
/// - Will convert anything that has the `.to_string()` function.
/// - This can also perform concatination.
/// - Will generate a `String::new()` if no value is passed.
/// - Automatically insert spaces with a `.` prefix to the arguments: `s!(./*Rest of arguments*/)`
///
/// # Examples
///
/// ```no_run
/// use simplicio::s;
///
/// // Creating an enum for example purposes
/// enum Enum { Value }  //Create the enum, Enum
/// impl std::fmt::Display for Enum {  // Implement the Display trait
///    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
///        match self { Enum::Value => write!(f, "value"), }
///    }
/// }
///
/// let new_string = s!();
/// assert_eq!(new_string, String::new());
///
/// let value = s!(Enum::Value);
/// assert_eq!(value, String::from("value"));
///
/// let this = String::from("This");
/// let concat_string = s!(.this, "is number", 1, Enum::Value, "macro:", true); // The prefix '.' Tells macro to insert spaces
/// assert_eq!(concat_string, String::from("This is number 1 value macro: true"));
/// ```
#[macro_export]
macro_rules! s {
    //Default input
    ($($e:expr),* $(,)?) => {
        {
            let mut string: String = String::new();
            $(
                let add: &str = &$e.to_string();
                string.push_str(add);
            )*
            string
        }
    };

    //Automatic spacing concatination
    (.$($e:expr),* $(,)?) => {
        {
            let mut string: String = String::new();
            let mut first = true;
            $(
                if !first { string.push(' '); } else { first = false; }
                let add: &str = &$e.to_string();
                string.push_str(add);
            )*
            string
        }
    };
}

/// Concatenates multiple string slices and/or `String` objects.
///
/// A wrapper for the `s!()` macro that allows people to assign purpose to one for ease of readability.
///
/// # Examples
///
/// ```no_run
/// use simplicio::cnct;
///
/// // Creating an enum for example purposes
/// enum Enum { Value }  //Create the enum, Enum
/// impl std::fmt::Display for Enum {  // Implement the Display trait
///    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
///        match self { Enum::Value => write!(f, "value"), }
///    }
/// }
///
/// let concatenated_string = cnct!("Yel", "low", "!");
/// assert_eq!(concatenated_string, String::from("Yellow!"));
///
/// let text = String::from("this");
/// let concat_string = cnct!(.text, true, 123, Enum::Value); //Assuming Enum implements the ToString trait and is set to "Value"
/// assert_eq!(concat_string, String::from("this true 123 value"));
/// ```
#[macro_export]
macro_rules! cnct {
    //Default case
    ($($e:expr),* $(,)?) => {
        $crate::s!($($e),*);
    };

    //Automatic spacing
    (.$($e:expr),* $(,)?) => {
        $crate::s!(.$($e),*);
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
/// // Create the HashMap test_map = {"k1": "v1", "k2": "v2"}
/// let mut test_map = std::collections::HashMap::new(); 
/// test_map.insert("k1", "v1");
/// test_map.insert("k2", "v2");
///
/// let mut hashmaps: Vec<std::collections::HashMap<&str, &str>> = vec![]; // Holds all variants of map!()
/// let vecmap = vec![("k1", "v1"), ("k2", "v2")]; // Vector of key/value tuples
/// let arrmap = [("k1", "v1"), ("k2", "v2")]; // Array of key/value tuples
/// hashmaps.push( map!(vecmap) );  // Can insert a vector
/// hashmaps.push( map!(arrmap) );  // Can insert an array
/// hashmaps.push( map!("k1" "v1", "k2" "v2") );     // Direct insert: uses `' '` 
/// hashmaps.push( map!("k1": "v1", "k2": "v2") );   // Direct insert: uses `:` 
/// hashmaps.push( map!("k1"->"v1", "k2"->"v2") );   // Direct insert: uses `->` 
/// hashmaps.push( map!("k1"=>"v1", "k2"=>"v2") );   // Direct insert: uses `=>` 
/// hashmaps.push( map!("k1"["v1"], "k2"["v2"]) ); // Direct insert: uses `[]` to annotate a value
///
/// assert!(hashmaps.iter().all(|map| map == &test_map)); // Assert that all cases are true
/// ```
#[macro_export]
macro_rules! map {
    // For new
    () => { std::collections::HashMap::new() };

    // key[value]
    ($($key:path[$val:expr]),+ $(,)?) => { map!(@mapper $($key, $val),+) };
    ($($key:tt[$val:expr]),+ $(,)?) => { map!(@mapper $($key, $val),+) };

    // key value || key: value || key -> value || key => value
    ($($key:tt$(:)?$(->)?$(=>)?$val:expr),+ $(,)?) => { map!(@mapper $($key, $val),+) };

    // When keys are paths and not tokens
    ($($key:path : $val:expr),+ $(,)?) => { map!(@mapper $($key, $val),+) };
    ($($key:path => $val:expr),+ $(,)?) => { map!(@mapper $($key, $val),+) };

    // Vec<(_,_)>, &[(_,_)], or manually input map!([(k,v)])
    ($($arr:expr),+ $(,)?) => {
        {
            let mut map = map!();
            $(
                map.extend($crate::helpers::mapper($arr));
            )+
            map 
        }
    };

    //Does the bulk of the mapping
    (@mapper $($key:expr, $value:expr),+ $(,)?) => {
        {
            let mut map = std::collections::HashMap::new();
            $(
                map.insert($key, $value);
            )+
            map
        }
    }
}

/// Prints to the console with a newline.
/// Convenient shorthand for `println!`.
///
/// # Examples
///
/// Basic usage:
///
/// ```
/// # use logfather::n;
/// n!();
/// ```
#[macro_export]
macro_rules! ln {
    () => { println!() };
}
