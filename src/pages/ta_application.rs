use leptos::prelude::*;
use leptos_router::components::A;

const DECAL_TA_APPLICATION_FORM: &str = "https://forms.gle/ZJyBk9brK8iRuAtR6";

#[component]
pub fn TaApplicationPage() -> impl IntoView {
    view! {
        <div class="min-h-screen">
            <HeroBanner />
            <ContentSection />
        </div>
    }
}

#[component]
fn HeroBanner() -> impl IntoView {
    view! {
        <section class="relative py-32"
                 style="background-image: url('/public/images/sp26board/Q62A0898.webp'); background-size: cover; background-position: center;">
            <div class="absolute inset-0" style="background-color: rgba(0, 0, 0, 0.45);"></div>
            <div class="relative z-10 max-w-4xl mx-auto text-center px-6">
                <h1 class="text-5xl md:text-6xl font-bold text-white mb-4">
                    "Become a Stat 198 TA"
                </h1>
                <p class="text-xl text-white/90">
                    "Join the instructional team behind the Poker DeCal"
                </p>
            </div>
        </section>
    }
}

#[component]
fn ContentSection() -> impl IntoView {
    view! {
        <section class="py-20 bg-white dark:bg-gray-800">
            <div class="max-w-4xl mx-auto px-6">
                <p class="text-lg text-gray-700 dark:text-gray-300 mb-10">
                    "STAT 198: Poker Theory & Fundamentals is a student-run DeCal on game theory, probability, and strategic decision-making through poker. Every semester, we bring on undergraduate TAs to help run it."
                </p>

                <h2 class="text-2xl font-bold text-gray-900 dark:text-white mb-4">
                    "What the role looks like:"
                </h2>
                <ul class="space-y-3 text-lg text-gray-700 dark:text-gray-300 mb-10">
                    <li class="flex items-start space-x-3">
                        <div class="w-2 h-2 bg-blue-600 dark:bg-blue-400 rounded-full mt-2 flex-shrink-0"></div>
                        <span>"Supporting in-class instruction and behind-the-scenes course operations"</span>
                    </li>
                    <li class="flex items-start space-x-3">
                        <div class="w-2 h-2 bg-blue-600 dark:bg-blue-400 rounded-full mt-2 flex-shrink-0"></div>
                        <span>"Collaborating with the team on content, delivery, and student engagement"</span>
                    </li>
                    <li class="flex items-start space-x-3">
                        <div class="w-2 h-2 bg-blue-600 dark:bg-blue-400 rounded-full mt-2 flex-shrink-0"></div>
                        <span>"Growing into a teaching role over time if that's something you want"</span>
                    </li>
                </ul>

                <h2 class="text-2xl font-bold text-gray-900 dark:text-white mb-4">
                    "What we're looking for:"
                </h2>
                <ul class="space-y-3 text-lg text-gray-700 dark:text-gray-300">
                    <li class="flex items-start space-x-3">
                        <div class="w-2 h-2 bg-blue-600 dark:bg-blue-400 rounded-full mt-2 flex-shrink-0"></div>
                        <span>"Genuine interest in poker, strategy, or teaching — no prior TA experience needed"</span>
                    </li>
                    <li class="flex items-start space-x-3">
                        <div class="w-2 h-2 bg-blue-600 dark:bg-blue-400 rounded-full mt-2 flex-shrink-0"></div>
                        <span>"TAs receive 2 units per semester"</span>
                    </li>
                </ul>

                <div class="flex flex-col sm:flex-row gap-4 sm:gap-6 justify-center mt-10">
                    <a
                        href=DECAL_TA_APPLICATION_FORM
                        target="_blank"
                        rel="noopener"
                        class="min-w-[220px] inline-flex items-center justify-center text-center bg-slate-600 hover:bg-slate-700 text-white font-semibold py-3 px-8 rounded-lg transition-colors"
                    >
                        "TA Application"
                    </a>
                    <A
                        href="/decal#course-staff"
                        attr:class="min-w-[220px] inline-flex items-center justify-center text-center bg-[#B2A08E]/70 hover:bg-[#9A6A4C] text-white font-semibold py-3 px-8 rounded-lg transition-colors"
                    >
                        "Meet the Course Staff"
                    </A>
                </div>

                <p class="text-center text-gray-500 dark:text-gray-400 mt-16">
                    "Interested in becoming a lecturer? Reach out to "
                    <a href="mailto:maysabarandish@berkeley.edu" class="underline hover:text-blue-600 dark:hover:text-blue-400 transition-colors">"maysabarandish@berkeley.edu"</a>
                    ". For any other questions, find us on the Poker Theory Discord or email Stat 198 Staff at "
                    <a href="mailto:decal@poker.studentorg.berkeley.edu" class="underline hover:text-blue-600 dark:hover:text-blue-400 transition-colors">"decal@poker.studentorg.berkeley.edu"</a>
                    "."
                </p>
            </div>
        </section>
    }
}
