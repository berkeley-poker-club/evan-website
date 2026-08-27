use leptos::prelude::*;
use leptos_router::hooks::use_location;
use web_sys::{ScrollBehavior, ScrollIntoViewOptions};

const JOIN_FORM: &str =
    "https://docs.google.com/forms/d/e/1FAIpQLSdxzvFVWmAr78rsoMCtL-yaQafVlElf3plTJhg7cEHNfUlq8Q/viewform?usp=dialog";
const STANFORD_JOIN_FORM: &str =
    "https://docs.google.com/forms/d/e/1FAIpQLSeMmF5-hdHQg8l-6DVjcQh7mwDMGapFE2DAfSMGnCTX9MgnAg/viewform?usp=dialog";
const OFFICER_INTEREST_FORM: &str =
    "https://docs.google.com/forms/d/e/1FAIpQLScehYwpXAbH_RNLrVDZ_vATDO9LTYikVJ62TjtfXOsZg784fw/viewform";

#[component]
pub fn JoinUsPage() -> impl IntoView {
    let location = use_location();

    Effect::new(move |_| {
        let hash = location.hash.get();
        let target_id = hash.trim_start_matches('#').to_string();
        if target_id.is_empty() {
            return;
        }

        request_animation_frame(move || {
            if let Some(window) = web_sys::window() {
                if let Some(document) = window.document() {
                    if let Some(element) = document.get_element_by_id(&target_id) {
                        let mut options = ScrollIntoViewOptions::new();
                        options.behavior(ScrollBehavior::Smooth);
                        element.scroll_into_view_with_scroll_into_view_options(&options);
                    }
                }
            }
        });
    });

    view! {
        <div class="min-h-screen">
            <HeroBanner />
            <MemberSection />
            <OfficerSection />
            <OfficerFaqSection />
        </div>
    }
}

#[component]
fn HeroBanner() -> impl IntoView {
    view! {
        <section
            id="banner"
            class="py-40"
            style="background-image: url('public/images/officergroup/DSCF1902.jpg'); background-size: cover; background-position: center;"
        >
            <div class="max-w-4xl mx-auto text-center px-6">
                <h1 class="text-5xl md:text-6xl font-bold text-white mb-4">
                    "Join Us"
                </h1>
                <p class="text-xl text-white/90">
                    "Get involved with Poker at Berkeley"
                </p>
            </div>
        </section>
    }
}

#[component]
fn MemberSection() -> impl IntoView {
    view! {
        <section id="member" class="scroll-mt-24 py-20 bg-white dark:bg-gray-800">
            <div class="max-w-6xl mx-auto px-6">
                <h2 class="text-4xl md:text-5xl font-bold text-gray-900 dark:text-white mb-12 text-center">
                    "Become a Member"
                </h2>
                <div class="grid grid-cols-1 lg:grid-cols-2 gap-12 items-center">
                    <div>
                        <ul class="space-y-3 text-gray-700 dark:text-gray-300">
                            <li class="flex items-start space-x-3">
                                <span>"For a small registration fee you have access to:"</span>
                            </li>
                            <li class="flex items-start space-x-3">
                                <div class="w-2 h-2 bg-blue-600 dark:bg-blue-400 rounded-full mt-2 flex-shrink-0"></div>
                                <span><b>"All Poker @ Berkeley tournaments, game nights, and events"</b></span>
                            </li>
                            <li class="flex items-start space-x-3">
                                <div class="w-2 h-2 bg-blue-600 dark:bg-blue-400 rounded-full mt-2 flex-shrink-0"></div>
                                <span>"A free, customizable player card."</span>
                            </li>
                            <li class="flex items-start space-x-3">
                                <div class="w-2 h-2 bg-blue-600 dark:bg-blue-400 rounded-full mt-2 flex-shrink-0"></div>
                                <span>"A place in the Poker @ Berkeley resume book used by our sponsors, and networking opportunities with our sponsors."</span>
                            </li>
                            <li class="flex items-start space-x-3">
                                <div class="w-2 h-2 bg-blue-600 dark:bg-blue-400 rounded-full mt-2 flex-shrink-0"></div>
                                <span>"Access to tournament prize pools worth a total of $10000+ per year."</span>
                            </li>
                        </ul>
                    </div>

                    <div class="flex flex-col items-center gap-4 text-center">
                        <img
                            src="public/images/falltourney/DSCF2007.jpg"
                            alt="Poker at Berkeley member event"
                            class="w-full h-80 object-cover rounded-lg shadow-lg"
                            loading="lazy"
                        />
                        <div class="w-full flex flex-col sm:flex-row items-center justify-center gap-4 sm:gap-6">
                            <a
                                href=JOIN_FORM
                                class="min-w-[220px] inline-flex items-center justify-center text-center bg-slate-400 hover:bg-slate-500 text-white font-semibold py-3 px-8 rounded-lg transition-colors"
                            >
                                "Berkeley Students"
                            </a>
                            <a
                                href=STANFORD_JOIN_FORM
                                class="min-w-[220px] inline-flex items-center justify-center text-center bg-[#8E3E3B] hover:bg-[#7F3835] text-white font-semibold py-3 px-8 rounded-lg transition-colors"
                            >
                                "Stanford Students"
                            </a>
                        </div>
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
                            src="public/images/board-sp26.jpg"
                            alt="Poker at Berkeley officer team"
                            class="w-full h-80 object-cover rounded-lg shadow-lg"
                            loading="lazy"
                        />
                        <a
                            href=OFFICER_INTEREST_FORM
                            class="min-w-[220px] inline-flex items-center justify-center text-center bg-[#B2A08E]/70 hover:bg-[#9A6A4C] text-white font-semibold py-3 px-8 rounded-lg transition-colors"
                        >
                            "Interest Form"
                        </a>
                    </div>

                    <div>
                        <h2 class="text-3xl font-bold text-gray-900 dark:text-white mb-6">
                            "Become an Officer"
                        </h2>
                        <ul class="space-y-3 text-gray-700 dark:text-gray-300">
                            <li class="flex items-start space-x-3">
                                <div class="w-2 h-2 bg-blue-600 dark:bg-blue-400 rounded-full mt-2 flex-shrink-0"></div>
                                <span>"Poker @ Berkeley is a purely officer-driven club. Officers are a tight-knit group committed to building the poker community in Berkeley and beyond. Despite varied academic interests, our members share a passion for poker that translates into lifelong friendships."</span>
                            </li>
                            <li class="flex items-start space-x-3">
                                <div class="w-2 h-2 bg-blue-600 dark:bg-blue-400 rounded-full mt-2 flex-shrink-0"></div>
                                <span>"All grade levels, especially freshmen, are encouraged to apply."</span>
                            </li>
                        </ul>
                    </div>
                </div>
            </div>
        </section>
    }
}

#[component]
fn OfficerFaqSection() -> impl IntoView {
    view! {
        <section class="py-20 bg-white dark:bg-gray-800">
            <div class="max-w-6xl mx-auto px-6">
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
                        <h2 class="text-3xl font-bold text-gray-900 dark:text-white mb-6">
                            "Recruitment Timeline"
                        </h2>
                        <div class="space-y-6">
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
