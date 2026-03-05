//! The components module contains all shared components for our app. Components are the building blocks of dioxus apps.
//! They can be used to defined common UI elements like buttons, forms, and modals. In this template, we define a Hero
//! component and an Echo component for fullstack apps to be used in our app.

pub mod header;
pub mod tour_list;
pub mod tour_detail;
pub mod tour_card;
pub mod etappe_card;
pub mod tour_images;

pub use header::Header;
pub use tour_list::TourListView;
pub use tour_detail::TourDetailView;
pub use tour_card::TourCard;
pub use etappe_card::EtappeCard;
pub use tour_images::ImageGallery;