use leptos::prelude::*;

#[component]
pub fn ContactPage() -> impl IntoView {
    view! {
        <div class="min-h-screen">
            <HeroBanner />
            <ContactSection />
        </div>
    }
}

#[component]
fn HeroBanner() -> impl IntoView {
    view! {
        <section id="banner" class="py-32 bg-gradient-to-r from-gray-900 to-blue-900">
            <div class="max-w-4xl mx-auto text-center px-6">
                <h1 class="text-5xl md:text-6xl font-bold text-white mb-4">
                    "Contact Us"
                </h1>
                <p class="text-xl text-white/90">
                    "Get in touch with Poker at Berkeley"
                </p>
            </div>
        </section>
    }
}

#[component]
fn ContactSection() -> impl IntoView {
    view! {
        <section class="py-20 bg-white">
            <div class="max-w-6xl mx-auto px-6">
                <h2 class="text-4xl font-bold text-center text-gray-900 mb-12">
                    "Get In Touch"
                </h2>

                <div class="grid grid-cols-1 lg:grid-cols-2 gap-12">
                    <div>
                        <h3 class="text-2xl font-bold text-gray-900 mb-6">"Contact Information"</h3>

                        <div class="space-y-6">
                            <ContactItem
                                icon="@"
                                title="Email"
                                detail="poker@berkeley.edu"
                                link="mailto:poker@berkeley.edu"
                            />
                            <ContactItem
                                icon="IG"
                                title="Instagram"
                                detail="@pokeratberkeley"
                                link="https://instagram.com/pokeratberkeley"
                            />
                            <ContactItem
                                icon="LI"
                                title="LinkedIn"
                                detail="Poker at Berkeley"
                                link="https://www.linkedin.com/company/poker-at-berkeley"
                            />
                            <div class="p-4 bg-gray-100 rounded-lg">
                                <div class="flex items-center space-x-3">
                                    <div class="w-12 h-12 bg-gray-400 rounded-full flex items-center justify-center">
                                        <span class="text-white font-semibold">DC</span>
                                    </div>
                                    <div>
                                        <h4 class="font-semibold text-gray-700">Discord</h4>
                                        <p class="text-gray-500 text-sm">Coming Soon</p>
                                    </div>
                                </div>
                            </div>
                        </div>
                    </div>

                    <div>
                        <h3 class="text-2xl font-bold text-gray-900 mb-6">"Send Us a Message"</h3>
                        <ContactForm />
                    </div>
                </div>
            </div>
        </section>
    }
}

#[component]
fn ContactItem(
    icon: &'static str,
    title: &'static str,
    detail: &'static str,
    link: &'static str,
) -> impl IntoView {
    view! {
        <div class="flex items-start space-x-4">
            <div class="w-12 h-12 bg-blue-100 rounded-lg flex items-center justify-center text-xl flex-shrink-0">
                {icon}
            </div>
            <div>
                <h4 class="font-semibold text-gray-900 mb-1">{title}</h4>
                {if link.is_empty() {
                    view! {
                        <p class="text-gray-600">{detail}</p>
                    }.into_any()
                } else {
                    view! {
                        <a href=link class="text-blue-600 hover:text-blue-800 transition-colors">
                            {detail}
                        </a>
                    }.into_any()
                }}
            </div>
        </div>
    }
}

#[component]
fn ContactForm() -> impl IntoView {
    view! {
        <form class="space-y-6">
            <div>
                <label class="block text-sm font-medium text-gray-700 mb-2">"Name"</label>
                <input
                    type="text"
                    class="w-full px-4 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-blue-500 focus:border-transparent"
                    placeholder="Your Name"
                />
            </div>

            <div>
                <label class="block text-sm font-medium text-gray-700 mb-2">"Email"</label>
                <input
                    type="email"
                    class="w-full px-4 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-blue-500 focus:border-transparent"
                    placeholder="your.email@berkeley.edu"
                />
            </div>

            <div>
                <label class="block text-sm font-medium text-gray-700 mb-2">"Subject"</label>
                <select class="w-full px-4 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-blue-500 focus:border-transparent">
                    <option>"General Inquiry"</option>
                    <option>"Join the Club"</option>
                    <option>"DeCal Application"</option>
                    <option>"Tournament Information"</option>
                    <option>"Sponsorship"</option>
                    <option>"Other"</option>
                </select>
            </div>

            <div>
                <label class="block text-sm font-medium text-gray-700 mb-2">"Message"</label>
                <textarea
                    rows=5
                    class="w-full px-4 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-blue-500 focus:border-transparent"
                    placeholder="Your message..."
                ></textarea>
            </div>

            <button type="submit" class="w-full btn-primary">
                "Send Message"
            </button>
        </form>
    }
}

#[component]
fn FAQSection() -> impl IntoView {
    view! {
        <section class="py-20 bg-white">
            <div class="max-w-4xl mx-auto px-6">
                <h2 class="text-4xl font-bold text-center text-gray-900 mb-12">
                    "Frequently Asked Questions"
                </h2>

                <div class="space-y-6">
                    <FAQItem
                        question="How do I join Poker at Berkeley?"
                        answer="Follow us on Instagram @pokeratberkeley and join our Discord server. Applications typically open at the beginning of each semester."
                    />
                    <FAQItem
                        question="Do I need poker experience to join?"
                        answer="No! We welcome players of all skill levels. We have resources and mentorship programs specifically for beginners."
                    />
                    <FAQItem
                        question="Is real money involved?"
                        answer="No, all our activities use play money or tournament chips. We focus on strategy, learning, and community building."
                    />
                    <FAQItem
                        question="How do I apply for the DeCal?"
                        answer="DeCal applications are separate from club membership and open before each semester. Check our website and social media for application deadlines."
                    />
                    <FAQItem
                        question="What are the membership fees?"
                        answer="Membership fees vary by semester and are used to fund events, tournaments, and club activities. Current fees are announced with applications."
                    />
                    <FAQItem
                        question="Can I attend events as a non-member?"
                        answer="Some events are open to the public, but regular game nights and tournaments are typically for members only. Follow our social media for public event announcements."
                    />
                </div>
            </div>
        </section>
    }
}

#[component]
fn FAQItem(question: &'static str, answer: &'static str) -> impl IntoView {
    view! {
        <div class="bg-gray-50 rounded-lg p-6">
            <h3 class="text-lg font-bold text-gray-900 mb-3">{question}</h3>
            <p class="text-gray-700">{answer}</p>
        </div>
    }
}
