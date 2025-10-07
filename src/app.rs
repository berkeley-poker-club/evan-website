use crate::components::*;
use crate::pages::*;
use leptos::prelude::*;
use leptos_router::{
    components::{Route, Router, Routes},
    path,
};

#[component]
pub fn App() -> impl IntoView {
    view! {
        <Router>
            <div class="min-h-screen bg-gray-50">
                <Navigation />
                <main>
                    <Routes fallback=|| view! {
                        <div class="min-h-screen flex items-center justify-center">
                            <div class="text-center">
                                <h1 class="text-4xl font-bold text-gray-900 mb-4">"Page not found"</h1>
                                <p class="text-gray-600 mb-6">"The page you're looking for doesn't exist."</p>
                                <a href="/" class="bg-blue-600 hover:bg-blue-700 text-white font-semibold py-2 px-6 rounded-lg transition-colors">
                                    "Go Home"
                                </a>
                            </div>
                        </div>
                    }>
                        <Route path=path!("/") view=HomePage/>
                        <Route path=path!("/sponsors") view=SponsorsPage/>
                        <Route path=path!("/people") view=PeoplePage/>
                        <Route path=path!("/tournaments") view=TournamentsPage/>
                        <Route path=path!("/decal") view=DecalPage/>
                        <Route path=path!("/resources") view=ResourcesPage/>
                    </Routes>
                </main>
                <Footer />
            </div>
        </Router>
    }
}
