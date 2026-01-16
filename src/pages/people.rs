use leptos::prelude::*;
use crate::components::OptimizedImage;

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
        <section id="banner" class="py-40"
                 style="background-image: url('public/images/board_fall25_tourney.jpg'); background-size: cover; background-position: center;">
            <div class="max-w-4xl mx-auto text-center px-6">
                <h1 class="text-5xl md:text-6xl font-bold text-white mb-4">
                    "Who We Are"
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
            <div class="max-w-6xl mx-auto px-6">
                <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-8"> 
                    <MemberCard
                        name="Maysa Barandish"
                        role="President, Head of DeCal"
                        image="public/images/officers/maysa.png"
                        bio=|| view! {
                            <>
                                "Dallas native studying Econ + DS. Favorite hand is the changed diaper (dirty diaper but suited), a very reliable wealth distributor.\n\n"
                                "When I'm not playing poker, I’m usually gambling in some other form. Outside of that, I enjoy backgammon and hikes with my dog. \n\n"
                                "Not from Chicago. Not studying EECS. "
                                <em>"Someone"</em> 
                                " has to defend the bottom of the range."
                            </>
                        }.into_any()
                        linkedin="https://www.linkedin.com/in/maysa-barandish-5ba59084/"
                    />
                    <MemberCard
                        name="David Chen"
                        role="Head of Sponsorships"
                        image="public/images/officers/dc.jpg"
                        bio=|| view! {
                            <>
                                "I’m David, a 3rd year “studying” Business + Stats. Besides poker, I enjoy golfing, fantasizing about Peter Thiel, watching Soccer/Tennis, increasing shareholder value, and reading (I’m illiterate)."
                            </>
                        }.into_any()
                        linkedin="https://www.linkedin.com/in/david-chen-b639a4274"
                    />
                    <MemberCard
                        name="Evan Luo"
                        role="Head of Finance"
                        image="public/images/officers/levan.png"
                        bio=|| view! {
                            <>
                                "I’m Evan, a sophomore"
                                " studying EECS. I was"
                                " born in Chicago, but raised in San Diego.\n\n"
                                "My favorite hand is KJo. When not playing poker (most of the time) I can be found dilly dallying at the RSF, various sushi restaurants, or at a boba shop."
                            </>
                        }.into_any()
                        linkedin="https://www.linkedin.com/in/theevanluo/"
                    />
                    <MemberCard
                        name="Pranshu Rao"
                        role="Head of External"
                        image="public/images/officers/pranshu.png"
                        bio=|| view! {
                            <>
                                "Hey, I'm Pranshu, "
                                <em>"another"</em>
                                " Chicago native who "
                                <em>"also"</em>
                                " happens to study EECS.\n\n"
                                "My VPIP is close to 70%"
                                " and when I'm not punting my money away I enjoy soccer, hiking, and late night drives.\n\n"
                            </>
                        }.into_any()
                        linkedin="https://www.linkedin.com/in/pranshurao/"
                    />
                    <MemberCard
                        name="Jones Dickerson"
                        role="Head of Tournaments"
                        image="public/images/officers/jones.png"
                        bio=|| view! { <></> }.into_any()
                        linkedin=""
                    />
                    <MemberCard
                        name="Juan Belza-Garcia"
                        role="Head of Internal"
                        image="public/images/officers/juan.png"
                        bio=|| view! { <></> }.into_any()
                        linkedin="https://www.linkedin.com/in/juan-belza"
                    />
                    <MemberCard
                        name="Nicolas Bruzzeze"
                        role="Head of Game Nights"
                        image="public/images/officers/nic.png"
                        bio=|| view! { <></> }.into_any()
                        linkedin=""
                    />
                    <MemberCard
                        name="Jonathan James"
                        role="Head of Design"
                        image="public/images/officers/jj.png"
                        bio=|| view! { <></> }.into_any()
                        linkedin="https://www.linkedin.com/in/jonathan-c-james/"
                    />
                    <MemberCard
                        name="Jennifer Ren"
                        role="Head of Media"
                        image="public/images/officers/jen.png"
                        bio=|| view! { <></> }.into_any()
                        linkedin=""
                    />
                    <MemberCard
                        name="Sriram Srivatsan"
                        role="Special Projects Team"
                        image="public/images/officers/sriram.png"
                        bio=|| view! { <></> }.into_any()
                        linkedin="https://www.linkedin.com/in/sriram-eecs/"
                    />
                    <MemberCard
                        name="Chris Dodla"
                        role="DeCal Staff"
                        image="public/images/officers/chris.png"
                        bio=|| view! { <></> }.into_any()
                        linkedin=""
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
                    <AlumniCard name="Grace Tang" image="public/images/officers/grace.png" />
                    <AlumniCard name="Evan Yeager" image="public/images/officers/yevan.png" />
                    <AlumniCard name="Johan Ko" image="public/images/officers/johan.png" />
                    <AlumniCard name="Welford Chen" image="public/images/alumni/welford.png" />
                    <AlumniCard name="Ariel Qian" image="public/images/alumni/ariel.jpeg" />
                    <AlumniCard name="Samarth Goel" image="public/images/alumni/samarth.jpeg" />
                    <AlumniCard name="Reagan Lee" image="public/images/alumni/reagan.jpeg" />
                    <AlumniCard name="William Lin" image="public/images/alumni/william.jpeg" />
                    <AlumniCard name="Gavin Yu" image="public/images/alumni/gavin.jpeg" />
                    <AlumniCard name="Prakash Srivastava" image="public/images/alumni/prakash.jpeg" />
                    <AlumniCard name="Aidan Reilly" image="public/images/alumni/aidan.jpeg" />
                    <AlumniCard name="Kevin An" image="public/images/alumni/kevin.jpeg" />
                </div>

                <h2 class="text-4xl font-bold text-center text-gray-900 dark:text-white mb-8">
                    "Officer career destinations"
                </h2>

                <div class="text-center">
                    <OptimizedImage src="public/images/destinations.png" alt="Member Destinations" class="w-full max-w-4xl mx-auto rounded-lg shadow-lg" loading="lazy" />
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
    bio: impl Fn() -> AnyView + 'static,
    linkedin: &'static str,
) -> impl IntoView {
    view! {
        <div class="bg-white dark:bg-gray-700 rounded-lg shadow-lg overflow-hidden hover:shadow-xl transition-shadow">
            {if image.is_empty() {
                view! {
                    <div class="w-full h-64 bg-gray-200 dark:bg-gray-600 flex items-center justify-center">
                        <span class="text-gray-500 dark:text-gray-300 text-4xl font-bold">
                            {name.chars().next().unwrap_or('?').to_string()}
                        </span>
                    </div>
                }.into_any()
            } else {
                view! {
                    <div class="w-full h-64">
                        <img src=image alt=name class="w-full h-64 object-cover" loading="lazy" />
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
                <p class="text-blue-600 dark:text-blue-400 font-semibold mb-3">{role}</p>
                <div class="text-gray-600 dark:text-gray-300 text-sm whitespace-pre-line">
                    {bio()}
                </div>
            </div>
        </div>
    }
}

#[component]
fn AlumniCard(name: &'static str, image: &'static str) -> impl IntoView {
    view! {
        <div class="text-center">
            <img src=image alt=name class="w-20 h-20 rounded-full mx-auto mb-2 object-cover shadow-lg" loading="lazy" />
            <p class="text-sm text-gray-700 dark:text-gray-300 font-medium">{name}</p>
        </div>
    }
}
