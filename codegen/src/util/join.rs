use std::fmt::Display;

pub fn join_list<T: Display>(list: impl IntoIterator<Item = T>) -> String {
    let vec = list.into_iter().map(|x| x.to_string()).collect::<Vec<_>>();

    if vec.len() < 2 {
        return vec.join(", ");
    }

    format!(
        "{} and {}",
        vec[..vec.len() - 1].join(", "),
        vec[vec.len() - 1],
    )
}
