pub fn join_and<T: Into<String>>(iter: impl Iterator<Item = T>) -> String {
    let mut vec = iter.map(|item| item.into()).collect::<Vec<String>>();
    let last = vec.pop();

    if let Some(last) = last {
        if vec.is_empty() {
            last
        } else {
            format!("{} and {last}", vec.join(", "))
        }
    } else {
        String::new()
    }
}

pub fn join_or<T: Into<String>>(iter: impl Iterator<Item = T>) -> String {
    let mut vec = iter.map(|item| item.into()).collect::<Vec<String>>();
    let last = vec.pop();

    if let Some(last) = last {
        if vec.is_empty() {
            last
        } else {
            format!("{} or {last}", vec.join(", "))
        }
    } else {
        String::new()
    }
}
