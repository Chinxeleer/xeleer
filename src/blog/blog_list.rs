use std::collections::HashMap;

use leptos::*;
use leptos_meta::*;

use crate::{
    blog::blog_card::BlogCard,
    error_template::{AppError, ErrorTemplate},
    server_functions::posts::{Post, PostType},
};

#[component]
pub fn BlogList() -> impl IntoView {
    let posts = use_context::<Resource<(), Result<HashMap<PostType, Vec<Post>>, ServerFnError>>>()
        .expect("unable to find context");

    view! {
        <Title text="Blog Posts" />
        <Meta name="description" content="chinxeleer's backdoor innovative thoughts" />

        <div class="mx-auto">
            <div class="min-h-full mx-auto space-y-4 mt-10 overflow-scroll">
                <ul>
                    <Transition fallback=move || {
                        view! { <div>Loading...</div> }
                    }>
                        {move || {
                            posts()
                                .map(|posts| {
                                    match posts {
                                        Ok(posts) => {
                                            let display_post = posts.get(&PostType::Blog);
                                            if let Some(posts_data) = display_post {
                                                view! {
                                                    {posts_data
                                                        .iter()
                                                        .map(|value| {
                                                            view! {
                                                                <li>
                                                                    <BlogCard
                                                                        title=value.post_metadata.title.clone()
                                                                        link=value.post_metadata.create_href()
                                                                        date=value.post_metadata.date.clone()
                                                                        tags=value.post_metadata.get_tags()
                                                                    />
                                                                </li>
                                                            }
                                                        })
                                                        .collect_view()}
                                                }
                                                    .into_view()
                                            } else {
                                                let mut outside_errors = Errors::default();
                                                outside_errors.insert_with_default_key(AppError::NotFound);
                                                view! { <ErrorTemplate outside_errors /> }.into_view()
                                            }
                                        }
                                        Err(_) => todo!(),
                                    }
                                })
                        }}
                    </Transition>
                </ul>
            </div>
        </div>
    }
}
