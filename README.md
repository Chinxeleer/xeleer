# 🦀 Chinxeleer-rs

Welcome to my personal website—a full-stack Rust adventure! This isn't your typical JavaScript framework affair. We're talking **type safety on steroids**, **blazingly fast** performance, and code that actually compiles correctly the first time (well, most of the time 😄).

## Why Rust for Full-Stack? 🚀

Look, I get it. JavaScript frameworks dominate the web. React, Vue, Svelte—they're all battle-tested and *everywhere*. So why did I choose Rust instead? **Just for fun lol**.

## 🛠️ Tech Stack

- **Frontend:** [Leptos](https://github.com/leptos-rs/leptos) - A reactive full-stack Rust web framework with component-driven design
- **Backend:** [Axum](https://github.com/tokio-rs/axum) - Lightning-fast async web framework from the Tokio team
- **Styling:** Tailwind CSS 
- **Runtime:** Tokio async runtime - Non-blocking, highly concurrent, and ridiculously performant
- **Deployment:** Ready for Docker containerization and fly.io for hosting it.

## 📚 What's Inside

- **Blog** - Technical posts and musings (check out the `/posts/blog` directory)
- **Profile** - Who I am and what I do.
- **Responsive Design** - Looks good on your phone, tablet, and that ancient monitor at grandma's house

## 🏃 Getting Started

```bash
# Build and run the project
cargo leptos watch

# Build for production
cargo leptos build --release


Check out `Cargo.toml` and `rust-toolchain.toml` for dependency details and Rust version requirements.

## 🐳 Deployment

Dockerfile is ready to containerize this bad boy for deployment. Deploy to Fly.io or wherever your heart desires.

---

**Built with 🦀 and a whole lot of type safety.** If you're curious about Rust web development, feel free to poke around the source code—it's way more fun than you'd think!
