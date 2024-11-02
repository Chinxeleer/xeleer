use std::collections::HashMap;

use leptos::*;
use leptos_meta::{Meta, Title};
use leptos_router::use_params_map;

use crate::{
    error_template::{AppError, ErrorTemplate},
    server_functions::posts::{Post, PostType},
};

#[component]
pub fn BlogView() -> impl IntoView {
    let params = use_params_map();
    let post_query = move || params.with(|params| params.get("post").cloned().unwrap_or_default());
    let posts = use_context::<Resource<(), Result<HashMap<PostType, Vec<Post>>, ServerFnError>>>()
        .expect("unable to find context");
    view! {
        <div class="text-slate-100 mx-auto">
            <div class="max-w-5xl mx-auto">
            </div>
            <div>
            <Transition
            fallback = move || view! { <div>Loading</div> }
            >
                <div class="mx-auto">
                {move ||{
                         posts().map(|posts| {
                                match posts{
                                    Ok(posts) => {
                                        let post = posts
                                .get(&PostType::Blog)
                                .expect("Unable to read the right post_type")
                                .iter()
                                .find(|&p| p.post_metadata.create_href() == post_query().to_lowercase());
                            if let Some(post) = post {
                                view! {
                                    <Title text=post.post_metadata.title.clone()/>
                                    <Meta
                                        name="description"
                                        content=post.post_metadata.description.clone()
                                    />
                                    <article class="prose prose-sm md:prose-lg rose-pre:bg-gray-900 text-wrap prose-pre:p-2 prose-pre:m-0 md:prose-code:p-2 prose-p:m-1 prose-p:text-pink-50 prose-strong:text-purple-200 prose-code:text-orange-200 prose-code:bg-gray-900 prose-p:text-sm prose-headings:text-purple-200 prose-slate prose-a:text-purple-200 underline-offset-2 decoration-2 text-justify prose-h1:text-center prose-h2:text-2xl prose-h1:text-3xl pb-20 " inner_html=post.post_content.clone()></article>
                                }
                                    .into_view()
                            } else {
                                let mut outside_errors = Errors::default();
                                outside_errors.insert_with_default_key(AppError::NotFound);

                                view! {  <ErrorTemplate outside_errors/> }
                                    .into_view()
                            }},
                                    Err(_) => todo!(),


                            }})
                    }

                }
                </div>
            </Transition>
            </div>
        </div>
    }
}
