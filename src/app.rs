use leptos::prelude::*;
use leptos_router::{components::{Router, Routes, Route}, path};
use crate::components::*;
use crate::pages::*;

#[component]
pub fn App() -> impl IntoView {
    view! {
        <Router>
            <div class="min-h-screen bg-gray-50">
                <Navigation />
                <main>
                    <Routes fallback=|| view! { <div>"Page not found."</div> }>
                        <Route path=path!("/") view=HomePage/>
                        <Route path=path!("/sponsors") view=SponsorsPage/>
                        <Route path=path!("/people") view=PeoplePage/>
                        <Route path=path!("/tournaments") view=TournamentsPage/>
                        <Route path=path!("/decal") view=DecalPage/>
                        <Route path=path!("/resources") view=ResourcesPage/>
                        <Route path=path!("/contact") view=ContactPage/>
                    </Routes>
                </main>
                <Footer />
            </div>
        </Router>
    }
}
