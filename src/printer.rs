use std::io::Write;

use crate::cells::{Dimension, RowCells, TableCells};

pub struct Printer<const C: usize> {
    table_cells: TableCells<C>,
    printed_row_dimension: Dimension
}

impl<const C: usize> Printer<C> {
    pub fn new(table_cells: TableCells<C>) -> Self {
        let printed_row_dimension = Self::calculate_printed_row_dimension(table_cells.cell_dimension);

        Printer { printed_row_dimension, table_cells }
    }

    pub fn print_to<W: Write>(&self, mut target: W) {
        for (i, rc) in self.table_cells.row_cells.iter().enumerate() {
            self.print_row(i, rc, &mut target)
        }
    }

    fn calculate_printed_row_dimension(cell_dimension: Dimension) -> Dimension {
        let width = (cell_dimension.width + 2) * C + C + 1;
        let height = cell_dimension.height + 4;
        Dimension { width, height }
    }

    fn print_row<W: Write>(&self, row_index: usize, row_cells: &RowCells<C>, target: &mut W) {
        let mut current_line = 0;

        for i in 0..self.printed_row_dimension.height {
            let line_to_write = match i {
                i if i == 0 && row_index == 0 => self.create_full_line(),
                i if i == 0 => continue,
                i if i == self.printed_row_dimension.height - 1 => self.create_full_line(),
                i if i == 1 || i == self.printed_row_dimension.height - 2 => self.create_blank_line(),
                _ => {
                    let line = self.create_cell_values_line(row_cells.get_line_values(current_line));
                    current_line += 1;
                    line
                }
            };

            Self::write_line(line_to_write, target)
        }
    }

    fn create_full_line(&self) -> String {
        let row_width = self.printed_row_dimension.width;
        let cell_width = self.table_cells.cell_dimension.width;

        let line = (0..row_width).into_iter()
            .map(|i| match i {
                0 => "+",
                i if i % (cell_width + 3) == 0 => "+",
                _ => "-"
            })
            .collect::<String>();

        line
    }

    fn create_blank_line(&self) -> String {
        let row_width = self.printed_row_dimension.width;
        let cell_width = self.table_cells.cell_dimension.width;

        let line = (0..row_width).into_iter()
            .map(|i| match i {
                0 => "|",
                i if i % (cell_width + 3) == 0 => "|",
                _ => " "
            }).collect::<String>();

        line
    }

    fn create_cell_values_line(&self, cell_values: Vec<String>) -> String {
        let cell_width = self.table_cells.cell_dimension.width;
        let mut line = String::new();

        for (i, val) in cell_values.into_iter().enumerate() {
            let whitespace = cell_width - val.len();

            if i == 0 { line.push_str("|") };
            line.push_str(" ");
            line.push_str(val.as_str());
            for _ in 0..whitespace { line.push_str(" ") }
            line.push_str(" ");
            line.push_str("|");
        }

        line
    }

    fn write_line<W: Write>(mut line: String, target: &mut W) {
        line.push_str("\n");
        target.write_all(line.as_bytes()).unwrap()
    }
}