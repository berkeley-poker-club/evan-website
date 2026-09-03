use leptos::prelude::*;

#[component]
pub fn MediaCarousel(
    #[prop(optional)] title: Option<&'static str>,
    images: Vec<&'static str>,
) -> impl IntoView {
    view! {
        <section class="pt-12 pb-8 bg-white dark:bg-gray-900">

            <div class="max-w-7xl mx-auto px-6">
                <h2 class="text-3xl font-bold text-gray-900 dark:text-white mb-6">
                    {title}
                </h2>
                <p class="text-sm text-gray-500 dark:text-gray-400 mb-3">"Drag or swipe to browse photos"</p>

                <div class="flex gap-6 overflow-x-auto scroll-smooth snap-x snap-mandatory pb-4">
                    {images.into_iter().enumerate().map(|(i, src)| view! {
                        <div class="snap-center shrink-0 w-[85%] md:w-[70%] lg:w-[60%] rounded-2xl overflow-hidden shadow-xl">
                            <img
                                src=src
                                alt=format!("Event photo {}", i + 1)
                                class="w-full h-[420px] object-cover"
                                loading="lazy"
                            />
                        </div>
                    }).collect::<Vec<_>>()}
                </div>
            </div>
        </section>
    }
}
