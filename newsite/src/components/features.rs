use leptos::prelude::*;

#[component]
pub fn Features() -> impl IntoView {
    view! {
        <section id="tournaments" class="section-padding bg-gradient-to-br from-gray-900 to-black relative">
            <div class="container-custom">
                <div class="text-center mb-16">
                    <h2 class="text-4xl md:text-5xl font-bold text-white mb-4">
                        "What We " <span class="gradient-text">"Offer"</span>
                    </h2>
                    <p class="text-xl text-gray-300 max-w-3xl mx-auto">
                        "From weekly games to academic courses, we provide comprehensive poker education and community experiences."
                    </p>
                </div>

                <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-8">
                    <FeatureCard 
                        icon="🎮"
                        title="Game Nights"
                        description="Weekly tournaments and cash games with players of all skill levels"
                        gradient="from-blue-500 to-blue-700"
                    />
                    <FeatureCard 
                        icon="🎓"
                        title="Poker DeCal"
                        description="Stat 198: Learn poker theory, strategy, and fundamentals for academic credit"
                        gradient="from-green-500 to-green-700"
                    />
                    <FeatureCard 
                        icon="🏆"
                        title="Tournaments"
                        description="Berkeley x Stanford Spring Tournament and other competitive events"
                        gradient="from-purple-500 to-purple-700"
                    />
                    <FeatureCard 
                        icon="💼"
                        title="Professional Development"
                        description="Network with industry professionals and develop career connections"
                        gradient="from-orange-500 to-orange-700"
                    />
                </div>

                <div class="mt-20">
                    <TournamentHighlight />
                </div>
            </div>
        </section>
    }
}

#[component]
fn FeatureCard(icon: &'static str, title: &'static str, description: &'static str, gradient: &'static str) -> impl IntoView {
    view! {
        <div class="glass-card p-6 text-center transform hover:scale-105 hover:-translate-y-2 transition-all duration-300 group">
            <div class=format!("w-16 h-16 mx-auto mb-4 bg-gradient-to-br {} rounded-full flex items-center justify-center text-2xl group-hover:animate-bounce", gradient)>
                {icon}
            </div>
            <h3 class="text-xl font-bold text-white mb-3">{title}</h3>
            <p class="text-gray-300">{description}</p>
        </div>
    }
}

#[component]
fn TournamentHighlight() -> impl IntoView {
    view! {
        <div class="glass-card p-8 lg:p-12">
            <div class="grid grid-cols-1 lg:grid-cols-2 gap-8 items-center">
                <div>
                    <h3 class="text-3xl font-bold text-white mb-4">
                        <span class="gradient-text">"Berkeley x Stanford"</span>
                        <br/>
                        "Spring 2025 Tournament"
                    </h3>
                    <p class="text-gray-300 text-lg mb-6">
                        "Join us for our premier tournament event featuring players from both UC Berkeley and Stanford University."
                    </p>
                    
                    <div class="grid grid-cols-1 sm:grid-cols-2 gap-4 mb-8">
                        <TournamentDetail label="Main Event" value="April 26-27, 2025" />
                        <TournamentDetail label="Qualifiers" value="April 19, 2025" />
                        <TournamentDetail label="Presented by" value="SideBetz" />
                        <TournamentDetail label="Format" value="No-Limit Hold'em" />
                    </div>

                    <div class="flex flex-col sm:flex-row gap-4">
                        <button class="btn-primary">"Register Now"</button>
                        <button class="btn-secondary">"Learn More"</button>
                    </div>
                </div>

                <div class="relative">
                    <div class="bg-gradient-to-br from-poker-gold-500 to-poker-gold-700 rounded-2xl p-6 transform rotate-3 hover:rotate-0 transition-transform duration-300">
                        <div class="bg-white rounded-xl p-6">
                            <div class="text-center">
                                <div class="text-4xl mb-4">"🏆"</div>
                                <h4 class="text-2xl font-bold text-gray-800 mb-2">"Champion"</h4>
                                <p class="text-gray-600">"Winner takes all"</p>
                                <div class="mt-4 text-3xl font-bold gradient-text">"$10,000+"</div>
                                <p class="text-sm text-gray-500">"Prize Pool"</p>
                            </div>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    }
}

#[component]
fn TournamentDetail(label: &'static str, value: &'static str) -> impl IntoView {
    view! {
        <div class="bg-gray-800/50 rounded-lg p-3">
            <div class="text-sm text-gray-400">{label}</div>
            <div class="text-white font-semibold">{value}</div>
        </div>
    }
}