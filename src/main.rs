fn sort_usernames<T>(usernames: &mut Vec<T>)
where T: Ord + AsRef<str> {
    usernames.sort_by(|el1, el2| {
        return el1.as_ref().to_lowercase().cmp(&el2.as_ref().to_lowercase())
    })
}

fn main() {
    let mut users = vec!["Todd", "Amy", "mike99", "Jennifer", "alison"];

    println!("unsorted: {:?}", &users);
    sort_usernames(&mut users);
    println!("sorted:   {:?}", &users);
}

#[test]
fn five_users() {
    let mut users = vec!["Todd", "Amy", "mike99", "Jennifer", "alison"];
    let sorted = vec!["alison", "Amy", "Jennifer", "mike99", "Todd"];
    sort_usernames(&mut users);

    assert_eq!(users, sorted);
}
