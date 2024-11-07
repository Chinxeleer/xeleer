use leptos::*;
use leptos_image::Image;

#[component]
pub fn HomeHero() -> impl IntoView {
    view! {

    <main class="flex flex-col text-pink-100 items-center mt-4">
        <div class="max-w-5xl mx-auto space-y-10">
            //<!-- Social Links Section -->
            <div class="flex rounded-full justify-center">
                <Image
                src="pictures/profile.jpg"
                blur=true
                width=150
                height=150
                quality=100
                priority=true
                alt="Profile Picture"
                class="rounded-full anti-aliasing"
                />
            </div>
            //<!-- About Section -->
            <div class="flex flex-col items-start text-justify text-sm lg:text-base mt-10 max-w-xl space-y-6">
                <h2 class="text-2xl font-extrabold">About</h2>
                <p class="px-1">
                    "I'm a Computer Science and Applied Mathematics student at the University of the Witwatersrand,
                    with a strong foundation in both programming and mathematics. My work focuses on leveraging these 
                    skills to build robust and innovative projects.
                    I’m particularly drawn to Rust 🦀 and web development,
                    and I aim to build as much as possible in Rust — including this very website."</p>

                <h3 class="text-xl font-bold">"🔨 Projects"</h3>
                <p class="px-1">
                    "My journey as a developer has taken me through a diverse range of projects:"
                </p>
                <ul class="list-disc pl-5">
                <li><span class="font-bold text-purple-200">"Terminal Applications"</span>" – From efficient, minimalist tools to full-scale games."</li>
                <li><span class="font-bold text-purple-200">"APIs and Web Services"</span>" – Crafting backend solutions with reliability and performance in mind."</li>
                <li><span class="font-bold text-purple-200">"Advent of Code Solutions"</span>" – Tackling complex, algorithmic challenges during the holiday season."</li>
                </ul>
                <p>"Each project is an opportunity to explore new areas and expand my understanding of programming, Rust, and mathematics."</p>

                <h3 class="text-xl font-bold">"🐧 Linux Enthusiast"</h3>
                <p>"Linux is central to my development workflow. I use Arch Linux as my daily driver, a choice that allows me to engage deeply with the Linux Kernel and its infrastructure. This hands-on experience continually strengthens my technical expertise and understanding of operating systems."</p>
                <h3 class="text-xl font-bold">"🌱 Leadership at Every Nation Campus Wits"</h3>
                <p>"Outside of tech, I’m involved in student mentorship as a leader at Every Nation Campus Wits.
                Here, I invest my time in guiding young students, challenging their perspectives, and helping them grow into leaders ready to make a positive impact in the world."</p>
            </div>
        </div>
    </main>

        }
}
