use crate::row::ToRow;
use crate::printer::Printer;

pub struct Table<'a, const C: usize> {
    header: Option<Header<'a, C>>,
}

impl<'a, const C: usize> Table<'a, C> {
    pub fn new() -> Self {
        Table {
            header: None,
        }
    }

    pub fn add_header(mut self, header_values: [&'a str; C]) -> Self {
        self.header = Some(Header::new(header_values));
        self
    }

    pub fn print_data<R, I>(&self, data: I)
        where R: 'a + ToRow<C>,
              I: IntoIterator<Item=&'a R> {
        // TODO don't forget the header
        Printer.print(data)
    }
}



pub struct Header<'a, const N: usize> {
    header_values: [&'a str; N],
}

impl<'a, const N: usize> Header<'a, N> {
    pub fn new(values: [&'a str; N]) -> Self {
        Header {
            header_values: values
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::table::Table;
    use crate::row::{ToRow, Row};

    #[test]
    fn foo() {
        // amazing!
        let table = Table::new().add_header(["foo", "bar", "baz"]);
    }

    #[test]
    fn bar() {
        struct Bar {
            val: usize,
            name: String,
            complex: f32
        }

        impl Bar {
            fn new(val: usize, name: &'static str, complex: f32) -> Self {
                Bar {val, name: name.to_string(), complex}
            }
        }

        impl ToRow<3> for Bar {
            fn to_table_row(&self) -> Row<3> {
                Row::new([self.val.to_string(), self.name.clone(), self.complex.to_string()])
            }
        }

        Table::new().print_data(&vec![Bar::new(2, "foo", 3.2), Bar::new(42, "be", 4.5321)])
    }
}