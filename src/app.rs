use crate::components::*;
use crate::pages::*;
use leptos::prelude::*;
use leptos_router::{
    components::{Route, Router, Routes},
    hooks::use_location,
    path,
};

#[component]
pub fn App() -> impl IntoView {
    view! {
        <ThemeProvider>
            <CustomCursor />
            <Router>
                <AppShell />
            </Router>
        </ThemeProvider>
    }
}

#[component]
fn AppShell() -> impl IntoView {
    let location = use_location();
    let is_pokerbots = move || location.pathname.get().starts_with("/pokerbots");

    view! {
        <div class=move || if is_pokerbots() { "bg-[#02070d]" } else { "min-h-screen bg-gray-50 dark:bg-gray-900" }>
            {move || (!is_pokerbots()).then(|| view! { <Navigation /> })}
                    <main>
                        <Routes fallback=|| view! {
                            <div class="min-h-screen flex items-center justify-center">
                                <div class="text-center">
                                    <h1 class="text-4xl font-bold text-gray-900 dark:text-white mb-4">"Page not found"</h1>
                                    <p class="text-gray-600 dark:text-gray-400 mb-6">"The page you're looking for doesn't exist."</p>
                                    <a href="/" class="bg-blue-600 hover:bg-blue-700 text-white font-semibold py-2 px-6 rounded-lg transition-colors">
                                        "Go Home"
                                    </a>
                                </div>
                            </div>
                        }>
                            <Route path=path!("") view=HomePage/>
                            <Route path=path!("join") view=JoinUsPage/>
                            <Route path=path!("sponsors") view=SponsorsPage/>
                            <Route path=path!("people") view=PeoplePage/>
                            <Route path=path!("tournaments") view=TournamentsPage/>
                            <Route path=path!("decal") view=DecalPage/>
                            <Route path=path!("resources") view=ResourcesPage/>
                            <Route path=path!("pokerbots") view=PokerBotsPage/>
                        </Routes>
                    </main>
            {move || (!is_pokerbots()).then(|| view! { <Footer /> })}
        </div>
    }
}
