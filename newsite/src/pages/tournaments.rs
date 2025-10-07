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
        <section class="py-20 bg-white">
            <div class="max-w-6xl mx-auto px-6">
                <h2 class="text-4xl font-bold text-center text-gray-900 mb-12">
                    "Upcoming Tournaments"
                </h2>

                <div class="max-w-4xl mx-auto">
                    <div class="bg-gradient-to-r from-blue-900 to-blue-950 rounded-xl shadow-2xl overflow-hidden">
                        <div class="p-8 md:p-12">
                            <div class="text-center mb-8">
                                <h3 class="text-3xl md:text-4xl font-bold text-white mb-2">
                                    "Fall Tournament 2025"
                                </h3>
                                <p class="text-xl text-blue-100">
                                    "October 9-11, 2025"
                                </p>
                            </div>

                            <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-4 mb-8">
                                <TournamentDetail label="Format" value="Online + In-Person" />
                                <TournamentDetail label="Qualifier" value="250 Players" />
                                <TournamentDetail label="Finals" value="70 Players" />
                                <TournamentDetail label="Prize Pool" value="$5,000" />
                            </div>

                            <div class="bg-white/10 backdrop-blur-sm rounded-lg p-6 mb-6">
                                <h4 class="text-xl font-bold text-white mb-4">"Event Schedule"</h4>
                                <ul class="space-y-3 text-white/90">
                                    <li class="flex items-start space-x-3">
                                        <div class="w-2 h-2 bg-blue-300 rounded-full mt-2 flex-shrink-0"></div>
                                        <span>"October 9th: 250 person qualifying round (online)"</span>
                                    </li>
                                    <li class="flex items-start space-x-3">
                                        <div class="w-2 h-2 bg-blue-300 rounded-full mt-2 flex-shrink-0"></div>
                                        <span>"October 11th: 70 person final day (in-person)"</span>
                                    </li>
                                </ul>
                            </div>

                            <div class="bg-yellow-400/20 backdrop-blur-sm rounded-lg p-6 border border-yellow-400/30">
                                <h4 class="text-xl font-bold text-yellow-100 mb-2">"Special Guest"</h4>
                                <p class="text-white text-lg font-semibold">"Citadel"</p>
                            </div>

                            <div class="mt-8 text-center">
                                <a
                                    href="https://pokeratberkeley.notion.site/2025-Poker-at-Berkeley-Fall-Tournament-2909b562b20148c48d49465a8d331d33"
                                    target="_blank"
                                    class="inline-block bg-white text-blue-700 font-bold px-8 py-3 rounded-lg hover:bg-blue-50 transition-colors shadow-lg"
                                >
                                    "View Full Tournament Wiki"
                                </a>
                            </div>
                        </div>
                    </div>
                </div>
            </div>
        </section>
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
        <section class="py-20 bg-gray-50">
            <div class="max-w-6xl mx-auto px-6">
                <h2 class="text-4xl font-bold text-center text-gray-900 mb-12">
                    "Past Tournaments"
                </h2>

                <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-8">
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
        <div class="bg-white rounded-lg shadow-lg p-6 hover:shadow-xl transition-shadow">
            <h3 class="text-xl font-bold text-gray-900 mb-2">{title}</h3>
            <p class="text-gray-600 mb-4">{date}</p>

            <div class=format!("space-y-3 {}", if notion_link.is_empty() { "" } else { "mb-4" })>
                <div>
                    <span class="text-gray-600 text-sm">"Qualifier:"</span>
                    <p class="font-semibold text-gray-900">{qualifier_date}</p>
                </div>
                <div>
                    <span class="text-gray-600 text-sm">"Prize Pool:"</span>
                    <p class="font-semibold text-green-600 text-lg">{prize_pool}</p>
                </div>
                <div>
                    <span class="text-gray-600 text-sm">"Sponsors:"</span>
                    <p class="text-gray-800 text-sm leading-relaxed">{sponsors}</p>
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
