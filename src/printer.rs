use crate::row::{Dimension, Row, RowCells};

pub struct Printer<const C: usize>;

impl<const C: usize> Printer<C> {
    pub fn print<I: IntoIterator<Item=Row<C>>>(&self, data: I) {
        let row_cells: Vec<RowCells> = data.into_iter()
            .map(RowCells::from)
            .collect();

        let max_dimension = row_cells.iter()
            .map(|rc| rc.max_dimension)
            .fold(Dimension::default(), Dimension::max_merge);

        self.print_cells(row_cells, max_dimension)
    }

    fn print_cells(&self, row_cells: Vec<RowCells>, max_cell_dimension: Dimension) {
        let printed_row_dimension = Self::calculate_printed_row_dimension(max_cell_dimension);

        Self::print_full_line(printed_row_dimension.width);
        for rc in row_cells {
            Self::print_row(rc, printed_row_dimension, max_cell_dimension.width)
        }
    }

    fn calculate_printed_row_dimension(max_cell_dimension: Dimension) -> Dimension {
        let width = (max_cell_dimension.width + 2) * C + C + 1;
        let height = max_cell_dimension.height + 4;
        Dimension { width, height }
    }

    fn print_row(mut row_cells: RowCells, printed_row_dimension: Dimension, cell_width: usize) {
        for i in 0..printed_row_dimension.height {
            match i {
                i if i == 0 => (),
                i if i == printed_row_dimension.height - 1 => Self::print_full_line(printed_row_dimension.width),
                i if i == 1 || i == printed_row_dimension.height - 2 => Self::print_blank_line(printed_row_dimension.width, cell_width),
                _ => Self::print_cell_values(row_cells.next_values(), cell_width)
            }
        }
    }

    fn print_full_line(width: usize) {
        println!("{}", (0..width).into_iter().map(|_| "*").collect::<String>())
    }

    fn print_blank_line(width: usize, cell_width: usize) {
        let line = (0..width).into_iter()
            .map(|i| match i {
                0 => "*",
                i if i % (cell_width + 3) == 0 => "*",
                _ => " "
            }).collect::<String>();
        println!("{}", line)
    }

    fn print_cell_values(cell_values: Vec<String>, cell_width: usize) {
        let mut line = String::new();

        for (i, val) in cell_values.into_iter().enumerate() {
            let whitespace = cell_width - val.len();

            if i == 0 { line.push_str("*") };
            line.push_str(" ");
            line.push_str(val.as_str());
            for _ in 0..whitespace { line.push_str(" ") }
            line.push_str(" ");
            line.push_str("*");
        }

        println!("{}", line)
    }
}