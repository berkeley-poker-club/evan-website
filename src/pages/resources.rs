use leptos::prelude::*;

#[component]
pub fn ResourcesPage() -> impl IntoView {
    view! {
        <div class="min-h-screen">
            <HeroBanner />
            <ResourcesSection />
        </div>
    }
}

#[component]
fn HeroBanner() -> impl IntoView {
    view! {
        <section id="banner" class="py-32 bg-gradient-to-r from-gray-900 to-gray-800">
            <div class="max-w-4xl mx-auto text-center px-6">
                <h1 class="text-5xl md:text-6xl font-bold text-white mb-4">
                    "Resources"
                </h1>
                <p class="text-xl text-white/90">
                    "Tools, books, and content to help build your poker game"
                </p>
            </div>
        </section>
    }
}

#[component]
fn ResourcesSection() -> impl IntoView {
    view! {
        <section class="py-20 bg-white">
            <div class="max-w-6xl mx-auto px-6">
                <div class="grid grid-cols-1 lg:grid-cols-2 gap-12">
                    <ResourceCategory
                        title="Books"
                        description="Recommended texts from the Poker Theory and Fundamentals DeCal:"
                        items=vec![
                            ("The Theory of Poker", "David Sklansky", ""),
                            ("The Mathematics of Poker", "Bill Chen & Jerrod Ankenman", ""),
                            ("Modern Poker Theory", "Michael Acevedo", ""),
                            ("Verbal Poker Tells & Exploiting Poker Tells", "Zachary Elwood", ""),
                        ]
                    />

                    <ResourceCategory
                        title="Range Equity & Solvers"
                        description="Essential tools for studying poker strategy:"
                        items=vec![
                            ("Open Poker Tools", "Equity Calculator", "https://openpokertools.com/equity/"),
                            ("GTO Wizard", "Industry-leading solver tools (free preflop charts available)", "https://gtowizard.com"),
                        ]
                    />

                    <ResourceCategory
                        title="Discussion Forums"
                        description="Communities for hand reviews and strategy discussion:"
                        items=vec![
                            ("Two Plus Two", "Largest poker strategy community", "http://forumserver.twoplustwo.com"),
                            ("r/poker on Reddit", "Hand reviews & community feedback", "https://www.reddit.com/r/poker/"),
                        ]
                    />

                    <ResourceCategory
                        title="YouTube Channels"
                        description="Video content for learning poker strategy:"
                        items=vec![
                            ("Savant Poker Coaching", "Strategic poker instruction", "https://www.youtube.com/@savantpoker"),
                            ("Hungry Horse Poker", "In-depth poker analysis", "https://www.youtube.com/@hungryhorsepoker"),
                        ]
                    />

                    <ResourceCategory
                        title="Podcasts & Blogs"
                        description="Audio and written content for poker improvement:"
                        items=vec![
                            ("Red Chip Poker Podcast", "Free, concise poker coaching", "http://redchippoker.com/podcast"),
                            ("Upswing Poker Blog", "Technical articles and chart analysis", "http://upswingpoker.com/blog/"),
                        ]
                    />

                    <ResourceCategory
                        title="Past DeCal Iterations"
                        description="Course materials from previous semesters:"
                        items=vec![
                            ("Spring 2022 DeCal", "Archived course website", "https://stat198-poker.github.io/"),
                        ]
                    />
                </div>
            </div>
        </section>
    }
}

#[component]
fn ResourceCategory(
    title: &'static str,
    description: &'static str,
    items: Vec<(&'static str, &'static str, &'static str)>,
) -> impl IntoView {
    view! {
        <div class="bg-gray-50 rounded-lg p-8 shadow-md">
            <h3 class="text-2xl font-bold text-gray-900 mb-3">{title}</h3>
            <p class="text-gray-600 mb-6">{description}</p>
            <ul class="space-y-4">
                {items.into_iter().map(|(name, desc, url)| {
                    view! {
                        <li class="border-l-2 border-blue-600 pl-4">
                            {if url.is_empty() {
                                view! {
                                    <div>
                                        <p class="font-semibold text-gray-900">{name}</p>
                                        <p class="text-sm text-gray-600">{desc}</p>
                                    </div>
                                }.into_any()
                            } else {
                                view! {
                                    <div>
                                        <a
                                            href=url
                                            target="_blank"
                                            class="font-semibold text-blue-600 hover:text-blue-800 transition-colors"
                                        >
                                            {name}
                                        </a>
                                        <p class="text-sm text-gray-600">{desc}</p>
                                    </div>
                                }.into_any()
                            }}
                        </li>
                    }
                }).collect::<Vec<_>>()}
            </ul>
        </div>
    }
}
