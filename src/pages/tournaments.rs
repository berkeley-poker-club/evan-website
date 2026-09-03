use leptos::prelude::*;

const STANFORD26_IMAGES: &[&str] = &[
    "/public/images/stanfxcal26/Q62A1237.webp",
    "/public/images/stanfxcal26/Q62A128-6.webp",
    "/public/images/stanfxcal26/Q62A1213.webp",
    "/public/images/stanfxcal26/Q62A1241.webp",
    "/public/images/stanfxcal26/Q62A128-7.webp",
    "/public/images/stanfxcal26/Q62A1192.webp",
    "/public/images/stanfxcal26/Q62A1274.webp",
    "/public/images/stanfxcal26/Q62A1261.webp",
    "/public/images/stanfxcal26/Q62A1275.webp",
    "/public/images/stanfxcal26/Q62A1246.webp",
    "/public/images/stanfxcal26/Q62A1176.webp",
    "/public/images/stanfxcal26/Q62A1257.webp",
    "/public/images/stanfxcal26/Q62A1155.webp",
    "/public/images/stanfxcal26/Q62A1245.webp",
    "/public/images/stanfxcal26/Q62A1273.webp",
    "/public/images/stanfxcal26/Q62A128-8.webp",
    "/public/images/stanfxcal26/Q62A1111.webp",
    "/public/images/stanfxcal26/Q62A1269.webp",
    "/public/images/stanfxcal26/Q62A1137.webp",
    "/public/images/stanfxcal26/Q62A128-16.webp",
    "/public/images/stanfxcal26/Q62A128-17.webp",
    "/public/images/stanfxcal26/Q62A1115.webp",
    "/public/images/stanfxcal26/Q62A1219.webp",
    "/public/images/stanfxcal26/Q62A1161.webp",
    "/public/images/stanfxcal26/Q62A128-14.webp",
    "/public/images/stanfxcal26/Q62A1282.webp",
    "/public/images/stanfxcal26/Q62A1065.webp",
    "/public/images/stanfxcal26/Q62A1090.webp",
    "/public/images/stanfxcal26/Q62A128-10.webp",
    "/public/images/stanfxcal26/Q62A1166.webp",
    "/public/images/stanfxcal26/Q62A1225.webp",
    "/public/images/stanfxcal26/Q62A1216.webp",
    "/public/images/stanfxcal26/Q62A128-9.webp",
    "/public/images/stanfxcal26/Q62A1206.webp",
    "/public/images/stanfxcal26/Q62A1026.webp",
    "/public/images/stanfxcal26/Q62A1183.webp",
    "/public/images/stanfxcal26/Q62A1185.webp",
    "/public/images/stanfxcal26/Q62A1197.webp",
    "/public/images/stanfxcal26/Q62A1158.webp",
    "/public/images/stanfxcal26/Q62A1127.webp",
    "/public/images/stanfxcal26/Q62A1110.webp",
    "/public/images/stanfxcal26/Q62A1203.webp",
    "/public/images/stanfxcal26/Q62A128-11.webp",
    "/public/images/stanfxcal26/Q62A1060.webp",
    "/public/images/stanfxcal26/Q62A1163.webp",
    "/public/images/stanfxcal26/Q62A128-12.webp",
    "/public/images/stanfxcal26/Q62A1038.webp",
    "/public/images/stanfxcal26/Q62A1171.webp",
    "/public/images/stanfxcal26/Q62A1189.webp",
    "/public/images/stanfxcal26/Q62A1124.webp",
    "/public/images/stanfxcal26/Q62A1204.webp",
    "/public/images/stanfxcal26/Q62A1130.webp",
    "/public/images/stanfxcal26/Q62A1050.webp",
    "/public/images/stanfxcal26/Q62A1053.webp",
    "/public/images/stanfxcal26/Q62A1248.webp",
    "/public/images/stanfxcal26/Q62A128-5.webp",
    "/public/images/stanfxcal26/Q62A1073.webp",
    "/public/images/stanfxcal26/Q62A1255.webp",
    "/public/images/stanfxcal26/Q62A1116.webp",
    "/public/images/stanfxcal26/Q62A128-4.webp",
    "/public/images/stanfxcal26/Q62A1199.webp",
    "/public/images/stanfxcal26/Q62A1082.webp",
    "/public/images/stanfxcal26/Q62A1253.webp",
    "/public/images/stanfxcal26/Q62A1210.webp",
    "/public/images/stanfxcal26/Q62A0998.webp",
    "/public/images/stanfxcal26/Q62A1222.webp",
    "/public/images/stanfxcal26/Q62A127.webp",
    "/public/images/stanfxcal26/Q62A1234.webp",
    "/public/images/stanfxcal26/Q62A1135.webp",
    "/public/images/stanfxcal26/Q62A128-15.webp",
    "/public/images/stanfxcal26/Q62A1271.webp",
    "/public/images/stanfxcal26/Q62A1145.webp",
    "/public/images/stanfxcal26/Q62A128-18.webp",
    "/public/images/stanfxcal26/Q62A1283.webp",
    "/public/images/stanfxcal26/Q62A1041.webp",
    "/public/images/stanfxcal26/Q62A1089.webp",
    "/public/images/stanfxcal26/Q62A128-13.webp",
    "/public/images/stanfxcal26/Q62A1091.webp",
    "/public/images/stanfxcal26/Q62A1141.webp",
    "/public/images/stanfxcal26/Q62A1156.webp",
    "/public/images/stanfxcal26/Q62A1164.webp",
    "/public/images/stanfxcal26/Q62A1039.webp",
    "/public/images/stanfxcal26/Q62A128.webp",
    "/public/images/stanfxcal26/Q62A1193.webp",
    "/public/images/stanfxcal26/Q62A1228.webp",
    "/public/images/stanfxcal26/Q62A1230.webp",
];

const QRT_TOURNEY_IMAGES: &[&str] = &[
    "/public/images/qrt-tourney/DSCF4293.webp",
    "/public/images/qrt-tourney/DSCF4250.webp",
    "/public/images/qrt-tourney/DSCF4295.webp",
    "/public/images/qrt-tourney/DSCF4301.webp",
    "/public/images/qrt-tourney/DSCF4222.webp",
    "/public/images/qrt-tourney/DSCF4304.webp",
    "/public/images/qrt-tourney/DSCF4400.webp",
    "/public/images/qrt-tourney/DSCF4323.webp",
    "/public/images/qrt-tourney/DSCF4390.webp",
    "/public/images/qrt-tourney/DSCF4402.webp",
    "/public/images/qrt-tourney/DSCF4404.webp",
    "/public/images/qrt-tourney/DSCF4436.webp",
    "/public/images/qrt-tourney/DSCF4471.webp",
    "/public/images/qrt-tourney/DSCF4395.webp",
    "/public/images/qrt-tourney/DSCF4384.webp",
    "/public/images/qrt-tourney/DSCF4267.webp",
    "/public/images/qrt-tourney/DSCF4223.webp",
    "/public/images/qrt-tourney/DSCF4224.webp",
    "/public/images/qrt-tourney/DSCF4226.webp",
    "/public/images/qrt-tourney/DSCF4228.webp",
    "/public/images/qrt-tourney/DSCF4232.webp",
    "/public/images/qrt-tourney/DSCF4234.webp",
    "/public/images/qrt-tourney/DSCF4236.webp",
    "/public/images/qrt-tourney/DSCF4237.webp",
    "/public/images/qrt-tourney/DSCF4238.webp",
    "/public/images/qrt-tourney/DSCF4239.webp",
    "/public/images/qrt-tourney/DSCF4242.webp",
    "/public/images/qrt-tourney/DSCF4243.webp",
    "/public/images/qrt-tourney/DSCF4252.webp",
    "/public/images/qrt-tourney/DSCF4257.webp",
    "/public/images/qrt-tourney/DSCF4259.webp",
    "/public/images/qrt-tourney/DSCF4265.webp",
    "/public/images/qrt-tourney/DSCF4270.webp",
    "/public/images/qrt-tourney/DSCF4271.webp",
    "/public/images/qrt-tourney/DSCF4272.webp",
    "/public/images/qrt-tourney/DSCF4274.webp",
    "/public/images/qrt-tourney/DSCF4275.webp",
    "/public/images/qrt-tourney/DSCF4276.webp",
    "/public/images/qrt-tourney/DSCF4278.webp",
    "/public/images/qrt-tourney/DSCF4280.webp",
    "/public/images/qrt-tourney/DSCF4283.webp",
    "/public/images/qrt-tourney/DSCF4285.webp",
    "/public/images/qrt-tourney/DSCF4290.webp",
    "/public/images/qrt-tourney/DSCF4292.webp",
    "/public/images/qrt-tourney/DSCF4294.webp",
    "/public/images/qrt-tourney/DSCF4296.webp",
    "/public/images/qrt-tourney/DSCF4297.webp",
    "/public/images/qrt-tourney/DSCF4299.webp",
    "/public/images/qrt-tourney/DSCF4300.webp",
    "/public/images/qrt-tourney/DSCF4306.webp",
    "/public/images/qrt-tourney/DSCF4310.webp",
    "/public/images/qrt-tourney/DSCF4312.webp",
    "/public/images/qrt-tourney/DSCF4313.webp",
    "/public/images/qrt-tourney/DSCF4315.webp",
    "/public/images/qrt-tourney/DSCF4317.webp",
    "/public/images/qrt-tourney/DSCF4319.webp",
    "/public/images/qrt-tourney/DSCF4332.webp",
    "/public/images/qrt-tourney/DSCF4338.webp",
    "/public/images/qrt-tourney/DSCF4339.webp",
    "/public/images/qrt-tourney/DSCF4340.webp",
    "/public/images/qrt-tourney/DSCF4345.webp",
    "/public/images/qrt-tourney/DSCF4351.webp",
    "/public/images/qrt-tourney/DSCF4353.webp",
    "/public/images/qrt-tourney/DSCF4366.webp",
    "/public/images/qrt-tourney/DSCF4371.webp",
    "/public/images/qrt-tourney/DSCF4373.webp",
    "/public/images/qrt-tourney/DSCF4380.webp",
    "/public/images/qrt-tourney/DSCF4382.webp",
    "/public/images/qrt-tourney/DSCF4385.webp",
    "/public/images/qrt-tourney/DSCF4388.webp",
    "/public/images/qrt-tourney/DSCF4394.webp",
    "/public/images/qrt-tourney/DSCF4403.webp",
    "/public/images/qrt-tourney/DSCF4405.webp",
    "/public/images/qrt-tourney/DSCF4414.webp",
    "/public/images/qrt-tourney/DSCF4415.webp",
    "/public/images/qrt-tourney/DSCF4418.webp",
    "/public/images/qrt-tourney/DSCF4420.webp",
    "/public/images/qrt-tourney/DSCF4422.webp",
    "/public/images/qrt-tourney/DSCF4429.webp",
    "/public/images/qrt-tourney/DSCF4431.webp",
    "/public/images/qrt-tourney/DSCF4433.webp",
    "/public/images/qrt-tourney/DSCF4434.webp",
    "/public/images/qrt-tourney/DSCF4460.webp",
    "/public/images/qrt-tourney/DSCF4461.webp",
    "/public/images/qrt-tourney/DSCF4463.webp",
    "/public/images/qrt-tourney/DSCF4466.webp",
    "/public/images/qrt-tourney/DSCF4468.webp",
    "/public/images/qrt-tourney/DSCF4473.webp",
    "/public/images/qrt-tourney/DSCF4475.webp",
    "/public/images/qrt-tourney/DSCF4477.webp",
    "/public/images/qrt-tourney/DSCF4478.webp",
    "/public/images/qrt-tourney/DSCF4483.webp",
    "/public/images/qrt-tourney/DSCF4488.webp",
];

const STANFORD_IMAGES: &[&str] = &[
    "/public/images/stanfxcal25/DSCF1288.webp",
    "/public/images/stanfxcal25/DSCF0839.webp",
    "/public/images/stanfxcal25/DSCF0932.webp",
    "/public/images/stanfxcal25/DSCF0964.webp",
    "/public/images/stanfxcal25/DSCF0929.webp",
    "/public/images/stanfxcal25/DSCF0865.webp",
    "/public/images/stanfxcal25/DSCF0551.webp",
    "/public/images/stanfxcal25/DSCF0698.webp",
    "/public/images/stanfxcal25/DSCF0978.webp",
    "/public/images/stanfxcal25/DSCF1412.webp",
    "/public/images/stanfxcal25/DSCF1493.webp",
    "/public/images/stanfxcal25/DSCF1542.webp",
    "/public/images/stanfxcal25/DSCF1526.webp",
    "/public/images/stanfxcal25/DSCF1495.webp",
    "/public/images/stanfxcal25/DSCF1504.webp",
    "/public/images/stanfxcal25/DSCF1505.webp",
    "/public/images/stanfxcal25/DSCF1550.webp",
    "/public/images/stanfxcal25/DSCF1180.webp",
    "/public/images/stanfxcal25/DSCF1152.webp",
    "/public/images/stanfxcal25/DSCF1139.webp",
    "/public/images/stanfxcal25/DSCF1091.webp",
    "/public/images/stanfxcal25/DSCF1070.webp",
    "/public/images/stanfxcal25/DSCF1011.webp",
    "/public/images/stanfxcal25/DSCF0944.webp",
    "/public/images/stanfxcal25/DSCF0769.webp",
    "/public/images/stanfxcal25/DSCF0640.webp",
    "/public/images/stanfxcal25/DSCF0435.webp",
    "/public/images/stanfxcal25/DSCF1257.webp",
    "/public/images/stanfxcal25/DSCF1092.webp",
    "/public/images/stanfxcal25/DSCF1018.webp",
    "/public/images/stanfxcal25/DSCF0939.webp",
    "/public/images/stanfxcal25/DSCF0852.webp",
    "/public/images/stanfxcal25/DSCF0844.webp",
];

const FALL_TOURNEY_IMAGES: &[&str] = &[
    "/public/images/falltourney/DSCF4016.webp",
    "/public/images/falltourney/DSCF2022.webp",
    "/public/images/falltourney/DSCF3008.webp",
    "/public/images/falltourney/DSCF2937.webp",
    "/public/images/falltourney/DSCF2122.webp",
    "/public/images/falltourney/DSCF2364.webp",
    "/public/images/falltourney/DSCF2312.webp",
    "/public/images/falltourney/DSCF2753.webp",
    "/public/images/falltourney/DSCF2756.webp",
    "/public/images/falltourney/DSCF2593.webp",
    "/public/images/falltourney/DSCF2226.webp",
    "/public/images/falltourney/DSCF4009.webp",
    "/public/images/falltourney/DSCF2199.webp",
    "/public/images/falltourney/DSCF2188.webp",
    "/public/images/falltourney/DSCF2116.webp",
    "/public/images/falltourney/DSCF2047.webp",
    "/public/images/falltourney/DSCF2018.webp",
    "/public/images/falltourney/DSCF2011.webp",
    "/public/images/falltourney/DSCF2005.webp",
    "/public/images/falltourney/DSCF2619.webp",
    "/public/images/falltourney/DSCF2605.webp",
    "/public/images/falltourney/DSCF2519.webp",
    "/public/images/falltourney/DSCF2367.webp",
    "/public/images/falltourney/DSCF2185.webp",
    "/public/images/falltourney/DSCF2156.webp",
    "/public/images/falltourney/DSCF2146.webp",
    "/public/images/falltourney/DSCF2133.webp",
    "/public/images/falltourney/DSCF2097.webp",
    "/public/images/falltourney/DSCF2007.webp",
    "/public/images/falltourney/DSCF2451.webp",
];

#[component]
pub fn TournamentsPage() -> impl IntoView {
    view! {
        <div class="min-h-screen">
            <HeroBanner />
            <div style="background-color: #111010;">
                <MediaCarousel
                    title="Spring 2026 Stanford x Berkeley Highlights"
                    images=STANFORD26_IMAGES.to_vec()
                />

                <MediaCarousel
                    title="Spring 2026 QRT Tournament"
                    images=QRT_TOURNEY_IMAGES.to_vec()
                />

                <MediaCarousel
                    title="Spring 2025 Stanford x Berkeley Highlights"
                    images=STANFORD_IMAGES.to_vec()
                />

                <MediaCarousel
                    title="Fall 2025 Tournament Highlights"
                    images=FALL_TOURNEY_IMAGES.to_vec()
                />
            </div>

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
        <style>
            "@import url('https://fonts.googleapis.com/css2?family=Figtree:wght@500;600&display=swap');"
        </style>
        <section class="py-6">
            <div class="max-w-7xl mx-auto px-4 sm:px-6">
                <div class="bg-zinc-900 border border-zinc-700 rounded-2xl p-4 sm:p-6 md:p-8">
                    <h2
                        class="text-2xl sm:text-3xl mb-2 sm:mb-6"
                        style="color: #F0EDE8; font-family: 'Figtree', sans-serif; font-weight: 600; letter-spacing: 0.05em;"
                    >
                        {title}
                    </h2>
                    <p class="text-sm text-zinc-400 mb-4">"Drag or swipe to browse photos"</p>

                    <div
                        class="
                            flex gap-3 sm:gap-6 overflow-x-auto scroll-smooth
                            snap-x snap-mandatory pb-3 sm:pb-4
                        "
                        style="scrollbar-width: thin; -webkit-overflow-scrolling: touch;"
                    >
                        {images.into_iter().enumerate().map(|(i, src)| view! {
                            <div
                                class="
                                    snap-start shrink-0
                                    w-[calc(100vw-3rem)] sm:w-[85%] md:w-[70%] lg:w-[60%]
                                    rounded-xl sm:rounded-2xl overflow-hidden
                                    shadow-xl
                                "
                            >
                                <img
                                    src=src
                                    alt=format!("{} photo {}", title, i + 1)
                                    class="w-full aspect-[4/3] h-auto object-cover"
                                    loading="lazy"
                                />
                            </div>
                        }).collect::<Vec<_>>()}
                    </div>
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
            class="relative w-full flex items-center justify-center bg-cover"
            style="height: 70vh; background-image: url('/public/images/falltourney/DSCF2029.webp'); background-position: 75% bottom;"
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
                        title="Spring 2026 Berkeley x Stanford Tournament"
                        date="Spring 2026"
                        qualifier_date="April 19, 2026"
                        prize_pool="$10,000"
                        sponsors="Jump Trading, Jane Street, Citadel, HRT, QRT"
                        notion_link="https://pokeratberkeley.notion.site/berkeley-stanford-2026?v=3447865cff64814f9bbe000cdd22c079"
                    />
                    <TournamentHistoryCard
                        title="QRT Tournament"
                        date="February 26, 2026"
                        qualifier_date="February 17"
                        prize_pool="$2,500"
                        sponsors="QRT"
                        notion_link="https://pokeratberkeley.notion.site/qrt-tournament-2026?v=31d7865cff64806a8965000c37c54a78"
                    />
                    <TournamentHistoryCard
                        title="2025 Fall Tournament"
                        date="October 9-11, 2025"
                        qualifier_date="October 8, 2025"
                        prize_pool="$5,000"
                        sponsors="Jump Trading, Slowplay, Jane Street, Citadel, HRT"
                        notion_link="https://pokeratberkeley.notion.site/2025-Poker-at-Berkeley-Fall-Tournament-2909b562b20148c48d49465a8d331d33"
                    />
                    <TournamentHistoryCard
                        title="Spring 2025 Berkeley x Stanford Tournament"
                        date="April 25-27, 2025"
                        qualifier_date="April 19, 2025"
                        prize_pool="$10,000"
                        sponsors="SideBetz, Jane Street, Jump Trading, HRT, Susquehanna"
                        notion_link="https://pokeratberkeley.notion.site/berkeley-stanford-2025?v=2bf7865cff64803bb102000ce09207b9"
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
                        title="Spring 2024 Berkeley x Stanford Tournament"
                        date="April 5-7, 2024"
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
