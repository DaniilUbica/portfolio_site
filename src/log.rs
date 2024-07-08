pub enum LogLevel {
    FATAL,
    ERROR,
    WARN,
    INFO,
    DEBUG,
    TRACE,
    ALL,
}

#[macro_export]
macro_rules! current_function {
    () => {{
        fn f() {}
        fn type_name_of<T>(_: T) -> &'static str {
            std::any::type_name::<T>()
        }
        let name = type_name_of(f);
        name.strip_suffix("::f").unwrap()
    }};
}

#[macro_export]
macro_rules! log {
    ($level: expr, $msg: expr) => {
        log!($level, $msg, false);
    };

    ($level: expr, $msg: expr, $clear_prev_logs: expr) => {
        pub use std::io::Write;
        pub use std::path::Path;
        use crate::current_function;

        const LOG_FILE_PATH: &str = "./logs/app.log";

        let curr_file = Path::new(file!()).file_name().and_then(|s| s.to_str()).unwrap();
        let curr_func = current_function!();
        let date = chrono::offset::Local::now();
        let level: &str = match $level {
            FATAL => "FATAL",
            ERROR => "ERROR",
            WARN => "WARN",
            INFO => "INFO",
            DEBUG => "DEBUG",
            TRACE => "TRACE",
            ALL => "ALL",
        };
        let log_msg: String = format!("{} - {} - {} - in fuction {} - in file {}\0", level, date.to_string(), $msg, curr_func, curr_file);
        let mut file: std::fs::File = std::fs::OpenOptions::new().write($clear_prev_logs).truncate($clear_prev_logs).append(!$clear_prev_logs).open(LOG_FILE_PATH).expect("Can't open log file");
        writeln!(file, "{}", log_msg).unwrap();
    };
}