use leptos::*;
use leptos_router::*;

#[component]
pub fn BlogCard(title: String, date: String, link: String) -> impl IntoView {
    view! {
        <A href=link class="hover:bg-slate-900 text-purple-100">
            <div class="w-[400px] md:w-[600px] flex flex-col bg-inherit rounded-lg shadow-sm p-4 md:p-5 ">
                <h3 class="text-lg font-bold">{title}</h3>
                <p class="mt-1 text-xs uppercase text-slate-300 justify-end">
                    {date}
                </p>
            </div>
        </A>
    }
}
