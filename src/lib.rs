/// Macro to flatten nested calls of `new`s.
///
/// # Examples
///
/// ```rust
/// use new_flattened::new_flattened;
/// assert_eq!(
///     Box::new(Box::new(Box::new(42))),
///     new_flattened!(42, Box, Box, Box),
/// )
/// ```
#[macro_export]
macro_rules! new_flattened {
    ($value:expr, $container:ident) => {
        $container::new($value)
    };
    ($value:expr, $container:ident, $($containers:ident),+) => {
        $container::new(new_flattened!($value, $($containers),+))
    };
}

#[cfg(test)]
mod tests {
    #[test]
    fn simple() {
        assert_eq!(
            Box::new(Box::new(Box::new(5))),
            new_flattened!(5, Box, Box, Box),
        );
    }
}
