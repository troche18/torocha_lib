struct CumulativeSum<T: std::ops::Add<Output=T> + std::ops::Sub<Output=T> + From<u8> + Copy> {
    list: Vec<T>,
}
impl<T: std::ops::Add<Output=T> + std::ops::Sub<Output=T> + From<u8> + Copy> CumulativeSum<T> {
    pub fn from(v: Vec<T>) -> Self {
        let mut list = vec![0.into(); v.len() + 1];
        for i in 1..=v.len() {
            list[i] = v[i - 1] + list[i - 1];
        }
        Self { list }
    }

    pub fn sum<R: std::ops::RangeBounds<usize>>(&self, range: R) -> T {
        let (l, r) = self.get_range(range, self.list.len() - 1);
        self.list[r] - self.list[l]
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
fn test_cumulative_sum() {
    let v = vec![1, 2, 3];
    let cumulative_sum = CumulativeSum::from(v);
    assert_eq!(6, cumulative_sum.sum(..));
    assert_eq!(1, cumulative_sum.sum(0..1));
    assert_eq!(2, cumulative_sum.sum(1..=1));
    assert_eq!(3, cumulative_sum.sum(2..=2));
    assert_eq!(3, cumulative_sum.sum(0..2));
}
