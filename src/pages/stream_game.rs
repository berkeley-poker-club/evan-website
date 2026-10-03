use leptos::prelude::*;
use leptos_meta::Title;
use crate::components::OptimizedImage;

const YOUTUBE_URL: &str = "https://youtube.com/@pokeratberkeley?si=FDNtvp8VSFir_Vk-";
const TWITCH_URL: &str = "http://twitch.tv/pokeratberkeley";

#[derive(Clone, Copy)]
enum Platform {
    YouTube,
    Twitch,
}

impl Platform {
    fn label(&self) -> &'static str {
        match self {
            Platform::YouTube => "YouTube",
            Platform::Twitch => "Twitch",
        }
    }

    fn badge_class(&self) -> &'static str {
        match self {
            Platform::YouTube => "bg-[#FF0000] text-white",
            Platform::Twitch => "bg-[#9146FF] text-white",
        }
    }
}

struct LineupEntry {
    name: &'static str,
    alias: &'static str,
    result: &'static str,
}

struct Episode {
    number: &'static str,
    title: &'static str,
    platform: Platform,
    embed_src: &'static str,
    lineup: &'static [LineupEntry],
    game_info: &'static str,
}

// `result` is left blank ("—") until final results for that specific episode
// are known — fill in per episode as they come in.
// 2k starting stack + 1 rebuy = 4000 total in play; 0 means busted out for -4000.
const STREAM_LINEUP_EP00: &[LineupEntry] = &[
    LineupEntry { name: "Ray Tan", alias: "Ray", result: "+7180" },
    LineupEntry { name: "Henry Lee", alias: "Henry", result: "+3880" },
    LineupEntry { name: "Vincent Chen", alias: "Vincent", result: "+950" },
    LineupEntry { name: "Ethan Hull", alias: "Ethan", result: "-4000" },
    LineupEntry { name: "Afraz Ahmed", alias: "Afraz", result: "+5220" },
    LineupEntry { name: "Frances Jing", alias: "Frances", result: "+11660" },
    LineupEntry { name: "Matthew Naidu", alias: "Martial", result: "-4000" },
    LineupEntry { name: "Fanou Zhang", alias: "Fan", result: "+2000" },
    LineupEntry { name: "Tobias Arntzen", alias: "Tobias", result: "+4910" },
];

// Freeroll cash game, 1/1 blinds, 100 buy-in + 1 rebuy (200 total in play).
// Anyone not given a final amount busted out for -200.
const STREAM_LINEUP_EP01: &[LineupEntry] = &[
    LineupEntry { name: "Howard Chen", alias: "Spieler", result: "+356" },
    LineupEntry { name: "Fanou Zhang", alias: "Fancy Fan", result: "+534" },
    LineupEntry { name: "Vishesh Verma", alias: "Vvern", result: "+260" },
    LineupEntry { name: "Bill Young", alias: "Bill", result: "-200" },
    LineupEntry { name: "Rigo Torres", alias: "Rigo", result: "+90" },
    LineupEntry { name: "Oscar Näslund Cuesta", alias: "Oscar", result: "-200" },
    LineupEntry { name: "Azad Parikh", alias: "Azad", result: "-200" },
    LineupEntry { name: "Ben Finch", alias: "Ben", result: "+175" },
    LineupEntry { name: "Arhan Vontela", alias: "Von", result: "+385" },
];

const STREAM_EPISODES: &[Episode] = &[
    Episode {
        number: "EP. 00",
        title: "Episode 0 (Tester)",
        platform: Platform::YouTube,
        embed_src: "https://www.youtube.com/embed/3kktPT5cJnE",
        lineup: STREAM_LINEUP_EP00,
        game_info: "Blinds 10/20 · Freeroll 2k Starting Stack + 1 Rebuy",
    },
    Episode {
        number: "EP. 01",
        title: "Episode 1",
        platform: Platform::YouTube,
        embed_src: "https://www.youtube.com/embed/7buLpyOZCsI",
        lineup: STREAM_LINEUP_EP01,
        game_info: "Blinds $1/$1 · Freeroll $100 Starting Stack + 1 Rebuy",
    },
];

const TOURNAMENT_EPISODES: &[Episode] = &[Episode {
    number: "VOL. 03",
    title: "FINAL DAY Highlights - 3rd annual Berkeley x Stanford Poker Tournament!",
    platform: Platform::YouTube,
    embed_src: "https://www.youtube.com/embed/xwhVQmdWD0k",
    lineup: &[],
    game_info: "",
}];

#[component]
pub fn StreamGamePage() -> impl IntoView {
    view! {
        <Title text="Stream Game | Poker at Berkeley" />
        <div class="min-h-screen bg-[#0B0E14]">
            <FollowBar />
            <HeroBanner />
            <EpisodeSection
                heading="Stream Game"
                subheading="Weekly cash game streams, run by Poker at Berkeley."
                accent="#FDB515"
                episodes=STREAM_EPISODES
            />
            <PlayCallout />
            <EpisodeSection
                heading="Tournament Series"
                subheading="Highlights from the Berkeley x Stanford Poker Tournament."
                accent="#8C1515"
                episodes=TOURNAMENT_EPISODES
            />
        </div>
    }
}

#[component]
fn FollowBar() -> impl IntoView {
    view! {
        <div class="bg-[#003262] border-b-2 border-[#FDB515]/40">
            <div class="max-w-6xl mx-auto px-6 py-3 flex flex-col sm:flex-row items-center justify-center gap-3 sm:gap-8">
                <a
                    href=YOUTUBE_URL
                    target="_blank"
                    rel="noopener noreferrer"
                    class="group flex items-center gap-2 text-[#FDB515] hover:text-white transition-colors font-mono text-sm uppercase tracking-wide"
                >
                    <svg class="w-5 h-5 fill-current" viewBox="0 0 24 24">
                        <path d="M23.498 6.186a2.994 2.994 0 0 0-2.112-2.12C19.505 3.545 12 3.545 12 3.545s-7.505 0-9.386.521a2.994 2.994 0 0 0-2.112 2.12A31.31 31.31 0 0 0 0 12a31.31 31.31 0 0 0 .502 5.814 2.994 2.994 0 0 0 2.112 2.12c1.881.521 9.386.521 9.386.521s7.505 0 9.386-.521a2.994 2.994 0 0 0 2.112-2.12A31.31 31.31 0 0 0 24 12a31.31 31.31 0 0 0-.502-5.814zM9.75 15.568V8.432L15.818 12 9.75 15.568z"></path>
                    </svg>
                    "Follow us on YouTube!"
                </a>
                <span class="hidden sm:block w-px h-4 bg-[#FDB515]/30"></span>
                <a
                    href=TWITCH_URL
                    target="_blank"
                    rel="noopener noreferrer"
                    class="group flex items-center gap-2 text-[#FDB515] hover:text-white transition-colors font-mono text-sm uppercase tracking-wide"
                >
                    <svg class="w-5 h-5 fill-current" viewBox="0 0 24 24">
                        <path d="M11.571 4.714h1.715v5.143H11.57zm4.715 0H18v5.143h-1.714zM6 0 1.714 4.286v15.428h5.143V24l4.286-4.286h3.428L22.286 12V0zm14.571 11.143-3.428 3.428h-3.429l-3 3v-3H6.857V1.714h13.714Z"></path>
                    </svg>
                    "Follow us on Twitch!"
                </a>
            </div>
        </div>
    }
}

#[component]
fn HeroBanner() -> impl IntoView {
    view! {
        <section class="relative overflow-hidden py-24 border-b-4 border-[#FDB515]" style="background: repeating-linear-gradient(0deg, #0B0E14 0px, #0B0E14 3px, #12161f 3px, #12161f 4px);">
            <div class="pointer-events-none absolute inset-0" style="background: radial-gradient(ellipse at center, rgba(253,181,21,0.10) 0%, transparent 65%);"></div>
            <div class="relative z-10 max-w-4xl mx-auto text-center px-6">
                <span class="inline-flex items-center gap-2 px-4 py-1 mb-6 rounded-full border border-[#FDB515]/50 text-[#FDB515] text-xs font-mono uppercase tracking-[0.3em]">
                    <span class="w-2 h-2 rounded-full bg-[#FDB515] animate-pulse"></span>
                    "Broadcast Archive"
                </span>
                <h1
                    class="text-5xl md:text-7xl font-black text-white mb-4 uppercase tracking-tight"
                    style="font-family: 'Archivo Black', sans-serif; text-shadow: 3px 3px 0 #FDB515, 6px 6px 0 rgba(0,50,98,0.6);"
                >
                    "Stream Game"
                </h1>
                <p class="text-lg text-white font-mono">
                    "Cash games and tournaments — watch live, or catch the replay."
                </p>
            </div>
        </section>
    }
}

#[component]
fn PlayCallout() -> impl IntoView {
    view! {
        <section class="py-10 border-b border-white/5">
            <div class="max-w-6xl mx-auto px-6 grid md:grid-cols-[5fr_4fr] items-center gap-6">
                <div>
                    <OptimizedImage
                        src="/public/images/streamgame_snapshot.png"
                        alt="Poker at Berkeley livestream"
                        class="w-full h-auto rounded-xl"
                        loading="lazy"
                    />
                </div>
                <div class="rounded-xl border-2 border-[#FDB515]/40 bg-gradient-to-b from-[#1a2030] to-[#0B0E14] p-6 md:p-8">
                    <h2
                        class="text-xl md:text-2xl font-black text-white uppercase tracking-tight mb-3"
                        style="font-family: 'Archivo Black', sans-serif;"
                    >
                        "Want a Chance to Play on Our Livestream Game?"
                    </h2>
                    <ul class="space-y-1.5 text-gray-300 font-mono text-sm mb-4">
                        <li>"• $300 freeroll cash game"</li>
                        <li>"• We cover your buy-in plus one rebuy — it costs you nothing to play"</li>
                        <li>"• Players are randomly selected each episode, so new faces make it to the table every time"</li>
                        <li>"• Open to P@B members only"</li>
                        <li>"• You must be comfortable being on Twitch/YouTube and recorded"</li>
                    </ul>
                    <p class="text-gray-400 font-mono text-xs mb-4">
                        "Interested? Join our Discord and watch for an announcement when the next round of player selections opens."
                    </p>
                    <a
                        href="https://discord.gg/SbS9UbZW2a"
                        target="_blank"
                        rel="noopener noreferrer"
                        class="inline-flex items-center gap-2 px-4 py-2 rounded-full bg-[#FDB515] text-[#0B0E14] font-bold text-sm uppercase tracking-wide hover:bg-white transition-colors"
                    >
                        "Join the Discord"
                    </a>
                </div>
            </div>
        </section>
    }
}

#[component]
fn EpisodeSection(
    heading: &'static str,
    subheading: &'static str,
    accent: &'static str,
    episodes: &'static [Episode],
) -> impl IntoView {
    view! {
        <section class="py-16 border-b border-white/5">
            <div class="max-w-6xl mx-auto px-6">
                <div class="flex items-center gap-4 mb-2">
                    <span class="h-2.5 w-2.5 rotate-45" style=format!("background-color: {accent};")></span>
                    <h2
                        class="text-3xl md:text-4xl font-black text-white uppercase tracking-tight"
                        style="font-family: 'Archivo Black', sans-serif;"
                    >
                        {heading}
                    </h2>
                    <span class="flex-1 h-px" style=format!("background: linear-gradient(90deg, {accent}55, transparent);")></span>
                </div>
                <p class="text-gray-300 font-mono text-sm mb-10 pl-[26px]">{subheading}</p>

                <div class="grid grid-cols-1 md:grid-cols-2 gap-10 items-start">
                    {episodes.iter().map(|ep| view! { <StreamCard episode=ep accent=accent /> }).collect::<Vec<_>>()}
                </div>
            </div>
        </section>
    }
}

#[component]
fn StreamCard(episode: &'static Episode, accent: &'static str) -> impl IntoView {
    let border_style = format!("box-shadow: 0 0 0 1px {accent}26, 0 20px 40px -15px rgba(0,0,0,0.6);");
    let badge_dot_style = format!("background-color: {accent};");
    let number_style = format!("color: {accent};");

    view! {
        <div class="rounded-xl p-3 bg-gradient-to-b from-[#1a2030] to-[#0B0E14] border-2 border-[#2a3346]" style=border_style>
            <div class="flex items-center justify-between px-2 pb-3">
                <div class="flex items-center gap-2">
                    <span class="w-2.5 h-2.5 rounded-full bg-red-500"></span>
                    <span class="w-2.5 h-2.5 rounded-full" style=badge_dot_style></span>
                    <span class="w-2.5 h-2.5 rounded-full bg-green-500"></span>
                </div>
                <span class=format!("px-2.5 py-0.5 rounded text-[11px] font-bold uppercase tracking-wide {}", episode.platform.badge_class())>
                    {episode.platform.label()}
                </span>
            </div>
            <div class="relative rounded-lg overflow-hidden bg-black aspect-video ring-1 ring-black/50">
                <iframe
                    src=episode.embed_src
                    class="absolute inset-0 w-full h-full"
                    allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture"
                    allowfullscreen=""
                ></iframe>
            </div>
            <div class="pt-4 px-2 pb-1">
                <span class="font-mono text-xs tracking-[0.2em]" style=number_style.clone()>{episode.number}</span>
                <h3 class="text-white font-bold text-lg leading-snug mt-1">{episode.title}</h3>
                {if !episode.game_info.is_empty() {
                    view! {
                        <p class="text-gray-400 font-mono text-xs mt-1">{episode.game_info}</p>
                    }.into_any()
                } else {
                    view! {}.into_any()
                }}
            </div>
            {if !episode.lineup.is_empty() {
                view! { <LineupTable lineup=episode.lineup accent=accent number_style=number_style.clone() /> }.into_any()
            } else {
                view! {}.into_any()
            }}
        </div>
    }
}

#[component]
fn LineupTable(lineup: &'static [LineupEntry], accent: &'static str, number_style: String) -> impl IntoView {
    view! {
        <details class="mt-3 mx-2 mb-2 group">
            <summary class="cursor-pointer select-none list-none flex items-center gap-2 px-2 py-1.5 rounded font-mono text-xs uppercase tracking-wide text-gray-300 hover:text-white transition-colors">
                <span class="transition-transform duration-150 group-open:rotate-90" style=number_style>"▸"</span>
                "Lineup & Results"
            </summary>
            <div class="mt-2 overflow-x-auto rounded-lg border border-white/10">
                <table class="w-full text-sm font-mono">
                    <thead>
                        <tr class="bg-white/5 text-gray-400 uppercase text-[11px] tracking-wide">
                            <th class="text-left px-3 py-2 font-normal">"Player"</th>
                            <th class="text-left px-3 py-2 font-normal">"Alias"</th>
                            <th class="text-left px-3 py-2 font-normal">"Result"</th>
                        </tr>
                    </thead>
                    <tbody>
                        {lineup
                            .iter()
                            .map(|p| {
                                let result = if p.result.is_empty() { "—" } else { p.result };
                                let result_cell = if result.starts_with('-') {
                                    view! {
                                        <td class="px-3 py-2 whitespace-nowrap" title="Eliminated">
                                            <svg class="w-3 h-3 fill-red-500" viewBox="0 0 16 16">
                                                <polygon points="8,2 15,14 1,14"></polygon>
                                            </svg>
                                        </td>
                                    }.into_any()
                                } else if result.starts_with('+') {
                                    view! {
                                        <td class="px-3 py-2 whitespace-nowrap font-bold text-green-400">{result}</td>
                                    }.into_any()
                                } else {
                                    view! {
                                        <td class="px-3 py-2 whitespace-nowrap text-gray-300">{result}</td>
                                    }.into_any()
                                };
                                view! {
                                    <tr class="border-t border-white/5">
                                        <td class="px-3 py-2 text-white whitespace-nowrap">{p.name}</td>
                                        <td class="px-3 py-2 whitespace-nowrap" style=format!("color: {accent};")>{p.alias}</td>
                                        {result_cell}
                                    </tr>
                                }
                            })
                            .collect::<Vec<_>>()}
                    </tbody>
                </table>
            </div>
        </details>
    }
}
