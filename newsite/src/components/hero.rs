use crate::components::poker_elements::*;
use leptos::prelude::*;

#[component]
pub fn Hero() -> impl IntoView {
    view! {
        <section id="home" class="relative min-h-screen flex items-center justify-center overflow-hidden bg-gradient-to-br from-gray-900 via-gray-800 to-black">
            <div class="absolute inset-0 opacity-20">
                <div class="poker-table absolute top-20 left-10 w-32 h-20 animate-bounce-slow"></div>
                <div class="poker-table absolute bottom-32 right-20 w-24 h-16 animate-bounce-slow" style="animation-delay: 1s;"></div>
                <FloatingCards />
            </div>

            <div class="container-custom text-center relative z-10">
                <div class="animate-fade-in">
                    <div class="flex justify-center mb-8">
                        <div class="flex space-x-2">
                            <PokerCard suit="♠" value="A" color="black" />
                            <PokerCard suit="♥" value="K" color="red" />
                        </div>
                    </div>

                    <h1 class="text-6xl md:text-8xl font-bold text-white mb-6">
                        <span class="gradient-text">"Poker"</span>
                        <br/>
                        <span class="text-white">"at Berkeley"</span>
                    </h1>

                    <p class="text-xl md:text-2xl text-gray-300 mb-12 max-w-3xl mx-auto">
                        "UC Berkeley's Premier Poker Organization"
                        <br/>
                        "Where Strategy Meets Community"
                    </p>

                    <div class="flex flex-col sm:flex-row gap-6 justify-center items-center animate-slide-up">
                        <button class="btn-secondary text-lg">"Join Our Community"</button>
                        <button class="btn-secondary text-lg">"Learn About DeCal"</button>
                    </div>

                    <div class="mt-16 grid grid-cols-1 md:grid-cols-3 gap-8 max-w-4xl mx-auto">
                        <StatCard number="500+" label="Active Members" icon="" />
                        <StatCard number="20+" label="Weekly Games" icon="" />
                        <StatCard number="2x" label="Annual Tournaments" icon="" />
                    </div>
                </div>
            </div>

            <div class="absolute bottom-8 left-1/2 transform -translate-x-1/2 animate-bounce">
                <svg class="w-6 h-6 text-white/60" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 14l-7 7m0 0l-7-7m7 7V3"></path>
                </svg>
            </div>
        </section>
    }
}

#[component]
fn StatCard(number: &'static str, label: &'static str, icon: &'static str) -> impl IntoView {
    view! {
        <div class="glass-card p-6 text-center transform hover:scale-105 transition-all duration-300">
            <div class="text-3xl mb-2">{icon}</div>
            <div class="text-3xl font-bold text-white mb-2">{number}</div>
            <div class="text-gray-300">{label}</div>
        </div>
    }
}
