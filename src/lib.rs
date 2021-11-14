pub mod table;
pub mod row;
mod printer;
mod header;
mod cells;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
