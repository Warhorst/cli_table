use std::cmp::max;
use std::convert::TryInto;

use crate::row::Row;

// TODO: only save the max dimension here. Its used temporary in cells and rows
#[derive(Debug, Eq, PartialEq)]
pub struct TableCells<const C: usize> {
    pub row_cells: Vec<RowCells<C>>,
    pub cell_dimension: Dimension,
}

impl<const C: usize> TableCells<C> {
    pub fn from_rows<R>(rows: R) -> Self
        where R: IntoIterator<Item=Row<C>> {
        let row_cells = rows.into_iter()
            .map(RowCells::from)
            .collect::<Vec<_>>();

        let cell_dimension = row_cells.iter()
            .map(|rc| rc.max_dimension)
            .fold(Dimension::default(), Dimension::max_merge);
        TableCells { row_cells, cell_dimension }
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct RowCells<const C: usize> {
    pub max_dimension: Dimension,
    pub cells: [Cell; C],
}

impl<const C: usize> RowCells<C> {
    pub fn get_line_values(&self, line: usize) -> Vec<String> {
        self.cells.iter()
            .map(|c| c.get_line(line))
            .collect()
    }
}

impl<const C: usize> From<Row<C>> for RowCells<C> {
    fn from(row: Row<C>) -> Self {
        let cell_vec = row.values
            .iter()
            .map(Cell::from_string)
            .collect::<Vec<_>>();

        let max_dimension = cell_vec.iter()
            .map(|c| c.dimension)
            .fold(Dimension::default(), Dimension::max_merge);
        let cells = cell_vec.try_into().unwrap();

        RowCells { cells, max_dimension }
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct Cell {
    pub data: Vec<String>,
    pub dimension: Dimension,
}

impl Cell {
    /// Create a Cell from a String by splitting it line by line.
    pub fn from_string(string: &String) -> Self {
        let dimension = Dimension::from_string(string);
        let data: Vec<String> = string.lines().map(String::from).collect();
        Cell { data, dimension }
    }

    pub fn get_line(&self, line: usize) -> String {
        self.data.get(line).cloned().unwrap_or("".to_string())
    }
}

#[derive(Copy, Clone, Debug, Default, Eq, PartialEq)]
pub struct Dimension {
    pub width: usize,
    pub height: usize,
}

impl Dimension {
    pub fn from_string(string: &String) -> Self {
        let width = string.split("\n").map(str::len).max().unwrap_or_default();
        let height = string.split("\n").count();
        Dimension { width, height }
    }

    pub fn max_merge(dim_one: Dimension, dim_two: Dimension) -> Self {
        let width = max(dim_one.width, dim_two.width);
        let height = max(dim_one.height, dim_two.height);
        Dimension { width, height }
    }
}

#[cfg(test)]
mod table_cells_test {
    use crate::cells::TableCells;
    use crate::cells::test_helper::{cell, dimension, row_cells, table_cells};
    use crate::row::Row;

    #[test]
    fn from_rows_works() {
        let rows = vec![
            Row::from(["42".to_string(), "Foo".to_string()]),
            Row::from(["69".to_string(), "Bar".to_string()]),
        ];

        let expected_table_cells = table_cells(
            vec![
                row_cells([
                              cell(vec!["42"], dimension(2, 1)),
                              cell(vec!["Foo"], dimension(3, 1))
                          ], dimension(3, 1)),
                row_cells([
                    cell(vec!["69"], dimension(2, 1)),
                    cell(vec!["Bar"], dimension(3, 1))
                ], dimension(3, 1)),
            ],
            dimension(3, 1)
        );

        assert_eq!(TableCells::from_rows(rows), expected_table_cells)
    }
}

#[cfg(test)]
mod row_cells_tests {
    use crate::cells::RowCells;
    use crate::cells::test_helper::{cell, dimension, row_cells};
    use crate::row::Row;

    #[test]
    fn from_row_works() {
        let row = Row::from(["foo".to_string(), "bar\nbaz".to_string()]);
        let row_cells = row_cells(
            [
                cell(vec!["foo"], dimension(3, 1)),
                cell(vec!["bar", "baz"], dimension(3, 2))
            ],
            dimension(3, 2),
        );

        assert_eq!(
            RowCells::from(row),
            row_cells
        )
    }

    #[test]
    fn get_line_values_works() {
        let row_cells = row_cells(
            [
                cell(vec!["foo"], dimension(3, 1)),
                cell(vec!["bar", "baz"], dimension(3, 2))
            ],
            dimension(3, 2),
        );

        assert_eq!(row_cells.get_line_values(0), vec!["foo".to_string(), "bar".to_string()]);
        assert_eq!(row_cells.get_line_values(1), vec!["".to_string(), "baz".to_string()]);
        assert_eq!(row_cells.get_line_values(2), vec!["".to_string(), "".to_string()]);
        assert_eq!(row_cells.get_line_values(3), vec!["".to_string(), "".to_string()]);
    }
}

#[cfg(test)]
mod cell_tests {
    use crate::cells::{Cell, Dimension};
    use crate::cells::test_helper::cell;

    #[test]
    fn from_string_works() {
        let string = "foo\nbarbaz".to_string();

        assert_eq!(
            Cell::from_string(&string),
            cell(vec!["foo", "barbaz"], Dimension { width: 6, height: 2 })
        )
    }

    #[test]
    fn get_line_works() {
        let cell = cell(vec!["foo", "bar"], Dimension { width: 3, height: 2 });

        assert_eq!(cell.get_line(0), "foo".to_string());
        assert_eq!(cell.get_line(1), "bar".to_string());
        assert_eq!(cell.get_line(2), "".to_string());
        assert_eq!(cell.get_line(3), "".to_string());
    }
}

#[cfg(test)]
mod dimension_tests {
    use crate::cells::Dimension;
    use crate::cells::test_helper::dimension;

    #[test]
    fn from_string_works() {
        assert_eq!(Dimension::from_string(&String::new()), dimension(0, 1));
        assert_eq!(Dimension::from_string(&"foo".to_string()), dimension(3, 1));
        assert_eq!(Dimension::from_string(&"foo\nbar".to_string()), dimension(3, 2));
        assert_eq!(Dimension::from_string(&"foo\nbarbaz".to_string()), dimension(6, 2))
    }

    #[test]
    fn max_merge_works() {
        assert_eq!(
            Dimension::max_merge(dimension(5, 5), dimension(5, 5)),
            dimension(5, 5)
        );

        assert_eq!(
            Dimension::max_merge(dimension(5, 1), dimension(1, 5)),
            dimension(5, 5)
        )
    }
}

#[cfg(test)]
mod test_helper {
    use crate::cells::{Cell, Dimension, RowCells, TableCells};

    pub fn table_cells<const C: usize>(row_cells: Vec<RowCells<C>>, cell_dimension: Dimension) -> TableCells<C> {
        TableCells { row_cells, cell_dimension }
    }

    pub fn row_cells<const C: usize>(cells: [Cell; C], max_dimension: Dimension) -> RowCells<C> {
        RowCells { cells, max_dimension }
    }

    pub fn cell(data: Vec<&str>, dimension: Dimension) -> Cell {
        Cell { data: data.into_iter().map(ToOwned::to_owned).collect(), dimension }
    }

    pub fn dimension(width: usize, height: usize) -> Dimension {
        Dimension { width, height }
    }
}