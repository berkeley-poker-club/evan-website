use leptos::prelude::*;

#[component]
pub fn TournamentsPage() -> impl IntoView {
    view! {
        <div class="min-h-screen">
            <HeroBanner />
            <UpcomingTournamentSection />
            <TournamentHistorySection />
        </div>
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
                    "Spring 2026"
                </h3>

                <div class="grid grid-cols-1 lg:grid-cols-3 gap-8">
                    <UpcomingTournamentCard
                        title="3rd Annual Berkeley x Stanford Tournament"
                        date="Late April"
                        details=vec![
                            "150 participants: 75 from Berkeley, 75 from Stanford",
                            "Online qualifier, 2 full days of in-person play",
                            "Mini career fairs both in-person days",
                        ]
                        highlight=true
                    />
                    <UpcomingTournamentCard
                        title="Berkeley Poker Bots Competition"
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
                        title="2025 Fall Tournament"
                        date="October 9-11, 2025"
                        qualifier_date="October 9, 2025"
                        prize_pool="$5,000"
                        sponsors="Citadel"
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
