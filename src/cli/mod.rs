pub mod base;
pub mod user_mngmt;

fn clear_console() {
    println!("{}[2J", 27 as char);
}