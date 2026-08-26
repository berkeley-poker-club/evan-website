use leptos::prelude::*;

#[component]
pub fn PokerBotsPage() -> impl IntoView {
    view! {
        <section class="font-terminal relative min-h-screen overflow-hidden bg-[#02070d] px-6 py-8 text-cyan-100">
            <div
                class="pointer-events-none absolute inset-0 opacity-30"
                style="background-image: linear-gradient(rgba(34, 211, 238, 0.08) 1px, transparent 1px), linear-gradient(90deg, rgba(34, 211, 238, 0.08) 1px, transparent 1px); background-size: 48px 48px;"
                aria-hidden="true"
            ></div>
            <div class="pointer-events-none absolute inset-0 bg-[radial-gradient(circle_at_center,rgba(8,145,178,0.16),transparent_48%)]" aria-hidden="true"></div>
            <div
                class="pointer-events-none absolute inset-x-0 top-3 h-px opacity-45"
                style="background-image: repeating-linear-gradient(115deg, transparent 0 7px, rgb(103, 232, 249) 7px 8px, transparent 8px 14px);"
                aria-hidden="true"
            ></div>

            <div class="relative mx-auto flex min-h-[calc(100vh-4rem)] max-w-6xl flex-col">
                <a href="/" class="fixed left-6 top-8 z-20 text-sm font-semibold text-cyan-100 underline-offset-4 hover:text-amber-200 hover:underline focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-4 focus-visible:outline-cyan-100">
                    "< Poker at Berkeley"
                </a>
                <main class="flex min-h-[calc(100vh-4rem)] items-center justify-center text-center">
                    <div>
                        <img
                            src="public/images/banner-logo.png"
                            alt="Poker at Berkeley"
                            class="mx-auto mb-10 w-full max-w-[280px] drop-shadow-[0_0_20px_rgba(251,191,36,0.35)] sm:max-w-[380px]"
                        />
                        <h1 class="text-4xl font-semibold uppercase tracking-[0.16em] text-cyan-50 sm:text-6xl">
                            "Poker"<span class="text-amber-200">"Bots"</span>
                        </h1>
                        <p class="mt-6 text-sm uppercase tracking-[0.2em] text-cyan-100 sm:text-base">"Coming soon"<span class="animate-pulse motion-reduce:animate-none text-cyan-300" aria-hidden="true">"_"</span></p>
                    </div>
                </main>

                <section class="mx-auto w-full max-w-3xl border-y border-cyan-200/20 py-16 sm:py-24">
                    <p class="mb-5 text-sm uppercase tracking-[0.22em] text-amber-200">"PokerBots // the challenge"</p>
                    <div class="space-y-6 text-left text-lg leading-relaxed text-cyan-50/85 sm:text-xl">
                        <p>
                            "Teams have a month to program a completely autonomous poker bot, then put it up against bots built by other teams."
                        </p>
                        <p class="text-cyan-100">
                            "Build a strong bot by combining math, computer science, and strategy to outplay the competition."
                        </p>
                    </div>
                </section>

                <section class="mx-auto w-full max-w-3xl py-16 sm:py-24" aria-labelledby="pokerbots-faq">
                    <h2 id="pokerbots-faq" class="mb-10 text-2xl font-semibold uppercase tracking-[0.16em] text-cyan-50 sm:text-3xl">
                        "FAQ"
                    </h2>
                    <dl class="border-t border-cyan-200/30">
                        <div class="border-b border-cyan-200/30 py-7">
                            <dt class="text-lg font-semibold text-amber-200 sm:text-xl">"Who can play?"</dt>
                            <dd class="mt-3 text-base leading-relaxed text-cyan-100 sm:text-lg">
                                "College students from any U.S. university. There will be a Berkeley/Stanford-specific prize pool as well as a general prize pool."
                            </dd>
                        </div>
                        <div class="border-b border-cyan-200/30 py-7">
                            <dt class="text-lg font-semibold text-amber-200 sm:text-xl">"What variants?"</dt>
                            <dd class="mt-3 text-base leading-relaxed text-cyan-100 sm:text-lg">
                                "The variant we will use is secret until the competition begins."
                            </dd>
                        </div>
                        <div class="border-b border-cyan-200/30 py-7">
                            <dt class="text-lg font-semibold text-amber-200 sm:text-xl">"When will signups begin?"</dt>
                            <dd class="mt-3 text-base leading-relaxed text-cyan-100 sm:text-lg">
                                "We plan to release signups in early November. The competition will start in January."
                            </dd>
                        </div>
                        <div class="border-b border-cyan-200/30 py-7">
                            <dt class="text-lg font-semibold text-amber-200 sm:text-xl">"What's the prize pool?"</dt>
                            <dd class="mt-3 text-base leading-relaxed text-cyan-100 sm:text-lg">
                                "Depending on interest and support, the prize pool will be between $5,000 and $20,000."
                            </dd>
                        </div>
                        <div class="border-b border-cyan-200/30 py-7">
                            <dt class="text-lg font-semibold text-amber-200 sm:text-xl">"What languages will be available?"</dt>
                            <dd class="mt-3 text-base leading-relaxed text-cyan-100 sm:text-lg">
                                "We plan to support Python, Java, C++, and Rust."
                            </dd>
                        </div>
                        <div class="border-b border-cyan-200/30 py-7">
                            <dt class="text-lg font-semibold text-amber-200 sm:text-xl">"How much time will this take?"</dt>
                            <dd class="mt-3 text-base leading-relaxed text-cyan-100 sm:text-lg">
                                "It depends on how strong you want your poker bot to be. A working bot may not take much time, while a competitive one will take more thought and iteration."
                            </dd>
                        </div>
                    </dl>
                </section>

                <section class="mx-auto w-full max-w-5xl border-t border-cyan-200/20 py-16 sm:py-24" aria-labelledby="pokerbots-sponsors">
                    <h2 id="pokerbots-sponsors" class="mb-10 text-center text-2xl font-semibold uppercase tracking-[0.16em] text-cyan-50 sm:text-3xl">
                        "Sponsors"
                    </h2>
                    <div class="grid grid-cols-1 gap-5 sm:grid-cols-2 lg:grid-cols-3">
                        <PokerBotsSponsor name="Jump Trading" url="https://www.jumptrading.com/" tier_underline="decoration-sky-300" />
                        <PokerBotsSponsor name="Jane Street" url="https://www.janestreet.com/" tier_underline="decoration-amber-300" />
                        <PokerBotsSponsor name="QRT" url="https://www.qube-rt.com/" tier_underline="decoration-amber-300" />
                        <PokerBotsSponsor name="Citadel" url="https://www.citadel.com/" tier_underline="decoration-slate-300" />
                        <PokerBotsSponsor name="HRT" url="https://www.hudsonrivertrading.com/" tier_underline="decoration-slate-300" />
                        <PokerBotsSponsor name="Duper" url="https://www.duper.gg/" tier_underline="decoration-orange-400" />
                    </div>
                    <p class="mt-10 text-center text-base text-cyan-100 sm:text-lg">
                        "Interested in sponsoring? "
                        <a href="mailto:sponsorships@poker.studentorg.berkeley.edu" class="font-semibold text-amber-200 underline underline-offset-4 hover:text-cyan-50 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-4 focus-visible:outline-cyan-100">
                            "sponsorships@poker.studentorg.berkeley.edu"
                        </a>
                    </p>
                </section>
            </div>
        </section>
    }
}

#[component]
fn PokerBotsSponsor(
    name: &'static str,
    url: &'static str,
    tier_underline: &'static str,
) -> impl IntoView {
    view! {
        <a
            href=url
            target="_blank"
            rel="noopener noreferrer"
            aria-label=format!("Visit {}", name)
            class=format!("flex min-h-20 items-center justify-center rounded border border-cyan-200/30 px-6 py-5 text-center text-lg font-semibold text-cyan-50 underline decoration-2 underline-offset-8 transition-colors hover:border-amber-200 hover:text-amber-200 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-4 focus-visible:outline-cyan-100 {}", tier_underline)
        >
            {name}
        </a>
    }
}
