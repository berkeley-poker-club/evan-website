use leptos::prelude::*;
use leptos_router::components::A;
use crate::components::ThemeToggle;

#[component]
pub fn Navigation() -> impl IntoView {
    let (is_open, set_is_open) = signal(false);
    let (mobile_group, set_mobile_group) = signal::<Option<&'static str>>(None);
    let close_mobile_menu = move || {
        set_is_open.set(false);
        set_mobile_group.set(None);
    };
    let toggle_mobile_group = move |key: &'static str| {
        set_mobile_group.update(|g| *g = if *g == Some(key) { None } else { Some(key) });
    };

    view! {
        <nav class="sticky top-0 z-50 bg-white dark:bg-gray-800 shadow-md">
            <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
                <div class="flex items-center justify-between h-[4.5rem]">
                    <div class="flex items-center space-x-4">
                        <A href="/" attr:class="group flex items-center space-x-2 text-gray-900 dark:text-white hover:text-[#B08B5B] dark:hover:text-[#C9A876] transition-colors">
                            <span class="relative h-8 w-auto inline-block">
                                <img src="/public/images/banner.webp" alt="Poker at Berkeley Logo" class="h-8 w-auto block" />
                                <img src="/public/images/inversebanner.webp" alt="" class="h-8 w-auto absolute inset-0 opacity-0 group-hover:opacity-100 transition-opacity duration-300" />
                            </span>
                            <span class="font-bold text-xl">"Poker at Berkeley"</span>
                        </A>
                    </div>

                    <div class="hidden lg:flex items-center space-x-8">
                        <div class="relative group py-2 -my-2">
                            <button type="button" class="flex items-center gap-1 text-gray-700 dark:text-gray-200 hover:text-[#B08B5B] dark:hover:text-[#C9A876] font-medium transition-colors">
                                "Join Us"
                                <svg class="w-3.5 h-3.5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 9l-7 7-7-7"></path>
                                </svg>
                            </button>
                            <div class="absolute left-0 top-full pt-2 hidden group-hover:block">
                                <div class="bg-white dark:bg-gray-800 shadow-md rounded-md border border-gray-100 dark:border-gray-700 py-2 min-w-max">
                                    <A href="/become-member" attr:class="block px-4 py-2 text-sm whitespace-nowrap text-gray-700 dark:text-gray-200 hover:text-[#B08B5B] dark:hover:text-[#C9A876] hover:bg-gray-50 dark:hover:bg-gray-700 transition-colors">"Become a Member"</A>
                                    <A href="/become-officer" attr:class="block px-4 py-2 text-sm whitespace-nowrap text-gray-700 dark:text-gray-200 hover:text-[#B08B5B] dark:hover:text-[#C9A876] hover:bg-gray-50 dark:hover:bg-gray-700 transition-colors">"Become an Officer"</A>
                                    <A href="/ta-application" attr:class="block px-4 py-2 text-sm whitespace-nowrap text-gray-700 dark:text-gray-200 hover:text-[#B08B5B] dark:hover:text-[#C9A876] hover:bg-gray-50 dark:hover:bg-gray-700 transition-colors">"Become a Stat 198 TA"</A>
                                    <a href="https://discord.gg/SbS9UbZW2a" target="_blank" rel="noopener" class="block px-4 py-2 text-sm whitespace-nowrap text-gray-700 dark:text-gray-200 hover:text-[#B08B5B] dark:hover:text-[#C9A876] hover:bg-gray-50 dark:hover:bg-gray-700 transition-colors">"Join the P@B Discord"</a>
                                </div>
                            </div>
                        </div>

                        <div class="relative group py-2 -my-2">
                            <button type="button" class="flex items-center gap-1 text-gray-700 dark:text-gray-200 hover:text-[#B08B5B] dark:hover:text-[#C9A876] font-medium transition-colors">
                                "People"
                                <svg class="w-3.5 h-3.5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 9l-7 7-7-7"></path>
                                </svg>
                            </button>
                            <div class="absolute left-0 top-full pt-2 hidden group-hover:block">
                                <div class="bg-white dark:bg-gray-800 shadow-md rounded-md border border-gray-100 dark:border-gray-700 py-2 min-w-max">
                                    <A href="/people" attr:class="block px-4 py-2 text-sm whitespace-nowrap text-gray-700 dark:text-gray-200 hover:text-[#B08B5B] dark:hover:text-[#C9A876] hover:bg-gray-50 dark:hover:bg-gray-700 transition-colors">"Poker@Berkeley Officers"</A>
                                    <A href="/decal#course-staff" attr:class="block px-4 py-2 text-sm whitespace-nowrap text-gray-700 dark:text-gray-200 hover:text-[#B08B5B] dark:hover:text-[#C9A876] hover:bg-gray-50 dark:hover:bg-gray-700 transition-colors">"Stat 198 Course Staff"</A>
                                </div>
                            </div>
                        </div>

                        <div class="relative group py-2 -my-2">
                            <button type="button" class="flex items-center gap-1 text-gray-700 dark:text-gray-200 hover:text-[#B08B5B] dark:hover:text-[#C9A876] font-medium transition-colors">
                                "Events"
                                <svg class="w-3.5 h-3.5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 9l-7 7-7-7"></path>
                                </svg>
                            </button>
                            <div class="absolute left-0 top-full pt-2 hidden group-hover:block">
                                <div class="bg-white dark:bg-gray-800 shadow-md rounded-md border border-gray-100 dark:border-gray-700 py-2 min-w-[10rem]">
                                    <A href="/tournaments" attr:class="block px-4 py-2 text-sm text-gray-700 dark:text-gray-200 hover:text-[#B08B5B] dark:hover:text-[#C9A876] hover:bg-gray-50 dark:hover:bg-gray-700 transition-colors">"Tournaments"</A>
                                    <A href="/game-nights" attr:class="block px-4 py-2 text-sm text-gray-700 dark:text-gray-200 hover:text-[#B08B5B] dark:hover:text-[#C9A876] hover:bg-gray-50 dark:hover:bg-gray-700 transition-colors">"Game Nights"</A>
                                    <A href="/pokerbots" attr:class="block px-4 py-2 text-sm text-gray-700 dark:text-gray-200 hover:text-[#B08B5B] dark:hover:text-[#C9A876] hover:bg-gray-50 dark:hover:bg-gray-700 transition-colors">"PokerBots"</A>
                                </div>
                            </div>
                        </div>

                        <div class="relative group py-2 -my-2">
                            <button type="button" class="flex items-center gap-1 text-gray-700 dark:text-gray-200 hover:text-[#B08B5B] dark:hover:text-[#C9A876] font-medium transition-colors">
                                "Learn"
                                <svg class="w-3.5 h-3.5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 9l-7 7-7-7"></path>
                                </svg>
                            </button>
                            <div class="absolute left-0 top-full pt-2 hidden group-hover:block">
                                <div class="bg-white dark:bg-gray-800 shadow-md rounded-md border border-gray-100 dark:border-gray-700 py-2 min-w-[10rem]">
                                    <A href="/decal" attr:class="block px-4 py-2 text-sm text-gray-700 dark:text-gray-200 hover:text-[#B08B5B] dark:hover:text-[#C9A876] hover:bg-gray-50 dark:hover:bg-gray-700 transition-colors">"Poker Theory DeCal"</A>
                                    <A href="/resources" attr:class="block px-4 py-2 text-sm text-gray-700 dark:text-gray-200 hover:text-[#B08B5B] dark:hover:text-[#C9A876] hover:bg-gray-50 dark:hover:bg-gray-700 transition-colors">"Resources"</A>
                                    <A href="/blog" attr:class="block px-4 py-2 text-sm text-gray-700 dark:text-gray-200 hover:text-[#B08B5B] dark:hover:text-[#C9A876] hover:bg-gray-50 dark:hover:bg-gray-700 transition-colors">"Blog"</A>
                                    <a href="https://discord.gg/wWrZj5cM5X" target="_blank" rel="noopener" class="block px-4 py-2 text-sm whitespace-nowrap text-gray-700 dark:text-gray-200 hover:text-[#B08B5B] dark:hover:text-[#C9A876] hover:bg-gray-50 dark:hover:bg-gray-700 transition-colors">"Join the Stat 198 Discord"</a>
                                </div>
                            </div>
                        </div>

                        <A href="/sponsors" attr:class="text-gray-700 dark:text-gray-200 hover:text-[#B08B5B] dark:hover:text-[#C9A876] font-medium transition-colors">"Sponsors"</A>
                        <A href="/merch" attr:class="text-gray-700 dark:text-gray-200 hover:text-[#B08B5B] dark:hover:text-[#C9A876] font-medium transition-colors">"Merch"</A>
                        <ThemeToggle />
                    </div>

                    <div class="lg:hidden flex items-center space-x-2">
                        <ThemeToggle />
                        <button
                            class="text-gray-700 dark:text-gray-200 hover:text-[#B08B5B] dark:hover:text-[#C9A876] transition-colors"
                            on:click=move |_| {
                                set_is_open.update(|open| *open = !*open);
                                set_mobile_group.set(None);
                            }
                        >
                            <svg class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 6h16M4 12h16M4 18h16"></path>
                            </svg>
                        </button>
                    </div>
                </div>

                <div class=move || if is_open.get() { "lg:hidden" } else { "hidden" }>
                    <div class="px-2 pt-2 pb-3 space-y-1 bg-white dark:bg-gray-800 border-t border-gray-200 dark:border-gray-700">
                        <button
                            type="button"
                            class="w-full flex items-center justify-between px-3 py-2 text-gray-700 dark:text-gray-200 hover:text-[#B08B5B] dark:hover:text-[#C9A876] transition-colors"
                            on:click=move |_| toggle_mobile_group("join")
                        >
                            <span class="font-medium">"Join Us"</span>
                            <svg class=move || if mobile_group.get() == Some("join") { "w-3.5 h-3.5 transition-transform rotate-180" } else { "w-3.5 h-3.5 transition-transform" } fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 9l-7 7-7-7"></path>
                            </svg>
                        </button>
                        <div class=move || if mobile_group.get() == Some("join") { "pl-4 space-y-1" } else { "hidden" }>
                            <A href="/become-member" on:click=move |_| close_mobile_menu() attr:class="block px-3 py-2 text-sm text-gray-700 dark:text-gray-200 hover:text-[#B08B5B] dark:hover:text-[#C9A876] transition-colors">"Become a Member"</A>
                            <A href="/become-officer" on:click=move |_| close_mobile_menu() attr:class="block px-3 py-2 text-sm text-gray-700 dark:text-gray-200 hover:text-[#B08B5B] dark:hover:text-[#C9A876] transition-colors">"Become an Officer"</A>
                            <A href="/ta-application" on:click=move |_| close_mobile_menu() attr:class="block px-3 py-2 text-sm text-gray-700 dark:text-gray-200 hover:text-[#B08B5B] dark:hover:text-[#C9A876] transition-colors">"Become a Stat 198 TA"</A>
                            <a href="https://discord.gg/SbS9UbZW2a" target="_blank" rel="noopener" on:click=move |_| close_mobile_menu() class="block px-3 py-2 text-sm text-gray-700 dark:text-gray-200 hover:text-[#B08B5B] dark:hover:text-[#C9A876] transition-colors">"Join the P@B Discord"</a>
                        </div>

                        <button
                            type="button"
                            class="w-full flex items-center justify-between px-3 py-2 text-gray-700 dark:text-gray-200 hover:text-[#B08B5B] dark:hover:text-[#C9A876] transition-colors"
                            on:click=move |_| toggle_mobile_group("people")
                        >
                            <span class="font-medium">"People"</span>
                            <svg class=move || if mobile_group.get() == Some("people") { "w-3.5 h-3.5 transition-transform rotate-180" } else { "w-3.5 h-3.5 transition-transform" } fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 9l-7 7-7-7"></path>
                            </svg>
                        </button>
                        <div class=move || if mobile_group.get() == Some("people") { "pl-4 space-y-1" } else { "hidden" }>
                            <A href="/people" on:click=move |_| close_mobile_menu() attr:class="block px-3 py-2 text-sm text-gray-700 dark:text-gray-200 hover:text-[#B08B5B] dark:hover:text-[#C9A876] transition-colors">"Poker@Berkeley Officers"</A>
                            <A href="/decal#course-staff" on:click=move |_| close_mobile_menu() attr:class="block px-3 py-2 text-sm text-gray-700 dark:text-gray-200 hover:text-[#B08B5B] dark:hover:text-[#C9A876] transition-colors">"Stat 198 Course Staff"</A>
                        </div>

                        <button
                            type="button"
                            class="w-full flex items-center justify-between px-3 py-2 text-gray-700 dark:text-gray-200 hover:text-[#B08B5B] dark:hover:text-[#C9A876] transition-colors"
                            on:click=move |_| toggle_mobile_group("events")
                        >
                            <span class="font-medium">"Events"</span>
                            <svg class=move || if mobile_group.get() == Some("events") { "w-3.5 h-3.5 transition-transform rotate-180" } else { "w-3.5 h-3.5 transition-transform" } fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 9l-7 7-7-7"></path>
                            </svg>
                        </button>
                        <div class=move || if mobile_group.get() == Some("events") { "pl-4 space-y-1" } else { "hidden" }>
                            <A href="/tournaments" on:click=move |_| close_mobile_menu() attr:class="block px-3 py-2 text-sm text-gray-700 dark:text-gray-200 hover:text-[#B08B5B] dark:hover:text-[#C9A876] transition-colors">"Tournaments"</A>
                            <A href="/game-nights" on:click=move |_| close_mobile_menu() attr:class="block px-3 py-2 text-sm text-gray-700 dark:text-gray-200 hover:text-[#B08B5B] dark:hover:text-[#C9A876] transition-colors">"Game Nights"</A>
                            <A href="/pokerbots" on:click=move |_| close_mobile_menu() attr:class="block px-3 py-2 text-sm text-gray-700 dark:text-gray-200 hover:text-[#B08B5B] dark:hover:text-[#C9A876] transition-colors">"PokerBots"</A>
                        </div>

                        <button
                            type="button"
                            class="w-full flex items-center justify-between px-3 py-2 text-gray-700 dark:text-gray-200 hover:text-[#B08B5B] dark:hover:text-[#C9A876] transition-colors"
                            on:click=move |_| toggle_mobile_group("learn")
                        >
                            <span class="font-medium">"Learn"</span>
                            <svg class=move || if mobile_group.get() == Some("learn") { "w-3.5 h-3.5 transition-transform rotate-180" } else { "w-3.5 h-3.5 transition-transform" } fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 9l-7 7-7-7"></path>
                            </svg>
                        </button>
                        <div class=move || if mobile_group.get() == Some("learn") { "pl-4 space-y-1" } else { "hidden" }>
                            <A href="/decal" on:click=move |_| close_mobile_menu() attr:class="block px-3 py-2 text-sm text-gray-700 dark:text-gray-200 hover:text-[#B08B5B] dark:hover:text-[#C9A876] transition-colors">"Poker Theory DeCal"</A>
                            <A href="/resources" on:click=move |_| close_mobile_menu() attr:class="block px-3 py-2 text-sm text-gray-700 dark:text-gray-200 hover:text-[#B08B5B] dark:hover:text-[#C9A876] transition-colors">"Resources"</A>
                            <A href="/blog" on:click=move |_| close_mobile_menu() attr:class="block px-3 py-2 text-sm text-gray-700 dark:text-gray-200 hover:text-[#B08B5B] dark:hover:text-[#C9A876] transition-colors">"Blog"</A>
                            <a href="https://discord.gg/wWrZj5cM5X" target="_blank" rel="noopener" on:click=move |_| close_mobile_menu() class="block px-3 py-2 text-sm text-gray-700 dark:text-gray-200 hover:text-[#B08B5B] dark:hover:text-[#C9A876] transition-colors">"Join the Stat 198 Discord"</a>
                        </div>

                        <A href="/sponsors" on:click=move |_| close_mobile_menu() attr:class="block px-3 py-2 text-gray-700 dark:text-gray-200 hover:text-[#B08B5B] dark:hover:text-[#C9A876] transition-colors">"Sponsors"</A>
                        <A href="/merch" on:click=move |_| close_mobile_menu() attr:class="block px-3 py-2 text-gray-700 dark:text-gray-200 hover:text-[#B08B5B] dark:hover:text-[#C9A876] transition-colors">"Merch"</A>
                    </div>
                </div>
            </div>
        </nav>
    }
}
