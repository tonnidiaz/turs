#[macro_export]
macro_rules! log {

    () => {
        println!();
    };
    ($($arg:expr),*) => {{
        use turs::chrono::Local;
        #[allow(unused_macros)]
        {
            let now = Local::now();
            let msg = format!($($arg),*);
            println!("[{}] {}", now.format("%Y-%m-%d %H:%M:%S"), msg);
        }
    }};
}
#[macro_export]
macro_rules! elog {

    () => {
        eprintln!();
    };
    ($($arg:expr),*) => {{
        use turs::chrono::Local;
        #[allow(unused_macros)]
        {
            let now = Local::now();
            let msg = format!($($arg),*);
            eprintln!("[{}] {}", now.format("%Y-%m-%d %H:%M:%S"), msg);
        }
    }};
}
