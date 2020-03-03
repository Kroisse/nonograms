use ndarray::{Array2, Axis};
use rand::Rng;

#[derive(Debug)]
pub struct GameState {
    pub field: Array2<Cell>,
    pub rows: Vec<Vec<u32>>,
    pub columns: Vec<Vec<u32>>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Cell {
    Empty,
    Space,
    Filled,
}

impl Default for Cell {
    fn default() -> Self {
        Cell::Empty
    }
}

impl GameState {
    pub fn new(shape: (usize, usize)) -> Self {
        let mut rng = rand::thread_rng();
        let field = Array2::from_shape_fn(shape, |_| rng.gen_bool(0.5));
        let rows = field
            .axis_iter(Axis(0))
            .map(|v| count(v.iter().copied()))
            .collect();
        let columns = field
            .axis_iter(Axis(1))
            .map(|v| count(v.iter().copied()))
            .collect();
        GameState {
            field: Array2::from_elem(shape, Cell::Empty),
            rows,
            columns,
        }
    }
}

fn count(iter: impl IntoIterator<Item = bool>) -> Vec<u32> {
    let mut res = vec![];
    let mut c = 0;
    for i in iter {
        match (c, i) {
            (_, true) => {
                c += 1;
            }
            (0, false) => {}
            (_, false) => {
                res.push(c);
                c = 0;
            }
        }
    }
    if c > 0 || res.is_empty() {
        res.push(c);
    }
    res
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn count_empty() {
        assert_eq!(count(None), &[0]);
    }

    #[test]
    fn count_space() {
        assert_eq!(count(vec![false, false, false]), &[0]);
    }

    #[test]
    fn count_single() {
        assert_eq!(count(Some(true)), &[1]);
    }

    #[test]
    fn count_consecutive() {
        assert_eq!(count(vec![true, true, true, true, true]), &[5]);
        assert_eq!(count(vec![true, true, true, true, false]), &[4]);
        assert_eq!(count(vec![false, true, true, true, true]), &[4]);
        assert_eq!(count(vec![false, true, true, true, false]), &[3]);
    }

    #[test]
    fn count_1() {
        assert_eq!(count(vec![true, false, true, false, true]), &[1, 1, 1]);
        assert_eq!(count(vec![false, true, true, false, true]), &[2, 1]);
    }
}
