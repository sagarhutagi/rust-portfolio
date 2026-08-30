use leptos::prelude::*;
use leptos_router::{
    components::{Route, Router, Routes, A},
    hooks::use_params_map,
    StaticSegment, ParamSegment,
};

// ─────────────────────────────
// CONTACT LINKS
// ─────────────────────────────

const GITHUB_URL: &str = "https://github.com/sagarhutagi";
const GITHUB_HANDLE: &str = "sagarhutagi";
const LINKEDIN_URL: &str = "https://linkedin.com/in/sagarhutagi";
const LINKEDIN_HANDLE: &str = "/in/sagarhutagi";
const EMAIL: &str = "sagarhutagi@gmail.com";
const RESUME_URL: &str =
    "https://drive.google.com/file/d/1W_GuzvKKbIJEHw4Qs9iy-xjm6i8vybdh/view?usp=sharing";

// ─────────────────────────────
// PROJECT DATA
// Paths in `images` are relative to your site root — Trunk copies
// everything in `public/` there, so a file at public/images/foo.png
// is served at "/images/foo.png". Fill these in per project.
// ─────────────────────────────

#[derive(Clone)]
pub struct Project {
    pub id: &'static str,
    pub title: &'static str,
    pub summary: &'static str,
    pub description: &'static str,
    pub link: &'static str,
    pub tags: &'static [&'static str],
    pub images: &'static [&'static str],
}

fn projects() -> Vec<Project> {
    vec![
        Project {
            id: "pesu-helper",
            title: "PESU Helper",
            summary: "A browser extension that turns PESU Academy into a personalized study dashboard.",
            description: r#"
<p>PESU Helper started as a simple tool for making PDF handling on <b>PESU Academy</b> less frustrating, but gradually evolved into a complete personal study companion.</p>

<h3>🎯 The Goal</h3>
<p>To reduce small, repetitive actions that interrupt studying by bringing everything into a centralized interface that integrates directly into the university portal.</p>

<h3>✨ Key Features</h3>
<ul style="margin-left: 20px; margin-bottom: 16px;">
    <li><b>Integrated PDF Controls:</b> Automatically names downloaded files after their corresponding topics.</li>
    <li><b>Live Dashboard:</b> Tracks the active time spent studying course materials and PDFs.</li>
    <li><b>Subject Syncing:</b> Pulls enrolled subjects directly from the <i>My Courses</i> page.</li>
    <li><b>Exam Filtering:</b> Sorts progress across Theory, Numericals, and Revisions for ISA 1, ISA 2, and ESA exams.</li>
</ul>

<p>Building this provided deep experience working with <i>browser extension architecture, background service workers, browser storage, and DOM manipulation</i>.</p>
            "#,
            link: "https://github.com/sagarhutagi/pesu-helper",
            tags: &["JavaScript", "Browser Extension", "PESU Academy"],
            images: &["/images/pesu-helper-1.png"],
        },

        Project {
            id: "ai-code-doctor",
            title: "AI Code Doctor",
            summary: "An AI-powered tool for analyzing and diagnosing code with locally hosted models.",
            description: r#"
<p>AI Code Doctor is built around the concept of utilizing locally running AI models to help developers diagnose code without sending sensitive data to cloud providers.</p>

<h3>🧠 Local-First Architecture</h3>
<p>Instead of relying entirely on remote APIs, this project leverages <b>Ollama</b> to run language models entirely on your local machine.</p>

<h3>🛠️ Tech Stack Breakdown</h3>
<ul style="margin-left: 20px; margin-bottom: 16px;">
    <li><b>Backend:</b> Served using <i>FastAPI</i> to handle core logic and model communication.</li>
    <li><b>Frontend:</b> A lightweight interface for submitting and analyzing code snippets.</li>
    <li><b>AI Engine:</b> Powered seamlessly by local LLM inference.</li>
</ul>

<p>This project was an excellent opportunity to explore the architecture behind AI-assisted developer tooling and how a web frontend interacts with local language-model inference.</p>
            "#,
            link: "https://github.com/sagarhutagi/ai-code-doctor",
            tags: &["Python", "AI", "Ollama", "FastAPI"],
            images: &["/images/ai-code-doctor-1.png"],
        },

        Project {
            id: "black-hole",
            title: "Black Hole",
            summary: "An anonymous real-time social network built for campus conversations.",
            description: r#"
<p>Black Hole is designed specifically for raw, unfiltered campus conversations. Students can share confessions, ideas, and hot takes without attaching their real identity to every message.</p>

<h3>🎭 Identity & Privacy</h3>
<p>Each user is given a randomly generated identity to keep the experience completely focused on the content of the conversation rather than the person behind it.</p>

<h3>🔥 Core Mechanics</h3>
<ul style="margin-left: 20px; margin-bottom: 16px;">
    <li><b>Real-time Updates:</b> Powered by <b>Supabase</b> (PostgreSQL + Realtime) for instant message delivery, presence tracking, and emoji reactions.</li>
    <li><b>Hashtag Groups:</b> Topic-based channels for organized discussions.</li>
    <li><b>The Daily Purge:</b> To ensure ephemerality, all messages and inactive groups are permanently wiped at midnight IST.</li>
</ul>

<p>Built with <i>React, TypeScript, and Tailwind CSS</i>, this was a massive exploration into real-time WebSockets, database security policies, and responsive app design.</p>
            "#,
            link: "https://github.com/sagarhutagi/black-hole",
            tags: &["React", "TypeScript", "Supabase", "Tailwind CSS"],
            images: &["/images/black-hole-1.png"],
        },

        Project {
            id: "encrypted-chat",
            title: "Encrypted Chat",
            summary: "A privacy-focused messaging application built around PGP encryption and Tor.",
            description: r#"
<p>A privacy-focused messaging application designed around the principle that message contents should remain private—even from the infrastructure carrying them.</p>

<h3>🔒 True End-to-End Encryption</h3>
<p>Using <b>PGP-based encryption</b>, messages are encrypted on the client before they are sent to the backend. The server stores only encrypted ciphertext, never plaintext.</p>

<h3>🌐 Key Features</h3>
<ul style="margin-left: 20px; margin-bottom: 16px;">
    <li><b>Tor Network Ready:</b> Designed to operate seamlessly over anonymity networks.</li>
    <li><b>Secure Key Management:</b> Public keys are stored via <i>Supabase</i>, while private keys never leave the user's device.</li>
    <li><b>Cross-Platform UI:</b> A clean, responsive dark theme that works across desktop, tablet, and mobile.</li>
</ul>

<p>This project reinforced vital secure software principles: working with <i>OpenPGP.js</i>, strict input validation, correct backend configuration, and protecting user metadata.</p>
            "#,
            link: "https://github.com/sagarhutagi/encrypted-chat",
            tags: &["JavaScript", "PGP", "Supabase", "Tor"],
            images: &["/images/encrypted-chat-1.png"],
        },

        Project {
            id: "9-lives",
            title: "9 Lives",
            summary: "A game-jam platformer built around the idea that death is part of progression.",
            description: r#"
<p>Created during a 48-hour game jam (Delta Time) at PES University EC Campus around the theme <i>"Death is not the end."</i></p>

<h3>🐈 The Concept</h3>
<p>You play as a lost cat trying to find its owner. Rather than treating death as a failure state, the game makes it an essential mechanic for progression.</p>

<h3>🎮 Gameplay Loop</h3>
<ul style="margin-left: 20px; margin-bottom: 16px;">
    <li><b>9 Lives, 9 Abilities:</b> Each time you die, you return with a new advantage like <i>reduced gravity</i> or <i>enhanced vision</i>.</li>
    <li><b>Strategic Sacrifice:</b> Environments are structured so you must intentionally sacrifice a life to bypass certain obstacles.</li>
</ul>

<p>I handled the <b>Game Programming</b> and <b>Level Design</b> from scratch. Building and balancing this under a strict 48-hour deadline paid off—we secured <b>1st place in Art</b> and <b>3rd place overall</b>.</p>
            "#,
            link: "https://github.com/sagarhutagi/9-lives",
            tags: &["Game Development", "Game Programming", "Level Design"],
            images: &["/images/9-lives-1.png"],
        },
    ]
}

fn find_project(id: &str) -> Option<Project> {
    projects().into_iter().find(|p| p.id == id)
}

// ─────────────────────────────
// APP / ROUTER
// ─────────────────────────────

#[component]
pub fn App() -> impl IntoView {
    view! {
        <Router>
            <div class="site">
                <Header/>
                <main>
                    <Routes fallback=|| view! { <NotFound/> }>
                        <Route path=StaticSegment("") view=Home/>
                        <Route
                            path=(StaticSegment("project"), ParamSegment("id"))
                            view=ProjectPage
                        />
                    </Routes>
                </main>
                <Footer/>
            </div>
        </Router>
    }
}

#[component]
fn Header() -> impl IntoView {
    view! {
        <header class="navbar">
            <a href="/" class="nav-name">
                <svg class="logo-mark" width="18" height="18" viewBox="0 0 24 24" fill="none">
                    <circle cx="12" cy="12" r="8.5" stroke="currentColor" stroke-width="1.6"/>
                    <circle cx="12" cy="12" r="2.75" fill="currentColor"/>
                </svg>
                "Sagar"
            </a>

            <nav>
                <a href="/#work">"Work"</a>
                <a href="/#contact">"Contact"</a>
            </nav>
        </header>
    }
}

// ─────────────────────────────
// HOME
// ─────────────────────────────

#[component]
fn Home() -> impl IntoView {
    view! {
        <section id="home" class="hero">
            <p class="hero-greeting">"Sup, I'm"</p>
            <h1>"Sagar Hutagi"</h1>

            <div class="hero-bottom">
                <p class="tagline">"I go where the rabbit holes lead."</p>

                <p class="intro">
                    "I'm a developer exploring everything from intelligent
                    systems to the software that runs beneath them. I build
                    with AI/ML, Rust, and the web, usually starting with a
                    simple question and ending somewhere I didn't expect."
                </p>

                <div class="links">
                    <a
                        href="#work"
                        class="btn swap-btn"
                        aria-label="Projects"
                    >
                        <span class="text">"Projects"</span>
                        <span class="icon">"↓"</span>
                    </a>
                    <a
                        href=RESUME_URL
                        target="_blank"
                        rel="noopener noreferrer"
                        class="btn swap-btn"
                        aria-label="Resume"
                    >
                        <span class="text">"Resume"</span>
                        <span class="icon">"↗"</span>
                    </a>
                </div>
            </div>
        </section>

        <section id="work" class="work">
            <div class="section-title">"Selected Work"</div>

            <div class="projects">
                {projects()
                    .into_iter()
                    .map(|p| {
                        let href = format!("/project/{}", p.id);
                        view! {
                            <A href=href attr:class="project">
                                <h2>{p.title}</h2>
                                <p>{p.summary}</p>
                                {(!p.tags.is_empty())
                                    .then(|| view! { 
                                        <div class="project-tags">
                                            {p.tags.iter().map(|t| view! { <span class="tag">{*t}</span> }).collect_view()}
                                        </div> 
                                    })}
                            </A>
                        }
                    })
                    .collect_view()}
            </div>
        </section>

        <section class="currently">
            <div class="section-title">"Currently"</div>
            <p class="currently-text">
                "Exploring AI/ML, Rust, Linux, and full-stack development."
            </p>
        </section>

        <section id="contact" class="contact">
            <div class="section-title">"Contact"</div>

            <div class="contact-list">
                <a href=GITHUB_URL target="_blank" rel="noopener noreferrer" class="contact-row">
                    <span>"GitHub"</span>
                    <span class="contact-value">{GITHUB_HANDLE}</span>
                </a>

                <a href=LINKEDIN_URL target="_blank" rel="noopener noreferrer" class="contact-row">
                    <span>"LinkedIn"</span>
                    <span class="contact-value">{LINKEDIN_HANDLE}</span>
                </a>

                <a href=format!("mailto:{}", EMAIL) class="contact-row">
                    <span>"Email"</span>
                    <span class="contact-value">{EMAIL}</span>
                </a>

                <a href=RESUME_URL target="_blank" rel="noopener noreferrer" class="contact-row">
                    <span>"Resume"</span>
                    <span class="contact-value">"View PDF"</span>
                </a>
            </div>
        </section>
    }
}

// ─────────────────────────────
// PROJECT PAGE
// ─────────────────────────────

#[component]
fn ProjectPage() -> impl IntoView {
    let params = use_params_map();
    let (lightbox, set_lightbox) = signal(None::<String>);

    view! {
        <section class="project-page">
            {move || {
                let id = params.read().get("id").unwrap_or_default();
                match find_project(&id) {
                    Some(p) => {
                        view! {
                            <div>
                                <div class="back-nav">
                                    <A href="/" attr:class="back-arrow" attr:aria-label="Back to home">
                                        "←"
                                    </A>
                                </div>
                                
                                <h1>{p.title}</h1>
                                
                                {(!p.tags.is_empty())
                                    .then(|| view! { 
                                        <div class="project-tags center-tags">
                                            {p.tags.iter().map(|t| view! { <span class="tag">{*t}</span> }).collect_view()}
                                        </div> 
                                    })}
                                
                                <div class="project-description" inner_html={p.description}></div>
                                
                                <div class="project-actions">
                                    <a
                                        href=p.link
                                        target="_blank"
                                        rel="noopener noreferrer"
                                        class="btn"
                                    >
                                        "View Project"
                                    </a>
                                    <a
                                        href=p.link
                                        target="_blank"
                                        rel="noopener noreferrer"
                                        class="btn"
                                    >
                                        "Code"
                                    </a>
                                </div>

                                <div class="project-gallery">
                                    {p.images
                                        .iter()
                                        .map(|src| {
                                            let src_owned = src.to_string();
                                            let src_click = src_owned.clone();
                                            view! {
                                                <button
                                                    class="gallery-thumb"
                                                    on:click=move |_| set_lightbox.set(Some(src_click.clone()))
                                                >
                                                    <img src=src_owned alt=p.title/>
                                                </button>
                                            }
                                        })
                                        .collect_view()}
                                </div>
                            </div>
                        }
                            .into_any()
                    }
                    None => view! { <NotFound/> }.into_any(),
                }
            }}

            {move || {
                lightbox
                    .get()
                    .map(|src| {
                        view! {
                            <div class="lightbox" on:click=move |_| set_lightbox.set(None)>
                                <button
                                    class="lightbox-close"
                                    aria-label="Close image"
                                    on:click=move |ev| {
                                        ev.stop_propagation();
                                        set_lightbox.set(None);
                                    }
                                >
                                    "✕"
                                </button>
                                <img
                                    class="lightbox-image"
                                    src=src
                                    on:click=move |ev| ev.stop_propagation()
                                />
                            </div>
                        }
                    })
            }}
        </section>
    }
}

#[component]
fn NotFound() -> impl IntoView {
    view! {
        <section class="project-page">
            <div class="back-nav">
                <A href="/" attr:class="back-arrow" attr:aria-label="Back to home">
                    "←"
                </A>
            </div>
            <h1 style="text-align: center;">"Not Found"</h1>
        </section>
    }
}

// ─────────────────────────────
// FOOTER
// ─────────────────────────────

#[component]
fn Footer() -> impl IntoView {
    view! {
        <footer>
            <span>"Sagar Hutagi © 2026"</span>
            <span class="footer-note">"Built with Rust + Leptos"</span>
        </footer>
    }
}