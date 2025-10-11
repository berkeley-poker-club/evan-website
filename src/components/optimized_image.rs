use leptos::prelude::*;

#[component]
pub fn OptimizedImage(
    src: &'static str,
    alt: &'static str,
    #[prop(optional, default = "")] class: &'static str,
    #[prop(optional, default = "eager")] loading: &'static str,
) -> impl IntoView {
    // Create WebP version path at compile time
    let webp_src = if src.ends_with(".png") || src.ends_with(".PNG") {
        src.replace(".png", ".webp").replace(".PNG", ".webp")
    } else if src.ends_with(".jpg") || src.ends_with(".JPG") {
        src.replace(".jpg", ".webp").replace(".JPG", ".webp")
    } else if src.ends_with(".jpeg") || src.ends_with(".JPEG") {
        src.replace(".jpeg", ".webp").replace(".JPEG", ".webp")
    } else {
        src.to_string()
    };

    view! {
        <picture>
            <source srcset=webp_src type="image/webp" />
            <img src=src alt=alt class=class loading=loading />
        </picture>
    }
}
