use chrono::{Datelike, Local};
use leptos::*;
use leptos_icons::*;

#[component]
pub fn HomeFooter() -> impl IntoView {
    let time = Local::now().year().to_string();
    view! {
        <div class="py-4 mx-20 md:flex text-sm space-x-2 items-center justify-around">
            <div class="flex space-x-2 items-center justify-center">
                <p>"Built with "</p>
                <Icon icon=icondata::FaFaceGrinHeartsSolid />
                <p>"using Rust & Leptos"</p>
            </div>

            <div>"Blessing Kodze "{time}</div>
        </div>
    }
}
