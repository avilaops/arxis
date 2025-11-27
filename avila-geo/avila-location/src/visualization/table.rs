//! Table utilities

use comfy_table::{Table, Cell, Attribute};

pub fn create_simple_table(headers: Vec<&str>, rows: Vec<Vec<String>>) -> Table {
    let mut table = Table::new();

    let header_cells: Vec<Cell> = headers
        .iter()
        .map(|h| Cell::new(h).add_attribute(Attribute::Bold))
        .collect();

    table.set_header(header_cells);

    for row in rows {
        table.add_row(row);
    }

    table
}
