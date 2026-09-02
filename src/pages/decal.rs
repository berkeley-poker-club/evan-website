use leptos::prelude::*;

const DECAL_SYLLABUS: &str =
    "https://docs.google.com/document/d/1j2qeTiDadAEusrmj_eKZ_TBn-Y7Vs2GrTfd-CVPuqCc/edit?usp=sharing";
const DECAL_TA_APPLICATION_FORM: &str = "https://forms.gle/ZJyBk9brK8iRuAtR6";
const DECAL_APPLICATION_FORM: &str = "https://forms.gle/KeZfYyJtCgSvezSf7";

#[component]
pub fn DecalPage() -> impl IntoView {
    view! {
        <div class="min-h-screen">
            <style>
                "@keyframes pageLoadOverlayFade {
                    0% { opacity: 1; }
                    60% { opacity: 1; }
                    100% { opacity: 0; visibility: hidden; }
                }"
            </style>
            <div style="position: fixed; top: 0; left: 0; width: 100vw; height: 100vh; background-color: #000; z-index: 9999; pointer-events: none; animation: pageLoadOverlayFade 1s ease-out forwards;"></div>
            <HeroBanner />
            <DecalApplyBar />
            <HistorySection />
            <CourseOverviewSection />
            <CourseStructureSection />
            <CourseScheduleSection />
            <InstructorsSection />
            <GradingSection />
            <ApplySection title="Ready to Apply?" />
        </div>
    }
}

#[component]
fn HeroBanner() -> impl IntoView {
    view! {
        <section id="banner" class="relative flex items-end justify-center pb-16" style="height: 70vh; background-image: url('public/images/sp26board/decal.webp'); background-size: cover; background-position: center;">
            <div class="absolute inset-0" style="background-color: rgba(0, 0, 0, 0.2);"></div>
            <div class="relative z-10 max-w-4xl mx-auto text-center px-6">
                <h1 class="text-6xl md:text-7xl font-bold mb-2" style="color: #E0BC72; text-shadow: 0 2px 12px rgba(0,0,0,0.7);">
                    "Poker DeCal"
                </h1>
                <p class="text-xl md:text-2xl text-white font-semibold" style="text-shadow: 0 2px 12px rgba(0,0,0,0.7);">
                    "Stat 198: Poker Theory & Fundamentals | Fall 2026"
                </p>
            </div>
        </section>
    }
}

#[component]
fn CourseDescriptionSection() -> impl IntoView {
    view! {
        <section class="py-20 bg-white dark:bg-gray-800">
            <div class="max-w-4xl mx-auto px-6">
                <h2 class="text-4xl font-bold text-center text-gray-900 dark:text-white mb-8">
                    "Course Description"
                </h2>
                <div class="bg-gray-50 dark:bg-gray-700 rounded-lg p-8">
                    <p class="text-lg text-gray-700 dark:text-gray-300 leading-relaxed mb-6">
                        "This course introduces the fundamentals of poker with an emphasis on strategic thinking and decision-making, open to students of all experience levels. While basic rules will be covered quickly, most of the course focuses on higher-level concepts in 6-max No-Limit Texas Hold'em—the most widely played form of poker today."
                    </p>
                    <p class="text-lg text-gray-700 dark:text-gray-300 leading-relaxed mb-6">
                        "Students will examine each stage of a hand and explore the mathematics, heuristics, and structured reasoning that strong players use, with some attention to psychological factors such as live reads and behavioral patterns. Beyond poker, these concepts develop analytical skills relevant to statistics, game theory, economics, finance, and investing."
                    </p>
                    <p class="text-lg text-gray-700 dark:text-gray-300 leading-relaxed">
                        "The DeCal was originally started in TBD by UC Berkeley undergraduate David Daneshgar, who went on to win a WSOP bracelet in TBD."
                    </p>
                </div>
            </div>
        </section>
    }
}

#[component]
fn HistorySection() -> impl IntoView {
    view! {
        <section class="pt-10 pb-10 bg-gray-100 dark:bg-gray-800">
            <div class="max-w-6xl mx-auto px-6">
                <div class="flex flex-col lg:flex-row gap-12 items-center">
                    <div class="text-left lg:w-1/2">
                        <p class="text-4xl md:text-5xl font-bold text-gray-900 dark:text-white leading-tight">
                            "Our history: The DeCal was originally started in 2003 by UC Berkeley undergraduate David Daneshgar, who went on to win a WSOP bracelet in 2008."
                        </p>
                    </div>
                    <div class="lg:w-1/2">
                        <img
                            src="public/images/daviddaneshgar.webp"
                            alt="David Daneshgar"
                            class="w-full h-auto object-contain object-top rounded-lg shadow-lg"
                            loading="lazy"
                        />
                        <p class="text-sm italic text-gray-400 mt-2">
                            "After that, Daneshgar used cash he won in a poker tournament as seed funding for his first company, "
                            <a href="https://bloomnation.com/" target="_blank" rel="noopener noreferrer" class="underline hover:text-gray-300">"BloomNation"</a>
                            ", which would go on to raise tens of millions."
                        </p>
                    </div>
                </div>
            </div>
        </section>
    }
}

#[component]
fn CourseOverviewSection() -> impl IntoView {
    view! {
        <section class="pt-16 pb-20 bg-gray-50 dark:bg-gray-900">
            <div class="max-w-6xl mx-auto px-6">
                <h2 class="text-4xl font-bold text-center text-gray-900 dark:text-white mb-12">
                    "Course Details"
                </h2>

                <div class="grid grid-cols-1 lg:grid-cols-2 gap-12">
                    <div>
                        <h3 class="text-2xl font-bold text-gray-900 dark:text-white mb-6">"Quick Facts"</h3>
                        <div class="space-y-4 bg-white dark:bg-gray-700 rounded-lg p-6 shadow-md">
                            <CourseDetail label="Course Number" value="STAT 198" />
                            <CourseDetail label="Units" value="2 Units" />
                            <CourseDetail label="Meeting Time" value="TBD" />
                            <CourseDetail label="Location" value="Birge 50" />
                            <CourseDetail label="Prerequisites" value="None (Stat 20/21/88 recommended)" />
                            <CourseDetail label="Faculty Sponsor" value="Everett Wetchler" />
                        </div>
                    </div>

                    <div>
                        <h3 class="text-2xl font-bold text-gray-900 dark:text-white mb-6">"Learning Outcomes"</h3>
                        <div class="space-y-4">
                            <LearningObjective
                                text="Demonstrate knowledge of foundational and advanced poker concepts"
                            />
                            <LearningObjective
                                text="Apply poker strategy to play a fundamentally sound game"
                            />
                            <LearningObjective
                                text="Think critically using structured logic and extend reasoning to other contexts"
                            />
                            <LearningObjective
                                text="Develop understanding of game-theory-optimal (GTO) play and exploitative strategies"
                            />
                        </div>
                    </div>
                </div>
            </div>
        </section>
    }
}

#[component]
fn CourseStructureSection() -> impl IntoView {
    view! {
        <section class="py-20 bg-white dark:bg-gray-800">
            <div class="max-w-6xl mx-auto px-6">
                <h2 class="text-4xl font-bold text-center text-gray-900 dark:text-white mb-12">
                    "Course Structure"
                </h2>

                <div class="max-w-4xl mx-auto space-y-8">
                    <div class="bg-blue-50 dark:bg-blue-900 rounded-lg p-8 border-l-4 border-blue-600">
                        <h3 class="text-2xl font-bold text-gray-900 dark:text-white mb-4">"Class Format"</h3>
                        <p class="text-lg text-gray-700 dark:text-gray-300 mb-4">
                            "This course meets twice a week, with each class lasting 2 hours and divided into two parts:"
                        </p>
                        <ul class="space-y-3">
                            <li class="flex items-start space-x-3">
                                <div class="w-2 h-2 bg-blue-600 dark:bg-blue-400 rounded-full mt-2 flex-shrink-0"></div>
                                <span class="text-gray-700 dark:text-gray-300">"60 minutes of lecture covering poker theory and strategic decision-making"</span>
                            </li>
                            <li class="flex items-start space-x-3">
                                <div class="w-2 h-2 bg-blue-600 dark:bg-blue-400 rounded-full mt-2 flex-shrink-0"></div>
                                <span class="text-gray-700 dark:text-gray-300">"60 minutes of guided playing session using play money on PokerNow"</span>
                            </li>
                        </ul>
                    </div>

                    <div class="bg-yellow-50 dark:bg-yellow-900 rounded-lg p-8 border-l-4 border-yellow-600">
                        <h3 class="text-2xl font-bold text-gray-900 dark:text-white mb-4">"Leaderboard & IPA Team Spot"</h3>
                        <p class="text-lg text-gray-700 dark:text-gray-300">
                            "Player results from in-class sessions are tracked throughout the course, with a class leaderboard updated weekly based on PnL win rates. At the end of the semester, the top performer on the leaderboard will earn a guaranteed spot on Berkeley's Intercollegiate Poker Association (IPA) team for the following term."
                        </p>
                    </div>

                </div>
            </div>
        </section>
    }
}

#[component]
fn CourseScheduleSection() -> impl IntoView {
    view! {
        <section class="py-20 bg-gray-50 dark:bg-gray-900">
            <div class="max-w-7xl mx-auto px-6">
                <h2 class="text-4xl font-bold text-center text-gray-900 dark:text-white mb-4">
                    "Course Schedule"
                </h2>
                <p class="text-center text-gray-600 dark:text-gray-400 mb-12">"Subject to change"</p>

                <div class="bg-white dark:bg-gray-700 rounded-lg shadow-lg overflow-x-auto">
                    <table class="w-full">
                        <thead class="bg-blue-600 text-white">
                            <tr>
                                <th class="px-4 py-3 text-center font-semibold w-16">"Week"</th>
                                <th class="px-4 py-3 text-left font-semibold">"Topic/Lecture"</th>
                                <th class="px-4 py-3 text-left font-semibold w-48">"Reading"</th>
                                <th class="px-4 py-3 text-left font-semibold w-48">"Assignment"</th>
                                <th class="px-4 py-3 text-left font-semibold w-32">"Slides"</th>
                            </tr>
                        </thead>
                        <tbody class="divide-y divide-gray-200 dark:divide-gray-600">
                            <ScheduleRow
                                week="1"
                                topics=vec!["About Us", "TBD - Infosession", "TBD - Applications Due"]
                                reading="N/A"
                                assignment="N/A"
                                slides=vec![("About Us", "public/lecture_slides/AboutUs_SP26-Stat198.pdf")]
                            />
                            <ScheduleRow
                                week="2"
                                topics=vec!["TBD - Course Structure: Rules of Play, Expected Value, and Variance", "TBD - Introduction to Game Theory Optimal (GTO) Play and Hand Ranges"]
                                reading="Play Optimal Poker (Brokos), Ch. 1–2"
                                assignment="Homework 1: Rules of Poker"
                                slides=vec![("Lecture 1", "public/lecture_slides/Lecture1_SP26-Stat198.pdf")]
                            />
                            <ScheduleRow
                                week="3"
                                topics=vec!["TBD - Preflop Fundamentals: Open Raising, Big Blind Defense, and Constructing Ranges", "TBD - Preflop Strategy II: Combinatorics and Relative vs. Absolute Hand Strength"]
                                reading="Play Optimal Poker (Brokos), Ch. 3–4"
                                assignment="Homework 2: Open-Raising"
                                slides=vec![("Lecture 2", "public/lecture_slides/Lecture2_SP26-Stat198.pdf")]
                            />
                            <ScheduleRow
                                week="4"
                                topics=vec!["TBD - Advanced Preflop: Pot Odds, Equity Realization, Combo/Draw Math, Isolation Plays", "TBD - Advanced Preflop II: 3-Betting, 4-Betting, Flatting, Squeezing, and Exploitative Adjustments"]
                                reading="Play Optimal Poker (Brokos), Ch. 5–6"
                                assignment="Homework 3: Advanced Preflop"
                                slides=vec![("Lecture 3", "public/lecture_slides/Lecture3_SP26-Stat198.pdf")]
                            />
                            <ScheduleRow
                                week="5"
                                topics=vec!["TBD - Flop Play I: Made Hands vs. Draws, Board Texture, and Calculating Equity", "TBD - Flop Play II: Continuation Betting (IP vs. OOP), Range Advantage, and Sizing Strategies"]
                                reading="Play Optimal Poker (Brokos), Ch. 7–8"
                                assignment="Homework 4: Pot Odds & Draw Calculations"
                            />
                            <ScheduleRow
                                week="6"
                                topics=vec!["TBD - Betting the Flop: Continuation Bets, Bet Sizing, Pot Control, and Raising Dynamics", "TBD - Flop Defense: Non-Aggressor Plays, Hero vs. Opener Scenarios"]
                                reading="Play Optimal Poker 2 (Brokos), Ch. 1–3"
                                assignment="Homework 5: C-Betting & Sizing"
                            />
                            <ScheduleRow
                                week="7"
                                topics=vec!["TBD - Turn Play I: Delayed Continuation Bets, Probing the Turn, and Range Elasticity", "TBD - Turn Play II: Thin vs. Thick Value Betting, Range Adjustments, and Advanced Elasticity Concepts"]
                                reading="Play Optimal Poker 2 (Brokos), Ch. 4–5"
                                assignment="Homework 6: The Turn"
                            />
                            <ScheduleRow
                                week="8"
                                topics=vec!["TBD - River Play I: Minimum Defense Frequency, Bluffing the River, and Bet Sizing Frameworks", "TBD - River Play II: Multi-Street Bluffing and Constructing Bluffing Frequencies"]
                                reading="Play Optimal Poker 2 (Brokos), Ch. 6–8"
                                assignment="Homework 7: The River"
                            />
                            <ScheduleRow
                                week="9"
                                topics=vec!["TBD - Hand Analysis I: Street-by-Street Decision Tracking and Adjusting Ranges", "TBD - Hand Analysis II: Deep Stack Play, Revisiting Flop/Turn Defense, and River Decision Making"]
                                reading="Play Optimal Poker 2 (Brokos), Ch. 9–11"
                                assignment="Homework 8: Hand Analysis"
                            />
                            <ScheduleRow
                                week="10"
                                topics=vec!["TBD - Rake Effects, Session Dynamics, and Special Betting Lines (Donk Bets, Unusual Lines)", "TBD - Common Turn/River Lines: Double Check-Raising, x-x Flop Dynamics, and River Give-Ups"]
                                reading="Modern Poker Theory (Acevedo), Ch. 14"
                                assignment="Homework 9: Common Lines"
                            />
                            <ScheduleRow
                                week="11"
                                topics=vec!["TBD - MTT Strategy", "TBD - MTT Strategy"]
                                reading="Modern Poker Theory (Acevedo), Ch. 6–9"
                                assignment="Homework 10: Exploits"
                            />
                            <ScheduleRow
                                week="12"
                                topics=vec!["TBD"]
                                reading="Reading Poker Tells (Elwood), Ch. 1–2"
                                assignment="Final Project: Checkpoint"
                            />
                            <ScheduleRow
                                week="13"
                                topics=vec!["TBD - Modern Applications: Solvers, Exploitative Adjustments, Live Reads, Tells, and Node-Locking", "TBD - Final Lecture: Course Wrap-Up, Reflection, and Poker Beyond the Table (Community & Culture)"]
                                reading="Reading Poker Tells (Elwood), Ch. 3"
                                assignment="Final Project Due: Hand History Analysis"
                            />
                        </tbody>
                    </table>
                </div>
            </div>
        </section>
    }
}

#[component]
fn ScheduleRow(
    week: &'static str,
    topics: Vec<&'static str>,
    reading: &'static str,
    assignment: &'static str,
    #[prop(optional)] slides: Option<Vec<(&'static str, &'static str)>>,
) -> impl IntoView {
    view! {
        <tr class="hover:bg-gray-50 dark:hover:bg-gray-600">
            <td class="px-4 py-4 text-center font-semibold text-gray-900 dark:text-white">{week}</td>
            <td class="px-4 py-4">
                <ul class="space-y-2">
                    {topics.into_iter().map(|topic| {
                        view! {
                            <li class="text-sm text-gray-700 dark:text-gray-300">{topic}</li>
                        }
                    }).collect::<Vec<_>>()}
                </ul>
            </td>
            <td class="px-4 py-4 text-sm text-gray-600 dark:text-gray-400">{reading}</td>
            <td class="px-4 py-4 text-sm text-gray-600 dark:text-gray-400">{assignment}</td>
            <td class="px-4 py-4 whitespace-nowrap">
                {slides.map(|slide_list| {
                    view! {
                        <ul class="space-y-2">
                            {slide_list.into_iter().map(|(label, url)| {
                                view! {
                                    <li>
                                        <a href=url target="_blank" class="text-sm text-blue-600 dark:text-blue-400 hover:text-blue-800 dark:hover:text-blue-300 underline">
                                            {label}
                                        </a>
                                    </li>
                                }
                            }).collect::<Vec<_>>()}
                        </ul>
                    }
                })}
            </td>
        </tr>
    }
}

#[component]
fn CourseDetail(label: &'static str, value: &'static str) -> impl IntoView {
    view! {
        <div class="flex justify-between items-center py-2 border-b border-gray-200 dark:border-gray-600">
            <span class="text-gray-600 dark:text-gray-400 font-medium">{label}</span>
            <span class="text-gray-900 dark:text-white font-semibold">{value}</span>
        </div>
    }
}

#[component]
fn LearningObjective(text: &'static str) -> impl IntoView {
    view! {
        <div class="flex items-start space-x-4">
            <div class="w-10 h-10 bg-blue-100 dark:bg-blue-900 rounded-lg flex items-center justify-center text-lg flex-shrink-0 text-gray-900 dark:text-white">
                "✓"
            </div>
            <p class="text-gray-700 dark:text-gray-300 pt-2">{text}</p>
        </div>
    }
}

#[component]
fn TopicItem(week: &'static str, topic: &'static str) -> impl IntoView {
    view! {
        <div class="flex items-start space-x-3 p-4 bg-white dark:bg-gray-700 rounded-lg">
            <div class="text-blue-600 dark:text-blue-400 font-bold text-sm flex-shrink-0 pt-1">
                {week}
            </div>
            <div class="text-gray-700 dark:text-gray-300 text-sm">
                {topic}
            </div>
        </div>
    }
}

#[component]
fn InstructorsSection() -> impl IntoView {
    view! {
        <section class="py-20 bg-gray-50 dark:bg-gray-900">
            <div class="max-w-6xl mx-auto px-6">
                <h2 class="text-4xl font-bold text-center text-gray-900 dark:text-white mb-12">
                    "Course Staff"
                </h2>

                <div class="grid grid-cols-2 sm:grid-cols-4 gap-6 max-w-6xl mx-auto">
                    <InstructorCard
                        name="Maysa Eleka Barandish"
                        role="Head of DeCal"
                        email="maysabarandish@berkeley.edu"
                        image="public/images/officers/maysa.webp"
                    />
                    <InstructorCard
                        name="Jones Arthur Dickerson"
                        role="Instructor"
                        email="jones.dickerson@berkeley.edu"
                        image="public/images/officers/jones.webp"
                    />
                    <InstructorCard
                        name="David Y. Chen"
                        role="Instructor"
                        email="ipo@berkeley.edu"
                        image="public/images/officers/david.webp"
                    />
                    <InstructorCard
                        name="Dawson Ryan Kern"
                        role="Instructor"
                        email="kerndr@berkeley.edu"
                        image="public/images/decal-staff/dawson.webp"
                    />
                    <InstructorCard
                        name="Mete Ehliz"
                        role="Instructor"
                        email="meteehliz@berkeley.edu"
                        image="public/images/decal-staff/mete.webp"
                    />
                    <InstructorCard
                        name="Fanou Zhang"
                        role="TA"
                        email="fanou_zhang@berkeley.edu"
                        image="public/images/decal-staff/fanou.webp"
                    />
                    <InstructorCard
                        name="Aidan Spain"
                        role="TA"
                        email="aidans13@berkeley.edu"
                        image="public/images/decal-staff/aidan.webp"
                    />
                    <InstructorCard
                        name="Matthew Naidu"
                        role="TA"
                        email="matthewnaidu@berkeley.edu"
                        image="public/images/decal-staff/matthew.webp"
                    />
                </div>
                <p class="text-center text-gray-600 dark:text-gray-400 mt-8">
                    "Office Hours: By Appointment"
                </p>
            </div>
        </section>
    }
}

#[component]
fn InstructorCard(
    name: &'static str,
    role: &'static str,
    email: &'static str,
    image: &'static str,
) -> impl IntoView {
    let obfuscated_email = email.replace('.', " [dot] ");

    view! {
        <div class="border border-gray-800 rounded-lg overflow-hidden shadow-lg" style="background-color: #1a2540;">
            <div class="w-full h-72 md:h-80 lg:h-72 xl:h-80">
                <img src=image alt=name class="w-full h-72 md:h-80 lg:h-72 xl:h-80 object-cover" loading="lazy" />
            </div>
            <div class="px-4 py-5 text-center">
                <h3 class="text-base font-bold text-white mb-1">{name}</h3>
                <p class="text-sm font-semibold mb-2" style="color: #F5C842;">{role}</p>
                <p class="text-xs text-gray-400">{obfuscated_email}</p>
            </div>
        </div>
    }
}

#[component]
fn GradingSection() -> impl IntoView {
    view! {
        <section class="py-20 bg-white dark:bg-gray-800">
            <div class="max-w-6xl mx-auto px-6">
                <h2 class="text-4xl font-bold text-center text-gray-900 dark:text-white mb-12">
                    "Grading & Requirements"
                </h2>

                <div class="max-w-4xl mx-auto">
                    <div class="bg-blue-50 dark:bg-blue-900 rounded-lg p-8 mb-8 border-l-4 border-blue-600">
                        <h3 class="text-2xl font-bold text-gray-900 dark:text-white mb-4">"Pass Requirement"</h3>
                        <p class="text-lg text-gray-700 dark:text-gray-300">
                            "To receive a 'P' (Pass) for this course, students must complete the Final Project and achieve an overall score of 70% or higher."
                        </p>
                    </div>

                    <div class="grid grid-cols-1 md:grid-cols-2 gap-6 mb-8">
                        <GradingItem
                            category="Attendance"
                            percentage="25%"
                            description="Tracked via Attendance Deck QR codes. 3% deduction per absence after 3 unexcused absences."
                        />
                        <GradingItem
                            category="Playing Sessions"
                            percentage="15%"
                            description="Participation tracked through PokerNow platform. 0.5% deduction per missed session beyond 75% attendance."
                        />
                        <GradingItem
                            category="Homework"
                            percentage="25%"
                            description="Weekly assignments (~1 hour). Graded on effort and completion. Late submissions accepted for 50% credit."
                        />
                        <GradingItem
                            category="Final Project"
                            percentage="35%"
                            description="Hand history analysis from class sessions. Graded on effort and thoroughness."
                        />
                    </div>

                    <div class="bg-gray-50 dark:bg-gray-700 rounded-lg p-6">
                        <h3 class="text-xl font-bold text-gray-900 dark:text-white mb-3">"Important Note"</h3>
                        <p class="text-gray-700 dark:text-gray-300">
                            "This course uses play money only—no real money is wagered at any time. The focus is on probability, statistics, and decision-making, not gambling."
                        </p>
                    </div>
                </div>
            </div>
        </section>
    }
}

#[component]
fn GradingItem(
    category: &'static str,
    percentage: &'static str,
    description: &'static str,
) -> impl IntoView {
    view! {
        <div class="bg-white dark:bg-gray-700 rounded-lg shadow-md p-6 border-t-4 border-blue-600">
            <div class="flex justify-between items-center mb-3">
                <h4 class="text-lg font-bold text-gray-900 dark:text-white">{category}</h4>
                <span class="text-2xl font-bold text-blue-600">{percentage}</span>
            </div>
            <p class="text-sm text-gray-600 dark:text-gray-400">{description}</p>
        </div>
    }
}

#[component]
fn ApplySection(title: &'static str) -> impl IntoView {
    view! {
        <section class="py-16 bg-slate-900">
            <div class="max-w-6xl mx-auto px-6">
                <div class="flex flex-col md:flex-row md:items-center md:justify-between gap-6 text-center md:text-left">
                    <h2 class="text-4xl font-bold text-white">
                        {title}
                    </h2>
                    <div class="flex flex-col sm:flex-row gap-4 sm:gap-6 justify-center md:justify-end">
                        <a
                            href=DECAL_APPLICATION_FORM
                            target="_blank"
                            rel="noopener"
                            class="bg-white text-slate-900 font-semibold py-3 px-6 rounded-lg shadow-md ring-2 ring-white/40 hover:bg-slate-50 hover:shadow-lg transition-all"
                        >
                            "Apply"
                        </a>
                        <a
                            href=DECAL_TA_APPLICATION_FORM
                            class="bg-[#9A6A4C] text-white font-semibold py-3 px-6 rounded-lg shadow-md hover:bg-[#B2A08E] hover:shadow-lg transition-all"
                        >
                            "TA Application"
                        </a>
                        <a
                            href=DECAL_SYLLABUS
                            class="bg-slate-800 text-white font-semibold py-3 px-6 rounded-lg shadow-md border border-white/30 hover:bg-slate-700 hover:shadow-lg transition-all"
                        >
                            "Syllabus"
                        </a>
                    </div>
                </div>
            </div>
        </section>
    }
}

#[component]
fn DecalApplyBar() -> impl IntoView {
    view! {
        <section class="pt-16 pb-6 bg-slate-900">
            <div class="max-w-4xl mx-auto px-6 text-center">
                <div class="flex flex-col sm:flex-row gap-4 sm:gap-6 justify-center">
                    <a
                        href=DECAL_APPLICATION_FORM
                        target="_blank"
                        rel="noopener"
                        class="bg-white text-slate-900 font-semibold py-3 px-6 rounded-lg shadow-md ring-2 ring-white/40 hover:bg-slate-50 hover:shadow-lg transition-all"
                    >
                        "Apply"
                    </a>
                    <a
                        href=DECAL_TA_APPLICATION_FORM
                        class="bg-[#9A6A4C] text-white font-semibold py-3 px-6 rounded-lg shadow-md hover:bg-[#B2A08E] hover:shadow-lg transition-all"
                    >
                        "TA Application"
                    </a>
                    <a
                        href=DECAL_SYLLABUS
                        class="bg-slate-800 text-white font-semibold py-3 px-6 rounded-lg shadow-md border border-white/30 hover:bg-slate-700 hover:shadow-lg transition-all"
                    >
                        "Syllabus"
                    </a>
                </div>
                <p class="text-sm text-white/80 mt-6">
                    "Applications Due: TBD"
                </p>
                <p class="text-sm text-white/80">
                    "2 Units | Section Times: TBD"
                </p>
            </div>
        </section>
    }
}
