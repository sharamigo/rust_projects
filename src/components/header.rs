use dioxus::prelude::*;

#[component]
pub fn Header() -> Element {
    rsx! {
        header {
            style: "background: linear-gradient(135deg, #4a90e2, #7b68ee); color: white; padding: 30px; border-radius: 10px; margin-bottom: 30px; text-align: center;",
            h1 { 
                style: "margin: 0; font-size: 2.5em; text-shadow: 2px 2px 4px rgba(0,0,0,0.3);",
                "🥾 Wandertouren" 
            }
            p { 
                style: "margin: 10px 0 0 0; font-size: 1.1em; opacity: 0.9;",
                "Entdecke die schönsten Wanderrouten" 
            }
        }
    }
}