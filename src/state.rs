#[derive(Clone, Debug, PartialEq)]
pub enum AppState {
    TourList,
    TourDetail(u32),
}