use leptos::prelude::*;

#[component]
pub fn Team() -> impl IntoView {
    view! {
        <section id="team" class="section-padding bg-gradient-to-br from-black to-gray-900 relative">
            <div class="container-custom">
                <div class="text-center mb-16">
                    <h2 class="text-4xl md:text-5xl font-bold text-white mb-4">
                        "Meet Our " <span class="gradient-text">"Team"</span>
                    </h2>
                    <p class="text-xl text-gray-300 max-w-3xl mx-auto">
                        "Dedicated leaders passionate about building Berkeley's poker community."
                    </p>
                </div>

                <div class="space-y-16">
                    <TeamSection 
                        title="Executive Board"
                        members=vec![
                            TeamMember { 
                                name: "Welford Chen", 
                                role: "President", 
                                bio: "Leading strategic initiatives and community growth",
                                avatar: "👤"
                            },
                            TeamMember { 
                                name: "Jones Dickerson", 
                                role: "Vice President", 
                                bio: "Coordinating events and member engagement",
                                avatar: "👤"
                            },
                            TeamMember { 
                                name: "Maysa Barandish", 
                                role: "Treasurer", 
                                bio: "Managing finances and sponsor relationships",
                                avatar: "👤"
                            },
                            TeamMember { 
                                name: "Mete Ehliz", 
                                role: "Secretary", 
                                bio: "Communications and organizational management",
                                avatar: "👤"
                            },
                        ]
                    />

                    <TeamSection 
                        title="DeCal Instructors"
                        members=vec![
                            TeamMember { 
                                name: "Welford Chen", 
                                role: "Lead Instructor", 
                                bio: "Expert in poker theory and strategy",
                                avatar: "🎓"
                            },
                            TeamMember { 
                                name: "Jones Dickerson", 
                                role: "Instructor", 
                                bio: "Specializes in game theory applications",
                                avatar: "🎓"
                            },
                            TeamMember { 
                                name: "Maysa Barandish", 
                                role: "Instructor", 
                                bio: "Focus on bankroll management and psychology",
                                avatar: "🎓"
                            },
                            TeamMember { 
                                name: "Mete Ehliz", 
                                role: "Instructor", 
                                bio: "Statistical analysis and probability expert",
                                avatar: "🎓"
                            },
                        ]
                    />
                </div>

                <div class="mt-16 text-center">
                    <div class="glass-card p-8 max-w-4xl mx-auto">
                        <h3 class="text-2xl font-bold text-white mb-4">"Alumni Success Stories"</h3>
                        <p class="text-gray-300 text-lg mb-6">
                            "Our graduates have gone on to successful careers at top firms including Goldman Sachs, Two Sigma, Citadel, and many more."
                        </p>
                        <div class="flex justify-center space-x-4 text-3xl">
                            {"💼 🏛️ 📈 💻 🎯"}
                        </div>
                    </div>
                </div>
            </div>
        </section>
    }
}

struct TeamMember {
    name: &'static str,
    role: &'static str,
    bio: &'static str,
    avatar: &'static str,
}

#[component]
fn TeamSection(title: &'static str, members: Vec<TeamMember>) -> impl IntoView {
    view! {
        <div>
            <h3 class="text-3xl font-bold text-white text-center mb-12">{title}</h3>
            <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-8">
                {members.into_iter().map(|member| {
                    view! {
                        <MemberCard 
                            name=member.name
                            role=member.role
                            bio=member.bio
                            avatar=member.avatar
                        />
                    }
                }).collect::<Vec<_>>()}
            </div>
        </div>
    }
}

#[component]
fn MemberCard(name: &'static str, role: &'static str, bio: &'static str, avatar: &'static str) -> impl IntoView {
    view! {
        <div class="glass-card p-6 text-center transform hover:scale-105 transition-all duration-300 group">
            <div class="w-20 h-20 mx-auto mb-4 bg-gradient-to-br from-poker-gold-500 to-poker-gold-600 rounded-full flex items-center justify-center text-3xl group-hover:animate-bounce">
                {avatar}
            </div>
            <h4 class="text-xl font-bold text-white mb-1">{name}</h4>
            <p class="text-poker-gold-400 font-semibold mb-3">{role}</p>
            <p class="text-gray-300 text-sm">{bio}</p>
            
            <button class="mt-4 text-sm text-poker-gold-400 hover:text-poker-gold-300 transition-colors">
                "View Profile →"
            </button>
        </div>
    }
}