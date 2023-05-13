use std::collections::HashMap;

#[derive(Debug, PartialEq, Hash, Eq)]
struct User {
    id: i32,
    username: String,
    first_name: String,
    last_name: String,
}

pub fn hashmap_example() {
    let mut users_by_id: HashMap<i32, User> = HashMap::from([
        (
            1,
            User {
                id: 1,
                username: String::from("batman"),
                first_name: String::from("Bruce"),
                last_name: String::from("Wayne"),
            },
        ),
        (
            2,
            User {
                id: 2,
                username: String::from("wonder-woman"),
                first_name: String::from("Diana"),
                last_name: String::from("Prince"),
            },
        ),
    ]);

    users_by_id.insert(
        3,
        User {
            id: 3,
            username: String::from("superman"),
            first_name: String::from("Clark"),
            last_name: String::from("Kent"),
        },
    );

    users_by_id.remove(3);
    users_by_id.entry(3).or_insert(User {
        id: 3,
        username: String::from("superman"),
        first_name: String::from("Clark"),
        last_name: String::from("Kent"),
    });

    users_by_id.entry(4).or_insert(User {
        id: 4,
        username: String::from("green-lantern"),
        first_name: String::from("Hal"),
        last_name: String::from("Jordan"),
    });

    let batman = users_by_id.get(1);
    match batman {
        Some(user) => println!("{user}"),
        None => println!("Not found"),
    }

    for (id, user) in users_by_id {
        println!("{} = {:?}", id, user)
    }
    
    users_by_id.retain(|id, user| user.id >= 0);
}
