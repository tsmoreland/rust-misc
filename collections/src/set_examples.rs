use std::collections::HashSet;

pub fn set_example() {
    let mut new_set:HashSet<i32> = HashSet::new();
    new_set.insert(0);
    new_set.insert(1);
    new_set.insert(2);
    new_set.insert(3);
    new_set.retain(|n| *n < 2);

    let from_set = HashSet::from([0, 1, 2]);
    if new_set.is_subset(&from_set) {
        println!("new set {:?} is subset of from set {:?}", new_set, from_set)
    }

    let mut cap_set:HashSet<i32> = HashSet::with_capacity(5);
    cap_set.insert(2);
    cap_set.insert(3);
    cap_set.insert(4);

    let union_set = new_set.union(&cap_set);
    println!("Union: {:?}", union_set)
}