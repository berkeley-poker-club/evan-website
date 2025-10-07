use crate::components::poker_elements::*;
use leptos::prelude::*;

#[component]
pub fn About() -> impl IntoView {
    view! {
        <section id="about" class="section-padding bg-gray-800 relative overflow-hidden">
            <div class="absolute top-10 right-10 opacity-10">
                <ChipStack />
            </div>

            <div class="container-custom">
                <div class="grid grid-cols-1 lg:grid-cols-2 gap-12 items-center">
                    <div class="space-y-6">
                        <h2 class="text-4xl md:text-5xl font-bold text-white">
                            "About "
                            <span class="gradient-text">"Our Community"</span>
                        </h2>

                        <p class="text-xl text-gray-300 leading-relaxed">
                            "We're UC Berkeley's premier poker organization, dedicated to fostering a community of strategic thinkers, skilled players, and passionate learners."
                        </p>

                        <div class="space-y-4">
                            <FeaturePoint
                                icon=""
                                title="Academic Excellence"
                                description="Our members excel both at the tables and in their studies, with many pursuing careers in finance, tech, and academia."
                            />
                            <FeaturePoint
                                icon=""
                                title="Professional Network"
                                description="Connect with industry professionals and build relationships that extend far beyond the poker table."
                            />
                            <FeaturePoint
                                icon=""
                                title="Skill Development"
                                description="Learn advanced strategy, bankroll management, and decision-making skills applicable to all areas of life."
                            />
                        </div>
                    </div>

                    <div class="relative">
                        <div class="glass-card p-8 transform hover:scale-105 transition-all duration-300">
                            <div class="poker-table w-full h-64 mx-auto relative flex items-center justify-center">
                                <div class="grid grid-cols-3 gap-4">
                                    <PokerCard suit="♠" value="A" color="black" />
                                    <PokerCard suit="♥" value="A" color="red" />
                                    <PokerCard suit="♦" value="A" color="red" />
                                </div>
                            </div>
                            <div class="text-center mt-6">
                                <p class="text-white font-semibold">"Weekly Game Night"</p>
                                <p class="text-gray-300 text-sm">"Join us every Friday at 7 PM"</p>
                            </div>
                        </div>
                    </div>
                </div>
            </div>
        </section>
    }
}

#[component]
fn FeaturePoint(
    icon: &'static str,
    title: &'static str,
    description: &'static str,
) -> impl IntoView {
    view! {
        <div class="flex items-start space-x-4">
            <div class="flex-shrink-0 w-12 h-12 bg-gradient-to-br from-poker-gold-500 to-poker-gold-600 rounded-lg flex items-center justify-center text-xl">
                {icon}
            </div>
            <div>
                <h3 class="text-lg font-semibold text-white mb-2">{title}</h3>
                <p class="text-gray-300">{description}</p>
            </div>
        </div>
    }
}
