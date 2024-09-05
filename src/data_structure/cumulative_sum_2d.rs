struct CumulativeSum2d<T: std::ops::Add<Output=T> + std::ops::Sub<Output=T> + From<u8> + Copy> {
    table: Vec<Vec<T>>,
}
impl<T: std::ops::Add<Output=T> + std::ops::Sub<Output=T> + From<u8> + Copy> CumulativeSum2d<T> {
    pub fn from(v: Vec<Vec<T>>) -> Self {
        let mut table = vec![vec![0.into(); v[0].len() + 1]; v.len() + 1];
        for i in 1..v.len() + 1 {
            for j in 1..v[0].len() + 1 {
                table[i][j] = v[i - 1][j - 1] + table[i][j - 1] + table[i - 1][j] - table[i - 1][j - 1];
            }
        }
        Self { table }
    }

    pub fn sum<R: std::ops::RangeBounds<usize>>(&self, range_h: R, range_w: R) -> T {
        let (h1, h2) = self.get_range(range_h, self.table.len() - 1);
        let (w1, w2) = self.get_range(range_w, self.table[0].len() - 1);
        self.table[h1][w1] + self.table[h2][w2] - self.table[h1][w2] - self.table[h2][w1]
    }

    fn get_range<R: std::ops::RangeBounds<usize>>(&self, range: R, max_value: usize) -> (usize, usize) {
        let l = match range.start_bound() {
            std::ops::Bound::Unbounded => 0,
            std::ops::Bound::Included(&l) => l,
            _ => unreachable!()
        };
        let r = match range.end_bound() {
            std::ops::Bound::Unbounded => max_value,
            std::ops::Bound::Included(&x) => x + 1,
            std::ops::Bound::Excluded(&x) => x,
        };
        (l, r)
    }
}

#[test]
fn test_cumulative_sum_2d() {
    let v = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
    let cumulative_sum = CumulativeSum2d::from(v);
    assert_eq!(45, cumulative_sum.sum(.., ..));
    assert_eq!(0, cumulative_sum.sum(1..1, 1..1));
    assert_eq!(1, cumulative_sum.sum(0..1, 0..1));
    assert_eq!(1, cumulative_sum.sum(0..=0, 0..=0));
    assert_eq!(28, cumulative_sum.sum(1..=2, 1..=2));
}