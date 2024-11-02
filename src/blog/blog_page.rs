use leptos::*;
use leptos_router::*;

#[component]
pub fn BlogPage() -> impl IntoView {
    view! {
        <main class="flex flex-col justify-center align-center space-y-2 md:px-20">
            <Outlet />
        </main>
    }
}
