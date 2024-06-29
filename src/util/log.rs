macro_rules! message {
    ($c:ident, $m:expr, $p:expr) => {{
        let padding = String::from_utf8(vec![b' '; 7 - $m.len()]).unwrap();
        #[cfg(not(debug_assertions))]
        let file_line = "".to_string();
        #[cfg(debug_assertions)]
        let file_line = format!("{}:{}", file!(), line!());
        if *$crate::config::env::no_color() {
            eprintln!(" {}{} {} {}", padding, $m, $p, file_line);
        } else {
            use ansi_term::Colour::{$c, Fixed};
            eprint!(" {}{} {}", padding, $c.bold().paint($m), $p);
            eprintln!(" {}", Fixed(240).paint(file_line));
        }
    }};
}

#[macro_export]
macro_rules! error {
    ( $x:expr, $( $y:expr ),* $(,)? ) => {
        message!(Red, "error", format!($x, $($y),*));
    };
    ( $x:expr ) => {
        message!(Red, "error", format!($x));
    };
}

#[macro_export]
macro_rules! warn {
    ( $x:expr, $( $y:expr ),* $(,)? ) => {
        message!(Yellow, "warning", format!($x, $($y),*));
    };
    ( $x:expr ) => {
        message!(Yellow, "warning", format!($x));
    };
}

#[macro_export]
macro_rules! info {
    ( $x:expr, $( $y:expr ),* $(,)? ) => {
        message!(Cyan, "info", format!($x, $($y),*));
    };
    ( $x:expr ) => {
        message!(Cyan, "info", format!($x));
    };
}
