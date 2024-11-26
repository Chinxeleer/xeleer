use leptos::*;
use leptos_meta::*;
use leptos_router::Outlet;

use crate::home::{home_footer::HomeFooter, home_nav::Nav};

/// Renders the home page of your application.

#[component]
pub fn HomePage() -> impl IntoView {
    view! {
        <Title text="Chinxeleer" />
        <Meta
            name="description"
            content="Explore the journey of a passionate Computer Science and Applied Mathematics student at Wits University, specializing in Rust and web development. Discover innovative projects, blogs on tech, Linux insights, and tools for modern development."
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
