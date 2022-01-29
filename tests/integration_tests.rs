use cli_table::table::Table;

#[test]
fn print_with_header_works() {
    Table::new(|some_value: SomeValue| [
        some_value.number.to_string(),
        some_value.name
    ]).header(["The number", "My name"]).print(values())
}

#[test]
fn print_without_header_works() {
    Table::new(|some_value: SomeValue| [
        some_value.number.to_string(),
        some_value.name
    ]).print(values())
}

#[test]
fn print_many_works() {
    Table::new(|i: usize| [
        i.to_string()
    ])
        .header(["val"])
        .print(0..=100)
}

struct SomeValue {
    number: usize,
    name: String,
}

fn values() -> Vec<SomeValue> {
    vec![
        SomeValue { number: 42, name: "Foo".to_string() },
        SomeValue { number: 69, name: "Bar".to_string() }
    ]
}