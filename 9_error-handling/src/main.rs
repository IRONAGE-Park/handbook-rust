use crate::example::panic;
use crate::example::result;

fn main() {
    // panic::run();
    // panic::backtrace();

    // result::file_open();
    // result::file_open_match();
    // result::file_open_match_by_unwrap();
    // result::file_open_unwrap();
    // result::file_open_expect();
    result::read_username_from_file();
}

mod example;
