use leptos::prelude::*;

#[component]
pub fn Decal() -> impl IntoView {
    view! {
        <section id="decal" class="section-padding bg-gradient-to-br from-berkeley-blue-900 to-berkeley-blue-800 relative overflow-hidden">
            <div class="absolute inset-0 opacity-10">
                <div class="text-9xl font-bold text-white/5 absolute top-20 left-20 transform -rotate-12">"📚"</div>
                <div class="text-6xl text-white/5 absolute bottom-20 right-20">"🎓"</div>
            </div>

            <div class="container-custom relative z-10">
                <div class="text-center mb-16">
                    <h2 class="text-4xl md:text-5xl font-bold text-white mb-4">
                        <span class="gradient-text">"Poker DeCal"</span>
                        <br/>
                        "Stat 198: Theory & Fundamentals"
                    </h2>
                    <p class="text-xl text-blue-100 max-w-3xl mx-auto">
                        "Learn poker strategy, game theory, and mathematical fundamentals for academic credit at UC Berkeley."
                    </p>
                </div>

                <div class="grid grid-cols-1 lg:grid-cols-2 gap-12 items-center">
                    <div class="space-y-8">
                        <div class="glass-card p-6">
                            <h3 class="text-2xl font-bold text-white mb-4">"Course Overview"</h3>
                            <div class="space-y-4">
                                <CourseDetail label="Credits" value="2 Units (P/NP)" />
                                <CourseDetail label="Prerequisites" value="None" />
                                <CourseDetail label="Format" value="90min Theory + 30min Practice" />
                                <CourseDetail label="Applications" value="Open for Spring 2025" />
                            </div>
                        </div>

                        <div class="space-y-4">
                            <LearningObjective 
                                icon="🎯"
                                text="Master fundamental poker rules and terminology"
                            />
                            <LearningObjective 
                                icon="🧠"
                                text="Apply game theory and statistical analysis"
                            />
                            <LearningObjective 
                                icon="💡"
                                text="Develop logical thinking and decision-making skills"
                            />
                            <LearningObjective 
                                icon="📊"
                                text="Understand bankroll management and psychology"
                            />
                        </div>
                    </div>

                    <div class="space-y-6">
                        <div class="glass-card p-8">
                            <h3 class="text-2xl font-bold text-white mb-6 text-center">"Grading Breakdown"</h3>
                            <div class="space-y-4">
                                <GradeComponent 
                                    category="Lecture Attendance"
                                    percentage=30
                                    color="from-blue-500 to-blue-600"
                                />
                                <GradeComponent 
                                    category="Homework Assignments"
                                    percentage=40
                                    color="from-green-500 to-green-600"
                                />
                                <GradeComponent 
                                    category="Final Project"
                                    percentage=20
                                    color="from-purple-500 to-purple-600"
                                />
                                <GradeComponent 
                                    category="Playing Sessions"
                                    percentage=10
                                    color="from-orange-500 to-orange-600"
                                />
                            </div>
                            <div class="mt-6 pt-4 border-t border-white/20">
                                <p class="text-center text-blue-100">
                                    <span class="font-semibold">"Passing Threshold: "</span>
                                    <span class="text-poker-gold-400 font-bold">"70%"</span>
                                </p>
                            </div>
                        </div>

                        <div class="text-center">
                            <div class="mb-6">
                                <p class="text-blue-100 mb-4">"Ready to join our next cohort?"</p>
                                <div class="flex flex-col sm:flex-row gap-4 justify-center">
                                    <button class="btn-primary">"Apply Now"</button>
                                    <button class="btn-secondary bg-white/20 hover:bg-white/30 border-white/30">"View Syllabus"</button>
                                </div>
                            </div>
                        </div>
                    </div>
                </div>
            </div>
        </section>
    }
}

#[component]
fn CourseDetail(label: &'static str, value: &'static str) -> impl IntoView {
    view! {
        <div class="flex justify-between items-center py-2 border-b border-white/10 last:border-b-0">
            <span class="text-blue-200">{label}</span>
            <span class="text-white font-semibold">{value}</span>
        </div>
    }
}

#[component]
fn LearningObjective(icon: &'static str, text: &'static str) -> impl IntoView {
    view! {
        <div class="flex items-start space-x-4">
            <div class="flex-shrink-0 w-10 h-10 bg-poker-gold-500 rounded-lg flex items-center justify-center text-lg">
                {icon}
            </div>
            <p class="text-blue-100 pt-2">{text}</p>
        </div>
    }
}

#[component]
fn GradeComponent(category: &'static str, percentage: u32, color: &'static str) -> impl IntoView {
    view! {
        <div class="space-y-2">
            <div class="flex justify-between items-center">
                <span class="text-white font-medium">{category}</span>
                <span class="text-white font-bold">{percentage}"%"</span>
            </div>
            <div class="w-full bg-gray-700 rounded-full h-2">
                <div 
                    class=format!("bg-gradient-to-r {} h-2 rounded-full transition-all duration-1000 ease-out", color)
                    style=format!("width: {}%", percentage)
                ></div>
            </div>
        </div>
    }
}