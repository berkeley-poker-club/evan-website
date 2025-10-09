use leptos::prelude::*;
use leptos_router::components::A;

#[component]
pub fn Navigation() -> impl IntoView {
    let (is_open, set_is_open) = signal(false);

    view! {
        <nav class="sticky top-0 z-50 bg-white shadow-md">
            <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
                <div class="flex items-center justify-between h-16">
                    <div class="flex items-center space-x-4">
                        <A href="/" attr:class="flex items-center space-x-2 text-gray-900 hover:text-blue-600 transition-colors">
                            <img src="public/images/blue.png" alt="Poker at Berkeley Logo" class="h-8 w-8" />
                            <span class="font-bold text-xl">"Poker at Berkeley"</span>
                        </A>
                    </div>

                    <div class="hidden md:flex items-center space-x-8">
                        <A href="/" attr:class="text-gray-700 hover:text-blue-600 font-medium transition-colors">"Home"</A>
                        <A href="/people" attr:class="text-gray-700 hover:text-blue-600 font-medium transition-colors">"Who We Are"</A>
                        <A href="/sponsors" attr:class="text-gray-700 hover:text-blue-600 font-medium transition-colors">"Sponsors"</A>
                        <A href="/tournaments" attr:class="text-gray-700 hover:text-blue-600 font-medium transition-colors">"Tournaments"</A>
                        <A href="/decal" attr:class="text-gray-700 hover:text-blue-600 font-medium transition-colors">"DeCal"</A>
                        <A href="/resources" attr:class="text-gray-700 hover:text-blue-600 font-medium transition-colors">"Resources"</A>
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
                        <A href="/" attr:class="block px-3 py-2 text-gray-700 hover:text-blue-600 transition-colors">"Home"</A>
                        <A href="/people" attr:class="block px-3 py-2 text-gray-700 hover:text-blue-600 transition-colors">"Who We Are"</A>
                        <A href="/sponsors" attr:class="block px-3 py-2 text-gray-700 hover:text-blue-600 transition-colors">"Sponsors"</A>
                        <A href="/tournaments" attr:class="block px-3 py-2 text-gray-700 hover:text-blue-600 transition-colors">"Tournaments"</A>
                        <A href="/decal" attr:class="block px-3 py-2 text-gray-700 hover:text-blue-600 transition-colors">"DeCal"</A>
                        <A href="/resources" attr:class="block px-3 py-2 text-gray-700 hover:text-blue-600 transition-colors">"Resources"</A>
                    </div>
                </div>
            </div>
        </nav>
    }
}
