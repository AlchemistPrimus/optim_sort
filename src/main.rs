//! OptimSort.
#![allow(unused_variables)]

fn main() {
    let mut unsorted_list = [5, 8, 8, 3, 9, 10, 2, 5, 4, 1, 0, 10, 99, 4, 7, 1, 345, 32,
    90, 845, 6, 87, 54, 76, 23, 45, 32, 53, 98, 20, 6, 33, 29,
    43, 54, 32, 33, 52, 75, 85, 32, 40, 9, 32, 18, 19, 23, 54, 12];
    println!("Original list is \n{:?}", &unsorted_list);
    let sorted_list = optim_merge_sort(&mut unsorted_list);
    println!("Sorted list is \n{:?}", sorted_list);
}

#[allow(dead_code)]
fn optim_merge_sort(collection: &mut [usize]) -> Vec<usize>{
    // The overal sorting procedure. This is divide procedure where sorting begins
    if collection.len() <= 43{
        let mut col = collection;
        let sorted_leaf = sort_leaf(&mut col);
        return Vec::from(sorted_leaf);
    } else {
        let mid = collection.len()/2;
        let sorted_left = optim_merge_sort(&mut collection[0..mid]);
        let sorted_right = optim_merge_sort(&mut collection[mid..]);
        merge(&sorted_left, &sorted_right)
    }
}

#[allow(dead_code)]
fn merge(left: &[usize], right: &[usize]) -> Vec<usize>{
    // This is the merging technique of our sorting procedure.
    let mut collection: Vec<usize> = vec![];
    let mut i = 0;
    let mut j = 0;
    while i<left.len() && j<right.len() {
        if left[i]<right[j] {
            collection.push(left[i]);
            i += 1;
        } else {
            collection.push(right[j]);
            j += 1;
        }
    }
    collection.extend(&left[i..]);
    collection.extend(&right[j..]);
    collection
}


fn sort_leaf(collection: &mut [usize]) -> &[usize] {
    // Returns a sorted sequence. Sorts via insertion.
    for i in 1..collection.len(){
        let mut j: usize = i;
        let key = collection[j];
        let pos = search_pos(&collection[0..j], key);
        while j>pos {
            collection.swap(j, j-1);
            j -= 1;
        }
        collection[pos] = key;
    }
    collection
}

fn search_pos(collection: &[usize], target: usize) -> usize {
    // Returns the position to insert the target item. Uses bin search technique. 
    let mut low = 0;
    let mut high = collection.len();
    let mut mid = 0;
    while low < high {
        mid = low + (high-low)/2;
        if collection[mid] == target {
            // Pos to insert the item when similar already exist in the list
            return mid+1;
        } else if collection[mid] < target {
            low = mid + 1;
        } else {
            high = mid;
        }
    }
    // position to insert the item when it doesn't exist in the list
    if mid > 0 {
        if collection[mid] > target {
            // Here the target is less than the adjacent lowest and highest elements.
            return mid
        }
        return mid+1
    } else if mid ==0 && collection[mid] < target {
        // when element at position 1 need not to be switched, return its position.
        return mid + 1
    }
    // When the item should be at position 0, or first position.
    mid
}
