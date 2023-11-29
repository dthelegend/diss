use std::{ops::{Index, IndexMut}, iter::zip, fmt::Debug};

pub struct SparseMatrixElement<T: Sized> {
    row: usize,
    column: usize,
    value: T
}

pub struct SparseMatrix<T: Sized + Copy> {
    shape: (usize, usize),
    elements: Vec<SparseMatrixElement<T>>,
    default_value: T
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
        write!(f, "SparseMatrix {:?}\n", self.shape)?;

        for i in 0..self.shape.0 {
            for j in 0..self.shape.1 {
                let value = self[(i,j)];

                write!(f, "{:?}\t", value)?;
            }
            write!(f, "\n")?;
        }

        Ok(())
    }
}

#[derive(Debug)]
pub struct QUBOProblem(SparseMatrix<i32>);

pub struct QUBOSolution(Vec<bool>);

impl Debug for QUBOSolution {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "QUBOSolution ")?;
        f.debug_list().entries(self.0.iter().map(|x| if *x { 1 } else { 0 })).finish()
    }
}

impl From<Vec<bool>> for QUBOSolution {
    fn from(value: Vec<bool>) -> Self {
        QUBOSolution(value)
    }
}

pub trait QUBOSolutionBackend {
    fn find_min_solution(&self, problem: &QUBOProblem) -> QUBOSolution;
}

impl QUBOProblem {
    pub fn new(size: usize) -> Self {
        QUBOProblem(SparseMatrix::new((size, size)))
    }

    fn adjust_index(index: (usize, usize)) -> (usize, usize) {
        let (x, y) = index;
        if x > y {
            (y, x)
        }
        else {
            (x,y)
        }
    }

    pub fn get_size(&self) -> usize {
        let QUBOProblem(problem_matrix) = self;
        let (x, y) = problem_matrix.shape;
        
        assert!(x == y, "Problem is not square!");
        
        x
    }

    pub fn evaluate_solution(&self, solution: &QUBOSolution) -> i32 {
        let QUBOSolution(solution_vector) = solution;
        assert!(solution_vector.len() == self.get_size());

        let mut x_q = vec![0; solution_vector.len()];

        // Vector multiply Sparse matrix
        for &SparseMatrixElement { row, column, value } in self.0.values() {
            x_q[column] += value * (if solution_vector[row] { 1 } else { 0 });
        }

        // multiply two matrices
        zip(x_q, solution_vector).map(|(a, b)| a * (if *b { 1 } else { 0 })).sum()
    }

    pub fn find_min_solution(&self, backend: &dyn QUBOSolutionBackend) -> QUBOSolution {
        backend.find_min_solution(&self)
    }
}

impl IndexMut<(usize, usize)> for QUBOProblem {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        &mut self.0[Self::adjust_index(index)]
    }
}

impl Index<(usize, usize)> for QUBOProblem {
    type Output = i32;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        &self.0[Self::adjust_index(index)]
    }
}