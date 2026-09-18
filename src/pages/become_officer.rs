use leptos::prelude::*;

const OFFICER_APPLICATION_FORM: &str = "https://forms.gle/2cWGidGdtvHewArk6";
const DECAL_TA_APPLICATION_FORM: &str = "https://forms.gle/ZJyBk9brK8iRuAtR6";

#[component]
pub fn BecomeOfficerPage() -> impl IntoView {
    view! {
        <div class="min-h-screen">
            <HeroBanner />
            <OfficerSection />
            <BoardCollageSection />
            <OfficerFaqSection />
        </div>
    }
}

#[component]
fn HeroBanner() -> impl IntoView {
    view! {
        <section id="banner" class="px-6 md:px-12">
            <div class="relative max-w-6xl mx-auto rounded-2xl shadow-xl overflow-hidden">
                <img
                    src="/public/images/sp26board/Q62A0974.webp?v=3"
                    alt="Poker at Berkeley"
                    class="w-full h-auto block"
                />
                <div class="absolute inset-0 flex items-center justify-center text-center px-6">
                    <div>
                        <h1 class="text-5xl md:text-6xl font-bold text-white mb-4" style="text-shadow: 0 2px 12px rgba(0,0,0,0.7);">
                            "Become an Officer"
                        </h1>
                        <p class="text-xl text-white/90" style="text-shadow: 0 2px 12px rgba(0,0,0,0.7);">
                            "Get involved with Poker at Berkeley"
                        </p>
                    </div>
                </div>
            </div>
        </section>
    }
}

#[component]
fn OfficerSection() -> impl IntoView {
    view! {
        <section id="officer" class="scroll-mt-24 py-20 bg-gray-50 dark:bg-gray-900">
            <div class="max-w-6xl mx-auto px-6">
                <div class="grid grid-cols-1 lg:grid-cols-2 gap-12 items-center">
                    <div class="flex flex-col items-center gap-4 text-center">
                        <img
                            src="/public/images/sp26board/finalseated-doe.webp"
                            alt="Poker at Berkeley officer team"
                            class="w-full aspect-[4/3] h-auto object-cover rounded-lg shadow-lg"
                            loading="lazy"
                        />
                        <div class="w-full flex flex-col sm:flex-row items-center justify-center gap-4 sm:gap-6">
                            <a
                                href=OFFICER_APPLICATION_FORM
                                class="min-w-[220px] inline-flex items-center justify-center text-center bg-[#B2A08E]/70 hover:bg-[#9A6A4C] text-white font-semibold py-3 px-8 rounded-lg transition-colors"
                            >
                                "Officer Application"
                            </a>
                            <a
                                href=DECAL_TA_APPLICATION_FORM
                                class="min-w-[220px] inline-flex items-center justify-center text-center bg-slate-600 hover:bg-slate-700 text-white font-semibold py-3 px-8 rounded-lg transition-colors"
                            >
                                "DeCal TA Application"
                            </a>
                        </div>
                    </div>

                    <div>
                        <ul class="space-y-3 text-gray-700 dark:text-gray-300">
                            <li class="flex items-start space-x-3">
                                <div class="w-2 h-2 bg-blue-600 dark:bg-blue-400 rounded-full mt-2 flex-shrink-0"></div>
                                <div>
                                    <p>"Poker @ Berkeley is a purely officer-driven club. Officers are a tight-knit group committed to building the poker community in Berkeley and beyond. Despite varied academic interests, our members share a passion for poker that translates into lifelong friendships."</p>
                                    <p class="mt-3">"Our officer applications are now open and all students are welcome to apply — undergrad (including freshman) and grad encouraged."</p>
                                </div>
                            </li>
                            <li class="flex items-start space-x-3">
                                <div class="w-2 h-2 bg-blue-600 dark:bg-blue-400 rounded-full mt-2 flex-shrink-0"></div>
                                <span>"Officers build projects, run the best events on campus, compete in outside tournaments, and enjoy socials and club retreats, both poker and non-poker alike."</span>
                            </li>
                            <li class="flex items-start space-x-3">
                                <div class="w-2 h-2 bg-blue-600 dark:bg-blue-400 rounded-full mt-2 flex-shrink-0"></div>
                                <span>"We partner with big names in poker tech and quant finance, including a recent collaboration with BBO Poker Tables to build a custom Poker @ Berkeley RFID table, a partnership with PokerGFX to run our own live stream, and work with GTO Wizard and PokerNow."</span>
                            </li>
                            <li class="flex items-start space-x-3">
                                <div class="w-2 h-2 bg-blue-600 dark:bg-blue-400 rounded-full mt-2 flex-shrink-0"></div>
                                <span>"These partnerships give you the chance to work closely with real industry tools and sponsors."</span>
                            </li>
                        </ul>
                    </div>
                </div>
            </div>
        </section>
    }
}

#[component]
fn BoardCollageSection() -> impl IntoView {
    view! {
        <section class="py-20 bg-white dark:bg-gray-800">
            <div class="max-w-7xl mx-auto px-6">
                <div class="flex flex-col md:flex-row gap-6">
                    <img
                        src="/public/images/collage1.webp"
                        alt="Poker at Berkeley board collage"
                        class="w-full md:w-1/2 h-auto rounded-lg shadow-lg"
                        loading="lazy"
                    />
                    <img
                        src="/public/images/board-filmstrip.webp"
                        alt="Poker at Berkeley board filmstrip"
                        class="w-full md:w-1/2 h-auto rounded-lg shadow-lg"
                        loading="lazy"
                    />
                </div>
            </div>
        </section>
    }
}

#[component]
fn OfficerFaqSection() -> impl IntoView {
    view! {
        <section class="py-20 bg-white dark:bg-gray-800">
            <div class="max-w-7xl mx-auto px-6">
                <div class="grid grid-cols-1 lg:grid-cols-2 gap-12">
                    <div>
                        <h2 class="text-3xl font-bold text-gray-900 dark:text-white mb-6">
                            "Officer FAQ"
                        </h2>
                        <div class="space-y-4">
                            <div class="bg-gray-50 dark:bg-gray-700 rounded-lg p-6 shadow-sm">
                                <h3 class="text-xl font-semibold text-gray-900 dark:text-white mb-2">
                                    "What do officers do?"
                                </h3>
                                <p class="text-gray-700 dark:text-gray-300">
                                    "Officers primarily help organize and run club events/tournaments, manage communications, and handle sponsorship relations. Specific responsibilities may vary based on the officer role."
                                </p>
                            </div>
                            <div class="bg-gray-50 dark:bg-gray-700 rounded-lg p-6 shadow-sm">
                                <h3 class="text-xl font-semibold text-gray-900 dark:text-white mb-2">
                                    "What qualities are you looking for in officers?"
                                </h3>
                                <p class="text-gray-700 dark:text-gray-300">
                                    "We look for individuals who are passionate about poker, have strong organizational and communication skills, and are committed to contributing to the club's growth and success. Some mathematical background is a plus, but not necessarily required. Other skills including design, programming, teaching, and marketing are also highly valued."
                                </p>
                            </div>
                            <div class="bg-gray-50 dark:bg-gray-700 rounded-lg p-6 shadow-sm">
                                <h3 class="text-xl font-semibold text-gray-900 dark:text-white mb-2">
                                    "Can officers participate in tournaments?"
                                </h3>
                                <p class="text-gray-700 dark:text-gray-300 mb-2">
                                    "No, but:"
                                </p>
                                <ol class="list-decimal list-inside space-y-1 text-gray-700 dark:text-gray-300">
                                    <li>"Officers who have graduated may participate in tournaments after they have graduated, forever."</li>
                                    <li>"Officers get a lot of cool merch, food, etc., the EV of which is possibly higher than the EV of your prize pool winnings in tournaments."</li>
                                </ol>
                            </div>
                            <div class="bg-gray-50 dark:bg-gray-700 rounded-lg p-6 shadow-sm">
                                <h3 class="text-xl font-semibold text-gray-900 dark:text-white mb-2">
                                    "I applied last semester but wasn't selected. Can I reapply?"
                                </h3>
                                <p class="text-gray-700 dark:text-gray-300">
                                    "Absolutely! We encourage previous applicants to reapply, especially if they have gained new experiences or skills since their last application."
                                </p>
                            </div>
                            <div class="bg-gray-50 dark:bg-gray-700 rounded-lg p-6 shadow-sm">
                                <h3 class="text-xl font-semibold text-gray-900 dark:text-white mb-2">
                                    "How many officers are typically selected each semester?"
                                </h3>
                                <p class="text-gray-700 dark:text-gray-300">
                                    "The number of officers selected can vary each semester based on the club's needs and the number of applications received. Generally, we aim to select a diverse group of officers to cover various roles within the club."
                                </p>
                            </div>
                        </div>
                    </div>

                    <div>
                        <img
                            src="/public/images/collage2.webp"
                            alt="Poker at Berkeley officer collage"
                            class="w-full h-auto object-contain rounded-lg shadow-lg mb-8"
                            loading="lazy"
                        />

                        <h2 class="text-3xl font-bold text-gray-900 dark:text-white mb-6">
                            "Recruitment Timeline"
                        </h2>
                        <div class="space-y-4">
                        <div class="bg-blue-50/70 dark:bg-slate-800 rounded-lg p-6 shadow-sm border border-blue-100/60 dark:border-slate-700">
                            <h3 class="text-xl font-semibold text-gray-900 dark:text-white mb-2">
                                "Tabling - August 26th to September 4th"
                            </h3>
                            <p class="text-gray-700 dark:text-gray-300">
                                "Find us on Sproul!"
                            </p>
                        </div>
                        <div class="bg-blue-50/70 dark:bg-slate-800 rounded-lg p-6 shadow-sm border border-blue-100/60 dark:border-slate-700">
                            <h3 class="text-xl font-semibold text-gray-900 dark:text-white mb-2">
                                "Infosession - September 9th"
                            </h3>
                        </div>
                        <div class="bg-blue-50/70 dark:bg-slate-800 rounded-lg p-6 shadow-sm border border-blue-100/60 dark:border-slate-700">
                            <h3 class="text-xl font-semibold text-gray-900 dark:text-white mb-2">
                                "Application Due - September 11th"
                            </h3>
                        </div>
                        <div class="bg-blue-50/70 dark:bg-slate-800 rounded-lg p-6 shadow-sm border border-blue-100/60 dark:border-slate-700">
                            <h3 class="text-xl font-semibold text-gray-900 dark:text-white mb-2">
                                "Interviews - September 14th to 16th"
                            </h3>
                        </div>
                        </div>
                    </div>
                </div>
            </div>
        </section>
    }
}
