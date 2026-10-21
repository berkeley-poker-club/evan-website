use leptos::prelude::*;
use std::time::Duration;

const CLOUDINARY_BASE: &str = "https://res.cloudinary.com/yf1qrlzz/image/upload/f_auto,q_auto,w_1600";
const CLOUDINARY_ROOT: &str = "poker-at-berkeley";
const MAX_CLOUD_RETRIES: u32 = 2;

/// Maps a local carousel image path (e.g. "/public/images/qrt-tourney/DSCF4250.webp")
/// to its Cloudinary URL. Returns None for folders that weren't uploaded.
pub fn cloudinary_url(local: &str) -> Option<String> {
    let rest = local.strip_prefix("/public/images/")?;
    let (local_folder, file) = rest.split_once('/')?;
    let cloud_folder = match local_folder {
        "stanfxcal26" => "spring-2026-stanford",
        "qrt-tourney" => "spring-2026-qrt",
        "stanfxcal25" => "spring-2025-stanford",
        "falltourney" => "fall-2025-tournament",
        "officergroup" => "officers",
        _ => return None,
    };
    let stem = file.rsplit_once('.').map_or(file, |(stem, _)| stem);
    Some(format!("{CLOUDINARY_BASE}/{CLOUDINARY_ROOT}/{cloud_folder}/{stem}"))
}

/// Carousel photo served from Cloudinary. On error it retries Cloudinary twice,
/// then falls back to the local copy.
#[component]
pub fn CarouselImage(
    local_src: &'static str,
    alt: String,
    class: &'static str,
    eager: bool,
) -> impl IntoView {
    let cloud = cloudinary_url(local_src);
    let initial = cloud.clone().unwrap_or_else(|| local_src.to_string());
    let (src, set_src) = signal(initial);
    let attempts = StoredValue::new(0u32);

    let on_error = move |_| {
        let n = attempts.get_value() + 1;
        attempts.set_value(n);
        match &cloud {
            Some(url) if n <= MAX_CLOUD_RETRIES => {
                let retry_url = format!("{url}?retry={n}");
                set_timeout(move || set_src.set(retry_url), Duration::from_millis(1000 * n as u64));
            }
            Some(_) if n == MAX_CLOUD_RETRIES + 1 => set_src.set(local_src.to_string()),
            _ => {}
        }
    };

    view! {
        <img
            src=move || src.get()
            alt=alt
            class=class
            loading=if eager { "eager" } else { "lazy" }
            on:error=on_error
        />
    }
}

#[component]
pub fn MediaCarousel(
    #[prop(optional)] title: Option<&'static str>,
    images: Vec<&'static str>,
) -> impl IntoView {
    view! {
        <section class="pt-12 pb-8 bg-[#c8d1d9] dark:bg-gray-900">

            <div class="max-w-7xl mx-auto px-6">
                <h2 class="text-3xl font-bold text-gray-900 dark:text-white mb-6">
                    {title}
                </h2>
                <p class="text-sm text-gray-500 dark:text-gray-400 mb-3">"Drag or swipe to browse photos"</p>

                <div class="flex gap-6 overflow-x-auto scroll-smooth snap-x snap-mandatory pb-4">
                    {images.into_iter().enumerate().map(|(i, src)| view! {
                        <div class="snap-center shrink-0 w-[85%] md:w-[70%] lg:w-[60%] rounded-2xl overflow-hidden shadow-xl">
                            <CarouselImage
                                local_src=src
                                alt=format!("Event photo {}", i + 1)
                                class="w-full h-[420px] object-cover"
                                eager=i == 0
                            />
                        </div>
                    }).collect::<Vec<_>>()}
                </div>
            </div>
        </section>
    }
}
