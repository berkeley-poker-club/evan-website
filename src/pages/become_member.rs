use leptos::prelude::*;
use leptos_meta::Title;

const JOIN_FORM: &str = "https://docs.google.com/forms/d/e/1FAIpQLSdxzvFVWmAr78rsoMCtL-yaQafVlElf3plTJhg7cEHNfUlq8Q/viewform?embedded=true";
const STANFORD_JOIN_FORM: &str = "https://docs.google.com/forms/d/e/1FAIpQLSeMmF5-hdHQg8l-6DVjcQh7mwDMGapFE2DAfSMGnCTX9MgnAg/viewform?embedded=true";

#[component]
pub fn BecomeMemberPage() -> impl IntoView {
    view! {
        <Title text="Become a Member | Poker at Berkeley" />
        <div class="min-h-screen">
            <HeroBanner />
            <MemberSection />
        </div>
    }
}

#[component]
fn HeroBanner() -> impl IntoView {
    view! {
        <section id="banner" class="px-6 md:px-12">
            <div class="relative max-w-6xl mx-auto rounded-2xl shadow-xl overflow-hidden">
                <img
                    src="/public/images/member.banner.webp"
                    alt="Poker at Berkeley"
                    class="w-full h-auto block"
                />
                <div class="absolute inset-0 pointer-events-none" style="background: radial-gradient(ellipse 45% 70% at center, rgba(0,0,0,0.85) 0%, rgba(0,0,0,0) 75%);"></div>
                <div class="absolute inset-0 flex items-center justify-center text-center px-6">
                    <div>
                        <h1 class="text-5xl md:text-6xl font-bold text-white mb-4" style="text-shadow: 0 2px 12px rgba(0,0,0,0.7);">
                            "Become a Member"
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
    let (active_form, set_active_form) = signal::<Option<&'static str>>(None);

    view! {
        <section id="member" class="scroll-mt-24 py-20 bg-white dark:bg-gray-800">
            <div class="max-w-6xl mx-auto px-6">
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
                            <button
                                type="button"
                                on:click=move |_| set_active_form.update(|f| *f = if *f == Some("berkeley") { None } else { Some("berkeley") })
                                class="min-w-[220px] inline-flex items-center justify-center text-center bg-[#386196] hover:bg-[#2D4E78] text-white font-semibold py-3 px-8 rounded-lg transition-colors"
                            >
                                "Berkeley Students"
                            </button>
                            <button
                                type="button"
                                on:click=move |_| set_active_form.update(|f| *f = if *f == Some("stanford") { None } else { Some("stanford") })
                                class="min-w-[220px] inline-flex items-center justify-center text-center bg-[#8E3E3B] hover:bg-[#7F3835] text-white font-semibold py-3 px-8 rounded-lg transition-colors"
                            >
                                "Stanford Students"
                            </button>
                        </div>
                    </div>
                </div>

                {move || active_form.get().map(|form| {
                    let src: &'static str = if form == "berkeley" { JOIN_FORM } else { STANFORD_JOIN_FORM };
                    let label = if form == "berkeley" { "Berkeley Students Membership Form" } else { "Stanford Students Membership Form" };
                    let color = if form == "berkeley" { "#386196" } else { "#8E3E3B" };
                    view! {
                        <div class="mt-12 rounded-lg p-4" style=format!("border: 2px solid {color};")>
                            <div class="flex items-center justify-between mb-3">
                                <h3 class="text-2xl font-bold" style=format!("color: {color};")>{label}</h3>
                                <button
                                    type="button"
                                    on:click=move |_| set_active_form.set(None)
                                    class="text-sm text-gray-500 dark:text-gray-400 hover:text-gray-700 dark:hover:text-gray-200 transition-colors"
                                >
                                    "Hide form"
                                </button>
                            </div>
                            <iframe
                                src=src
                                title=label
                                class="w-full rounded-lg shadow-lg"
                                style="height: 800px;"
                            ></iframe>
                        </div>
                    }
                })}
            </div>
        </section>
    }
}
