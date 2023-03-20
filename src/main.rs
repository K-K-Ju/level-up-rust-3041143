use std::{collections::HashSet, hash::Hash};

fn unique(a: Vec<i32>) -> Vec<i32> {
    if a.len() < 2 {return a}
    let mut set: HashSet<i32> = HashSet::new();
    a.iter()
        .filter(|&num| set.insert(*num))
        .copied()
        .collect()
}

fn gen_unique<T>(a: Vec<T>) -> Vec<T>
where T: Ord + Clone{
    let mut a_clone = a.clone();
    a_clone.sort();
    a_clone.dedup();
    a_clone
}

fn not_sorted_unique<T>(a: Vec<T>) -> Vec<T> 
where T: Ord + Copy{
    if a.len() < 2 {return a;}

    let mut res: Vec<T> = vec![];
    
    for i in 0..a.len() {
        if !res.contains(&a[i]) {
            res.push(a[i]);
        } else {
            continue;
        }
    }
    res
}

fn unique_from_iterator<T>(mut a: Vec<T>) -> Vec<T>
where T: Ord + Hash {
    let set: HashSet<_> = a.drain(..).collect();
    a.extend(set.into_iter());
    a
}

fn main() {
    let input = vec![2, 1, 1];
    let answer = unique_from_iterator(input);
    println!("unique items -> {:?}", answer)
}

#[test]
fn empty_list() {
    let input = vec![];
    let expected_output = vec![];
    let actual_output = unique(input);
    assert_eq!(actual_output, expected_output);
}

#[test]
fn sorted_list() {
    let input = vec![1, 4, 5];
    let mut expected_output = vec![1, 4, 5];
    let mut actual_output = unique(input);
    expected_output.sort_unstable();
    actual_output.sort_unstable();
    assert_eq!(actual_output, expected_output);
}

#[test]
fn unsorted_list() {
    let input = vec![1, 5, 2];
    let mut expected_output = vec![1, 2, 5];
    let mut actual_output = unique(input);
    expected_output.sort_unstable();
    actual_output.sort_unstable();
    assert_eq!(actual_output, expected_output);
}

#[test]
fn unsorted_list_with_duplicates() {
    let input = vec![1, 5, 2, 2, 1];
    let mut expected_output = vec![1, 2, 5];
    let mut actual_output = unique(input);
    expected_output.sort_unstable();
    actual_output.sort_unstable();
    assert_eq!(actual_output, expected_output);
}

#[test]
fn sorted_list_with_duplicates() {
    let mut input = vec![1, 5, 2, 2, 1];
    input.sort_by(|x, y| x.partial_cmp(y).unwrap());
    let mut expected_output = vec![1, 2, 5];
    let mut actual_output = unique(input);
    expected_output.sort_unstable();
    actual_output.sort_unstable();
    assert_eq!(actual_output, expected_output);
}
