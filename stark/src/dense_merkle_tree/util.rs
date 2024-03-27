/// Returns the sibling index of a node
pub fn sibling(index: usize) -> usize {
    if index == 0 {
        0
    } else if index % 2 == 0 {
        index - 1
    } else {
        index + 1
    }
}

/// Return parent index of a node
pub fn parent(index: usize) -> usize {
    (index - 1) / 2
}

/// Return the number of extra hash data needed to build a merkle tree
pub fn extra_hash_count(leaf_count: usize) -> usize {
    leaf_count - 1
}

/// Takes a slice of n elements, returns a slice of m elements
/// where m is a power of 2.
pub fn extend_to_power_of_two<T: Clone>(input: &mut Vec<T>, default_value: T) {
    let padding_count = input.len().next_power_of_two() - input.len();
    let padding = vec![default_value.clone(); padding_count];
    input.extend(padding);
}

/// Return the number of leaves in a tree
pub fn number_of_leaves(tree_len: usize) -> usize {
    (tree_len + 1) / 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_sibling_index() {
        assert_eq!(sibling(4), 3);
        assert_eq!(sibling(1), 2);
        assert_eq!(sibling(0), 0);
    }

    #[test]
    fn test_get_parent_index() {
        assert_eq!(parent(1), 0);
        assert_eq!(parent(2), 0);
        assert_eq!(parent(11), 5);
        assert_eq!(parent(13), 6);
    }

    #[test]
    fn test_extend_to_power_of_two() {
        // 5 elements, next values of 2 is 8
        let mut set1 = vec![5, 6, 7, 8, 9];
        extend_to_power_of_two(&mut set1, 0);
        assert_eq!(set1.len(), 8);
        assert_eq!(set1, vec![5, 6, 7, 8, 9, 0, 0, 0]);
    }

    #[test]
    fn test_number_of_leaves() {
        // leaves = 5 tree_len = 9
        assert_eq!(number_of_leaves(9), 5);
        // leaves = 10 tree_len = 19
        assert_eq!(number_of_leaves(20), 10);
    }
}
