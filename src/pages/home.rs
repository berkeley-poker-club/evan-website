use leptos::prelude::*;
use crate::components::MediaCarousel;


const DECAL: &str = "/decal";
const JOIN_FORM: &str =
    "https://docs.google.com/forms/d/e/1FAIpQLSdxzvFVWmAr78rsoMCtL-yaQafVlElf3plTJhg7cEHNfUlq8Q/viewform?usp=dialog";
const STANFORD_JOIN_FORM: &str =
    "https://docs.google.com/forms/d/e/1FAIpQLSeMmF5-hdHQg8l-6DVjcQh7mwDMGapFE2DAfSMGnCTX9MgnAg/viewform?usp=dialog";


#[component]
pub fn HomePage() -> impl IntoView {
    view! {
        <div class="min-h-screen bg-stone-100 dark:bg-gray-900">
            <HeroSection />
            <AboutSection />

            <MediaCarousel
                images=vec![
                    "public/images/stanfxcal25/DSCF1288.jpg",
                    "public/images/falltourney/DSCF4016.jpg",
                    "public/images/falltourney/DSCF2022.jpg",
                    "public/images/stanfxcal25/DSCF0839.jpg",
                    "public/images/stanfxcal25/DSCF0964.jpg",
                    "public/images/falltourney/DSCF2937.jpg",
                    "public/images/stanfxcal25/DSCF1494.jpg",
                    "public/images/stanfxcal25/DSCF1495.jpg",
                    "public/images/stanfxcal25/DSCF1505.jpg",
                    "public/images/falltourney/DSCF2753.jpg",
                    "public/images/falltourney/DSCF2756.jpg",
                    "public/images/stanfxcal25/DSCF0929.jpg",
                    "public/images/officergroup/DSCF4009.jpg",
                    "public/images/stanfxcal25/DSCF1092.jpg",                    "public/images/stanfxcal25/DSCF1341.jpg",
                    "public/images/stanfxcal25/DSCF1412.jpg",
                    "public/images/stanfxcal25/DSCF0978.jpg",
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
                 style="background-image: url('public/images/stanfxcal25/DSCF0844.jpg'); background-size: cover; background-position: center;">
            <div class="absolute inset-0" style="background-color: rgba(0, 0, 0, 0.30);"></div>
            <div class="relative z-10 text-center text-white max-w-6xl mx-auto px-6">
                <div class="w-full max-w-6xl mx-auto bg-black/10 backdrop-blur-sm rounded-2xl px-16 py-16 md:px-20 md:py-16 lg:px-24 lg:py-20 shadow-xl">
                    <div class="max-w-3xl mx-auto px-6">
                        <h1 class="mb-6">
                            <img
                                src="public/images/banner-logo.png"
                                alt="poker at berkeley"
                                class="mx-auto w-full max-w-[420px] sm:max-w-[460px] md:max-w-[520px]"
                            />
                        </h1>
                        <p class="text-xl md:text-2xl mb-8">
                            "UC Berkeley's Premier Poker Organization"
                        </p>
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
                        <div class="flex flex-col sm:flex-row gap-4 justify-center">
                            <a href=DECAL class="bg-[#A87454] hover:bg-[#9A6A4C] font-semibold py-2 px-4 rounded-lg transition-colors">"DeCal"</a>
                            <a href=STANFORD_JOIN_FORM class="bg-[#8E3E3B] hover:bg-[#7F3835] text-white font-semibold py-2 px-4 rounded-lg transition-colors"> "Stanford Students"</a>
                        </div>
                    </div>
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
fn AboutSection() -> impl IntoView {
    view! {
        <section class="py-20 dark:bg-gray-800">
            <div class="max-w-4xl mx-auto px-6">
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
                    />
                    <WhatWeDoCard
                        title="The Poker Decal"
                        description="Stat 198: Introduction to Poker - an official university course covering poker theory, strategy, and mathematical concepts."
                        color="blue"
                    />
                    <WhatWeDoCard
                        title="Career Development"
                        description="Career opportunities for Berkeley students passionate about math, probability, quantitative research, and trading."
                        color="blue"
                    />
                    <WhatWeDoCard
                        title="Special Projects"
                        description="We are working on building out a suite of solvers and open source tools for Berkeley students to use."
                        color="blue"
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
) -> impl IntoView {
    let border_color = match color {
        "blue" => "border-blue-600",
        "green" => "border-green-600",
        "purple" => "border-purple-600",
        _ => "border-gray-600",
    };

    view! {
        <div class=format!(
            "bg-white dark:bg-slate-800/90 ring-1 ring-black/5 dark:ring-white/10 rounded-lg shadow-lg p-8 hover:shadow-xl transition-shadow border-l-4 {}",border_color)>
            <h3 class="text-2xl font-bold text-gray-900 dark:text-white mb-3">{title}</h3>
            <p class="text-gray-600 dark:text-gray-300 leading-relaxed">{description}</p>
        </div>
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
