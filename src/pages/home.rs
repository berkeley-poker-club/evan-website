use leptos::prelude::*;
use leptos_meta::Title;
use crate::components::{MediaCarousel, OptimizedImage};
use wasm_bindgen::JsCast;


const DECAL: &str = "/decal";
const JOIN_FORM: &str = "https://forms.gle/yVsAAJ5PLBtrgWUx8";
const STANFORD_JOIN_FORM: &str = "https://forms.gle/iX7oCxR32DdWNAn16";


#[component]
pub fn HomePage() -> impl IntoView {
    view! {
        <Title text="Poker at Berkeley" />
        <div class="min-h-screen bg-[#c8d1d9] dark:bg-gray-900">
            <style>
                "@keyframes pageLoadOverlayFade {
                    0% { opacity: 1; }
                    60% { opacity: 1; }
                    100% { opacity: 0; visibility: hidden; }
                }"
            </style>
            <HeroSection />
            <AboutSection />

            <MediaCarousel
                images=vec![
                    "/public/images/stanfxcal26/Q62A128-7.webp",
                    "/public/images/stanfxcal25/DSCF1288.webp",
                    "/public/images/stanfxcal26/Q62A1257.webp",
                    "/public/images/falltourney/DSCF4016.webp",
                    "/public/images/stanfxcal26/Q62A128-4.webp",
                    "/public/images/qrt-tourney/DSCF4250.webp",
                    "/public/images/falltourney/DSCF2022.webp",
                    "/public/images/stanfxcal26/Q62A1176.webp",
                    "/public/images/qrt-tourney/DSCF4295.webp",
                    "/public/images/stanfxcal25/DSCF0839.webp",
                    "/public/images/qrt-tourney/DSCF4301.webp",
                    "/public/images/stanfxcal25/DSCF0964.webp",
                    "/public/images/stanfxcal26/Q62A1192.webp",
                    "/public/images/qrt-tourney/DSCF4304.webp",
                    "/public/images/falltourney/DSCF2937.webp",
                    "/public/images/stanfxcal25/DSCF1494.webp",
                    "/public/images/qrt-tourney/DSCF4400.webp",
                    "/public/images/stanfxcal26/Q62A1237.webp",
                    "/public/images/stanfxcal25/DSCF1495.webp",
                    "/public/images/qrt-tourney/DSCF4323.webp",
                    "/public/images/stanfxcal25/DSCF1505.webp",
                    "/public/images/qrt-tourney/DSCF4390.webp",
                    "/public/images/stanfxcal26/Q62A1261.webp",
                    "/public/images/falltourney/DSCF2753.webp",
                    "/public/images/qrt-tourney/DSCF4402.webp",
                    "/public/images/falltourney/DSCF2756.webp",
                    "/public/images/qrt-tourney/DSCF4404.webp",
                    "/public/images/stanfxcal25/DSCF0929.webp",
                    "/public/images/officergroup/DSCF4009.webp",
                    "/public/images/qrt-tourney/DSCF4436.webp",
                    "/public/images/stanfxcal26/Q62A1275.webp",
                    "/public/images/qrt-tourney/DSCF4471.webp",
                    "/public/images/stanfxcal25/DSCF1092.webp",
                    "/public/images/stanfxcal25/DSCF1341.webp",
                    "/public/images/qrt-tourney/DSCF4293.webp",
                    "/public/images/stanfxcal25/DSCF1412.webp",
                    "/public/images/stanfxcal25/DSCF0978.webp",
                ]
            />
            <WhatWeDoSection />
            <UpcomingEventsSection />
        </div>
    }
}

/// Picks the banner video for this visitor, or `None` to keep just the photo
/// (reduced motion or data saver turned on).
fn hero_video_src() -> Option<&'static str> {
    let window = web_sys::window()?;
    let matches = |query: &str| {
        window
            .match_media(query)
            .ok()
            .flatten()
            .is_some_and(|list| list.matches())
    };
    let get = |target: &wasm_bindgen::JsValue, key: &str| {
        js_sys::Reflect::get(target, &key.into()).ok()
    };
    let save_data = get(window.as_ref(), "navigator")
        .and_then(|navigator| get(&navigator, "connection"))
        .filter(|connection| connection.is_object())
        .and_then(|connection| get(&connection, "saveData"))
        .and_then(|value| value.as_bool())
        .unwrap_or(false);

    if save_data || matches("(prefers-reduced-motion: reduce)") {
        None
    } else if matches("(max-width: 768px)") {
        Some("/public/videos/hero-960.mp4")
    } else {
        Some("/public/videos/hero-1920.mp4")
    }
}

#[component]
fn HeroSection() -> impl IntoView {
    // The photo stays as the section background; the video only fades in once it
    // is actually playing, so a slow or failed video just leaves the photo showing.
    let (video_playing, set_video_playing) = signal(false);

    view! {
        <section id="banner" class="relative flex items-center justify-center"
                 style="height: 60vh; background-image: url('/public/images/DSC01107.webp'); background-size: cover; background-position: center 90%;">
            <style>
                "@keyframes heroFadeInUp {
                    from { opacity: 0; transform: translateY(24px); }
                    to { opacity: 1; transform: translateY(0); }
                }"
            </style>
            {hero_video_src().map(|src| view! {
                <video
                    src=src
                    autoplay=true
                    loop=true
                    playsinline=true
                    prop:muted=true
                    aria-hidden="true"
                    class="absolute inset-0"
                    style="width: 100%; height: 100%; object-fit: cover; transition: opacity 0.6s ease-out;"
                    style:opacity=move || if video_playing.get() { "1" } else { "0" }
                    on:playing=move |_| set_video_playing.set(true)
                ></video>
            })}
            <div class="absolute inset-0" style="background-color: rgba(0, 0, 0, 0.30);"></div>
            <div class="absolute inset-0" style="z-index: 5; background-color: #000; pointer-events: none; animation: pageLoadOverlayFade 1s ease-out forwards;"></div>
            <div class="absolute bottom-0 inset-x-0 h-32 z-10 pointer-events-none block dark:hidden" style="background: linear-gradient(to bottom, transparent 0%, #c8d1d9 65%, #c8d1d9 100%);"></div>
            <div class="absolute bottom-0 inset-x-0 h-32 z-10 pointer-events-none hidden dark:block" style="background: linear-gradient(to bottom, transparent 0%, #1f2937 65%, #1f2937 100%);"></div>
        </section>
    }
}

/// Hands the chip image to `public/js/chip-spin.js`, which lays the spinning 3D chip over it
/// when it can. If the script isn't there or can't run, nothing happens and the image stays.
fn mount_chip_spin(img: &web_sys::HtmlImageElement) {
    let Some(window) = web_sys::window() else { return };
    let mount = js_sys::Reflect::get(window.as_ref(), &"PabChipSpin".into())
        .ok()
        .filter(|spin| spin.is_object())
        .and_then(|spin| js_sys::Reflect::get(&spin, &"mount".into()).ok())
        .and_then(|mount| mount.dyn_into::<js_sys::Function>().ok());
    if let Some(mount) = mount {
        let _ = mount.call1(&wasm_bindgen::JsValue::NULL, img);
    }
}

#[component]
fn AboutSection() -> impl IntoView {
    let chip_ref = NodeRef::<leptos::html::Img>::new();
    Effect::new(move |_| {
        if let Some(img) = chip_ref.get() {
            mount_chip_spin(&img);
        }
    });

    view! {
        <section class="pt-2 pb-20 dark:bg-gray-800">
            <div class="max-w-4xl mx-auto px-6">
                <div class="text-center mb-8">
                    <h1 class="mb-2" style="opacity: 0; animation: heroFadeInUp 0.8s ease-out 1s forwards;">
                        // The wrapper gives the 3D canvas a box to sit in, exactly over the image.
                        <span style="position: relative; display: inline-block;">
                            <img
                                node_ref=chip_ref
                                src="/public/images/cal-x-stanford.png"
                                alt="poker at berkeley"
                                class="mx-auto"
                                style="width: min(200px, 45vw);"
                                data-chip-spin=""
                            />
                        </span>
                    </h1>
                    <p class="text-xl md:text-2xl text-gray-900 dark:text-white text-center">
                        "The largest collegiate poker organization in the country."
                    </p>
                </div>
                <div class="flex flex-col sm:flex-row gap-4 justify-center mb-6">
                    <a
                        href="https://discord.gg/SbS9UbZW2a"
                    class="flex w-full sm:w-auto items-center justify-center text-center text-white font-semibold py-3 px-8 rounded-lg text-lg leading-tight transition-colors"
                    style="background-color: #536682;"
                    >
                        "Join Our Discord"
                    </a>
                <a href=JOIN_FORM target="_blank" rel="noopener" class="flex w-full sm:w-auto items-center justify-center text-center bg-slate-400 hover:bg-slate-500 text-white font-semibold py-3 px-8 rounded-lg text-lg leading-tight transition-colors">"Become a Member"</a>
                </div>
                <div class="flex flex-col sm:flex-row gap-4 justify-center mb-10">
                    <a href=DECAL class="flex w-full sm:w-auto items-center justify-center text-center bg-[#A87454] hover:bg-[#9A6A4C] text-white font-semibold py-2 px-4 rounded-lg leading-tight transition-colors">"DeCal"</a>
                    <a href=STANFORD_JOIN_FORM class="flex w-full sm:w-auto items-center justify-center text-center bg-[#8E3E3B] hover:bg-[#7F3835] text-white font-semibold py-2 px-4 rounded-lg leading-tight transition-colors"> "Stanford Students"</a>
                </div>
                <h2 class="text-4xl font-bold text-gray-900 dark:text-white mb-6 text-center">
                    "About Poker at Berkeley"
                </h2>
                <p class="text-lg text-gray-700 dark:text-gray-300 leading-relaxed mb-6">
                    "We are UC Berkeley's and Stanford's premier poker organization and the only club on campus that operates across both Berkeley and Stanford. We're dedicated to fostering a community of strategic thinkers and skilled players, with opportunities to learn, compete, and network in a professional environment."
                </p>
                <p class="text-lg text-gray-700 dark:text-gray-300 leading-relaxed mb-10">
                    "Through weekly game nights, our renowned DeCal course, and major tournaments, we help members develop critical thinking skills that extend far beyond the poker table."
                </p>
                <div class="grid grid-cols-1 sm:grid-cols-2 gap-6">
                    <StatCard number="1200+" label="Active members across Berkeley and Stanford" />
                    <StatCard number="2003" label="Established" />
                </div>
            </div>
        </section>
    }
}

#[component]
fn StatCard(number: &'static str, label: &'static str) -> impl IntoView {
    view! {
        <div class="bg-[#dde3e8] dark:bg-slate-800/90 ring-1 ring-black/5 dark:ring-white/10 rounded-lg shadow-lg p-6 text-center">
            <div class="text-3xl font-bold text-gray-900 dark:text-white mb-1">{number}</div>
            <div class="text-gray-600 dark:text-gray-300 text-sm">{label}</div>
        </div>
    }
}


#[component]
fn JoinUsSection() -> impl IntoView {
    view! {
        <section class="py-20 bg-gray-900">
            <div class="max-w-4xl mx-auto text-center px-6">
                <h2 class="text-4xl font-bold text-white mb-6">
                    "Ready to Join?"
                </h2>
                <p class="text-xl text-gray-300 mb-4">
                    "Become part of UC Berkeley's most strategic community."
                </p>
                <p class="text-lg text-blue-300 mb-8 font-semibold">
                    "Poker at Berkeley is Open to All"
                </p>
                <a href="/contact" class="btn-primary text-lg">"Join Now"</a>
            </div>
        </section>
    }
}

#[component]
fn WhatWeDoSection() -> impl IntoView {
    view! {
        <section class="py-20 dark:bg-gray-900">
            <div class="max-w-6xl mx-auto px-6">
                <h2 class="text-4xl font-bold text-center text-gray-900 dark:text-white mb-12">
                    "What We Do"
                </h2>
                <div class="space-y-8">
                    <WhatWeDoCard
                        title="Tournaments"
                        description="Our flagship Berkeley x Stanford tournament runs every semester with a $10,000 prize pool. All tournaments are free to join."
                        color="blue"
                        link="/tournaments"
                    />
                    <WhatWeDoCard
                        title="The Poker Decal"
                        description="Stat 198: Introduction to Poker - an official university course covering poker theory, strategy, and mathematical concepts."
                        color="blue"
                        link="/decal"
                    />
                    <WhatWeDoCard
                        title="Career Development"
                        description="Career opportunities for Berkeley students passionate about math, probability, quantitative research, and trading."
                        color="blue"
                        link="/people"
                    />
                    <WhatWeDoCard
                        title="Special Projects"
                        description="We are working on building out a suite of solvers and open source tools for Berkeley students to use."
                        color="blue"
                        link="/pokerbots"
                    />
                    <WhatWeDoCard
                        title="Game Nights"
                        description="We host game nights every Friday, open to all P@B members. All stakes welcome — come play, meet the community, and run it up."
                        color="blue"
                        link="/game-nights"
                    />
                    <WhatWeDoCard
                        title="Stream Game"
                        description="Through a custom RFID-enabled table from BBO Poker Tables and a production partnership with PokerGFX, we broadcast live poker to our audience — covering game nights, stream games, and our tournaments."
                        color="blue"
                        link="/stream-game"
                        image="/public/images/streamgame_snapshot.png"
                    />
                </div>
            </div>
        </section>
    }
}

#[component]
fn WhatWeDoCard(
    title: &'static str,
    description: &'static str,
    color: &'static str,
    link: &'static str,
    #[prop(optional)] image: Option<&'static str>,
) -> impl IntoView {
    let border_color = match color {
        "blue" => "border-[#2a5189] dark:border-blue-600",
        "green" => "border-green-600",
        "purple" => "border-purple-600",
        _ => "border-gray-600",
    };

    view! {
        <a
            href=link
            class=format!(
                "block bg-[#dde3e8] dark:bg-slate-800/90 ring-1 ring-black/5 dark:ring-white/10 rounded-lg shadow-lg p-8 hover:shadow-xl hover:brightness-105 dark:hover:brightness-125 transition-all border-l-4 {}",border_color)>
            <h3 class="text-2xl font-bold text-gray-900 dark:text-white mb-3">{title}</h3>
            <p class="text-gray-600 dark:text-gray-300 leading-relaxed">{description}</p>
            {image.map(|src| view! {
                <OptimizedImage
                    src=src
                    alt=title
                    class="mt-6 w-full max-w-xl rounded-lg shadow-md"
                    loading="lazy"
                />
            })}
        </a>
    }
}

#[component]
fn UpcomingEventsSection() -> impl IntoView {
    view! {
        <section class="py-20">
            <div class="max-w-6xl mx-auto px-6">
                <h2 class="text-4xl font-bold text-center text-gray-900 dark:text-white mb-12">
                    "Upcoming Events"
                </h2>

                <div class="max-w-2xl mx-auto">
                    <h3 class="text-3xl font-bold text-gray-900 dark:text-white mb-8 text-center">
                        "2026–2027"
                    </h3>
                    <div class="space-y-6">
                        <EventCard
                            title="Berkeley Poker Bots Competition"
                            date="January–March"
                            details=vec![
                                "More details coming soon"
                            ]
                            highlight=true
                        />
                        <EventCard
                            title="5th Berkeley x Stanford Tournament"
                            date="Spring 2027"
                            details=vec![
                                "More details coming soon"
                            ]
                            highlight=false
                        />
                        <EventCard
                            title="Jump Trading Mini-Tournament"
                            date="Spring 2027"
                            details=vec![
                                "50 people",
                                "1 day in-person",
                                "Guest speakers from Jump Trading"
                            ]
                            highlight=false
                        />
                    </div>
                </div>
            </div>
        </section>
    }
}

#[component]
fn EventCard(
    title: &'static str,
    date: &'static str,
    details: Vec<&'static str>,
    highlight: bool,
) -> impl IntoView {
    let border_class = if highlight {
        "border-l-4 border-l-[#2a5189] dark:border-l-blue-600"
    } else {
        "border-l-4 border-l-gray-300 dark:border-l-gray-600"
    };

    view! {
        <div class=format!("bg-[#d6dde3] dark:bg-gray-700 rounded-lg p-6 {}", border_class)>
            <div class="mb-4">
                <h4 class="text-xl font-bold text-gray-900 dark:text-white mb-2">{title}</h4>
                <p class="text-[#2a5189] dark:text-blue-400 font-semibold">{date}</p>
            </div>
            <ul class="space-y-2">
                {details.into_iter().map(|detail| {
                    view! {
                        <li class="flex items-start space-x-2">
                            <div class="w-2 h-2 bg-[#2a5189] dark:bg-blue-400 rounded-full mt-2 flex-shrink-0"></div>
                            <span class="text-gray-700 dark:text-gray-300 text-sm">{detail}</span>
                        </li>
                    }
                }).collect::<Vec<_>>()}
            </ul>
        </div>
    }
}
