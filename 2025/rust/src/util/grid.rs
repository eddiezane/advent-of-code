use std::fmt;

pub type Grid<T> = Vec<Vec<T>>;

pub struct DisplayGrid<'a, T>(pub &'a Grid<T>);

impl<'a, T> fmt::Display for DisplayGrid<'a, T>
where
    T: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in self.0 {
            for (i, item) in row.iter().enumerate() {
                if i > 0 {
                    write!(f, " ")?;
                }
                write!(f, "{}", item)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

pub fn transpose<T>(grid: &Grid<T>) -> Grid<T>
where
    T: Copy,
{
    let width = grid[0].len();

    (0..width)
        .map(|col_idx| grid.iter().map(|row| row[col_idx]).collect())
        .collect()
}
