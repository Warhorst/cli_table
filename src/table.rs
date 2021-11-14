use crate::cells::TableCells;
use crate::header::Header;
use crate::printer::Printer;
use crate::row::ToRow;

pub struct Table<const C: usize> {
    header: Option<Header<C>>,
}

impl<const C: usize> Table<C> {
    pub fn new() -> Self {
        Table {
            header: None,
        }
    }

    pub fn header(mut self, header_values: [&str; C]) -> Self {
        self.header = Some(Header::new(header_values));
        self
    }

    pub fn print_data<'a, R, I>(&self, data: I)
        where R: 'a + ToRow<C>,
              I: IntoIterator<Item=&'a R> {
        let rows = self.header.as_ref()
            .map(ToRow::to_table_row)
            .into_iter()
            .chain(data.into_iter().map(ToRow::to_table_row));

        Printer::new(TableCells::from_rows(rows)).print()
    }
}

#[cfg(test)]
mod tests {
    use crate::row::{Row, ToRow};
    use crate::table::Table;

    #[test]
    fn print_works() {
        struct Foo {
            val: usize,
            name: String,
            complex: f32
        }

        impl Foo {
            fn new(val: usize, name: &'static str, complex: f32) -> Self {
                Foo {val, name: name.to_string(), complex}
            }
        }

        impl ToRow<3> for Foo {
            fn to_table_row(&self) -> Row<3> {
                Row::from([self.val.to_string(), self.name.clone(), self.complex.to_string()])
            }
        }

        Table::new()
            .header(["The value\nof interest", "My name", "A complex number"])
            .print_data(&vec![Foo::new(2, "foo", 3.2), Foo::new(42, "bar", 4.5321)])
    }
}