use leptos::prelude::*;
use leptos_meta::Title;

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

struct Episode {
    number: &'static str,
    title: &'static str,
    platform: Platform,
    embed_src: &'static str,
}

const STREAM_EPISODES: &[Episode] = &[Episode {
    number: "EP. 00",
    title: "Episode 0 (Tester)",
    platform: Platform::YouTube,
    embed_src: "https://www.youtube.com/embed/3kktPT5cJnE",
}];

const TOURNAMENT_EPISODES: &[Episode] = &[Episode {
    number: "VOL. 03",
    title: "FINAL DAY Highlights - 3rd annual Berkeley x Stanford Poker Tournament!",
    platform: Platform::YouTube,
    embed_src: "https://www.youtube.com/embed/xwhVQmdWD0k",
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

                <div class="grid grid-cols-1 md:grid-cols-2 gap-10">
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
                <span class="font-mono text-xs tracking-[0.2em]" style=number_style>{episode.number}</span>
                <h3 class="text-white font-bold text-lg leading-snug mt-1">{episode.title}</h3>
            </div>
        </div>
    }
}
