use leptos::*;
use leptos_icons::Icon;
use leptos_router::*;

struct Link<'a> {
    name: &'a str,
    href: &'a str,
}

#[component]
pub fn Nav() -> impl IntoView {
    let links = vec![
        Link {
            name: "Home",
            href: "/",
        },
        Link {
            name: "Blog",
            href: "blog",
        },
        Link {
            name: "Resume",
            href: "resume",
        },
        Link {
            name: "Projects",
            href: "projects",
        },
    ];
    view! {
        <nav class="flex text-pink-100 justify-center">
            <ul class="flex flex-col md:flex-row py-4 items-center space-x-4">
                <div class="flex">
                    {links
                        .into_iter()
                        .map(|n| {
                            view! {
                                {move || {
                                    if n.name == "Home" {
                                        view! {
                                            <A href=n.href>
                                                <h1 class="px-2  py-2 text-base antialiased md:text-xl font-bold hover:text-pink-200">
                                                "Blessing Kodze"
                                                </h1>
                                            </A>
                                        }
                                    } else {
                                        view! {
                                            <A href=n.href>
                                                <p class="px-2 md:px-4 py-2 font-normal hover:underline hover:text-pink-200 underline-offset-4 decoration-2">
                                                    {n.name}
                                                </p>
                                            </A>
                                        }
                                    }
                                }}

                            }
                        })
                        .collect_view()}
                </div>
                <div class="flex space-x-4">
                    <A href="https://github.com/Chinxeleer">
                        <Icon icon=icondata::AiGithubFilled width="1.5rem" height="2rem" class="hover:text-purple-200"/>
                    </A>
                    <A href="https://www.linkedin.com/in/blessing-kodze-a86302212/" >
                        <Icon icon=icondata::AiLinkedinFilled width="1.5rem" height="2rem" class="hover:text-purple-200"/>
                    </A>
                </div>
            </ul>
        </nav>
    }
}
