use std::convert::TryInto;
use std::marker::PhantomData;

use crate::new_table::Width::Dynamic;
use crate::table_writer::TableWriter;

pub struct Table<Row, RowMapper: Fn(Row) -> [String; Columns], const Columns: usize> {
    header: Option<[&'static str; Columns]>,
    column_widths: Option<[Width; Columns]>,
    row_mapper: RowMapper,
    _row: PhantomData<Row>,
}

impl<Row, RowMapper: Fn(Row) -> [String; Columns], const Columns: usize> Table<Row, RowMapper, Columns> {
    pub fn new(row_mapper: RowMapper) -> Self {
        Table {
            header: None,
            column_widths: None,
            row_mapper,
            _row: PhantomData,
        }
    }

    pub fn header(mut self, header_values: [&'static str; Columns]) -> Self {
        self.header = Some(header_values);
        self
    }

    pub fn column_widths(mut self, column_widths: [Width; Columns]) -> Self {
        self.column_widths = Some(column_widths);
        self
    }

    pub fn print<I>(self, values: I) where I: IntoIterator<Item=Row> {
        let mut rows: Vec<[String; Columns]> = self.header.into_iter()
            .map(|h| h.iter()
                .map(|s| s.to_string())
                .collect::<Vec<_>>()
                .try_into()
                .unwrap())
            .chain(values.into_iter().map(self.row_mapper))
            .collect();

        let writer = TableWriter::new(self.column_widths.unwrap_or([Width::Dynamic; Columns]));
        writer.write(rows, std::io::stdout().lock())
    }
}

#[derive(Copy, Clone)]
pub enum Width {
    Dynamic,
    Max(usize),
}

#[cfg(test)]
mod tests {
    use crate::new_table::{Table, Width};
    use crate::new_table::Width::Max;

    #[test]
    fn table_creation_works() {
        let strings = ["föööö0000 oooooooo00 oooooooo00", "baaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaar", "baz"];

        Table::new(
            |s: &str| [
                s.to_string(),
                s.to_string()
            ]
        )
            .header(["h1", "h2"])
            .column_widths([Width::Max(8), Width::Dynamic])
            .print(strings);
    }
}