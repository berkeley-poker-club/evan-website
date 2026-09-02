use leptos::prelude::*;
use std::cell::Cell;
use std::rc::Rc;
use std::time::Duration;

#[component]
pub fn SponsorsPage() -> impl IntoView {
    view! {
        <div class="min-h-screen">
            <TypingHero />
            <SponsorsSection />
            <BecomePartnerSection />
        </div>
    }
}

#[component]
fn TypingHero() -> impl IntoView {
    let (typed, set_typed) = signal(String::new());

    let names: &'static [&'static str] = &[
        "Jump Trading",
        "Jane Street",
        "QRT",
        "HRT",
        "Citadel Securities",
        "Duper",
        "PokerGFX",
        "Slowplay",
        "GTO Wizard",
        "BBO Poker Tables",
    ];

    let name_idx = Rc::new(Cell::new(0usize));
    let char_count = Rc::new(Cell::new(0usize));
    let deleting = Rc::new(Cell::new(false));
    let pause_ticks = Rc::new(Cell::new(0i32));

    set_interval(
        move || {
            if pause_ticks.get() > 0 {
                pause_ticks.set(pause_ticks.get() - 1);
                return;
            }

            let current_name = names[name_idx.get() % names.len()];
            let total_chars = current_name.chars().count();

            if !deleting.get() {
                let cc = char_count.get();
                if cc < total_chars {
                    char_count.set(cc + 1);
                    let text: String = current_name.chars().take(cc + 1).collect();
                    set_typed.set(text);
                } else {
                    deleting.set(true);
                    pause_ticks.set(17); // ~1.5s at 90ms/tick
                }
            } else {
                let cc = char_count.get();
                if cc > 0 {
                    char_count.set(cc - 1);
                    let text: String = current_name.chars().take(cc - 1).collect();
                    set_typed.set(text);
                } else {
                    deleting.set(false);
                    name_idx.set((name_idx.get() + 1) % names.len());
                }
            }
        },
        Duration::from_millis(90),
    );

    view! {
        <style>
            "@import url('https://fonts.googleapis.com/css2?family=Roboto+Mono:ital,wght@0,400;0,700;1,400;1,700&display=swap');
            @keyframes sponsorCursorBlink {
                0%, 50% { opacity: 1; }
                50.01%, 100% { opacity: 0; }
            }"
        </style>
        <section class="py-24 md:py-32" style="background: linear-gradient(to bottom, #0d1b3e, #060a14);">
            <div class="max-w-4xl mx-auto text-center px-6">
                <h1 class="text-5xl md:text-6xl font-bold text-white mb-4">
                    "Our Sponsors"
                </h1>
                <p class="text-xl text-white/90 mb-6">
                    "Proud to be supported by leading firms in finance, technology, and trading."
                </p>
                <div class="text-xl md:text-2xl font-bold" style="min-height: 1.2em; font-family: 'Roboto Mono', monospace;">
                    <span class="text-white">"Backed by "</span>
                    <span style="color: #F5C842;">{move || typed.get()}</span>
                    <span
                        class="ml-1"
                        style="color: #F5C842; font-weight: bold; animation: sponsorCursorBlink 0.7s step-end infinite;"
                    >
                        "|"
                    </span>
                </div>
            </div>
        </section>
    }
}

struct TierSponsor {
    name: &'static str,
    chip: &'static str,
    url: &'static str,
}

#[component]
fn SponsorsSection() -> impl IntoView {
    view! {
        <section class="py-20 bg-zinc-950">
            <div class="max-w-7xl mx-auto px-6 space-y-16">
                <TierSection
                    label="Platinum Partners"
                    color="#C0C0C0"
                    sponsors=vec![
                        TierSponsor {
                            name: "Jump Trading",
                            chip: "public/images/sponsors/chip-jump.webp",
                            url: "https://www.jumptrading.com/",
                        },
                    ]
                />

                <TierSection
                    label="Gold Partners"
                    color="#F5C842"
                    sponsors=vec![
                        TierSponsor {
                            name: "Jane Street",
                            chip: "public/images/sponsors/chip-janestreet.webp",
                            url: "https://www.janestreet.com/",
                        },
                        TierSponsor {
                            name: "QRT",
                            chip: "public/images/sponsors/chip-qrt.webp",
                            url: "https://www.qube-rt.com/",
                        },
                    ]
                />

                <TierSection
                    label="Silver Partners"
                    color="#A8A9AD"
                    sponsors=vec![
                        TierSponsor {
                            name: "HRT",
                            chip: "public/images/sponsors/chip-hrt.webp",
                            url: "https://www.hudsonrivertrading.com/",
                        },
                        TierSponsor {
                            name: "Citadel Securities",
                            chip: "public/images/sponsors/chip-citadel.webp",
                            url: "https://www.citadel.com/",
                        },
                    ]
                />

                <TierSection
                    label="Bronze Partners"
                    color="#CF885F"
                    sponsors=vec![
                        TierSponsor {
                            name: "Duper",
                            chip: "public/images/sponsors/chip-duper.webp",
                            url: "https://www.duper.gg/",
                        },
                        TierSponsor {
                            name: "PokerGFX",
                            chip: "public/images/sponsors/chip-pokergfx.webp",
                            url: "https://www.pokergfx.io/",
                        },
                    ]
                />

                <TierSection
                    label="Equipment Sponsors & Partnerships"
                    color="#9088BF"
                    sponsors=vec![
                        TierSponsor {
                            name: "Slowplay",
                            chip: "public/images/sponsors/chip-slowplay.webp",
                            url: "https://www.slowplay.store/",
                        },
                        TierSponsor {
                            name: "GTO Wizard",
                            chip: "public/images/sponsors/chip-gtowizard.webp",
                            url: "https://gtowizard.com/",
                        },
                        TierSponsor {
                            name: "BBO Poker Tables",
                            chip: "public/images/sponsors/chip-bbo.webp",
                            url: "https://www.bbopokertables.com/",
                        },
                    ]
                />
            </div>
        </section>
    }
}

#[component]
fn TierSection(
    label: &'static str,
    color: &'static str,
    sponsors: Vec<TierSponsor>,
) -> impl IntoView {
    view! {
        <div>
            <h2
                class="text-2xl md:text-3xl font-bold text-center mb-8"
                style=format!("color: {color};")
            >
                {label}
            </h2>
            <div class="flex flex-wrap justify-center gap-6">
                {sponsors
                    .into_iter()
                    .map(|s| view! {
                        <TierCard name=s.name chip=s.chip url=s.url color=color />
                    })
                    .collect::<Vec<_>>()
                }
            </div>
        </div>
    }
}

#[component]
fn TierCard(
    name: &'static str,
    chip: &'static str,
    url: &'static str,
    color: &'static str,
) -> impl IntoView {
    view! {
        <a
            href=url
            target="_blank"
            rel="noopener noreferrer"
            aria-label=format!("Visit {}", name)
            class="bg-zinc-900 rounded-xl p-1 flex items-center justify-center w-96 h-96 transition-transform duration-300 hover:scale-105"
            style=format!("border: 1px solid {color}66;")
        >
            <img
                src=chip
                alt=format!("{} poker chip", name)
                class="h-[365px] w-auto object-contain"
                loading="lazy"
            />
        </a>
    }
}

#[component]
fn BecomePartnerSection() -> impl IntoView {
    view! {
        <section class="py-20 bg-zinc-950">
            <div class="max-w-4xl mx-auto text-center px-6">
                <h2 class="text-4xl font-bold text-white mb-6">
                    "Become a Partner"
                </h2>
                <p class="text-lg text-gray-300 mb-8">
                    "Interested in partnering with Poker@Berkeley? Email "
                    <a href="mailto:sponsorships@poker.studentorg.berkeley.edu" class="underline" style="color: #F5C842;">"sponsorships@poker.studentorg.berkeley.edu"</a>
                    " or "
                    <a href="mailto:maysabarandish@berkeley.edu" class="underline" style="color: #F5C842;">"maysabarandish@berkeley.edu"</a>
                    " with inquiries."
                </p>
            </div>
        </section>
    }
}
