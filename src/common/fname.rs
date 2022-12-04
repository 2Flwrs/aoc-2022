use std::path::PathBuf;

pub(crate) fn input_filename(day: usize, real: bool) -> PathBuf {
    let mut path = PathBuf::from("input");
    let ext = if real { "txt" } else { "short.txt" };
    path.push(format!("day{day}.{ext}"));
    path
}
