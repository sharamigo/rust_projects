use dioxus::prelude::*;
use crate::models::Wandertour;
use crate::components::TourCard;

#[component]
pub fn TourListView(touren: Vec<Wandertour>, on_tour_select: EventHandler<u32>) -> Element {
    rsx! {
        div {
            h2 { 
                style: "color: #333; margin-bottom: 20px; font-size: 1.8em;",
                "Verfügbare Wandertouren ({touren.len()})" 
            }
            
            div {
                style: "display: grid; gap: 20px; grid-template-columns: repeat(auto-fill, minmax(350px, 1fr));",
                for tour in touren {
                    TourCard {
                        key: "{tour.id}",
                        tour: tour.clone(),
                        on_select: move |id: u32| on_tour_select.call(id)
                    }
                }
            }
        }
    }
}