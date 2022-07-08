//! The example can be run by this command
//! `cargo run --example table_width`

use tabled::{object::Segment, width::Percent, Alignment, Modify, Style, Table, Width};

fn main() {
    let data = [
        ["Hello World!!!", "3.3.22.2"],
        ["Guten Morgen", "1.1.1.1"],
        ["Добры вечар", "127.0.0.1"],
        ["Bonjour le monde", ""],
        ["Ciao mondo", ""],
    ];

    let table = Table::builder(data)
        .build()
        .with(Style::github_markdown())
        .with(Modify::new(Segment::all()).with(Alignment::left()));

    println!("Original table\n{}", table);

    let table = table.with(Width::truncate(20).suffix("..."));

    println!("Truncated table\n{}", table);

    let table = table.with(Modify::new(Segment::all()).with(Width::wrap(5)));

    println!("Wrapped table\n{}", table);

    let table = table.with(Width::increase(Percent(200)));

    println!("Widen table\n{}", table);
}