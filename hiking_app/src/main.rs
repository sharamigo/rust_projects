mod models;
mod components;
mod data;
mod state;

use dioxus::prelude::*;
//use models::*;
use components::*;
use data::*;
use state::*;

fn main() {
    launch(App);
}

#[allow(non_snake_case)]
fn App() -> Element {
    let mut app_state = use_signal(|| AppState::TourList);
    let touren = use_memo(move || get_sample_tours());

    static CSS: Asset = asset!("/assets/styling/main.css");

    rsx! {
        document::Stylesheet { href: CSS }
        div {
            class: "container",
            style: "max-width: 1200px; margin: 0 auto; padding: 20px; font-family: Arial, sans-serif;",
            
            Header {}

            match app_state.read().clone() {
                AppState::TourList => rsx! {
                    TourListView { 
                        touren: touren.read().clone(),
                        on_tour_select: move |id| app_state.set(AppState::TourDetail(id))
                    }
                },
                AppState::TourDetail(tour_id) => rsx! {
                    TourDetailView {
                        tour: touren.read().iter().find(|t| t.id == tour_id).cloned(),
                        on_back: move |_| app_state.set(AppState::TourList)
                    }
                }
            }
        }
    }
}