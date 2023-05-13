use std::collections::LinkedList;

pub fn list_example() {
    let mut new_list = LinkedList::new();
    let mut from_list = LinkedList::from([0, 1, 2]);

    new_list.push_front(1);
    new_list.push_front(0);
    new_list.push_back(2);

    if new_list.contains(0) {
        println!("new_list contains 0");
    }

    println!("new: {:?} from: {:?}", new_list, from_list);
}
