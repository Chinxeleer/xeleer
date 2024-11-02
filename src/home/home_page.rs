use leptos::*;
use leptos_meta::*;
use leptos_router::Outlet;

use crate::home::{home_footer::HomeFooter, home_nav::Nav};

/// Renders the home page of your application.

#[component]
pub fn HomePage() -> impl IntoView {
    view! {
        <Title text="Home"/>
        <Meta
            name="description"
            content="This is the landing page to Blessing's website"
        />
        <div class="flex flex-col min-h-screen overscroll-none">
            <div>
                <Nav />
            </div>
            <div class="flex-1 p-4">
                <Outlet />
            </div>

            <HomeFooter />
        </div>
    }
}
