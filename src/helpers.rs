/// Converts an iterator of key-value pairs into a HashMap.
/// 
/// This function is a utility for converting any iterator that yields
/// key-value pairs into a HashMap. It's used internally by the map! macro
/// to create HashMaps from various input types like arrays, vectors, etc.
///
/// # Arguments
///
/// * `iter` - An iterator that yields key-value pairs.
///
/// # Returns
///
/// A HashMap containing all the key-value pairs from the iterator.
///
/// # Examples
///
/// ```
/// use simplicio::helpers::mapper;
///
/// let pairs = vec![("key1", "value1"), ("key2", "value2")];
/// let hashmap = mapper(pairs);
/// assert_eq!(hashmap.get("key1"), Some(&"value1"));
/// assert_eq!(hashmap.get("key2"), Some(&"value2"));
/// ```
pub fn mapper<K, V, I>(iter: I) -> std::collections::HashMap<K, V> 
where
    K: std::hash::Hash + Eq + Clone,
    V: Clone,
    I: std::iter::IntoIterator<Item = (K, V)>,
{
    return iter.into_iter().collect();
}
