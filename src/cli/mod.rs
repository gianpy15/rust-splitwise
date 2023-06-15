pub mod base;
pub mod group_mngmt;
pub mod user_mngmt;

fn clear_console() {
    println!("{}[2J", 27 as char);
}
