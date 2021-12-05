pub trait Solver {
    fn name(&self) -> String;
    fn part01(&self, input: &str) -> i64;
    fn part02(&self, input: &str) -> i64;
}
