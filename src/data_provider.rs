//! TODO

/// TODO
pub(crate) trait DataProvider<T> {
    /// TODO
    fn data_ready(&self) -> bool;

    /// TODO
    fn get_data(&self) -> T;
}
