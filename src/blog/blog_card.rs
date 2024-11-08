use leptos::*;
use leptos_router::*;

#[component]
pub fn BlogCard(
    title: String,
    date: String,
    link: String,
    tags: Option<Vec<String>>,
) -> impl IntoView {
    view! {
        <A href=link class="hover:bg-slate-900 text-purple-100">
            <div class="w-[400px] md:w-[600px] flex flex-col bg-inherit rounded-lg shadow-sm p-4 md:p-5 ">
                <h3 class="text-lg font-bold">{title}</h3>
                <p class="mt-1 text-xs uppercase text-slate-300 justify-end">
                    {date}
                </p>
                <div class="flex space-x-2 mt-2">{
                    tags.unwrap_or_default().into_iter()
                            .map(|n| view! { <CardItem title=n /> })
                            .collect::<Vec<_>>()
                }
                </div>
            </div>
        </A>
    }
}

#[component]
fn CardItem(title: String) -> impl IntoView {
    view! {
        <div class="rounded-xl bg-purple-200 px-2">
            <p class="text-sm text-slate-900 font-bold">{title}</p>
        </div>
    }
}
