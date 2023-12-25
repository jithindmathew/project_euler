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
