use crate::printer::Printer;
use crate::row::{Row, ToRow};

pub struct Table<const C: usize> {
    header: Option<Header<C>>,
}

impl<const C: usize> Table<C> {
    pub fn new() -> Self {
        Table {
            header: None,
        }
    }

    pub fn add_header(mut self, header_values: [&str; C]) -> Self {
        self.header = Some(Header::new(header_values));
        self
    }

    pub fn print_data<'a, R, I>(&self, data: I)
        where R: 'a + ToRow<C>,
              I: IntoIterator<Item=&'a R> {
        Printer.print(self.header.as_ref().map(ToRow::to_table_row).into_iter().chain(data.into_iter().map(ToRow::to_table_row)))
    }
}



pub struct Header<const N: usize> {
    header_values: Vec<String>,
}

impl<const N: usize> Header<N> {
    pub fn new(values: [&str; N]) -> Self {
        Header {
            header_values: values.iter().map(|s| String::from(*s)).collect()
        }
    }
}

impl<const C: usize> ToRow<C> for Header<C> {
    fn to_table_row(&self) -> Row<C> {
        Row::from_vec(self.header_values.clone())
    }
}

#[cfg(test)]
mod tests {
    use crate::row::{Row, ToRow};
    use crate::table::Table;

    #[test]
    fn print_works() {
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
                Row::from_array([self.val.to_string(), self.name.clone(), self.complex.to_string()])
            }
        }

        Table::new()
            .add_header(["The value", "My name", "A complex number"])
            .print_data(&vec![Bar::new(2, "foo", 3.2), Bar::new(42, "be", 4.5321)])
    }
}