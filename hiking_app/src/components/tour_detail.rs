use dioxus::prelude::*;
use crate::models::Wandertour;
use crate::components::EtappeCard;
use crate::components::ImageGallery;


#[component]
pub fn TourDetailView(tour: Option<Wandertour>, on_back: EventHandler<()>) -> Element {
    match tour {
        Some(tour) => rsx! {
            div {
                button {
                    style: "
                        background: #3498db;
                        color: white;
                        border: none;
                        padding: 12px 20px;
                        border-radius: 6px;
                        cursor: pointer;
                        font-size: 1em;
                        margin-bottom: 20px;
                        transition: background 0.3s;
                    ",
                    onclick: move |_| on_back.call(()),
                    "⬅️ Zurück zur Übersicht"
                }
                
                div {
                    style: "background: white; border-radius: 12px; padding: 30px; box-shadow: 0 4px 6px rgba(0,0,0,0.1);",
                    
                    h1 { 
                        style: "color: #2c3e50; margin: 0 0 20px 0; font-size: 2.2em;",
                        "🗻 {tour.name}" 
                    }
                    
                    p { 
                        style: "color: #666; font-size: 1.1em; line-height: 1.6; margin-bottom: 25px;",
                        "{tour.beschreibung}" 
                    }
                    
                    // Statistiken
                    div {
                        style: "display: grid; grid-template-columns: repeat(auto-fit, minmax(200px, 1fr)); gap: 20px; margin-bottom: 30px;",
                        
                        div {
                            style: "background: #f8f9fa; padding: 20px; border-radius: 8px; text-align: center;",
                            h3 { 
                                style: "margin: 0 0 10px 0; color: #2980b9;",
                                "Gesamtlänge" 
                            }
                            p { 
                                style: "font-size: 1.5em; font-weight: bold; color: #2c3e50; margin: 0;",
                                "{tour.gesamtlaenge():.1} km" 
                            }
                        }
                        
                        div {
                            style: "background: #f8f9fa; padding: 20px; border-radius: 8px; text-align: center;",
                            h3 { 
                                style: "margin: 0 0 10px 0; color: #27ae60;",
                                "Gesamtsteigung" 
                            }
                            p { 
                                style: "font-size: 1.5em; font-weight: bold; color: #2c3e50; margin: 0;",
                                "{tour.gesamtsteigung()} hm" 
                            }
                        }
                        
                        div {
                            style: "background: #f8f9fa; padding: 20px; border-radius: 8px; text-align: center;",
                            h3 { 
                                style: "margin: 0 0 10px 0; color: #e67e22;",
                                "Gesamtabstieg" 
                            }
                            p { 
                                style: "font-size: 1.5em; font-weight: bold; color: #2c3e50; margin: 0;",
                                "{tour.gesamtabstieg()} hm" 
                            }
                        }
                        
                        div {
                            style: "background: #f8f9fa; padding: 20px; border-radius: 8px; text-align: center;",
                            h3 { 
                                style: "margin: 0 0 10px 0; color: #8e44ad;",
                                "Schwierigkeit" 
                            }
                            p { 
                                style: "font-size: 1.5em; font-weight: bold; color: #2c3e50; margin: 0;",
                                "{tour.schwierigkeitsgrad}" 
                            }
                        }
                    }
                    
                    h2 { 
                        style: "color: #2c3e50; margin: 30px 0 20px 0; font-size: 1.8em;",
                        "Etappen ({tour.etappen.len()})" 
                    }
                    
                    div {
                        style: "display: grid; gap: 20px;",
                        for (index, etappe) in tour.etappen.iter().enumerate() {
                            EtappeCard {
                                key: "{etappe.id}",
                                etappe: etappe.clone(),
                                index: index + 1
                            }
                        }
                    }
                    
                    // Image Gallery - hier werden nur die Bilder der aktuellen Tour angezeigt
                    ImageGallery {
                        images: tour.images.clone()
                    }
                }
            }
        },
        None => rsx! {
            div {
                style: "text-align: center; padding: 50px;",
                h2 { "Tour nicht gefunden" }
                button {
                    style: "
                        background: #3498db;
                        color: white;
                        border: none;
                        padding: 12px 20px;
                        border-radius: 6px;
                        cursor: pointer;
                        font-size: 1em;
                        margin-top: 20px;
                    ",
                    onclick: move |_| on_back.call(()),
                    "⬅️ Zurück zur Übersicht"
                }
            }
        }
    }
}