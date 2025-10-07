use leptos::prelude::*;

#[component]
pub fn Navigation() -> impl IntoView {
    let (is_open, set_is_open) = signal(false);

    view! {
        <nav class="sticky top-0 z-50 bg-white shadow-md">
            <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
                <div class="flex items-center justify-between h-16">
                    <div class="flex items-center space-x-4">
                        <a href="/" class="flex items-center space-x-2 text-gray-900 hover:text-blue-600 transition-colors">
                            <img src="public/images/blue.png" alt="Poker at Berkeley Logo" class="h-8 w-8" />
                            <span class="font-bold text-xl">"Poker at Berkeley"</span>
                        </a>
                    </div>

                    <div class="hidden md:flex items-center space-x-8">
                        <a href="/" class="text-gray-700 hover:text-blue-600 font-medium transition-colors">"Home"</a>
                        <a href="/people" class="text-gray-700 hover:text-blue-600 font-medium transition-colors">"Who We Are"</a>
                        <a href="/sponsors" class="text-gray-700 hover:text-blue-600 font-medium transition-colors">"Sponsors"</a>
                        <a href="/tournaments" class="text-gray-700 hover:text-blue-600 font-medium transition-colors">"Tournaments"</a>
                        <a href="/decal" class="text-gray-700 hover:text-blue-600 font-medium transition-colors">"DeCal"</a>
                        <a href="/resources" class="text-gray-700 hover:text-blue-600 font-medium transition-colors">"Resources"</a>
                    </div>

                    <div class="md:hidden">
                        <button
                            class="text-gray-700 hover:text-blue-600 transition-colors"
                            on:click=move |_| set_is_open.update(|open| *open = !*open)
                        >
                            <svg class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 6h16M4 12h16M4 18h16"></path>
                            </svg>
                        </button>
                    </div>
                </div>

                <div class=move || if is_open.get() { "md:hidden" } else { "hidden" }>
                    <div class="px-2 pt-2 pb-3 space-y-1 bg-white border-t border-gray-200">
                        <a href="/evan-website" class="block px-3 py-2 text-gray-700 hover:text-blue-600 transition-colors">"Home"</a>
                        <a href="/people" class="block px-3 py-2 text-gray-700 hover:text-blue-600 transition-colors">"Who We Are"</a>
                        <a href="/sponsors" class="block px-3 py-2 text-gray-700 hover:text-blue-600 transition-colors">"Sponsors"</a>
                        <a href="/tournaments" class="block px-3 py-2 text-gray-700 hover:text-blue-600 transition-colors">"Tournaments"</a>
                        <a href="/decal" class="block px-3 py-2 text-gray-700 hover:text-blue-600 transition-colors">"DeCal"</a>
                        <a href="/resources" class="block px-3 py-2 text-gray-700 hover:text-blue-600 transition-colors">"Resources"</a>
                        <div class="px-3 py-2">
                            <a href="/contact" class="block bg-blue-600 hover:bg-blue-700 text-white font-semibold py-2 px-4 rounded-lg transition-colors text-center">"Contact"</a>
                        </div>
                    </div>
                </div>
            </div>
        </nav>
    }
}
