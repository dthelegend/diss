use std::{fmt::Debug, ops::{Index, IndexMut}};

use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

pub struct SparseMatrixElement<T: Sized + PartialEq + Send + Sync> {
    pub(crate) row: usize,
    pub(crate) column: usize,
    pub(crate) value: T
}

pub struct SparseMatrix<T: Sized + PartialEq + Send + Sync> {
    pub(crate) shape: (usize, usize),
    pub(crate) elements: Vec<SparseMatrixElement<T>>,
    pub(crate) default_value: T
}

impl <T: PartialEq  + Send + Sync> SparseMatrix<T> {
    /// Creates a new [`SparseMatrix<T>`] with the default value specified.
    pub fn new_with_default(shape: (usize, usize), default: T) -> Self {
        SparseMatrix { shape, elements: Vec::new(), default_value: default }
    }

    pub fn set(&mut self, index: (usize, usize), value: T) {
        let (x, y) = index;

        for (i, a) in self.elements.iter_mut().enumerate() {
            if a.column == x && a.row == y {
                if a.value == self.default_value {
                    self.elements.remove(i);
                } else {
                    a.value = value;
                }
                return;
            }
        }
        
        let new_element: SparseMatrixElement<T> = SparseMatrixElement::<T> {
            row: x,
            column: y,
            value
        };

        self.elements.push(new_element);
    }

    pub fn remove(&mut self, index: (usize, usize)) -> Option<T> {
        let (x, y) = index;

        let elements = &mut self.elements;
    
        for (i, a) in elements.iter().enumerate() {
            if a.column == x && a.row == y {
                return Some(elements.remove(i).value);
            }
        }
        None
    }

    pub fn values(&self) -> &Vec<SparseMatrixElement<T>> {
        &self.elements
    }

    pub fn purge(&mut self) {
        let elements = &mut self.elements;

        elements.retain(|a| {
            a.value != self.default_value
        });
    }
}

impl <T:PartialEq + Default + Send + Sync> SparseMatrix<T> {
    /// Creates a new [`SparseMatrix<T>`].
    pub fn new(shape: (usize, usize)) -> Self {
        Self::new_with_default(shape, Default::default())
    }
}

impl <T: PartialEq + Copy + Send + Sync> IndexMut<(usize, usize)> for SparseMatrix<T> {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        let SparseMatrix { default_value, shape: (shape_x, shape_y), elements } = self;
        let (x, y) = index;

        assert!(x < *shape_x && y < *shape_y, "Index ({},{}) is out of bounds for matrix of shape ({},{})!", x, y, shape_x, shape_y);
    
        for (i, el) in elements.iter().enumerate() {
            if el.row == x && el.column == y {
                return &mut elements[i].value;
            }
        }

        let new_element: SparseMatrixElement<T> = SparseMatrixElement::<T> {
            row: x,
            column: y,
            value: *default_value
        };

        elements.push(new_element);

        &mut elements.last_mut()
            .expect("Just populated with a new element! List cannot be empty.")
            .value
    }
}

impl <T: PartialEq + Send + Sync> Index<(usize, usize)> for SparseMatrix<T> {
    type Output = T;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        let SparseMatrix {default_value, elements, shape: (shape_x, shape_y)} = &self;
        let (x, y) = index;

        assert!(x < *shape_x && y < *shape_y, "Index ({},{}) is out of bounds for matrix of shape ({},{})!", x, y, shape_x, shape_y);

        elements.par_iter().find_any(|SparseMatrixElement {row, column, .. }| *row == x && *column == y)
            .map(|x| &x.value)
            .unwrap_or(default_value)
    }
}

impl <T: PartialEq + Debug + Send + Sync> Debug for SparseMatrix<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "SparseMatrix {:?}", self.shape)?;

        for i in 0..self.shape.0 {
            for j in 0..self.shape.1 {
                let value = &self[(i,j)];

                write!(f, "{:?}\t", value)?;
            }
            writeln!(f)?;
        }

        Ok(())
    }
}
