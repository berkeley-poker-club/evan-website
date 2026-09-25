use leptos::prelude::*;
use leptos_meta::Title;

#[component]
pub fn BlogPage() -> impl IntoView {
    view! {
        <Title text="Blog | Poker at Berkeley" />
        <div class="min-h-screen bg-[#F7F2EA]">
            <HeroBanner />
            <ComingSoonSection />
        </div>
    }
}

#[component]
fn HeroBanner() -> impl IntoView {
    view! {
        <section id="banner" class="relative py-32"
                 style="background-image: url('/public/images/blogbanner2.webp'); background-size: cover; background-position: center;">
            <div class="absolute inset-0" style="background: linear-gradient(180deg, rgba(38,32,26,0.55) 0%, rgba(38,32,26,0.25) 55%, #F7F2EA 100%);"></div>
            <div class="relative z-10 max-w-4xl mx-auto text-center px-6">
                <h1 class="text-5xl md:text-6xl font-bold text-white mb-5" style="text-shadow: 0 2px 12px rgba(0,0,0,0.5);">
                    "Blog"
                </h1>
                <span class="inline-block px-5 py-1.5 rounded-full text-sm tracking-widest uppercase text-[#F7F2EA] border border-[#E4D3B8]/70 bg-[#26201A]/30 backdrop-blur-sm">
                    "Coming Soon"
                </span>
            </div>
        </section>
    }
}

#[component]
fn ComingSoonSection() -> impl IntoView {
    view! {
        <section class="py-24 bg-[#F7F2EA]">
            <div class="max-w-2xl mx-auto px-6 text-center">
                <svg class="w-10 h-10 mx-auto mb-6 text-[#B08B5B]" fill="none" stroke="currentColor" stroke-width="1.5" viewBox="0 0 24 24">
                    <path stroke-linecap="round" stroke-linejoin="round" d="M12 6.75C10.5 5.25 8.25 4.5 6 4.5c-1.036 0-2.033.15-2.975.43A.75.75 0 002.25 5.7v12.15c0 .393.407.653.775.5A9.75 9.75 0 016 18c2.25 0 4.5.75 6 2.25m0-13.5c1.5-1.5 3.75-2.25 6-2.25 1.036 0 2.033.15 2.975.43a.75.75 0 01.775.57v12.15a.518.518 0 01-.775.5A9.75 9.75 0 0018 18c-2.25 0-4.5.75-6 2.25m0-13.5v13.5" />
                </svg>
                <h2 class="text-3xl md:text-4xl font-bold text-[#2E2620] mb-4">
                    "we're working on it"
                </h2>
                <p class="text-lg text-[#7A6A57] leading-relaxed mb-10">
                    "stories, strategy, and updates from poker@berkeley are on the way. check back soon."
                </p>
                <div class="flex items-center justify-center gap-3">
                    <span class="h-px w-12 bg-[#D9C7A8]"></span>
                    <span class="w-1.5 h-1.5 rounded-full bg-[#B08B5B]"></span>
                    <span class="h-px w-12 bg-[#D9C7A8]"></span>
                </div>
            </div>
        </section>
    }
}
