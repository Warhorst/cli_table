use crate::cells::{Dimension, RowCells, TableCells};

pub struct Printer<const C: usize>;

impl<const C: usize> Printer<C> {
    pub fn print(&self, table_cells: TableCells<C>) {
        let cell_dimension = table_cells.cell_dimension;
        let printed_row_dimension = Self::calculate_printed_row_dimension(cell_dimension);

        Self::print_full_line(printed_row_dimension.width, cell_dimension.width);
        table_cells.row_cells
            .into_iter()
            .for_each(|rc| Self::print_row(rc, printed_row_dimension, cell_dimension.width))
    }

    fn calculate_printed_row_dimension(cell_dimension: Dimension) -> Dimension {
        let width = (cell_dimension.width + 2) * C + C + 1;
        let height = cell_dimension.height + 4;
        Dimension { width, height }
    }

    fn print_row(mut row_cells: RowCells<C>, printed_row_dimension: Dimension, cell_width: usize) {
        for i in 0..printed_row_dimension.height {
            match i {
                i if i == 0 => (),
                i if i == printed_row_dimension.height - 1 => Self::print_full_line(printed_row_dimension.width, cell_width),
                i if i == 1 || i == printed_row_dimension.height - 2 => Self::print_blank_line(printed_row_dimension.width, cell_width),
                _ => Self::print_cell_values(row_cells.next_values(), cell_width)
            }
        }
    }

    fn print_full_line(row_width: usize, cell_width: usize) {
        println!("{}", (0..row_width).into_iter()
            .map(|i| match i {
                0 => "+",
                i if i % (cell_width + 3) == 0 => "+",
                _ => "-"
            })
            .collect::<String>())
    }

    fn print_blank_line(width: usize, cell_width: usize) {
        let line = (0..width).into_iter()
            .map(|i| match i {
                0 => "|",
                i if i % (cell_width + 3) == 0 => "|",
                _ => " "
            }).collect::<String>();
        println!("{}", line)
    }

    fn print_cell_values(cell_values: Vec<String>, cell_width: usize) {
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

        println!("{}", line)
    }
}