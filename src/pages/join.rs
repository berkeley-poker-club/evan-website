use leptos::prelude::*;
use leptos_router::hooks::use_location;
use std::time::Duration;
use web_sys::{ScrollBehavior, ScrollIntoViewOptions};

fn scroll_to_hash(target_id: &str) {
    if let Some(window) = web_sys::window() {
        if let Some(document) = window.document() {
            if let Some(element) = document.get_element_by_id(target_id) {
                let mut options = ScrollIntoViewOptions::new();
                options.behavior(ScrollBehavior::Smooth);
                element.scroll_into_view_with_scroll_into_view_options(&options);
            }
        }
    }
}

const JOIN_FORM: &str = "https://forms.gle/yVsAAJ5PLBtrgWUx8";
const STANFORD_JOIN_FORM: &str = "https://forms.gle/iX7oCxR32DdWNAn16";
const OFFICER_APPLICATION_FORM: &str = "https://forms.gle/2cWGidGdtvHewArk6";
const DECAL_TA_APPLICATION_FORM: &str = "https://forms.gle/ZJyBk9brK8iRuAtR6";

#[component]
pub fn JoinUsPage() -> impl IntoView {
    let location = use_location();

    Effect::new(move |_| {
        let hash = location.hash.get();
        let target_id = hash.trim_start_matches('#').to_string();
        if target_id.is_empty() {
            return;
        }

        // Scroll once on the next frame, then twice more after short delays.
        // Images below the target (Spotify embed, flip cards, etc.) load in
        // asynchronously and shift the page layout, so a single immediate
        // scroll can land short of the target once everything settles.
        let id_for_raf = target_id.clone();
        request_animation_frame(move || {
            scroll_to_hash(&id_for_raf);
        });

        let id_for_retry_1 = target_id.clone();
        set_timeout(
            move || scroll_to_hash(&id_for_retry_1),
            Duration::from_millis(350),
        );

        let id_for_retry_2 = target_id.clone();
        set_timeout(
            move || scroll_to_hash(&id_for_retry_2),
            Duration::from_millis(900),
        );
    });

    view! {
        <div class="min-h-screen">
            <HeroBanner />
            <MemberSection />
            <OfficerSection />
            <BoardCollageSection />
            <OfficerFaqSection />
            <GameNightsSection />
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
                            "Join Us"
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
                            src="/public/images/stanfxcal25/DSCF0835.webp"
                            alt="Poker at Berkeley member event"
                            class="w-full aspect-[4/3] h-auto object-cover rounded-lg shadow-lg"
                            loading="lazy"
                        />
                        <div class="w-full flex flex-col sm:flex-row items-center justify-center gap-4 sm:gap-6">
                            <a
                                href=JOIN_FORM
                                class="min-w-[220px] inline-flex items-center justify-center text-center bg-[#386196] hover:bg-[#2D4E78] text-white font-semibold py-3 px-8 rounded-lg transition-colors"
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
                        <h2 class="text-3xl font-bold text-gray-900 dark:text-white mb-6">
                            "Become an Officer"
                        </h2>
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
                                <span>"We partner with big names in poker tech and quant finance, including a recent collaboration with BBO Poker Tables to build a custom Poker @ Berkeley RFID table, a partnership with PokerGFX to run our own live stream, and work with GTO Wizard and PokerBots."</span>
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
            <div class="max-w-6xl mx-auto px-6">
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

#[component]
fn GameNightsSection() -> impl IntoView {
    view! {
        <section id="game-nights" class="scroll-mt-24 py-20 bg-gray-50 dark:bg-gray-900">
            <div class="max-w-6xl mx-auto px-6">
                <h2 class="text-4xl font-bold text-gray-900 dark:text-white mb-6 text-center">
                    "Game Nights"
                </h2>
                <p class="text-lg text-gray-700 dark:text-gray-300 max-w-3xl mx-auto text-center mb-6">
                    "We host game nights every Friday, open to all Poker @ Berkeley members. All stakes welcome — from micro to mid-stakes. Come play, meet the community, and run it up."
                </p>

                <p class="text-sm italic text-gray-400 text-center mb-2">
                    "Feel free to add music to our game night playlist!"
                </p>
                <div class="max-w-[500px] mx-auto mb-12">
                    <iframe
                        src="https://open.spotify.com/embed/playlist/46wxKs4YB0TNa95bZp0aFk?utm_source=generator&theme=0"
                        width="100%"
                        height="352"
                        frameborder="0"
                        allowfullscreen=""
                        allow="autoplay; clipboard-write; encrypted-media; fullscreen; picture-in-picture"
                        loading="lazy"
                    ></iframe>
                </div>

                <div class="flex flex-col md:flex-row md:flex-wrap justify-center gap-3 mb-10">
                    <InfoCard
                        title="Before You Come"
                        bullets=vec![
                            "Player cards are required for entry this semester. No card, no entry.",
                            "If you ordered a card but haven't picked it up, it will be available at the check-in desk.",
                            "We track card pickups. If you've previously picked up your card and no longer have it, you must order a replacement.",
                            "If your replacement card is pending, you may still attend. Let the check-in desk know. We will verify.",
                            "Stanford P@B Members are welcome to attend game nights and may use game nights as an opportunity to pick up their player card.",
                            "Remember to check out and take your player card when you leave.",
                        ]
                    />

                    <InfoCard
                        title="Check-In"
                        bullets=vec![
                            "Game nights are only open to P@B Members & Stanford P@B Members. Your +1 does not count. Your cousin visiting from Fresno does not count.",
                            "When you arrive, you must check in at the door by giving us your player card.",
                            "State your stakes and wait to be seated. Do not seat yourself, don't make it complicated, and don't be the nuisance trying to make a table 11-handed.",
                            "Start a table only if space and equipment allows. If there's no game with room at your stakes, you can open a new one. Let us know and we'll set one up if possible.",
                        ]
                    />

                    <InfoCard
                        title="Banking & Conduct"
                        bullets=vec![
                            "We provide all the chips, mats, and cards. Please treat the setup with care — it takes real time and money to haul and maintain, and we do so for the love of the game.",
                            "Each table handles its own banking and dealing. P@B is not involved. The players at your table decide who banks and who deals — if something feels off, say something right away.",
                            "Act like adults. If someone slowrolls you, tanks forever preflop, or says \"one time\" nonstop, address it calmly or just change tables.",
                            "Settle banks before you leave. Any discrepancy with stacks must be raised on-site. Not tomorrow, not next week. Chips that leave the premises will be deemed dead.",
                            "DO NOT mention \"poker\" in the memo on Venmo, Cashapp, Zelle, or any digital payment method — it may get your account flagged.",
                        ]
                    />

                    <InfoCard
                        title="A Few Rules"
                        bullets=vec![
                            "Don't gamble with money you cannot afford to lose.",
                            "Playing in any poker game is inherently risky. While we take scamming very seriously, it is near impossible for us to enforce hosts to pay out other than by blacklisting them.",
                            "Taking a rake is illegal, no exceptions. Anyone found taking rake is breaking the law. Tipping should never be forced as it would operate as \"pseudo rake\" — it is solely the player's discretion.",
                            "We reserve the right to remove you permanently from game nights, events, and the server. Any player who repeatedly ignores these guidelines or makes the environment difficult for others may be removed from P@B activities. Game nights have been around for decades — don't be the one who jeopardizes that.",
                        ]
                    />
                </div>

                <div class="text-center mb-12">
                    <h3 class="text-2xl font-bold text-gray-900 dark:text-white mb-6">
                        "Lost Your Player Card?"
                    </h3>
                    <div class="flex flex-col sm:flex-row items-center justify-center gap-8 sm:gap-24 mb-12">
                        <a
                            href=JOIN_FORM
                            target="_blank"
                            rel="noopener noreferrer"
                            class="min-w-[220px] inline-flex items-center justify-center text-center bg-[#003262] hover:bg-[#0A4D8C] text-[#FDB515] font-semibold py-3 px-8 rounded-lg transition-colors"
                        >
                            "Berkeley Students"
                        </a>
                        <a
                            href=STANFORD_JOIN_FORM
                            target="_blank"
                            rel="noopener noreferrer"
                            class="min-w-[220px] inline-flex items-center justify-center text-center bg-[#8C1515] hover:bg-[#A31E1E] text-white font-semibold py-3 px-8 rounded-lg transition-colors"
                        >
                            "Stanford Students"
                        </a>
                    </div>
                    <div class="flex flex-col sm:flex-row items-center justify-center gap-4 sm:gap-6">
                        <div class="flex flex-col items-center gap-3">
                            <div class="rounded-full p-8 bg-[rgba(0,50,98,0.3)]">
                                <img
                                    src="/public/images/berkeley-playercard.webp"
                                    alt="Berkeley Students player card"
                                    class="w-[200px] rounded-lg shadow-sm"
                                    loading="lazy"
                                />
                            </div>
                            <span class="text-sm text-gray-500 dark:text-gray-400">"Berkeley Card"</span>
                        </div>
                        <div class="flex flex-col items-center gap-3">
                            <div class="rounded-full p-8 bg-[rgba(140,21,21,0.25)]">
                                <img
                                    src="/public/images/stanford-playercard.webp"
                                    alt="Stanford Students player card"
                                    class="w-[200px] rounded-lg shadow-sm"
                                    loading="lazy"
                                />
                            </div>
                            <span class="text-sm text-gray-500 dark:text-gray-400">"Stanford Card"</span>
                        </div>
                    </div>
                </div>

                <p class="text-sm italic text-gray-500 dark:text-gray-400 text-center">
                    "Poker @ Berkeley does not endorse poker or gambling involving real money. We endorse two things: the DeCal and our semesterly tournaments."
                </p>
            </div>
        </section>
    }
}

#[component]
fn InfoCard(
    title: &'static str,
    bullets: Vec<&'static str>,
) -> impl IntoView {
    view! {
        <article class="w-full max-w-[380px] self-stretch rounded-lg bg-gray-900 dark:bg-gray-800 p-6 shadow-sm">
            <h3 class="text-xl font-bold text-white mb-3">
                {title}
            </h3>
            <ul class="list-disc list-outside ml-5 space-y-2 text-sm leading-relaxed text-gray-300">
                {bullets.into_iter().map(|b| view! { <li>{b}</li> }).collect::<Vec<_>>()}
            </ul>
        </article>
    }
}
