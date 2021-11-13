use std::convert::TryInto;

use crate::row::{Row, ToRow};

pub struct Header<const C: usize> {
    header_values: [String; C],
}

impl<const N: usize> Header<N> {
    pub fn new(values: [&str; N]) -> Self {
        let values_vec: Vec<String> = values.iter().map(|s| String::from(*s)).collect();

        Header {
            header_values: values_vec.try_into().unwrap()
        }
    }
}

impl<const C: usize> ToRow<C> for Header<C> {
    fn to_table_row(&self) -> Row<C> {
        Row::from(self.header_values.clone())
    }
}