
pub fn print_day_header(day: i32, title: &str) {
    println!("");
    println!("--- Day {:0>2}: {} ---", day, title);
    println!("{}", "=".repeat(16 + title.len()));
    println!("");
}

pub fn print_part_header(id: i32, title: &str) {
    println!("\nPart {}: {}", id, title);
    println!("{}", "-".repeat(8 + title.len()));
}
