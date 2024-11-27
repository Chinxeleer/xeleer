use leptos::*;
use leptos_router::*;

#[component]
pub fn ProjectCard(
    title: String,
    description: String,
    link: String,
    picture: String,
    tags: Option<Vec<String>>,
) -> impl IntoView {
    view! {
        <A href=link>
            <div class="flex flex-col rounded-lg w-[250px] drop-shadow-lg space-y-2 p-4 min-h-[350px] bg-slate-900 hover:bg-slate-950 mx-auto border border-rose-300">
                <div class="min-w-[220px] h-[100px] mx-auto">
                    <img class="fit" src=picture alt="cool" />
                </div>
                <div class="">
                    <h1 class="font-bold text-xl text-purple-300">{title}</h1>
                </div>

                <div class="text-sm text-justify">
                    <p>{description}</p>
                </div>
                <br />
                <div class="flex space-x-2">
                    <div>
                        <h2>tags:</h2>
                    </div>
                    <div class="flex flex-wrap space-x-2  text-purple-900 text-sm font-bold">
                        {tags
                            .unwrap_or_default()
                            .into_iter()
                            .map(|n| {
                                view! {
                                    <div class="bg-pink-200 mb-2 rounded-full px-2">
                                        <p>{n}</p>
                                    </div>
                                }
                            })
                            .collect::<Vec<_>>()}
                    </div>
                </div>
            </div>
        </A>
    }
}
