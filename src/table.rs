use std::io::Write;

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
        self.print_data_to(data, std::io::stdout().lock())
    }

    pub fn print_data_to<'a, R, I, W>(&self, data: I, target: W)
        where R: 'a + ToRow<C>,
              I: IntoIterator<Item=&'a R>,
              W: Write {
        let rows = self.header.as_ref()
            .map(ToRow::to_table_row)
            .into_iter()
            .chain(data.into_iter().map(ToRow::to_table_row));

        Printer::new(TableCells::from_rows(rows)).print_to(target)
    }
}