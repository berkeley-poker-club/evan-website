use leptos::prelude::*;

#[component]
pub fn PeoplePage() -> impl IntoView {
    view! {
        <div class="min-h-screen">
            <HeroBanner />
            <BoardSection />
            <AlumniSection />
        </div>
    }
}

#[component]
fn HeroBanner() -> impl IntoView {
    view! {
        <section id="banner" class="relative min-h-screen flex items-center justify-center"
                 style="background-image: url('/public/images/sp26board/standinghearst-pano.webp'); background-size: cover; background-position: center center;">
            <div class="absolute inset-0" style="background-color: rgba(0, 0, 0, 0.30);"></div>
            <div class="relative z-10 max-w-4xl mx-auto text-center px-6" style="transform: translateY(150%);">
                <h1 class="text-5xl md:text-6xl font-bold text-white mb-4">
                    "Officers"
                </h1>
                <p class="text-xl text-white/90">
                    "Meet the dedicated team behind Poker at Berkeley"
                </p>
            </div>
        </section>
    }
}

#[component]
fn BoardSection() -> impl IntoView {
    view! {
        <section class="py-20 bg-white dark:bg-gray-800">
            <div class="max-w-[1600px] mx-auto px-6">
                <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-6">
                    <MemberCard
                        name="Maysa Barandish"
                        role="President\nHead of DeCal"
                        image="/public/images/officers/maysa.webp"
                        preview="Maysa is many things: a one-tabling TwoPlusTwo provocateur (banned), occasional luckbox,"
                        bio=|| view! {
                            <>
                                "Maysa is many things: a one-tabling TwoPlusTwo provocateur (banned), occasional luckbox, dog mom to Milly (outfits non-negotiable), backgammon enthusiast, collector of pre-Sunshine Act pharmaceutical relics (think Nexium mug, Viagra tie, Lexapro clock, Seroquel anything, etc.), a native Texan studying Poli Econ and DS, and her friends' ordained minister (has officiated 2 weddings).\n\n"
                                "She is the unholy lovechild of Anthony Bourdain and Tony G, raised by Alan Kessler, a casino nomad raised on comped buffet shrimp and unsolicited table talk. \n\n"
                                "Her favorite hand is the dirty diaper, but suited. Lexically, she calls it the changed diaper. She's been playing poker in card rooms for almost a decade, but has been open jamming UTG with the changed diaper since she was in them herself.\n\n"
                                "For these reasons, she is also the President of Poker@Berkeley."
                            </>
                        }.into_any()
                        linkedin="https://www.linkedin.com/in/maysa-barandish-5ba59084/"
                    />
                    <MemberCard
                        name="Nicolas Bruzzese"
                        role="Head of Game Nights"
                        image="/public/images/officers/nicolas.webp"
                        bio=|| view! { <></> }.into_any()
                        linkedin="https://www.linkedin.com/in/nicolas-bruzzese-064532293/"
                    />
                    <MemberCard
                        name="Jones Dickerson"
                        role="Head of Tournaments"
                        image="/public/images/officers/jones.webp"
                        preview="4th year studying Pure Math. Many many hours at all stakes."
                        bio=|| view! {
                            <>
                                "4th year studying Pure Math\n\n"
                                "Many many hours at all stakes\n\n"
                                "4th semester teaching Poker DeCal!\n\n"
                                "Outside of poker, I enjoy playing tennis, cello, learning something random, and traveling"
                            </>
                        }.into_any()
                        linkedin="https://www.linkedin.com/in/jones-dickerson/"
                    />
                    <MemberCard
                        name="Evan Luo"
                        role="Head of Finance"
                        image="/public/images/newheadshots/evan2.webp"
                        preview="I’m Evan, a sophomore studying EECS. I was born in Chicago,"
                        bio=|| view! {
                            <>
                                "I’m Evan, a sophomore studying EECS. I was born in Chicago, but raised in San Diego.\n\n"
                                "When not playing poker I can be found dilly dallying at Benchmark Climbing, various sushi restaurants, or at a boba shop."
                            </>
                        }.into_any()
                        linkedin="https://www.linkedin.com/in/theevanluo/"
                    />
                    <MemberCard
                        name="Pranshu Rao"
                        role="Head of Sponsorships"
                        image="/public/images/officers/pranshunew.webp"
                        preview="Hey, I'm Pranshu, a Chicago native who also happens to study"
                        bio=|| view! {
                            <>
                                "Hey, I'm Pranshu, a Chicago native who "
                                <em>"also"</em>
                                " happens to study EECS.\n\n"
                                "My VPIP is close to 70% and when I'm not punting my money away I enjoy soccer, hiking, and late night drives.\n\n"
                            </>
                        }.into_any()
                        linkedin="https://www.linkedin.com/in/pranshurao/"
                    />
                    <MemberCard
                        name="David Chen"
                        role="Senior Advisor"
                        image="/public/images/officers/david.webp"
                        preview="I’m David, a 3rd year “studying” Business + Stats. Besides poker,"
                        bio=|| view! {
                            <>
                                "I’m David, a 3rd year “studying” Business + Stats. Besides poker, I enjoy golfing, fantasizing about Peter Thiel, watching Soccer/Tennis, increasing shareholder value, and reading (I’m illiterate)."
                            </>
                        }.into_any()
                        linkedin="https://www.linkedin.com/in/david-chen-b639a4274"
                    />
                    <MemberCard
                        name="Jennifer Ren"
                        role="Head of Media"
                        image="/public/images/officers/jen.webp"
                        bio=|| view! { <></> }.into_any()
                        linkedin="https://www.linkedin.com/in/jennifer-ren/"
                    />
                    <MemberCard
                        name="Milly Barandish"
                        role="Head of Emotional Support"
                        image="/public/images/officers/milly.webp"
                        preview="Milly is originally from Texas, which is why she refuses the"
                        bio=|| view! {
                            <>
                                "Milly is originally from Texas, which is why she refuses the water bowl if it doesn't have ice in it and won't walk on cement without her booties. \n\n"
                                "10k+ lifetime hands in 500NLH+, mostly live, some iPad volume. Deep stack specialist, 2 WSOP Circuit rings. Elite table presence. Applies max ICM pressure when someone is eating food. Loves gambling. One flip away from a 12-step program. Does not study solvers. Pure instinct player.\n\n"
                                "Likes: watching Reels on Evan's phone, getting ice cream from Jen, crashing Jen and Jones's dates, walks with David her dogwalker, and being the center of attention.\n\n"
                                "Dislikes: being put in outfits by her mom, Maysa, oh and nits.\n\n"
                                "Fav hand: K9 of any combo, except spades (hasn't dug since domesticated, doesn't like to be reminded). Plays 100% of range minus spades, removes 585 combos, does not care.\n\n"
                                "woof."
                            </>
                        }.into_any()
                        linkedin="https://open.spotify.com/playlist/0MQXY5ehHGyycEMDBIGSPu?si=4e8c75a143fe4449"
                    />
                    <MemberCard
                        name="Szymon Jackowski"
                        role="Sponsorships"
                        image="/public/images/officers/szymon.webp"
                        bio=|| view! { <></> }.into_any()
                        linkedin="https://www.linkedin.com/in/szymonjackowski"
                    />
                    <MemberCard
                        name="Timur Usmonov"
                        role="Tournaments"
                        image="/public/images/officers/timur.webp"
                        bio=|| view! { <></> }.into_any()
                        linkedin="https://www.linkedin.com/in/timur-usmonov/"
                    />
                    <MemberCard
                        name="Vincent Chen"
                        role="Game Nights"
                        image="/public/images/officers/vincent.webp"
                        bio=|| view! { <></> }.into_any()
                        linkedin="https://www.linkedin.com/in/vincentchen8/"
                    />
                    <MemberCard
                        name="Joe Zhou"
                        role="Finance"
                        image="/public/images/officers/joe.webp"
                        bio=|| view! { <></> }.into_any()
                        linkedin="https://www.linkedin.com/in/joe-zhou-a6159b230/"
                    />
                    <MemberCard
                        name="Tanya Zhang"
                        role="Media"
                        image="/public/images/officers/tanya.webp"
                        bio=|| view! { <></> }.into_any()
                        linkedin="https://www.linkedin.com/in/tanyashenzhang/"
                    />
                </div>
            </div>
        </section>
    }
}

#[component]
fn AlumniSection() -> impl IntoView {
    view! {
        <section class="py-20 bg-white dark:bg-gray-900">
            <div class="max-w-6xl mx-auto px-6">
                <h2 class="text-4xl font-bold text-center text-gray-900 dark:text-white mb-8">
                    "Alumni"
                </h2>

                <div class="grid grid-cols-2 md:grid-cols-3 lg:grid-cols-4 gap-6 mb-12">
                    <AlumniCard name="Grace Tang" image="/public/images/newheadshots/grace.jpg" linkedin="https://www.linkedin.com/in/grace-j-tang/" />
                    <AlumniCard name="Juan Belza-Garcia" image="/public/images/officers/juan.webp" linkedin="https://www.linkedin.com/in/juan-belza/" />
                    <AlumniCard name="Sriram Srivatsan" image="/public/images/officers/sriram.webp" linkedin="https://www.linkedin.com/in/sriram-eecs/" />
                    <AlumniCard name="Evan Yeager" image="/public/images/officers/yevan.webp" linkedin="https://www.linkedin.com/in/evanyeager/" />
                    <AlumniCard name="Johan Ko" image="/public/images/officers/johan.webp" linkedin="https://www.linkedin.com/in/kojohan/" />
                    <AlumniCard name="Welford Chen" image="/public/images/alumni/welford.webp" linkedin="https://www.linkedin.com/in/welford-chen-803084301/" />
                    <AlumniCard name="Ariel Qian" image="/public/images/alumni/ariel.webp" linkedin="https://www.linkedin.com/in/arielqian/" />
                    <AlumniCard name="Samarth Goel" image="/public/images/alumni/samarth.webp" linkedin="https://www.linkedin.com/in/samarthgoel1/" />
                    <AlumniCard name="Reagan Lee" image="/public/images/alumni/reagan.webp" linkedin="https://www.linkedin.com/in/reaganjlee/" />
                    <AlumniCard name="William Lin" image="/public/images/alumni/william.webp" linkedin="https://www.linkedin.com/in/lin-w/" />
                    <AlumniCard name="Gavin Yu" image="/public/images/alumni/gavin.webp" linkedin="https://www.linkedin.com/in/gavin-yu/" />
                    <AlumniCard name="Prakash Srivastava" image="/public/images/alumni/prakash.webp" linkedin="https://www.linkedin.com/in/apsrivastava141/" />
                    <AlumniCard name="Aidan Reilly" image="/public/images/alumni/aidan.webp" linkedin="https://www.linkedin.com/in/aidannreilly/" />
                    <AlumniCard name="Kevin An" image="/public/images/alumni/kevin.webp" linkedin="https://www.linkedin.com/in/kevin-an-b083681a9/" />
                </div>

                <h2 class="text-4xl font-bold text-center text-gray-900 dark:text-white mb-8">
                    "Officer career destinations"
                </h2>

                <div class="text-center">
                    <img src="/public/images/logos_light.webp" alt="Member Destinations" class="w-full max-w-4xl mx-auto rounded-lg shadow-lg dark:hidden" loading="lazy" />
                    <img src="/public/images/logos_dark.webp" alt="Member Destinations" class="w-full max-w-4xl mx-auto rounded-lg shadow-lg hidden dark:block" loading="lazy" />
                </div>
            </div>
        </section>
    }
}

#[component]
fn MemberCard(
    name: &'static str,
    role: &'static str,
    image: &'static str,
    #[prop(optional, default = "")]
    preview: &'static str,
    bio: impl Fn() -> AnyView + 'static,
    linkedin: &'static str,
) -> impl IntoView {
    let has_more = !preview.is_empty();

    view! {
        <div class="bg-white dark:bg-gray-700 rounded-lg shadow-lg overflow-hidden hover:shadow-xl transition-shadow">
            {if image.is_empty() {
                view! {
                    <div class="w-full h-72 md:h-80 lg:h-72 xl:h-80 bg-gray-200 dark:bg-gray-600 flex items-center justify-center">
                        <span class="text-gray-500 dark:text-gray-300 text-4xl font-bold">
                            {name.chars().next().unwrap_or('?').to_string()}
                        </span>
                    </div>
                }.into_any()
            } else {
                view! {
                    <div class="w-full h-72 md:h-80 lg:h-72 xl:h-80">
                        <img src=image alt=name class="w-full h-72 md:h-80 lg:h-72 xl:h-80 object-cover" loading="lazy" />
                    </div>
                }.into_any()
            }}
            <div class="p-6">
                <h3 class="text-xl font-bold text-gray-900 dark:text-white mb-1">
                    {if linkedin.is_empty() {
                        view! { <span>{name}</span> }.into_any()
                    } else {
                        view! {
                            <a href=linkedin target="_blank" class="hover:text-blue-600 dark:hover:text-blue-400 transition-colors">
                                {name}
                            </a>
                        }.into_any()
                    }}
                </h3>
                {if name == "Maysa Barandish" {
                    view! {
                        <p class="font-semibold mb-3">
                            <span class="block" style="color: #D4A017;">"President"</span>
                            <span class="block text-blue-600 dark:text-blue-400">"Head of DeCal"</span>
                        </p>
                    }.into_any()
                } else {
                    view! {
                        <p class="text-blue-600 dark:text-blue-400 font-semibold mb-3 whitespace-pre-line">{role}</p>
                    }.into_any()
                }}
                {if has_more {
                    view! {
                        <style>
                            ".bio-details[open] { display: flex; flex-direction: column; }
                            .bio-details[open] summary { order: 2; margin-top: 0.5rem; }
                            .bio-details[open] .bio-preview { display: none; }
                            .bio-details .bio-collapse { display: none; }
                            .bio-details[open] .bio-collapse { display: inline; }
                            .bio-details[open] .bio-full { order: 1; }"
                        </style>
                        <details class="bio-details text-gray-600 dark:text-gray-300 text-xs whitespace-pre-line">
                            <summary class="cursor-pointer list-none">
                                <span class="bio-preview">
                                    {preview} "… "
                                    <span class="font-semibold text-blue-600 hover:text-blue-800 dark:text-blue-400 dark:hover:text-blue-300">
                                        "See more…"
                                    </span>
                                </span>
                                <span class="bio-collapse font-semibold text-blue-600 hover:text-blue-800 dark:text-blue-400 dark:hover:text-blue-300">
                                    "Show less"
                                </span>
                            </summary>
                            <div class="bio-full mt-2">{bio()}</div>
                        </details>
                    }.into_any()
                } else {
                    view! {
                        <div class="text-gray-600 dark:text-gray-300 text-xs whitespace-pre-line">
                            {bio()}
                        </div>
                    }.into_any()
                }}
            </div>
        </div>
    }
}

#[component]
fn AlumniCard(
    name: &'static str,
    image: &'static str,
    linkedin: &'static str,
) -> impl IntoView {
    view! {
        <div class="text-center">
            <img src=image alt=name class="w-20 h-20 rounded-full mx-auto mb-2 object-cover shadow-lg" loading="lazy" />
            {if linkedin.is_empty() {
                view! {
                    <p class="text-sm text-gray-700 dark:text-gray-300 font-medium">{name}</p>
                }.into_any()
            } else {
                view! {
                    <a
                        href=linkedin
                        target="_blank"
                        class="text-sm text-gray-700 dark:text-gray-300 font-medium hover:text-blue-600 dark:hover:text-blue-400 transition-colors"
                    >
                        {name}
                    </a>
                }.into_any()
            }}
        </div>
    }
}
