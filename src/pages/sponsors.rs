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
        <section id="banner" class="py-32 bg-gradient-to-r from-gray-900 to-gray-800">
            <div class="max-w-4xl mx-auto text-center px-6">
                <h1 class="text-5xl md:text-6xl font-bold text-white mb-4">
                    "Our Sponsors"
                </h1>
                <p class="text-xl text-white/90 mb-4">
                    "Proud to be supported by leading firms in finance, technology, and trading"
                </p>
                <p class="text-lg text-blue-300 font-semibold">
                    "Interested in partnering with Poker at Berkeley? Email yeager@berkeley.edu with inquiries."
                </p>
            </div>
        </section>
    }
}

#[component]
fn SponsorsSection() -> impl IntoView {
    view! {
        <section class="py-20 bg-white">
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
                        tier_class="border-purple-400 bg-gradient-to-r from-purple-50 to-purple-100"
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
                        tier_class="border-yellow-400 bg-gradient-to-r from-yellow-50 to-yellow-100"
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
                        tier_class="border-gray-400 bg-gradient-to-r from-gray-50 to-gray-100"
                    />

                    <SponsorTier
                        title="Equipment Sponsors"
                        description=""
                        sponsors=vec![
                            SponsorInfo {
                                name: "Slowplay",
                                logo: "public/images/sponsors/slowplay.png",
                                description: ""
                            },
                        ]
                        tier_class="border-amber-600 bg-gradient-to-r from-amber-50 to-amber-100"
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
        <div class=format!("border-2 rounded-lg p-8 {}", tier_class)>
            <div class="text-center mb-8">
                <h2 class="text-3xl font-bold text-gray-900 mb-3">{title}</h2>
                <p class="text-lg text-gray-700">{description}</p>
            </div>

            <div class="flex flex-wrap justify-center gap-8">
                {sponsors.into_iter().map(|sponsor| {
                    view! {
                        <SponsorCard
                            name=sponsor.name
                            logo=sponsor.logo
                            _description=sponsor.description
                        />
                    }
                }).collect::<Vec<_>>()}
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
