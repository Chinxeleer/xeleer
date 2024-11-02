use crate::{
    error_template::{AppError, ErrorTemplate},
    routes::MainRoutes,
    server_functions::posts::get_posts,
};
use leptos::*;
use leptos_meta::*;
use leptos_router::*;

#[component]
pub fn App() -> impl IntoView {
    leptos_image::provide_image_context();
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();
    let posts = create_resource(|| (), |_| async move { get_posts().await });
    provide_context(posts);
    view! {


        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/xeleer.css"/>

        // sets the document title
        <Title text="chinxeleer"/>

        // content for this welcome page
        <Router fallback=|| {
            let mut outside_errors = Errors::default();
            outside_errors.insert_with_default_key(AppError::NotFound);
            view! {
                <ErrorTemplate outside_errors/>
            }
            .into_view()
        }>
            <MainRoutes/>
        </Router>
    }
}
