use dioxus::prelude::*;
use crate::models::Etappe;

#[component]
pub fn EtappeCard(etappe: Etappe, index: usize) -> Element {
    rsx! {
        div {
            style: "
                border: 2px solid #e0e0e0;
                border-radius: 10px;
                padding: 25px;
                background: linear-gradient(135deg, #f8f9fa, #ffffff);
            ",
            
            h3 { 
                style: "margin: 0 0 15px 0; color: #2c3e50; font-size: 1.4em;",
                "🚶 Etappe {index} - {etappe.name}" 
            }
            
            p { 
                style: "color: #666; margin: 0 0 20px 0; line-height: 1.5; font-size: 1.1em;",
                "{etappe.beschreibung}" 
            }
            
            div {
                style: "display: grid; grid-template-columns: repeat(auto-fit, minmax(200px, 1fr)); gap: 15px; margin-bottom: 20px;",
                
                div {
                    style: "background: #fff; padding: 15px; border-radius: 6px; border-left: 4px solid #3498db;",
                    strong { "📏 Länge: " }
                    "{etappe.laenge_km} km"
                }
                
                div {
                    style: "background: #fff; padding: 15px; border-radius: 6px; border-left: 4px solid #27ae60;",
                    strong { "⬆️ Steigung: " }
                    "{etappe.steigung_hm} hm"
                }
                
                div {
                    style: "background: #fff; padding: 15px; border-radius: 6px; border-left: 4px solid #e67e22;",
                    strong { "⬇️ Abstieg: " }
                    "{etappe.abstieg_hm} hm"
                }
            }
            
            div {
                style: "display: grid; grid-template-columns: 1fr 1fr; gap: 20px;",
                
                div {
                    style: "background: #fff; padding: 15px; border-radius: 6px; border: 1px solid #ddd;",
                    h4 { 
                        style: "margin: 0 0 10px 0; color: #27ae60;",
                        "🟢 Start" 
                    }
                    p { 
                        style: "margin: 5px 0; font-weight: bold;",
                        "{etappe.startort}" 
                    }
                    p { 
                        style: "margin: 5px 0; font-family: monospace; background: #f8f9fa; padding: 5px; border-radius: 3px;",
                        "📍 {etappe.start_gps.latitude:.6}, {etappe.start_gps.longitude:.6}" 
                    }
                }
                
                div {
                    style: "background: #fff; padding: 15px; border-radius: 6px; border: 1px solid #ddd;",
                    h4 { 
                        style: "margin: 0 0 10px 0; color: #e74c3c;",
                        "🔴 Ziel" 
                    }
                    p { 
                        style: "margin: 5px 0; font-weight: bold;",
                        "{etappe.zielort}" 
                    }
                    p { 
                        style: "margin: 5px 0; font-family: monospace; background: #f8f9fa; padding: 5px; border-radius: 3px;",
                        "📍 {etappe.ziel_gps.latitude:.6}, {etappe.ziel_gps.longitude:.6}" 
                    }
                }
            }
        }
    }
}