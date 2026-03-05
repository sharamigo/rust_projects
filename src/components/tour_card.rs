use dioxus::prelude::*;
use crate::models::Wandertour;

#[component]
pub fn TourCard(tour: Wandertour, on_select: EventHandler<u32>) -> Element {
    let tour_id = tour.id;

    let difficulty_class = match tour.schwierigkeitsgrad.as_str() {
        "Leicht" => "easy",
        "Mittel" => "medium", 
        "Schwer" => "hard",
        _ => "unknown",
    };
    
    rsx! {
        div {
            class: "tour-card",
            style: "
                background: white;
                border: 2px solid #e0e0e0;
                border-radius: 12px;
                padding: 20px;
                cursor: pointer;
                transition: all 0.3s ease;
                box-shadow: 0 4px 6px rgba(0,0,0,0.1);
                hover: {{
                    transform: translateY(-2px);
                    box-shadow: 0 6px 12px rgba(0,0,0,0.15);
                }}
            ",
            onclick: move |_| on_select.call(tour_id),
            
            h3 { 
                style: "margin: 0 0 10px 0; color: #2c3e50; font-size: 1.3em;",
                "🗻 {tour.name}" 
            }
            
            p { 
                style: "color: #666; margin: 5px 0; line-height: 1.4;",
                "{tour.beschreibung}" 
            }
            
            div {
                style: "display: flex; justify-content: space-between; align-items: center; margin-top: 15px; padding-top: 15px; border-top: 1px solid #eee;",
                
                div {
                    span {
                        style: "
                            background: #e8f4f8;
                            color: #2980b9;
                            display: inline-block;
                            margin-bottom: 3px;
                            padding: 4px 8px;
                            border-radius: 20px;
                            font-size: 0.9em;
                            margin-right: 5px;
                        ",
                        "Länge 📏 {tour.gesamtlaenge():.1} km"
                    }
                    br {}
                    span {
                        style: "
                            background: #f0f8e8;
                            color: #27ae60;
                            display: inline-block;
                            margin-top: 3px;
                            padding: 4px 8px;
                            border-radius: 20px;
                            font-size: 0.9em;
                            margin-right: 5px;
                        ",
                        "Aufstieg ⬆️ {tour.gesamtsteigung()} hm"
                    }
                    br {}
                    span {
                        style: "
                            background: #fff2e8;
                            color: #e67e22;
                            display: inline-block;
                            margin-top: 5px;
                            padding: 4px 8px;
                            border-radius: 20px;
                            font-size: 0.9em;
                        ",
                        "Abstieg ⬇️ {tour.gesamtabstieg()} hm"
                    }
                }
                
                div {
                    style: "display: flex; align-items: center; gap: 10px;",
                    span {
                        class: "difficulty-badge difficulty-{difficulty_class}",
                        "{tour.schwierigkeitsgrad}"
                    }                    
                    span {
                        style: "
                            background: #d2ebf4ff;
                            color: #2737aeff;
                            padding: 4px 8px;
                            border-radius: 20px;
                            font-size: 0.9em;
                        ",
                        "{tour.etappen.len()} Etappen"
                    }
                }
            }
        }
    }
}