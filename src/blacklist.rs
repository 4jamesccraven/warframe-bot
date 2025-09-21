use std::collections::HashSet;

use once_cell::sync::Lazy;

pub static BLACKLIST: Lazy<HashSet<String>> = Lazy::new(|| {
    let mut s = HashSet::new();

    s.insert("62d31b87106360aa5703954d".into());
    s.insert("6824c85b6c30b5a005004018".into());
    s.insert("67ae4e9fca4611344608d246".into());

    s
});
