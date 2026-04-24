use leptos::prelude::*;

#[component]
pub fn SponsorsPage() -> impl IntoView {
    view! {
        <div class="min-h-screen">
            <HeroBanner />
            <SponsorsSection />
        </div>
    }
}

#[component]
fn HeroBanner() -> impl IntoView {
    view! {
        <section id="banner" class="py-32 bg-gradient-to-r from-gray-900 to-gray-800 dark:from-gray-950 dark:to-gray-900">
            <div class="max-w-4xl mx-auto text-center px-6">
                <h1 class="text-5xl md:text-6xl font-bold text-white mb-4">
                    "Our Sponsors"
                </h1>
                <p class="text-xl text-white/90 mb-4">
                    "Proud to be supported by leading firms in finance, technology, and trading"
                </p>
                <p class="text-lg text-blue-300 dark:text-blue-400 font-semibold">
                    "Interested in partnering with Poker at Berkeley? Email davidchen2027@berkeley.edu with inquiries."
                </p>
            </div>
        </section>
    }
}

#[component]
fn SponsorsSection() -> impl IntoView {
    view! {
        <section class="py-20 bg-white dark:bg-gray-800">
            <div class="max-w-6xl mx-auto px-6">
                <div class="space-y-16">
                    <SponsorTier
                        title="Platinum Sponsors"
                        description=""
                        sponsors=vec![
                            SponsorInfo {
                                name: "Jump Trading",
                                logo: "public/images/sponsors/jump.png",
                                url: "https://www.jumptrading.com/",
                                description: ""
                            },
                        ]
                        tier_class="border-sky-200/70 bg-gradient-to-br from-sky-300/75 via-blue-400/75 to-blue-500/75 dark:from-sky-400/65 dark:via-blue-500/65 dark:to-indigo-600/65"
                    />

                    <SponsorTier
                        title="Gold Sponsors"
                        description=""
                        sponsors=vec![
                            SponsorInfo {
                                name: "Jane Street",
                                logo: "public/images/sponsors/js.png",
                                url: "https://www.janestreet.com/",
                                description: ""
                            },
                            SponsorInfo {
                                name: "QRT",
                                logo: "public/images/sponsors/qrt.jpg",
                                url: "https://www.qube-rt.com/",
                                description: ""
                            },
                        ]
                        tier_class="border-amber-300/60 bg-gradient-to-br from-amber-400/70 via-amber-500/70 to-amber-600/70 dark:from-amber-500/60 dark:via-amber-600/60 dark:to-amber-700/60"
                    />

                    <SponsorTier
                        title="Silver Sponsors"
                        description=""
                        sponsors=vec![
                            SponsorInfo {
                                name: "Citadel",
                                logo: "public/images/sponsors/citadel.png",
                                url: "https://www.citadel.com/",
                                description: ""
                            },
                            SponsorInfo {
                                name: "HRT",
                                logo: "public/images/sponsors/hrt.png",
                                url: "https://www.hudsonrivertrading.com/",
                                description: ""
                            },
                        ]
                        tier_class="border-white/20 bg-gradient-to-br from-gray-300/70 via-gray-400/70 to-gray-500/70 dark:from-gray-400/60 dark:via-gray-500/60 dark:to-gray-600/60"
                    />

                    <SponsorTier
                        title="Bronze Sponsors"
                        description=""
                        sponsors=vec![
                            SponsorInfo {
                                name: "Duper",
                                logo: "public/images/sponsors/duper.png",
                                url: "https://www.duper.gg/",
                                description: ""
                            },
                            SponsorInfo {
                                name: "PokerGFX",
                                logo: "public/images/sponsors/pokergfx.png",
                                url: "https://www.pokergfx.io/",
                                description: ""
                            },
                        ]
                        tier_class="border-orange-300/60 bg-gradient-to-br from-orange-400/70 via-orange-500/70 to-orange-600/70"
                    />

                    <SponsorTier
                        title="Equipment Sponsors & Partnerships"
                        description=""
                        sponsors=vec![
                            SponsorInfo {
                                name: "Slowplay",
                                logo: "public/images/sponsors/slowplay.png",
                                url: "https://www.slowplay.store/",
                                description: ""
                            },
                            SponsorInfo {
                                name: "GTOW",
                                logo: "public/images/sponsors/gtow.png",
                                url: "https://gtowizard.com/",
                                description: ""
                            },
                            SponsorInfo {
                                name: "BBO",
                                logo: "public/images/sponsors/BBO.png",
                                url: "https://www.bbopokertables.com/",
                                description: ""
                            },
                        ]
                        tier_class="border-emerald-300/60 bg-gradient-to-br from-emerald-400/70 via-emerald-500/70 to-emerald-600/70 dark:from-emerald-500/60 dark:via-emerald-600/60 dark:to-emerald-700/60"
                    />
                </div>
            </div>
        </section>
    }
}

struct SponsorInfo {
    name: &'static str,
    logo: &'static str,
    url: &'static str,
    description: &'static str,
}

#[component]
fn SponsorTier(
    title: &'static str,
    description: &'static str,
    sponsors: Vec<SponsorInfo>,
    tier_class: &'static str,
) -> impl IntoView {
    view! {
        <div class="relative rounded-xl shadow-lg shadow-black/20 overflow-hidden">

            <div class="absolute inset-0 backdrop-blur-sm bg-white/5 dark:bg-black/30 pointer-events-none"></div>

            <div class=(move || format!(
                "relative border rounded-xl p-8 ring-1 ring-white/10 {}",
                tier_class
            ))>
                <div class="text-center mb-8">
                    <h2 class="text-3xl font-bold text-gray-900 dark:text-white mb-3">
                        {title}
                    </h2>

                    {if !description.is_empty() {
                        view! {
                            <p class="text-lg text-gray-700 dark:text-gray-300">
                                {description}
                            </p>
                        }.into_any()
                    } else {
                        view! { <></> }.into_any()
                    }}
                </div>

                <div class="flex flex-wrap justify-center gap-8">
                    {sponsors
                        .into_iter()
                        .map(|sponsor| view! {
                            <SponsorCard
                                name=sponsor.name
                                logo=sponsor.logo
                                url=sponsor.url
                                _description=sponsor.description
                            />
                        })
                        .collect::<Vec<_>>()
                    }
                </div>
            </div>
        </div>
    }
}

#[component]
fn SponsorCard(
    name: &'static str,
    logo: &'static str,
    url: &'static str,
    _description: &'static str,
) -> impl IntoView {
    view! {
        <a
            href=url
            target="_blank"
            rel="noopener noreferrer"
            aria-label=(format!("Visit {}", name))
            class="group rounded-lg focus:outline-none focus-visible:ring-2 focus-visible:ring-white/40"
        >
            // Fixed box size (same for all tiers) + centered
            <div class="flex items-center justify-center w-44 h-24 md:w-52 md:h-28">
                <img
                    src=logo
                    alt=name
                    class="max-w-full max-h-full object-contain bg-transparent transition-transform duration-300 group-hover:scale-105"
                    loading="lazy"
                    decoding="async"
                    style="filter: drop-shadow(0 1px 1px rgba(0,0,0,0.15));"
                />
            </div>
        </a>
    }
}

#[component]
fn BecomePartnerSection() -> impl IntoView {
    view! {
        <section class="py-20 bg-gray-50">
            <div class="max-w-4xl mx-auto text-center px-6">
                <h2 class="text-4xl font-bold text-gray-900 mb-6">
                    "Become a Partner"
                </h2>
                <p class="text-lg text-gray-700 mb-8">
                    "Interested in partnering with Poker at Berkeley? Email yeager@berkeley.edu with sponsorship inquiries."
                </p>
            </div>
        </section>
    }
}
