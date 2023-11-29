use std::{fmt::Debug, ops::{Index, IndexMut}};

pub struct SparseMatrixElement<T: Sized> {
    pub(crate) row: usize,
    pub(crate) column: usize,
    pub(crate) value: T
}

pub struct SparseMatrix<T: Sized + Copy> {
    pub(crate) shape: (usize, usize),
    pub(crate) elements: Vec<SparseMatrixElement<T>>,
    pub(crate) default_value: T
}

impl <T> SparseMatrix<T>
where T: Sized + Copy {
    /// Creates a new [`SparseMatrix<T>`] with the default value specified.
    pub fn new_with_default(shape: (usize, usize), default: T) -> Self {
        SparseMatrix { shape, elements: Vec::new(), default_value: default }
    }

    pub fn remove(&mut self, index: (usize, usize)) -> Option<T> {
        let (x, y) = index;

        let elements = &mut self.elements;
    
        let mut output: Option<T> = None;
        elements.retain(|a| {
            if a.column == x && a.row == y {
                output = Some(a.value);
                true
            }
            else {
                false
            }
        });

        output
    }

    pub fn values(&self) -> impl Iterator<Item = &SparseMatrixElement<T>> {
        self.elements.iter()
    }
}

impl <T> SparseMatrix<T>
where T: Sized + Copy + Default {
    /// Creates a new [`SparseMatrix<T>`].
    pub fn new(shape: (usize, usize)) -> Self {
        Self::new_with_default(shape, Default::default())
    }
}

impl <T> IndexMut<(usize, usize)> for SparseMatrix<T>
where T: Sized + Copy {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        let SparseMatrix { default_value, shape: (shape_x, shape_y), .. } = self;
        let (x, y) = index;

        assert!(x < *shape_x && y < *shape_y, "Index ({},{}) is out of bounds for matrix of shape ({},{})!", x, y, shape_x, shape_y);
    
        for (i, el) in self.elements.iter().enumerate() {
            if el.row == x && el.column == y {
                return &mut self.elements[i].value;
            }
        }

        let new_element: SparseMatrixElement<T> = SparseMatrixElement::<T> {
            row: x,
            column: y,
            value: *default_value
        };

        self.elements.push(new_element);

        &mut self.elements.last_mut()
            .expect("Just populated with a new element! List cannot be empty.")
            .value
    }
}

impl <T> Index<(usize, usize)> for SparseMatrix<T>
where T: Sized + Copy {
    type Output = T;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        let SparseMatrix {default_value, elements, shape: (shape_x, shape_y)} = &self;
        let (x, y) = index;

        assert!(x < *shape_x && y < *shape_y, "Index ({},{}) is out of bounds for matrix of shape ({},{})!", x, y, shape_x, shape_y);

        elements.iter().find(|SparseMatrixElement {row, column, .. }| *row == x && *column == y)
            .map(|x| &x.value)
            .unwrap_or(default_value)
    }
}

impl <T> Debug for SparseMatrix<T>
where T: Sized + Copy + Debug {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "SparseMatrix {:?}", self.shape)?;

        for i in 0..self.shape.0 {
            for j in 0..self.shape.1 {
                let value = self[(i,j)];

                write!(f, "{:?}\t", value)?;
            }
            writeln!(f)?;
        }

        Ok(())
    }
}
