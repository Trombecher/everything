use crate::content::Row;

pub fn exists(rows: &[Row], row: Row) -> bool {
    rows.iter().any(|r| r == &row)
}