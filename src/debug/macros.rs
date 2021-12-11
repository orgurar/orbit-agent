// these macros are used for debug build only

#[macro_export]
#[cfg(debug_assertions)]
macro_rules! orbit_debug {
    () => (println!());
    ($($arg:tt)*) => ({
        println!($($arg)*);
    })
}

#[macro_export]
#[cfg(debug_assertions)]
macro_rules! orbit_edebug {
    () => (eprintln!());
    ($($arg:tt)*) => ({
        eprintln!($($arg)*);
    })
}

#[macro_export]
#[cfg(not(debug_assertions))]
macro_rules! orbit_debug {
    () => ();
    ($($arg:tt)*) => ({
    })
}

#[macro_export]
#[cfg(not(debug_assertions))]
macro_rules! orbit_edebug {
    () => ();
    ($($arg:tt)*) => ({
    })
}