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
                                description: ""
                            },
                        ]
                        tier_class="border-white/30 bg-gradient-to-br from-slate-600/80 via-slate-500/80 to-slate-700/80 dark:from-slate-800/70 dark:to-slate-900/70"
                    />

                    <SponsorTier
                        title="Gold Sponsors"
                        description=""
                        sponsors=vec![
                            SponsorInfo {
                                name: "Jane Street",
                                logo: "public/images/sponsors/js.png",
                                description: ""
                            },
                        ]
                        tier_class="border-amber-300/60 bg-gradient-to-br from-amber-400/70 via-amber-300/70 to-amber-500/70 dark:from-amber-700/60 dark:to-amber-800/60"
                    />

                    <SponsorTier
                        title="Silver Sponsors"
                        description=""
                        sponsors=vec![
                            SponsorInfo {
                                name: "Citadel",
                                logo: "public/images/sponsors/citadel.png",
                                description: ""
                            },
                            SponsorInfo {
                                name: "HRT",
                                logo: "public/images/sponsors/hrt.png",
                                description: ""
                            },
                        ]
                        tier_class="border-white/20 bg-gradient-to-br from-gray-400/70 via-gray-300/70 to-gray-500/70 dark:from-gray-700/60 dark:to-gray-800/60"
                    />

                    <SponsorTier
                        title="Equipment Sponsors & Partnerships"
                        description=""
                        sponsors=vec![
                            SponsorInfo {
                                name: "Slowplay",
                                logo: "public/images/sponsors/slowplay.png",
                                description: ""
                            },
                            SponsorInfo {
                                name: "GTOW",
                                logo: "public/images/sponsors/gtow.png",
                                description: ""
                            },
                            SponsorInfo {
                                name: "BBO",
                                logo: "public/images/sponsors/BBO.png",
                                description: ""
                            },
                        ]
                        tier_class="border-amber-600 bg-gradient-to-r from-amber-50 to-amber-100 dark:from-amber-900 dark:to-amber-800"
                    />
                </div>
            </div>
        </section>
    }
}

struct SponsorInfo {
    name: &'static str,
    logo: &'static str,
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
    _description: &'static str,
) -> impl IntoView {
    view! {
        // Fixed box size (same for all tiers) + centered
        <div class="flex items-center justify-center w-44 h-24 md:w-52 md:h-28">
            <img
                src=logo
                alt=name
                class="max-w-full max-h-full object-contain bg-transparent"
                loading="lazy"
                decoding="async"
                style="filter: drop-shadow(0 1px 1px rgba(0,0,0,0.15));"
            />
        </div>
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
