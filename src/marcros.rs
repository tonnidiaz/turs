#[macro_export]
macro_rules! log {

    () => {
        println!();
    };
    ($($arg:expr),*) => {{
        use chrono::Local;
        #[allow(unused_macros)]
        {let now = Local::now();
        print!("\n[{}] ", now.format("%Y-%m-%d %H:%M:%S"));
        println!($($arg),*);}
    }};
}