use leptos::prelude::*;

const STANFORD_IMAGES: &[&str] = &[
    "public/images/stanfxcal25/DSCF1288.jpg",
    "public/images/stanfxcal25/DSCF0839.jpg",
    "public/images/stanfxcal25/DSCF0932.jpg",
    "public/images/stanfxcal25/DSCF0964.jpg",
    "public/images/stanfxcal25/DSCF0929.jpg",
    "public/images/stanfxcal25/DSCF0865.jpg",
    "public/images/stanfxcal25/DSCF0551.jpg",
    "public/images/stanfxcal25/DSCF0698.jpg",
    "public/images/stanfxcal25/DSCF0978.jpg",
    "public/images/stanfxcal25/DSCF1412.jpg",
    "public/images/stanfxcal25/DSCF1493.jpg",
    "public/images/stanfxcal25/DSCF1495.jpg",
    "public/images/stanfxcal25/DSCF1504.jpg",
    "public/images/stanfxcal25/DSCF1505.jpg",
    "public/images/stanfxcal25/DSCF1550.jpg",
    "public/images/stanfxcal25/DSCF1180.jpg",
    "public/images/stanfxcal25/DSCF1152.jpg",
    "public/images/stanfxcal25/DSCF1139.jpg",
    "public/images/stanfxcal25/DSCF1091.jpg",
    "public/images/stanfxcal25/DSCF1070.jpg",
    "public/images/stanfxcal25/DSCF1011.jpg",
    "public/images/stanfxcal25/DSCF0944.jpg",
    "public/images/stanfxcal25/DSCF0769.jpg",
    "public/images/stanfxcal25/DSCF0640.jpg",
    "public/images/stanfxcal25/DSCF0435.jpg",
    "public/images/stanfxcal25/DSCF1257.jpg",
    "public/images/stanfxcal25/DSCF1092.jpg",
    "public/images/stanfxcal25/DSCF1018.jpg",
    "public/images/stanfxcal25/DSCF0939.jpg",
    "public/images/stanfxcal25/DSCF0852.jpg",
    "public/images/stanfxcal25/DSCF0844.jpg",
];

const FALL_TOURNEY_IMAGES: &[&str] = &[
    "public/images/falltourney/DSCF4016.jpg",
    "public/images/falltourney/DSCF2022.jpg",
    "public/images/falltourney/DSCF3008.jpg",
    "public/images/falltourney/DSCF2937.jpg",
    "public/images/falltourney/DSCF2122.jpg",
    "public/images/falltourney/DSCF2364.jpg",
    "public/images/falltourney/DSCF2312.jpg",
    "public/images/falltourney/DSCF2753.jpg",
    "public/images/falltourney/DSCF2756.jpg",
    "public/images/falltourney/DSCF2593.jpg",
    "public/images/falltourney/DSCF2226.jpg",
    "public/images/falltourney/DSCF4009.jpg",
    "public/images/falltourney/DSCF2199.jpg",
    "public/images/falltourney/DSCF2188.jpg",
    "public/images/falltourney/DSCF2116.jpg",
    "public/images/falltourney/DSCF2047.jpg",
    "public/images/falltourney/DSCF2018.jpg",
    "public/images/falltourney/DSCF2011.jpg",
    "public/images/falltourney/DSCF2005.jpg",
    "public/images/falltourney/DSCF2619.jpg",
    "public/images/falltourney/DSCF2605.jpg",
    "public/images/falltourney/DSCF2519.jpg",
    "public/images/falltourney/DSCF2367.jpg",
    "public/images/falltourney/DSCF2185.jpg",
    "public/images/falltourney/DSCF2156.jpg",
    "public/images/falltourney/DSCF2146.jpg",
    "public/images/falltourney/DSCF2133.jpg",
    "public/images/falltourney/DSCF2097.jpg",
    "public/images/falltourney/DSCF2007.jpg",
    "public/images/falltourney/DSCF2451.jpg",
];

#[component]
pub fn TournamentsPage() -> impl IntoView {
    view! {
        <div class="min-h-screen">
            <HeroBanner />
            <MediaCarousel
                title="2025 Stanford x Berkeley Highlights"
                images=STANFORD_IMAGES.to_vec()
            />

            <MediaCarousel
                title="Fall 2025 Tournament Highlights"
                images=FALL_TOURNEY_IMAGES.to_vec()
            />

            <UpcomingTournamentSection />
            <TournamentHistorySection />
        </div>
    }
}

#[component]
fn MediaCarousel(
    title: &'static str,
    images: Vec<&'static str>,
) -> impl IntoView {
    view! {
        <section class="py-16 bg-white dark:bg-gray-900">
            <div class="max-w-7xl mx-auto px-6">
                <h2 class="text-3xl font-bold text-gray-900 dark:text-white mb-6">
                    {title}
                </h2>

                <div
                    class="
                        flex gap-6 overflow-x-auto scroll-smooth
                        snap-x snap-mandatory pb-4
                    "
                >
                    {images.into_iter().enumerate().map(|(i, src)| view! {
                        <div
                            class="
                                snap-center shrink-0
                                w-[85%] md:w-[70%] lg:w-[60%]
                                rounded-2xl overflow-hidden
                                shadow-xl
                            "
                        >
                            <img
                                src=src
                                alt=format!("{} photo {}", title, i + 1)
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

#[component]
fn HeroBanner() -> impl IntoView {
    view! {
        <section
            id="banner"
            class="relative w-full min-h-[60vh] md:min-h-[75vh] flex items-center justify-center bg-cover bg-center"
            style="background-image: url('public/images/tourney-winners-sp25.png');"
        >
            <div class="absolute inset-0 bg-black/50"></div>
            <div class="relative z-10 max-w-4xl mx-auto text-center px-6">
                <h1 class="text-5xl md:text-6xl font-bold text-white mb-4">Tournaments</h1>
                <p class="text-xl text-white/90">
                    "Compete at the highest level with Berkeley's best players"
                </p>
            </div>
        </section>
    }
}



#[component]
fn UpcomingTournamentSection() -> impl IntoView {
    view! {
        <section class="py-20 bg-white dark:bg-gray-800">
            <div class="max-w-6xl mx-auto px-6">
                <h2 class="text-4xl font-bold text-center text-gray-900 dark:text-white mb-4">
                    "Upcoming Tournaments"
                </h2>
                <h3 class="text-2xl font-semibold text-center text-blue-600 dark:text-blue-400 mb-12">
                    "2026–2027"
                </h3>

                <div class="grid grid-cols-1 lg:grid-cols-4 gap-8">
                    <UpcomingTournamentCard
                        title="4th Berkeley x Stanford Tournament"
                        date="Fall 2026"
                        details=vec![
                            "150 participants: 75 from Berkeley, 75 from Stanford",
                            "Online qualifier, 2 full days of in-person play",
                            "Mini career fairs both in-person days",
                        ]
                        highlight=true
                    />
                    <UpcomingTournamentCard
                        title="Berkeley Poker Bots Competition"
                        date="January–March"
                        details=vec![
                            "More details coming soon",
                        ]
                        highlight=false
                    />
                    <UpcomingTournamentCard
                        title="5th Berkeley x Stanford Tournament"
                        date="TBD"
                        details=vec![
                            "More details coming soon",
                        ]
                        highlight=false
                    />
                    <UpcomingTournamentCard
                        title="Jump Trading Mini-Tournament"
                        date="TBD"
                        details=vec![
                            "50 people",
                            "1 day in-person",
                            "Guest speakers from Jump Trading",
                        ]
                        highlight=false
                    />
                </div>
            </div>
        </section>
    }
}

#[component]
fn UpcomingTournamentCard(
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

#[component]
fn TournamentDetail(label: &'static str, value: &'static str) -> impl IntoView {
    view! {
        <div class="bg-white/10 rounded-lg p-3 backdrop-blur-sm">
            <div class="text-sm text-blue-200">{label}</div>
            <div class="text-white font-semibold">{value}</div>
        </div>
    }
}

#[component]
fn TournamentHistorySection() -> impl IntoView {
    view! {
        <section class="py-20 bg-gray-50 dark:bg-gray-900">
            <div class="max-w-6xl mx-auto px-6">
                <h2 class="text-4xl font-bold text-center text-gray-900 dark:text-white mb-12">
                    "Past Tournaments"
                </h2>

                <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-8">
                    <TournamentHistoryCard
                        title="3rd Annual Berkeley x Stanford Tournament"
                        date="Spring 2026"
                        qualifier_date="N/A"
                        prize_pool="N/A"
                        sponsors="N/A"
                        notion_link=""
                    />
                    <TournamentHistoryCard
                        title="QRT Tournament"
                        date="February 26, 2026"
                        qualifier_date="N/A"
                        prize_pool="$2,500"
                        sponsors="QRT"
                        notion_link="https://pokeratberkeley.notion.site/qrt-tournament-2026?v=31d7865cff64806a8965000c37c54a78"
                    />
                    <TournamentHistoryCard
                        title="2025 Fall Tournament"
                        date="October 9-11, 2025"
                        qualifier_date="October 9, 2025"
                        prize_pool="$5,000"
                        sponsors="Jump Trading, Slowplay, Jane Street, Citadel, HRT"
                        notion_link="https://pokeratberkeley.notion.site/2025-Poker-at-Berkeley-Fall-Tournament-2909b562b20148c48d49465a8d331d33"
                    />
                    <TournamentHistoryCard
                        title="2025 Berkeley x Stanford Tournament"
                        date="April 26-27, 2025"
                        qualifier_date="April 19, 2025"
                        prize_pool="$10,000"
                        sponsors="SideBetz, Jane Street, Jump Trading, HRT, Susquehanna"
                        notion_link="https://pokeratberkeley.notion.site/2025-berkeley-stanford-poker-tournament"
                    />
                    <TournamentHistoryCard
                        title="2024 Fall Tournament"
                        date="November 24, 2024"
                        qualifier_date="Nov 22, 2024"
                        prize_pool="$3,000"
                        sponsors="Jump Trading, Jane Street, HRT, Susquehanna"
                        notion_link=""
                    />
                    <TournamentHistoryCard
                        title="2024 Berkeley x Stanford Tournament"
                        date="April 6-7, 2024"
                        qualifier_date="April 4, 2024"
                        prize_pool="$10,000"
                        sponsors="Reazon, Citadel, DRW, Jane Street, Jump Trading, SIG, HRT"
                        notion_link="https://pokeratberkeley.notion.site/Berkeley-x-Stanford-Poker-Tournament-6301c0df6e3f4b01bba2e5993ddcc0be"
                    />
                </div>
            </div>
        </section>
    }
}

#[component]
fn TournamentHistoryCard(
    title: &'static str,
    date: &'static str,
    qualifier_date: &'static str,
    prize_pool: &'static str,
    sponsors: &'static str,
    notion_link: &'static str,
) -> impl IntoView {
    view! {
        <div class="bg-white dark:bg-gray-700 rounded-lg shadow-lg p-6 hover:shadow-xl transition-shadow">
            <h3 class="text-xl font-bold text-gray-900 dark:text-white mb-2">{title}</h3>
            <p class="text-gray-600 dark:text-gray-300 mb-4">{date}</p>

            <div class=format!("space-y-3 {}", if notion_link.is_empty() { "" } else { "mb-4" })>
                <div>
                    <span class="text-gray-600 dark:text-gray-400 text-sm">"Qualifier:"</span>
                    <p class="font-semibold text-gray-900 dark:text-white">{qualifier_date}</p>
                </div>
                <div>
                    <span class="text-gray-600 dark:text-gray-400 text-sm">"Prize Pool:"</span>
                    <p class="font-semibold text-green-600 dark:text-green-400 text-lg">{prize_pool}</p>
                </div>
                <div>
                    <span class="text-gray-600 dark:text-gray-400 text-sm">"Sponsors:"</span>
                    <p class="text-gray-800 dark:text-gray-200 text-sm leading-relaxed">{sponsors}</p>
                </div>
            </div>

            {if !notion_link.is_empty() {
                view! {
                    <a
                        href=notion_link
                        target="_blank"
                        class="inline-block w-full text-center bg-blue-600 hover:bg-blue-700 text-white font-semibold py-2 px-4 rounded transition-colors"
                    >
                        "View Details"
                    </a>
                }.into_any()
            } else {
                view! { <div></div> }.into_any()
            }}
        </div>
    }
}
