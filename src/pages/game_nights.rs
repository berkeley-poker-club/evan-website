use leptos::prelude::*;

const JOIN_FORM: &str = "https://forms.gle/yVsAAJ5PLBtrgWUx8";
const STANFORD_JOIN_FORM: &str = "https://forms.gle/iX7oCxR32DdWNAn16";

#[component]
pub fn GameNightsPage() -> impl IntoView {
    view! {
        <div class="min-h-screen">
            <HeroBanner />
            <GameNightsSection />
        </div>
    }
}

#[component]
fn HeroBanner() -> impl IntoView {
    view! {
        <section id="banner" class="relative py-40"
                 style="background-image: url('/public/images/stanfxcal26/Q62A1176.webp'); background-size: cover; background-position: center 65%;">
            <div class="absolute inset-0" style="background-color: rgba(0, 0, 0, 0.30);"></div>
            <div class="relative z-10 max-w-4xl mx-auto text-center px-6">
                <h1 class="text-5xl md:text-6xl font-bold text-white mb-4">
                    "Game Nights"
                </h1>
                <p class="text-xl text-white/90">
                    "Weekly poker with the Poker at Berkeley community"
                </p>
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
                    "We host game nights every Friday, open to all P@B members, both Berkeley and Stanford. All stakes are welcome, ranging from micro to mid, and we run a free instructional table for anyone looking to learn. Come play, meet the community, and run it up."
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
