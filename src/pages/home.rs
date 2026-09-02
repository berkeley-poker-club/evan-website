use leptos::prelude::*;
use crate::components::MediaCarousel;
use gloo_timers::future::TimeoutFuture;
use rand::Rng;


const DECAL: &str = "/decal";
const JOIN_FORM: &str = "https://forms.gle/yVsAAJ5PLBtrgWUx8";
const STANFORD_JOIN_FORM: &str = "https://forms.gle/iX7oCxR32DdWNAn16";


#[component]
pub fn HomePage() -> impl IntoView {
    view! {
        <div class="min-h-screen bg-stone-100 dark:bg-gray-900">
            <style>
                "@keyframes pageLoadOverlayFade {
                    0% { opacity: 1; }
                    60% { opacity: 1; }
                    100% { opacity: 0; visibility: hidden; }
                }"
            </style>
            <div style="position: fixed; top: 0; left: 0; width: 100vw; height: 100vh; background-color: #000; z-index: 9999; pointer-events: none; animation: pageLoadOverlayFade 1s ease-out forwards;"></div>
            <HeroSection />
            <AboutSection />

            <MediaCarousel
                images=vec![
                    "public/images/stanfxcal25/DSCF1288.webp",
                    "public/images/falltourney/DSCF4016.webp",
                    "public/images/qrt-tourney/DSCF4250.webp",
                    "public/images/falltourney/DSCF2022.webp",
                    "public/images/stanfxcal26/Q62A1176.webp",
                    "public/images/qrt-tourney/DSCF4295.webp",
                    "public/images/stanfxcal25/DSCF0839.webp",
                    "public/images/qrt-tourney/DSCF4301.webp",
                    "public/images/stanfxcal25/DSCF0964.webp",
                    "public/images/stanfxcal26/Q62A1192.webp",
                    "public/images/qrt-tourney/DSCF4304.webp",
                    "public/images/falltourney/DSCF2937.webp",
                    "public/images/stanfxcal25/DSCF1494.webp",
                    "public/images/qrt-tourney/DSCF4400.webp",
                    "public/images/stanfxcal26/Q62A1237.webp",
                    "public/images/stanfxcal25/DSCF1495.webp",
                    "public/images/qrt-tourney/DSCF4323.webp",
                    "public/images/stanfxcal25/DSCF1505.webp",
                    "public/images/qrt-tourney/DSCF4390.webp",
                    "public/images/stanfxcal26/Q62A1261.webp",
                    "public/images/falltourney/DSCF2753.webp",
                    "public/images/qrt-tourney/DSCF4402.webp",
                    "public/images/falltourney/DSCF2756.webp",
                    "public/images/stanfxcal26/Q62A1257.webp",
                    "public/images/qrt-tourney/DSCF4404.webp",
                    "public/images/stanfxcal25/DSCF0929.webp",
                    "public/images/officergroup/DSCF4009.webp",
                    "public/images/qrt-tourney/DSCF4436.webp",
                    "public/images/stanfxcal26/Q62A1275.webp",
                    "public/images/qrt-tourney/DSCF4471.webp",
                    "public/images/stanfxcal25/DSCF1092.webp",
                    "public/images/stanfxcal25/DSCF1341.webp",
                    "public/images/qrt-tourney/DSCF4293.webp",
                    "public/images/stanfxcal25/DSCF1412.webp",
                    "public/images/stanfxcal25/DSCF0978.webp",
                ]
            />
            <WhatWeDoSection />
            <UpcomingEventsSection />
        </div>
    }
}

#[component]
fn HeroSection() -> impl IntoView {
    view! {
        <section id="banner" class="relative min-h-screen flex items-center justify-center"
                 style="background-image: url('public/images/sp26board/finalhearstpano-Edit copy 2.webp'); background-size: cover; background-position: center 80%;">
            <style>
                "@keyframes heroFadeInUp {
                    from { opacity: 0; transform: translateY(24px); }
                    to { opacity: 1; transform: translateY(0); }
                }"
            </style>
            <div class="absolute inset-0" style="background-color: rgba(0, 0, 0, 0.30);"></div>
            <div class="relative z-10 text-center text-white max-w-6xl mx-auto px-6 mb-32" style="transform: translateY(15%);">
                <div class="max-w-3xl mx-auto px-6">
                    <h1 class="mb-2" style="opacity: 0; animation: heroFadeInUp 0.8s ease-out 1s forwards;">
                        <img
                            src="public/images/banner-logo.webp"
                            alt="poker at berkeley"
                            class="mx-auto w-[min(320px,55vw)]"
                        />
                    </h1>
                    <TypewriterLine
                        prefix="We "
                        phrases=vec![
                            ("are", "the largest collegiate poker club in the country"),
                            ("are", "the only club spanning Berkeley and Stanford"),
                            ("are", "home to Stat 198: Intro to Poker, running since 2003"),
                            ("are", "a university poker org with over 40 years of history"),
                            ("run", "Friday night game nights"),
                            ("run", "the largest DeCal course at UC Berkeley"),
                            ("run", "the flagship Stanford-Berkeley poker tournament"),
                            ("run", "PokerBots, a computerized poker tournament"),
                            ("run", "the P@B Poker Livestream using our custom RFID table"),
                        ]
                        delay_ms=1500
                        margin_class="mb-8"
                    />
                </div>
            </div>
            <div class="absolute bottom-20 left-0 z-10 flex w-full justify-center">
                <div class="animate-bounce" style="animation-duration: 2.6s;">
                    <svg class="w-10 h-10 text-white/60" fill="none" stroke="currentColor" viewBox="0 0 24 24" aria-hidden="true">
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 14l-7 7m0 0l-7-7m7 7V3"></path>
                    </svg>
                </div>
            </div>
        </section>
    }
}

#[component]
fn TypewriterLine(
    prefix: &'static str,
    phrases: Vec<(&'static str, &'static str)>,
    delay_ms: u32,
    margin_class: &'static str,
) -> impl IntoView {
    let (typed, set_typed) = signal(String::new());

    wasm_bindgen_futures::spawn_local(async move {
        TimeoutFuture::new(delay_ms).await;

        let mut phrase_idx = 0usize;
        let mut current_verb: &'static str = "";
        loop {
            let (verb, rest) = phrases[phrase_idx % phrases.len()];

            // Only erase/retype the verb ("are"/"run") when it actually changes.
            if verb != current_verb {
                if !current_verb.is_empty() {
                    let old_verb_prefix: Vec<char> = format!("{} ", current_verb).chars().collect();
                    for i in (0..old_verb_prefix.len()).rev() {
                        set_typed.set(old_verb_prefix[..i].iter().collect());
                        TimeoutFuture::new(rand::thread_rng().gen_range(15..40)).await;
                    }
                    TimeoutFuture::new(150).await;
                }

                let new_verb_prefix: Vec<char> = format!("{} ", verb).chars().collect();
                for i in 1..=new_verb_prefix.len() {
                    set_typed.set(new_verb_prefix[..i].iter().collect());
                    TimeoutFuture::new(rand::thread_rng().gen_range(30..80)).await;
                }
                current_verb = verb;
            }

            let verb_prefix = format!("{} ", verb);
            let rest_chars: Vec<char> = rest.chars().collect();

            for i in 1..=rest_chars.len() {
                let typed_rest: String = rest_chars[..i].iter().collect();
                set_typed.set(format!("{}{}", verb_prefix, typed_rest));
                TimeoutFuture::new(rand::thread_rng().gen_range(30..80)).await;
            }

            TimeoutFuture::new(1600).await;

            for i in (0..rest_chars.len()).rev() {
                let typed_rest: String = rest_chars[..i].iter().collect();
                set_typed.set(format!("{}{}", verb_prefix, typed_rest));
                TimeoutFuture::new(rand::thread_rng().gen_range(15..40)).await;
            }

            TimeoutFuture::new(300).await;
            phrase_idx += 1;
        }
    });

    view! {
        <style>
            "@import url('https://fonts.googleapis.com/css2?family=Roboto+Mono:ital,wght@0,400;0,700;1,400;1,700&display=swap');
            @keyframes heroCursorBlink {
                0%, 50% { opacity: 1; }
                50.01%, 100% { opacity: 0; }
            }"
        </style>
        <p
            class=format!("text-xl md:text-2xl text-white text-center {}", margin_class)
            style=format!("opacity: 0; animation: heroFadeInUp 0.8s ease-out {}ms forwards; font-family: 'Roboto Mono', monospace;", delay_ms)
        >
            {prefix}
            {move || typed.get()}
            <span
                class="ml-1"
                style="color: #F5C842; font-weight: bold; animation: heroCursorBlink 0.7s step-end infinite;"
            >
                "|"
            </span>
        </p>
    }
}

#[component]
fn AboutSection() -> impl IntoView {
    view! {
        <section class="py-20 dark:bg-gray-800">
            <div class="max-w-4xl mx-auto px-6">
                <div class="flex flex-col sm:flex-row gap-4 justify-center mb-6">
                    <a
                        href="https://discord.gg/SbS9UbZW2a"
                    class="inline-flex items-center justify-center text-white font-semibold py-3 px-8 rounded-lg text-lg transition-colors"
                    style="background-color: #536682;"
                    >
                        "Join Our Discord"
                    </a>
                <a href=JOIN_FORM target="_blank" rel="noopener" class="inline-flex items-center justify-center bg-slate-400 hover:bg-slate-500 text-white font-semibold py-3 px-8 rounded-lg text-lg transition-colors">"Become a Member"</a>
                </div>
                <div class="flex flex-col sm:flex-row gap-4 justify-center mb-10">
                    <a href=DECAL class="bg-[#A87454] hover:bg-[#9A6A4C] font-semibold py-2 px-4 rounded-lg transition-colors">"DeCal"</a>
                    <a href=STANFORD_JOIN_FORM class="bg-[#8E3E3B] hover:bg-[#7F3835] text-white font-semibold py-2 px-4 rounded-lg transition-colors"> "Stanford Students"</a>
                </div>
                <h2 class="text-4xl font-bold text-gray-900 dark:text-white mb-6 text-center">
                    "About Poker at Berkeley"
                </h2>
                <p class="text-lg text-gray-700 dark:text-gray-300 leading-relaxed mb-6">
                    "We are UC Berkeley's premier poker organization, dedicated to fostering a community of strategic thinkers and skilled players. Our club provides opportunities for students to learn, compete, and network in a professional environment."
                </p>
                <p class="text-lg text-gray-700 dark:text-gray-300 leading-relaxed">
                    "Through weekly game nights, our renowned DeCal course, and major tournaments, we help members develop critical thinking skills that extend far beyond the poker table."
                </p>
            </div>
        </section>
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
                        description="Our flagship events include an annual Fall tournament as well as a tournament with Stanford in the Spring. Prize pools can be up to $10,000 and tournaments are free to join."
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
                        link="/join#game-nights"
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
) -> impl IntoView {
    let border_color = match color {
        "blue" => "border-blue-600",
        "green" => "border-green-600",
        "purple" => "border-purple-600",
        _ => "border-gray-600",
    };

    view! {
        <a
            href=link
            class=format!(
                "block bg-white dark:bg-slate-800/90 ring-1 ring-black/5 dark:ring-white/10 rounded-lg shadow-lg p-8 hover:shadow-xl hover:brightness-105 dark:hover:brightness-125 transition-all border-l-4 {}",border_color)>
            <h3 class="text-2xl font-bold text-gray-900 dark:text-white mb-3">{title}</h3>
            <p class="text-gray-600 dark:text-gray-300 leading-relaxed">{description}</p>
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
                            title="4th Berkeley x Stanford Tournament"
                            date="Fall 2026"
                            details=vec![
                                "150 participants: 75 from Berkeley, 75 from Stanford",
                                "Online qualifier, 2 full days of in-person play",
                                "Mini career fairs both in-person days",
                            ]
                            highlight=true
                        />
                        <EventCard
                            title="Berkeley Poker Bots Competition"
                            date="January–March"
                            details=vec![
                                "More details coming soon"
                            ]
                            highlight=false
                        />
                        <EventCard
                            title="5th Berkeley x Stanford Tournament"
                            date="TBD"
                            details=vec![
                                "More details coming soon"
                            ]
                            highlight=false
                        />
                        <EventCard
                            title="Jump Trading Mini-Tournament"
                            date="TBD"
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
        "border-l-4 border-l-blue-600"
    } else {
        "border-l-4 border-l-gray-300 dark:border-l-gray-600"
    };

    view! {
        <div class=format!("bg-gray-50 dark:bg-gray-700 rounded-lg p-6 {}", border_class)>
            <div class="mb-4">
                <h4 class="text-xl font-bold text-gray-900 dark:text-white mb-2">{title}</h4>
                <p class="text-blue-600 dark:text-blue-400 font-semibold">{date}</p>
            </div>
            <ul class="space-y-2">
                {details.into_iter().map(|detail| {
                    view! {
                        <li class="flex items-start space-x-2">
                            <div class="w-2 h-2 bg-blue-600 dark:bg-blue-400 rounded-full mt-2 flex-shrink-0"></div>
                            <span class="text-gray-700 dark:text-gray-300 text-sm">{detail}</span>
                        </li>
                    }
                }).collect::<Vec<_>>()}
            </ul>
        </div>
    }
}
