pub trait ToRow<const C: usize> {
    fn to_table_row(&self) -> Row<C>;
}

pub struct Row<const C: usize> {
    pub values: [String; C],
}

impl<const C: usize> From<[String; C]> for Row<C> {
    fn from(values: [String; C]) -> Self {
        Row { values }
    }
}