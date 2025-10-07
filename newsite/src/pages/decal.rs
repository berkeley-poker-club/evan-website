use leptos::prelude::*;

#[component]
pub fn DecalPage() -> impl IntoView {
    view! {
        <div class="min-h-screen">
            <HeroBanner />
            <CourseOverviewSection />
            <CourseStructureSection />
            <CourseScheduleSection />
            <InstructorsSection />
            <GradingSection />
            <ApplicationSection />
        </div>
    }
}

#[component]
fn HeroBanner() -> impl IntoView {
    view! {
        <section id="banner" class="py-32 bg-gradient-to-r from-gray-900 to-gray-800">
            <div class="max-w-4xl mx-auto text-center px-6">
                <h1 class="text-5xl md:text-6xl font-bold text-white mb-4">
                    "Poker DeCal"
                </h1>
                <h2 class="text-2xl md:text-3xl text-blue-200 mb-6">
                    "Stat 198: Poker Theory & Fundamentals | Fall 2025"
                </h2>
                <p class="text-xl text-white/90 mb-4">
                    "Lecture: Wed & Fri 4-6pm, Birge 50"
                </p>
                <p class="text-lg text-blue-300">
                    "2 Units | Faculty Sponsor: Everett Wetchler"
                </p>
            </div>
        </section>
    }
}

#[component]
fn CourseDescriptionSection() -> impl IntoView {
    view! {
        <section class="py-20 bg-white">
            <div class="max-w-4xl mx-auto px-6">
                <h2 class="text-4xl font-bold text-center text-gray-900 mb-8">
                    "Course Description"
                </h2>
                <div class="bg-gray-50 rounded-lg p-8">
                    <p class="text-lg text-gray-700 leading-relaxed mb-6">
                        "This course introduces the fundamentals of poker with an emphasis on strategic thinking and decision-making, open to students of all experience levels. While basic rules will be covered quickly, most of the course focuses on higher-level concepts in 6-max No-Limit Texas Hold'em—the most widely played form of poker today."
                    </p>
                    <p class="text-lg text-gray-700 leading-relaxed mb-6">
                        "Students will examine each stage of a hand and explore the mathematics, heuristics, and structured reasoning that strong players use, with some attention to psychological factors such as live reads and behavioral patterns. Beyond poker, these concepts develop analytical skills relevant to statistics, game theory, economics, finance, and investing."
                    </p>
                    <p class="text-lg text-gray-700 leading-relaxed">
                        "The DeCal was originally started in 2003 by UC Berkeley undergraduate David Daneshgar, who went on to win a WSOP bracelet in 2008."
                    </p>
                </div>
            </div>
        </section>
    }
}

#[component]
fn CourseOverviewSection() -> impl IntoView {
    view! {
        <section class="py-20 bg-gray-50">
            <div class="max-w-6xl mx-auto px-6">
                <h2 class="text-4xl font-bold text-center text-gray-900 mb-12">
                    "Course Details"
                </h2>

                <div class="grid grid-cols-1 lg:grid-cols-2 gap-12">
                    <div>
                        <h3 class="text-2xl font-bold text-gray-900 mb-6">"Quick Facts"</h3>
                        <div class="space-y-4 bg-white rounded-lg p-6 shadow-md">
                            <CourseDetail label="Course Number" value="STAT 198" />
                            <CourseDetail label="Units" value="2 Units" />
                            <CourseDetail label="Meeting Time" value="Wed & Fri 4-6pm" />
                            <CourseDetail label="Location" value="Birge 50" />
                            <CourseDetail label="Prerequisites" value="None (Stat 20/21/88 recommended)" />
                            <CourseDetail label="Faculty Sponsor" value="Everett Wetchler" />
                        </div>
                    </div>

                    <div>
                        <h3 class="text-2xl font-bold text-gray-900 mb-6">"Learning Outcomes"</h3>
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
        <section class="py-20 bg-white">
            <div class="max-w-6xl mx-auto px-6">
                <h2 class="text-4xl font-bold text-center text-gray-900 mb-12">
                    "Course Structure"
                </h2>

                <div class="max-w-4xl mx-auto space-y-8">
                    <div class="bg-blue-50 rounded-lg p-8 border-l-4 border-blue-600">
                        <h3 class="text-2xl font-bold text-gray-900 mb-4">"Class Format"</h3>
                        <p class="text-lg text-gray-700 mb-4">
                            "This course meets twice a week, with each class lasting 2 hours and divided into two parts:"
                        </p>
                        <ul class="space-y-3">
                            <li class="flex items-start space-x-3">
                                <div class="w-2 h-2 bg-blue-600 rounded-full mt-2 flex-shrink-0"></div>
                                <span class="text-gray-700">"60 minutes of lecture covering poker theory and strategic decision-making"</span>
                            </li>
                            <li class="flex items-start space-x-3">
                                <div class="w-2 h-2 bg-blue-600 rounded-full mt-2 flex-shrink-0"></div>
                                <span class="text-gray-700">"60 minutes of guided playing session using play money on PokerNow"</span>
                            </li>
                        </ul>
                    </div>

                    <div class="bg-yellow-50 rounded-lg p-8 border-l-4 border-yellow-600">
                        <h3 class="text-2xl font-bold text-gray-900 mb-4">"Leaderboard & IPA Team Spot"</h3>
                        <p class="text-lg text-gray-700">
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
        <section class="py-20 bg-gray-50">
            <div class="max-w-7xl mx-auto px-6">
                <h2 class="text-4xl font-bold text-center text-gray-900 mb-4">
                    "Course Schedule"
                </h2>
                <p class="text-center text-gray-600 mb-12">"Subject to change"</p>

                <div class="bg-white rounded-lg shadow-lg overflow-x-auto">
                    <table class="w-full">
                        <thead class="bg-blue-600 text-white">
                            <tr>
                                <th class="px-4 py-3 text-left font-semibold">"Week"</th>
                                <th class="px-4 py-3 text-left font-semibold">"Topic/Lecture"</th>
                                <th class="px-4 py-3 text-left font-semibold">"Reading"</th>
                                <th class="px-4 py-3 text-left font-semibold">"Assignment"</th>
                            </tr>
                        </thead>
                        <tbody class="divide-y divide-gray-200">
                            <ScheduleRow
                                week="1"
                                topics=vec!["Wed 9/10 - Infosession", "Thu 9/11 - Applications Due"]
                                reading="N/A"
                                assignment="N/A"
                            />
                            <ScheduleRow
                                week="2"
                                topics=vec!["Wed 9/17 - Course Structure: Rules of Play, Expected Value, and Variance", "Fri 9/19 - Introduction to Game Theory Optimal (GTO) Play and Hand Ranges"]
                                reading="Play Optimal Poker (Brokos), Ch. 1–2"
                                assignment="Homework 1: Rules of Poker"
                            />
                            <ScheduleRow
                                week="3"
                                topics=vec!["Wed 9/24 - Preflop Fundamentals: Open Raising, Big Blind Defense, and Constructing Ranges", "Fri 9/26 - Preflop Strategy II: Combinatorics and Relative vs. Absolute Hand Strength"]
                                reading="Play Optimal Poker (Brokos), Ch. 3–4"
                                assignment="Homework 2: Open-Raising"
                            />
                            <ScheduleRow
                                week="4"
                                topics=vec!["Wed 10/1 - Advanced Preflop: Pot Odds, Equity Realization, Combo/Draw Math, Isolation Plays", "Fri 10/3 - Advanced Preflop II: 3-Betting, 4-Betting, Flatting, Squeezing, and Exploitative Adjustments"]
                                reading="Play Optimal Poker (Brokos), Ch. 5–6"
                                assignment="Homework 3: Advanced Preflop"
                            />
                            <ScheduleRow
                                week="5"
                                topics=vec!["Wed 10/8 - Flop Play I: Made Hands vs. Draws, Board Texture, and Calculating Equity", "Fri 10/10 - Flop Play II: Continuation Betting (IP vs. OOP), Range Advantage, and Sizing Strategies"]
                                reading="Play Optimal Poker (Brokos), Ch. 7–8"
                                assignment="Homework 4: Pot Odds & Draw Calculations"
                            />
                            <ScheduleRow
                                week="6"
                                topics=vec!["Wed 10/15 - Betting the Flop: Continuation Bets, Bet Sizing, Pot Control, and Raising Dynamics", "Fri 10/17 - Flop Defense: Non-Aggressor Plays, Hero vs. Opener Scenarios"]
                                reading="Play Optimal Poker 2 (Brokos), Ch. 1–3"
                                assignment="Homework 5: C-Betting & Sizing"
                            />
                            <ScheduleRow
                                week="7"
                                topics=vec!["Wed 10/22 - Turn Play I: Delayed Continuation Bets, Probing the Turn, and Range Elasticity", "Fri 10/24 - Turn Play II: Thin vs. Thick Value Betting, Range Adjustments, and Advanced Elasticity Concepts"]
                                reading="Play Optimal Poker 2 (Brokos), Ch. 4–5"
                                assignment="Homework 6: The Turn"
                            />
                            <ScheduleRow
                                week="8"
                                topics=vec!["Wed 10/29 - River Play I: Minimum Defense Frequency, Bluffing the River, and Bet Sizing Frameworks", "Fri 10/31 - River Play II: Multi-Street Bluffing and Constructing Bluffing Frequencies"]
                                reading="Play Optimal Poker 2 (Brokos), Ch. 6–8"
                                assignment="Homework 7: The River"
                            />
                            <ScheduleRow
                                week="9"
                                topics=vec!["Wed 11/5 - Hand Analysis I: Street-by-Street Decision Tracking and Adjusting Ranges", "Fri 11/7 - Hand Analysis II: Deep Stack Play, Revisiting Flop/Turn Defense, and River Decision Making"]
                                reading="Play Optimal Poker 2 (Brokos), Ch. 9–11"
                                assignment="Homework 8: Hand Analysis"
                            />
                            <ScheduleRow
                                week="10"
                                topics=vec!["Wed 11/12 - Rake Effects, Session Dynamics, and Special Betting Lines (Donk Bets, Unusual Lines)", "Fri 11/14 - Common Turn/River Lines: Double Check-Raising, x-x Flop Dynamics, and River Give-Ups"]
                                reading="Modern Poker Theory (Acevedo), Ch. 14"
                                assignment="Homework 9: Common Lines"
                            />
                            <ScheduleRow
                                week="11"
                                topics=vec!["Wed 11/19 - MTT Strategy", "Fri 11/21 - MTT Strategy"]
                                reading="Modern Poker Theory (Acevedo), Ch. 6–9"
                                assignment="Homework 10: Exploits"
                            />
                            <ScheduleRow
                                week="12"
                                topics=vec!["Thanksgiving Break: 11/26 - 11/28"]
                                reading="Reading Poker Tells (Elwood), Ch. 1–2"
                                assignment="Final Project: Checkpoint"
                            />
                            <ScheduleRow
                                week="13"
                                topics=vec!["Wed 12/3 - Modern Applications: Solvers, Exploitative Adjustments, Live Reads, Tells, and Node-Locking", "Fri 12/5 - Final Lecture: Course Wrap-Up, Reflection, and Poker Beyond the Table (Community & Culture)"]
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
) -> impl IntoView {
    view! {
        <tr class="hover:bg-gray-50">
            <td class="px-4 py-4 font-semibold text-gray-900">{week}</td>
            <td class="px-4 py-4">
                <ul class="space-y-2">
                    {topics.into_iter().map(|topic| {
                        view! {
                            <li class="text-sm text-gray-700">{topic}</li>
                        }
                    }).collect::<Vec<_>>()}
                </ul>
            </td>
            <td class="px-4 py-4 text-sm text-gray-600">{reading}</td>
            <td class="px-4 py-4 text-sm text-gray-600">{assignment}</td>
        </tr>
    }
}

#[component]
fn CourseDetail(label: &'static str, value: &'static str) -> impl IntoView {
    view! {
        <div class="flex justify-between items-center py-2 border-b border-gray-200">
            <span class="text-gray-600 font-medium">{label}</span>
            <span class="text-gray-900 font-semibold">{value}</span>
        </div>
    }
}

#[component]
fn LearningObjective(text: &'static str) -> impl IntoView {
    view! {
        <div class="flex items-start space-x-4">
            <div class="w-10 h-10 bg-blue-100 rounded-lg flex items-center justify-center text-lg flex-shrink-0">
                "✓"
            </div>
            <p class="text-gray-700 pt-2">{text}</p>
        </div>
    }
}

#[component]
fn TopicItem(week: &'static str, topic: &'static str) -> impl IntoView {
    view! {
        <div class="flex items-start space-x-3 p-4 bg-white rounded-lg">
            <div class="text-blue-600 font-bold text-sm flex-shrink-0 pt-1">
                {week}
            </div>
            <div class="text-gray-700 text-sm">
                {topic}
            </div>
        </div>
    }
}

#[component]
fn InstructorsSection() -> impl IntoView {
    view! {
        <section class="py-20 bg-gray-50">
            <div class="max-w-6xl mx-auto px-6">
                <h2 class="text-4xl font-bold text-center text-gray-900 mb-12">
                    "Instructors"
                </h2>

                <div class="grid grid-cols-1 md:grid-cols-2 gap-8 max-w-4xl mx-auto">
                    <InstructorCard
                        name="Maysa Barandish"
                        role="Head of DeCal"
                        email="maysabarandish@berkeley.edu"
                        image="public/images/officers/maysa.png"
                    />
                    <InstructorCard
                        name="Jones Dickerson"
                        role="Instructor"
                        email="jones.dickerson@berkeley.edu"
                        image="public/images/decal-staff/jones.png"
                    />
                </div>
                <p class="text-center text-gray-600 mt-8">
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
    view! {
        <div class="bg-white rounded-lg shadow-lg overflow-hidden">
            <div class="w-full h-64">
                <img src=image alt=name class="w-full h-64 object-cover" />
            </div>
            <div class="p-6 text-center">
                <h3 class="text-xl font-bold text-gray-900 mb-2">{name}</h3>
                <p class="text-blue-600 font-semibold mb-3">{role}</p>
                <a href=format!("mailto:{}", email) class="text-sm text-gray-600 hover:text-blue-600 transition-colors">
                    {email}
                </a>
            </div>
        </div>
    }
}

#[component]
fn GradingSection() -> impl IntoView {
    view! {
        <section class="py-20 bg-white">
            <div class="max-w-6xl mx-auto px-6">
                <h2 class="text-4xl font-bold text-center text-gray-900 mb-12">
                    "Grading & Requirements"
                </h2>

                <div class="max-w-4xl mx-auto">
                    <div class="bg-blue-50 rounded-lg p-8 mb-8 border-l-4 border-blue-600">
                        <h3 class="text-2xl font-bold text-gray-900 mb-4">"Pass Requirement"</h3>
                        <p class="text-lg text-gray-700">
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

                    <div class="bg-gray-50 rounded-lg p-6">
                        <h3 class="text-xl font-bold text-gray-900 mb-3">"Important Note"</h3>
                        <p class="text-gray-700">
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
        <div class="bg-white rounded-lg shadow-md p-6 border-t-4 border-blue-600">
            <div class="flex justify-between items-center mb-3">
                <h4 class="text-lg font-bold text-gray-900">{category}</h4>
                <span class="text-2xl font-bold text-blue-600">{percentage}</span>
            </div>
            <p class="text-sm text-gray-600">{description}</p>
        </div>
    }
}

#[component]
fn ApplicationSection() -> impl IntoView {
    view! {
        <section class="py-20 bg-blue-900">
            <div class="max-w-4xl mx-auto text-center px-6">
                <h2 class="text-4xl font-bold text-white mb-6">
                    "Ready to Apply?"
                </h2>
                <div class="flex justify-center">
                    <button disabled class="bg-gray-300 text-gray-600 font-semibold py-3 px-8 rounded-lg cursor-not-allowed">
                        "Applications Closed"
                    </button>
                </div>
            </div>
        </section>
    }
}
