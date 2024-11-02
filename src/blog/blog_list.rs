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
        <Title text="Blog Posts"/>
                                <Meta
                                    name="description"
                                    content="chinxeleer's backdoor innovative thoughts"                                />

    <div class="mx-auto">
            <div class="min-h-full mx-auto space-y-4 mt-10 overflow-scroll">
            {move ||{
                     posts().map(|posts| {
                            match posts{
                                Ok(posts) => {
                                    let display_post = posts
                            .get(&PostType::Blog);
                        if let Some(posts_data) = display_post {
                            view! {
                                {posts_data.iter().map(|value|{
                                    view! {
                                        <BlogCard title={value.post_metadata.title.clone()} link=value.post_metadata.create_href() date=value.post_metadata.date.clone() />
                                    }}
                                ).collect::<Vec<_>>()
                                }


                                // <Title text=post.post_metadata.title.clone()/>
                                // <Meta
                                //     name="description"
                                //     content=post.post_metadata.description.clone()
                                // />
                                // <article class="prose prose:text-purple-200" inner_html=post.post_content.clone()></article>
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
                }}
            </div>
    </div>}
}
