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
        <section id="banner" class="py-32"
                 style="background-image: url('public/images/pab_board_apr25.jpg'); background-size: cover; background-position: center;">
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
        <section class="py-20 bg-white">
            <div class="max-w-6xl mx-auto px-6">
                <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-8">
                    <MemberCard
                        name="Grace Tang"
                        role="Co-President, Head of Internal"
                        image="public/images/officers/grace.png"
                        bio="I'm Grace, a third-year studying EECS and math! My favorite hand is A4s. When I'm not playing poker, I can be found at the climbing gym, or more commonly at the research lab trying to teach robots to learn!"
                        linkedin="https://www.linkedin.com/in/grace-j-tang/"
                    />
                    <MemberCard
                        name="Evan Yeager"
                        role="Co-President, Head of External"
                        image="public/images/officers/yevan.png"
                        bio="I'm Evan, a Chicago native majoring in Poker who happens to study EECS as well. My playstyle is loose and aggressive with excessive hand commentary. When I'm not playing poker I enjoy weightlifting, travel, and predictive modeling."
                        linkedin="https://www.linkedin.com/in/evanyeager/"
                    />
                    <MemberCard
                        name="Pranshu Rao"
                        role="Head of Special Projects"
                        image="public/images/officers/pranshu.png"
                        bio="Hey, I'm Pranshu, another Chicago native who happens to also study EECS. My VPIP is close to 70% and when I'm not punting my money away I enjoy watching soccer, hiking, and going on late night drives."
                        linkedin="https://www.linkedin.com/in/pranshurao/"
                    />
                    <MemberCard
                        name="Maysa Barandish"
                        role="Head of Media & DeCal"
                        image="public/images/officers/maysa.png"
                        bio="I'm Maysa, A Dallas native in my third-year majoring in Economics and Data Science. My favorite hobbies include 5bet shoving A5s into aces, bubbling final tables, never realizing my equity, rationalizing my punts, and chasing gutters."
                        linkedin="https://www.linkedin.com/in/maysa-barandish-5ba59084/"
                    />
                    <MemberCard
                        name="Jones Dickerson"
                        role="Head of IPA, DeCal Instructor"
                        image="public/images/decal-staff/jones.png"
                        bio=""
                        linkedin=""
                    />
                    <MemberCard
                        name="Juan Belza-Garcia"
                        role="External"
                        image="public/images/officers/juan.png"
                        bio=""
                        linkedin="https://www.linkedin.com/in/juan-belza"
                    />
                    <MemberCard
                        name="David Chen"
                        role="DeCal Staff"
                        image="public/images/officers/dc.jpg"
                        bio=""
                        linkedin="https://www.linkedin.com/in/david-chen-b639a4274"
                    />
                    <MemberCard
                        name="Jennifer Ren"
                        role="External"
                        image="public/images/officers/jen.png"
                        bio=""
                        linkedin=""
                    />
                    <MemberCard
                        name="Jonathan James"
                        role="Internal"
                        image="public/images/officers/jj.png"
                        bio=""
                        linkedin="https://www.linkedin.com/in/jonathan-c-james/"
                    />
                    <MemberCard
                        name="Evan Luo"
                        role="Special Projects Team"
                        image="public/images/officers/levan.png"
                        bio=""
                        linkedin="https://www.linkedin.com/in/theevanluo/"
                    />
                    <MemberCard
                        name="Johan Ko"
                        role="Special Projects Team"
                        image="public/images/officers/johan.png"
                        bio=""
                        linkedin="https://www.linkedin.com/in/kojohan"
                    />
                    <MemberCard
                        name="Nicolas Bruzzeze"
                        role="IPA Team"
                        image="public/images/officers/nic.png"
                        bio=""
                        linkedin=""
                    />
                    <MemberCard
                        name="Sriram Srivatsan"
                        role="Special Projects Team"
                        image="public/images/officers/sriram.png"
                        bio=""
                        linkedin=""
                    />
                    <MemberCard
                        name="Chris Dodla"
                        role="DeCal Staff"
                        image="public/images/officers/chris.png"
                        bio=""
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
        <section class="py-20 bg-white">
            <div class="max-w-6xl mx-auto px-6">
                <h2 class="text-4xl font-bold text-center text-gray-900 mb-8">
                    "Alumni"
                </h2>

                <div class="grid grid-cols-2 md:grid-cols-3 lg:grid-cols-4 gap-6 mb-12">
                    <AlumniCard name="Rick Zhao" image="public/images/alumni/rick.png" />
                    <AlumniCard name="Welford Chen" image="public/images/alumni/welford.png" />
                    <AlumniCard name="Darsh Patel" image="public/images/alumni/darsh.png" />
                    <AlumniCard name="Brian Tong" image="public/images/alumni/brian.png" />
                    <AlumniCard name="Ariel Qian" image="public/images/alumni/ariel.jpeg" />
                    <AlumniCard name="Samarth Goel" image="public/images/alumni/samarth.jpeg" />
                    <AlumniCard name="Reagan Lee" image="public/images/alumni/reagan.jpeg" />
                    <AlumniCard name="William Lin" image="public/images/alumni/william.jpeg" />
                    <AlumniCard name="Gavin Yu" image="public/images/alumni/gavin.jpeg" />
                    <AlumniCard name="Prakash Srivastava" image="public/images/alumni/prakash.jpeg" />
                    <AlumniCard name="Aidan Reilly" image="public/images/alumni/aidan.jpeg" />
                    <AlumniCard name="Kevin An" image="public/images/alumni/kevin.jpeg" />
                </div>

                <h2 class="text-4xl font-bold text-center text-gray-900 mb-8">
                    "Officer career destinations"
                </h2>

                <div class="text-center">
                    <img src="public/images/destinations.png" alt="Member Destinations" class="w-full max-w-4xl mx-auto rounded-lg shadow-lg" />
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
    bio: &'static str,
    linkedin: &'static str,
) -> impl IntoView {
    view! {
        <div class="bg-white rounded-lg shadow-lg overflow-hidden hover:shadow-xl transition-shadow">
            {if image.is_empty() {
                view! {
                    <div class="w-full h-64 bg-gray-200 flex items-center justify-center">
                        <span class="text-gray-500 text-4xl font-bold">
                            {name.chars().next().unwrap_or('?').to_string()}
                        </span>
                    </div>
                }.into_any()
            } else {
                view! {
                    <div class="w-full h-64">
                        <img src=image alt=name class="w-full h-64 object-cover" />
                    </div>
                }.into_any()
            }}
            <div class="p-6">
                <h3 class="text-xl font-bold text-gray-900 mb-1">
                    {if linkedin.is_empty() {
                        view! { <span>{name}</span> }.into_any()
                    } else {
                        view! {
                            <a href=linkedin target="_blank" class="hover:text-blue-600 transition-colors">
                                {name}
                            </a>
                        }.into_any()
                    }}
                </h3>
                <p class="text-blue-600 font-semibold mb-3">{role}</p>
                {if !bio.is_empty() {
                    view! { <p class="text-gray-600 text-sm">{bio}</p> }.into_any()
                } else {
                    view! { <div></div> }.into_any()
                }}
            </div>
        </div>
    }
}

#[component]
fn AlumniCard(name: &'static str, image: &'static str) -> impl IntoView {
    view! {
        <div class="text-center">
            <img src=image alt=name class="w-20 h-20 rounded-full mx-auto mb-2 object-cover shadow-lg" />
            <p class="text-sm text-gray-700 font-medium">{name}</p>
        </div>
    }
}
