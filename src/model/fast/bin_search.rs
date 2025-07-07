pub fn bin_search<T: Ord>(arr: &[T], target: T) -> Option<usize> {
    let mut left = 0;
    let mut right = arr.len() - 1;

    while left <= right {
        let mid = left + (right - left) / 2;

        match arr[mid].cmp(&target) {
            std::cmp::Ordering::Less => left = mid + 1,
            std::cmp::Ordering::Greater => right = mid - 1,
            std::cmp::Ordering::Equal => return Some(mid),
        }
    }
    None
}

pub fn nomore_tar<T: Ord>(arr: &[T], target: T) -> Option<usize> {
    let mut left = 0;
    let mut right = arr.len() - 1;
    let mut best_index = None;

    while left <= right {
        let mid = left + (right - left) / 2;

        match arr[mid].cmp(&target) {
            std::cmp::Ordering::Less => {
                // 当前值小于目标值，可能是候选解
                best_index = Some(mid);
                left = mid + 1; // 继续向右查找是否有更大的符合条件的值
            }
            std::cmp::Ordering::Greater => {
                right = mid - 1; // 向左查找更小的值
            }
            std::cmp::Ordering::Equal => {
                // 找到相等的值，直接返回该索引
                return Some(mid);
            }
        }
    }
    best_index
}
