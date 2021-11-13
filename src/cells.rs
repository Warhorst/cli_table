use std::cmp::max;
use std::convert::TryInto;

use crate::row::Row;

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

pub struct RowCells<const C: usize> {
    pub max_dimension: Dimension,
    pub cells: [Cell; C],
}

impl<const C: usize> RowCells<C> {
    pub fn next_values(&mut self) -> Vec<String> {
        self.cells.iter_mut()
            .map(Cell::next_value)
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

#[derive(Debug)]
pub struct Cell {
    pub data: Vec<String>,
    pub dimension: Dimension,
}

impl Cell {
    /// Create a Cell from a String by splitting it line by line.
    /// The lines are stored in reverse order to enable faster retrieval of the next value
    /// TODO: better save an Iterator instead of a Vec (if I find out how...)
    pub fn from_string(string: &String) -> Self {
        let dimension = Dimension::from_string(string);
        let data: Vec<String> = string.lines().map(String::from).rev().collect();
        Cell { data, dimension }
    }

    /// Remove the last element from the cell and return it (which is the first line of the stored string).
    /// Return an empty String if the data is empty
    pub fn next_value(&mut self) -> String {
        match self.data.len() {
            0 => "".to_string(),
            len => self.data.remove(len - 1)
        }
    }
}

#[derive(Copy, Clone, Debug, Default)]
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