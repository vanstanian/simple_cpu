#[macro_export]
macro_rules! prompt {
    ($val:expr) => {
        stdin()
        .read_line(&mut $val)
        .ok()
        .expect("Failed to read line");
    };
}