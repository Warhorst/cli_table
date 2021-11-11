use std::cmp::max;

pub trait ToRow<const C: usize> {
    fn to_table_row(&self) -> Row<C>;
}

pub struct Row<const C: usize> {
    values: Vec<String>,
}

impl<const C: usize> Row<C> {
    pub fn from_array(values: [String; C]) -> Self {
        Row { values: values.into_iter().cloned().collect() }
    }

    pub fn from_vec(values: Vec<String>) -> Self {
        if values.len() != C { panic!("Wrong number of values for row provided! Expected {}, got {}", C, values.len()) }

        Row {values}
    }

    pub fn to_cells(self) -> RowCells {
        RowCells::from_row(self.values)
    }
}

pub struct RowCells {
    pub max_dimension: Dimension,
    pub cells: Vec<Cell>,
}

impl RowCells {
    fn from_row(row: Vec<String>) -> Self {
        let cells: Vec<Cell> = row.into_iter().map(Cell::from_string).collect();
        let max_dimension = cells.iter()
            .map(|cell| cell.dimension)
            .fold(Dimension::default(), Dimension::max_merge);

        RowCells { cells, max_dimension }
    }

    pub fn next_values(&mut self) -> Vec<String> {
        self.cells.iter_mut()
            .map(Cell::next_value)
            .collect()
    }
}

pub struct Cell {
    pub dimension: Dimension,
    pub data: Vec<String>,
}

impl Cell {
    pub fn from_string(string: String) -> Self {
        let data: Vec<String> = string.lines().map(String::from).collect();
        let width = data.iter().map(String::len).max().unwrap_or_default();
        let height = data.len();

        Cell { dimension: Dimension { width, height }, data }
    }

    pub fn next_value(&mut self) -> String {
        match self.data.len() {
            0 => "".to_string(),
            _ => self.data.remove(0)
        }
    }
}

#[derive(Copy, Clone, Default)]
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

    pub fn set_max(&mut self, other: Dimension) {
        self.width = max(self.width, other.width);
        self.height = max(self.height, other.height)
    }

    pub fn max_merge(dim_one: Dimension, dim_two: Dimension) -> Self {
        let width = max(dim_one.width, dim_two.width);
        let height = max(dim_one.height, dim_two.height);
        Dimension { width, height }
    }
}