use dioxus::prelude::*;
use crate::models::TourImage;


#[component]
pub fn ImageGallery(images: Vec<TourImage>) -> Element {
    let mut lightbox_open = use_signal(|| false);
    let mut current_image = use_signal(|| 0);
    let images_len = images.len();

    let mut open_lightbox = move |index: usize| {
        current_image.set(index);
        lightbox_open.set(true);
    };

    let close_lightbox = move |_| {
        lightbox_open.set(false);
    };

    let next_image = move |_| {
        if current_image() < images_len - 1 {
            current_image.set(current_image() + 1);
        } else {
            current_image.set(0);
        }
    };

    let prev_image = move |_| {
        if current_image() > 0 {
            current_image.set(current_image() - 1);
        } else {
            current_image.set(images_len - 1);
        }
    };

    if images.is_empty() {
        return rsx! {
            div {
                class: "no-images",
                p { "Keine Bilder für diese Tour verfügbar." }
            }
        };
    }

    rsx! {
        div {
            class: "tour-image-gallery",
            
            h3 {
                class: "gallery-title",
                "📸 Impressionen von der Tour ({images_len} Bilder)"
            }
            
            // Thumbnail Grid
            div {
                class: "thumbnail-grid",
                for (index, image) in images.iter().enumerate() {
                    div {
                        key: "{index}",
                        class: "thumbnail-container",
                        onclick: move |_| open_lightbox(index),
                        img {
                            src: "{image.src}",
                            alt: "{image.alt}",
                            class: "thumbnail",
                            loading: "lazy"
                        }
                        div {
                            class: "thumbnail-overlay",
                            div {
                                class: "thumbnail-overlay-content",
                                "🔍 Vergrößern"
                            }
                        }
                    }
                }
            }

            // Lightbox Modal
            if lightbox_open() {
                div {
                    class: "lightbox-overlay",
                    onclick: close_lightbox,
                    
                    div {
                        class: "lightbox-content",
                        onclick: |e| e.stop_propagation(),
                        
                        // Close Button
                        button {
                            class: "lightbox-close",
                            onclick: close_lightbox,
                            title: "Schließen (ESC)",
                            "×"
                        }
                        
                        // Navigation Buttons
                        if images_len > 1 {
                            button {
                                class: "lightbox-nav lightbox-prev",
                                onclick: prev_image,
                                title: "Vorheriges Bild",
                                "❮"
                            }
                            button {
                                class: "lightbox-nav lightbox-next", 
                                onclick: next_image,
                                title: "Nächstes Bild",
                                "❯"
                            }
                        }
                        
                        // Main Image
                        div {
                            class: "lightbox-image-container",
                            img {
                                src: "{images[current_image()].src}",
                                alt: "{images[current_image()].alt}",
                                class: "lightbox-image"
                            }
                        }
                        
                        // Caption
                        div {
                            class: "lightbox-info",
                            div {
                                class: "lightbox-caption",
                                "{images[current_image()].caption}"
                            }
                            
                            // Image Counter
                            if images_len > 1 {
                                div {
                                    class: "lightbox-counter",
                                    "{current_image() + 1} von {images_len}"
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

