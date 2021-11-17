use cli_table::row::{Row, ToRow};
use cli_table::table::Table;

#[test]
fn print_with_header_works() {
    Table::new()
        .header(["The number", "My name"])
        .print_data(values().iter())
}

#[test]
fn print_without_header_works() {
    Table::new()
        .print_data(values().iter())
}

#[test]
fn print_to_string_works() {
    let mut byte_vec = vec![];

    Table::new()
        .print_data_to(values().iter(), &mut byte_vec);

    println!("{}", String::from_utf8(byte_vec).unwrap());
}

struct SomeValue {
    number: usize,
    name: String,
}

impl ToRow<2> for SomeValue {
    fn to_table_row(&self) -> Row<2> {
        Row::from([self.number.to_string(), self.name.clone()])
    }
}

fn values() -> Vec<SomeValue> {
    vec![
        SomeValue { number: 42, name: "Foo".to_string() },
        SomeValue { number: 69, name: "Bar".to_string() }
    ]
}