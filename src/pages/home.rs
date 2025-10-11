use leptos::prelude::*;

const JOIN_FORM: &str =
    "https://docs.google.com/forms/d/1G1wLFNxLb-dXbT75VViyZuVlFoghDj5zliyPZHtbHp8/edit";
const OFFICER_FORM: &str =
    "https://docs.google.com/forms/d/1blZ5sOtsFsjfakVdsHZ2YtnjJBr1J2_Hn3XmIXL6Vlc/edit";
const DECAL: &str = "/decal";

#[component]
pub fn HomePage() -> impl IntoView {
    view! {
        <div class="min-h-screen">
            <HeroSection />
            <AboutSection />
            <WhatWeDoSection />
            <UpcomingEventsSection />
        </div>
    }
}

#[component]
fn HeroSection() -> impl IntoView {
    view! {
        <section id="banner" class="relative min-h-screen flex items-center justify-center"
                 style="background-image: url('public/images/banner.jpeg'); background-size: cover; background-position: center;">
            <div class="absolute inset-0 bg-black/30"></div>
            <div class="relative z-10 text-center text-white max-w-4xl mx-auto px-6">
                <h1 class="text-5xl md:text-7xl font-bold mb-6">
                    "Poker at Berkeley"
                </h1>
                <p class="text-xl md:text-2xl mb-8">
                    "UC Berkeley's Premier Poker Organization"
                </p>
                <div class="flex flex-col sm:flex-row gap-4 justify-center">
                    <a href=JOIN_FORM class="bg-blue-600 hover:bg-blue-700 text-white font-semibold py-2 px-4 rounded-lg transition-colors">"Become a Member"</a>
                    <a href=OFFICER_FORM class="bg-blue-600 hover:bg-blue-700 text-white font-semibold py-2 px-4 rounded-lg transition-colors">"Become an Officer"</a>
                    <a href=DECAL class="bg-blue-600 hover:bg-blue-700 text-white font-semibold py-2 px-4 rounded-lg transition-colors">"Learn About DeCal"</a>
                </div>
            </div>
        </section>
    }
}

#[component]
fn AboutSection() -> impl IntoView {
    view! {
        <section class="py-20 bg-white dark:bg-gray-800">
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
        <section class="py-20 bg-gray-50 dark:bg-gray-900">
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
        <div class=format!("bg-white dark:bg-gray-800 rounded-lg shadow-lg p-8 hover:shadow-xl transition-shadow border-l-4 {}", border_color)>
            <h3 class="text-2xl font-bold text-gray-900 dark:text-white mb-3">{title}</h3>
            <p class="text-gray-600 dark:text-gray-300 leading-relaxed">{description}</p>
        </div>
    }
}

#[component]
fn UpcomingEventsSection() -> impl IntoView {
    view! {
        <section class="py-20 bg-white dark:bg-gray-800">
            <div class="max-w-6xl mx-auto px-6">
                <h2 class="text-4xl font-bold text-center text-gray-900 dark:text-white mb-12">
                    "Upcoming Events"
                </h2>

                <div class="grid grid-cols-1 lg:grid-cols-2 gap-12">
                    <div>
                        <h3 class="text-3xl font-bold text-gray-900 dark:text-white mb-8 text-center">
                            "Fall 2025"
                        </h3>
                        <div class="space-y-6">
                            <EventCard
                                title="Fall Tournament"
                                date="October 9-11"
                                details=vec![
                                    "250 person qualifying round (online) on October 9th",
                                    "70 person final day (in-person) on October 11th",
                                    "Special guest: Citadel",
                                ]
                                highlight=true
                            />
                        </div>
                    </div>

                    <div>
                        <h3 class="text-3xl font-bold text-gray-900 dark:text-white mb-8 text-center">
                            "Spring 2026"
                        </h3>
                        <div class="space-y-6">
                            <EventCard
                                title="3rd Annual Berkeley x Stanford Tournament"
                                date="Late April"
                                details=vec![
                                    "150 participants: 75 from Berkeley, 75 from Stanford",
                                    "Online qualifier, 2 full days of in-person play",
                                    "Mini career fairs both in-person days",
                                ]
                                highlight=true
                            />
                            <EventCard
                                title="Berkeley Poker Bots Competition"
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
