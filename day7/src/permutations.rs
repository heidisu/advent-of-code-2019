pub fn permutations(min:usize, max: usize) -> Permutations {
    Permutations { idxs: (min..max + 1).collect(), swaps: vec![min; max + 1], i: min, min: min }
}
 
pub struct Permutations {
    idxs: Vec<usize>,
    swaps: Vec<usize>,
    i: usize,
    min: usize
}
 
impl Iterator for Permutations {
    type Item = Vec<usize>;
 
    fn next(&mut self) -> Option<Self::Item> {
        if self.i > self.min {
            loop {
                if self.i >= self.swaps.len() { return None; }
                if self.swaps[self.i] < self.i { break; }
                self.swaps[self.i] = 0;
                self.i += 1;
            }
            self.idxs.swap(self.i, (self.i & 1) * self.swaps[self.i]);
            self.swaps[self.i] += 1;
        }
        self.i = 1;
        Some(self.idxs.clone())
    }
}