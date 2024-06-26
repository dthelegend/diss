pub trait Problem {
    type Solution;
}

pub trait Reduction<U, V>
    where
        U: Problem,
        V: Problem,
{
    fn reduce(problem: &U) -> (V, Self);

    fn up_model(&self, solution: V::Solution) -> U::Solution;
}

pub trait Solver<T>
    where
        T: Problem,
{
    fn solve(&mut self, problem: &T) -> T::Solution;
}
