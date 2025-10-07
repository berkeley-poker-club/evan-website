use leptos::prelude::*;

#[component]
pub fn Sponsors() -> impl IntoView {
    view! {
        <section id="sponsors" class="section-padding bg-gray-800 relative">
            <div class="container-custom">
                <div class="text-center mb-16">
                    <h2 class="text-4xl md:text-5xl font-bold text-white mb-4">
                        "Our " <span class="gradient-text">"Partners"</span>
                    </h2>
                    <p class="text-xl text-gray-300 max-w-3xl mx-auto">
                        "We're proud to be supported by leading firms in finance, technology, and trading."
                    </p>
                </div>

                <div class="space-y-12">
                    <SponsorTier 
                        title="Gold Sponsors"
                        tier_color="from-yellow-400 to-yellow-600"
                        sponsors=vec![
                            SponsorInfo { name: "Jane Street", description: "Global trading firm and market maker", logo: "🏛️" },
                            SponsorInfo { name: "Jump Trading", description: "High-frequency trading and technology", logo: "📈" },
                        ]
                    />

                    <SponsorTier 
                        title="Silver Sponsors"
                        tier_color="from-gray-300 to-gray-500"
                        sponsors=vec![
                            SponsorInfo { name: "Hudson River Trading", description: "Algorithmic trading company", logo: "🌊" },
                        ]
                    />

                    <SponsorTier 
                        title="Bronze Sponsors"
                        tier_color="from-amber-600 to-amber-800"
                        sponsors=vec![
                            SponsorInfo { name: "SIG", description: "Susquehanna International Group", logo: "🔷" },
                        ]
                    />

                    <SponsorTier 
                        title="Tournament Partners"
                        tier_color="from-purple-400 to-purple-600"
                        sponsors=vec![
                            SponsorInfo { name: "SideBetz", description: "Poker tournament platform", logo: "🎰" },
                        ]
                    />
                </div>

                <div class="mt-16 text-center">
                    <p class="text-gray-300 text-lg mb-8">
                        "Interested in partnering with Poker at Berkeley?"
                    </p>
                    <button class="btn-primary">"Become a Sponsor"</button>
                </div>
            </div>
        </section>
    }
}

struct SponsorInfo {
    name: &'static str,
    description: &'static str,
    logo: &'static str,
}

#[component]
fn SponsorTier(
    title: &'static str,
    tier_color: &'static str,
    sponsors: Vec<SponsorInfo>
) -> impl IntoView {
    view! {
        <div>
            <div class="text-center mb-8">
                <h3 class=format!("text-2xl font-bold bg-gradient-to-r {} bg-clip-text text-transparent", tier_color)>
                    {title}
                </h3>
            </div>
            
            <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
                {sponsors.into_iter().map(|sponsor| {
                    view! {
                        <SponsorCard 
                            name=sponsor.name
                            description=sponsor.description
                            logo=sponsor.logo
                            tier_color=tier_color
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
    description: &'static str,
    logo: &'static str,
    tier_color: &'static str
) -> impl IntoView {
    view! {
        <div class="glass-card p-6 text-center transform hover:scale-105 transition-all duration-300 group">
            <div class=format!("w-20 h-20 mx-auto mb-4 bg-gradient-to-br {} rounded-full flex items-center justify-center text-3xl group-hover:animate-bounce", tier_color)>
                {logo}
            </div>
            <h4 class="text-xl font-bold text-white mb-2">{name}</h4>
            <p class="text-gray-300 text-sm">{description}</p>
        </div>
    }
}