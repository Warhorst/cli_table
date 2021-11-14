use cli_table::row::{Row, ToRow};
use cli_table::table::Table;

struct SomeValue<'a> {
    number: usize,
    name: &'a str,
}

impl<'a> ToRow<2> for SomeValue<'a> {
    fn to_table_row(&self) -> Row<2> {
        Row::from([self.number.to_string(), self.name.to_string()])
    }
}

#[test]
fn print_with_header_works() {
    let values = vec![
        SomeValue { number: 42, name: "Foo" },
        SomeValue { number: 69, name: "Bar" }
    ];

    Table::new()
        .header(["The number", "My name"])
        .print_data(values.iter())
}

#[test]
fn print_without_header_works() {
    let values = vec![
        SomeValue { number: 42, name: "Foo" },
        SomeValue { number: 69, name: "Bar" }
    ];

    Table::new()
        .print_data(values.iter())
}