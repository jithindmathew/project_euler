/// Used to time one or more functions.
///
/// ### Examples
///
/// ```
/// use project_euler::time_solutions;
///
/// fn with_parameters(n1: u128, n2: u64) -> u128 {
///     return n1 + n2 as u128;
/// }
///
/// fn without_parameters() -> u128 {
///     return 248724874;
/// }
///
/// time_solutions!(
///     with_parameters(45, 45454),
///     without_parameters()
/// );
///
/// ```
///
/// Output:
///
/// ```rust,ignore
/// -----------------------------------------------------------
/// with_parameters took 300ns
/// -----------------------------------------------------------
/// without_parameters took 100ns
///
/// ```
#[allow(unused_macros)]
#[macro_export]
macro_rules! time_solutions {
    () => {};

    ($func:ident ( $($param:expr),* $(,)? ) $(, $($rest:tt)*)* ) => {
        println!("-----------------------------------------------------------");
        {
            let start_time = std::time::Instant::now();
            $func($($param),*);
            let elapsed_time = start_time.elapsed();
            println!("{} took {:?}", stringify!($func), elapsed_time);
        }
        time_solutions!($($($rest)*)*);
    };
}
