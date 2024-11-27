use leptos::*;

use crate::projects::project_card::ProjectCard;

#[component]
pub fn ProjectsPage() -> impl IntoView {
    view! {
        <main class="flex flex-col space-y-2 md:px-20">
            <div class="mx-auto">
                <h1 class="font-bold text-xl">Projects</h1>
            </div>
            <div class="flex flex-col md:flex-row flex-wrap gap-4 justify-center align-center">
                <ProjectCard
                    title=String::from("Shellinx Shell")
                    description=String::from(
                        "Shellinx is a basic implementation of a unix shell, with commands like cd, ls, pwd, echo, etc.",
                    )
                    link=String::from("https://github.com/Chinxeleer/shellinx")
                    tags=Some(vec![String::from("Rust"), String::from("Shell")])
                    picture=String::from("cardimgs/shellinx.png")
                />

            </div>
        </main>
    }
}
