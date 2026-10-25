use leptos::prelude::*;
use leptos_meta::{provide_meta_context, Title};
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
const _RESUME_URL: &str =
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
    pub live_url: Option<&'static str>,
    pub code_url: Option<&'static str>,
    pub tags: &'static [&'static str],
    pub images: &'static [&'static str],
}

fn projects() -> Vec<Project> {
    vec![
        Project {
            id: "encrypted-chat",
            title: "PGP Encrypted Chat App",
            summary: "An end-to-end encrypted messaging application operating over Tor hidden services with client-side PGP encryption.",
            description: r#"
<p>A messaging application built to explore client-side cryptography and onion routing. The application runs as a Tor hidden service so both user traffic and server location remain anonymous.</p>

<h3>Tor Hidden Service</h3>
<p>Hosted locally through Tor. Used <code>mkp224o</code> on a 4-core CPU to brute-force a custom vanity v3 .onion address starting with <code>sagarh</code>, which took about 5 minutes.</p>

<h3>Client-Side PGP</h3>
<p>Explored after watching an IAmLucid video on PGP. Messages are encrypted on the client using OpenPGP.js before transmission. The backend only ever receives and stores ciphertext.</p>

<h3>Features</h3>
<ul style="margin-left: 20px; margin-bottom: 16px;">
    <li>Accessible over the Tor network using the custom .onion domain.</li>
    <li>Key generation happens directly in the browser; private keys stay in local storage and are never sent to the server.</li>
    <li>Supabase backend handles message routing and public key lookups with Row Level Security.</li>
    <li>Zero message metadata or plaintext retention on the server.</li>
</ul>
            "#,
            live_url: None,
            code_url: Some("https://github.com/sagarhutagi/tor-chat-web"),
            tags: &["Tor", "Dark Web", "PGP Encryption", "Supabase", "JavaScript"],
            images: &[
                "/images/PGP%20Encrypted%20Chat%20App/1777299784909.jpeg",
                "/images/PGP%20Encrypted%20Chat%20App/1777299784982.jpeg",
                "/images/PGP%20Encrypted%20Chat%20App/1777299785426.jpeg",
                "/images/PGP%20Encrypted%20Chat%20App/1777299785849.jpeg",
                "/images/PGP%20Encrypted%20Chat%20App/1777299785934.jpeg",
                "/images/PGP%20Encrypted%20Chat%20App/1777299787105.jpeg",
            ],
        },

        Project {
            id: "black-hole",
            title: "Black Hole",
            summary: "An anonymous real-time campus social platform with ephemeral posts and daily purges.",
            description: r#"
<p>An anonymous discussion platform designed for college campuses. Students can share thoughts, confessions, and discussions without creating accounts tied to their real identities.</p>

<h3>Anonymity</h3>
<p>Users receive randomized handles on each session so discussions focus entirely on content rather than profiles.</p>

<h3>Architecture</h3>
<ul style="margin-left: 20px; margin-bottom: 16px;">
    <li>Real-time message feed, presence tracking, and reactions powered by Supabase Realtime and PostgreSQL.</li>
    <li>Topic-based channels organized around hashtag groups.</li>
    <li>Automated midnight wipe (IST) that purges inactive threads and posts to keep discussions ephemeral.</li>
    <li>Responsive frontend built with React, TypeScript, and Tailwind CSS.</li>
</ul>
            "#,
            live_url: None,
            code_url: Some("https://github.com/sagarhutagi/blackhole"),
            tags: &["React", "TypeScript", "Supabase", "Tailwind CSS"],
            images: &[
                "/images/Black%20Hole/1771654373806-n1cotm.png",
                "/images/Black%20Hole/1771654385387-yarprl.png",
                "/images/Black%20Hole/1771654391907-u6cu9h.png",
                "/images/Black%20Hole/1771654399070-ioz2fj.png",
                "/images/Black%20Hole/1771654409129-zwpnbv.png",
                "/images/Black%20Hole/1771654436350-0wejnw.png",
                "/images/Black%20Hole/1771654474900-papx30.png",
            ],
        },

        Project {
            id: "ai-code-doctor",
            title: "AI Code Doctor",
            summary: "A local-first code analysis tool running language models on-device without cloud APIs.",
            description: r#"
<p>A developer utility for analyzing, reviewing, and debugging code snippets locally without sending source code to third-party cloud providers.</p>

<h3>Local Inference</h3>
<p>Uses Ollama to run open-weights LLMs entirely on the local machine, keeping sensitive code private and functional without an internet connection.</p>

<h3>Tech Stack</h3>
<ul style="margin-left: 20px; margin-bottom: 16px;">
    <li>FastAPI backend handling model communication, prompt formatting, and diagnostic responses.</li>
    <li>Web frontend for submitting code snippets and reviewing suggestions.</li>
    <li>Configurable model selection based on local hardware capabilities.</li>
</ul>
            "#,
            live_url: None,
            code_url: Some("https://github.com/sagarhutagi/ai-code-doctor"),
            tags: &["Python", "AI", "Ollama", "FastAPI"],
            images: &[
                "/images/AI%20Code%20Doctor/1773744840366-kmcus1.jpeg",
                "/images/AI%20Code%20Doctor/1773744842814-doykvj.jpeg",
                "/images/AI%20Code%20Doctor/1773744845082-n0wcoq.jpeg",
                "/images/AI%20Code%20Doctor/1773744847680-im0a0v.jpeg",
                "/images/AI%20Code%20Doctor/1773744850597-0slx8t.jpeg",
                "/images/AI%20Code%20Doctor/1773744870085-nruvv3.jpeg",
                "/images/AI%20Code%20Doctor/1773744874943-3b8wuq.jpeg",
                "/images/AI%20Code%20Doctor/1773744879999-wli1t5.jpeg",
                "/images/AI%20Code%20Doctor/1773744886862-et53tt.jpeg",
            ],
        },

        Project {
            id: "pesu-helper",
            title: "PESU Helper",
            summary: "A browser extension enhancing the PESU Academy portal with automated PDF organization and study tracking.",
            description: r#"
<p>A browser extension created to streamline downloading and viewing course materials on the PESU Academy student portal.</p>

<h3>Problem</h3>
<p>The default portal opens PDFs inside nested frames and names downloaded files with generic IDs, requiring students to manually rename and sort every file.</p>

<h3>Features</h3>
<ul style="margin-left: 20px; margin-bottom: 16px;">
    <li>Injected PDF controls: Adds direct open-in-tab and download buttons inside the portal PDF viewer.</li>
    <li>Automatic file naming: Names downloaded PDFs based on course topic and unit titles.</li>
    <li>Study dashboard: Tracks active reading time on lecture materials and resets daily.</li>
    <li>Course sync: Automatically pulls enrolled subjects from the portal to filter progress across ISA and ESA exam syllabi.</li>
</ul>
<p>Built with vanilla JavaScript, browser extension APIs, and direct DOM injection.</p>
            "#,
            live_url: None,
            code_url: Some("https://github.com/sagarhutagi/pesu-helper-extension"),
            tags: &["JavaScript", "Browser Extension", "PESU Academy"],
            images: &[
                "/images/PESU%20Helper/1771655197396-wvw2zz.jpg",
                "/images/PESU%20Helper/1771655199298-as38p7.jpg",
                "/images/PESU%20Helper/1771655201098-n5mej3.jpg",
                "/images/PESU%20Helper/1771655203019-6ua7yw.jpg",
                "/images/PESU%20Helper/1771655222324-dkaw5n.jpg",
                "/images/PESU%20Helper/1771655229985-cmkkgt.jpg",
                "/images/PESU%20Helper/1771655248448-flx81o.jpg",
            ],
        },

        Project {
            id: "smart-ship",
            title: "Smart Ship",
            summary: "An electric marine vessel prototype with automated obstacle avoidance, moisture sensing, and solar charging.",
            description: r#"
<p>A prototype vessel designed to improve safety and eliminate fossil fuel use in small watercraft through automated sensor systems and electric propulsion.</p>

<h3>Power System</h3>
<p>Runs fully on electric motors powered by onboard batteries, charged by a high-efficiency solar panel capable of replenishing the battery bank within an hour.</p>

<h3>Automated Sensors</h3>
<ul style="margin-left: 20px; margin-bottom: 16px;">
    <li>Ultrasonic obstacle detection: Continuously scans ahead and cuts motor power immediately when an object is detected to prevent collisions.</li>
    <li>Moisture-activated bilge pump: Senses water leakage inside the hull and automatically activates a pump motor to remove water.</li>
    <li>Day/night lighting: LDR sensor detects ambient light levels to turn navigation lights on at dusk and off at dawn.</li>
    <li>Functional rudder steering system coupled to helm control.</li>
</ul>
            "#,
            live_url: Some("https://drive.google.com/file/d/1BZz6glL9o3w-IC6s5pwm-dUTCaBAwCi-/view"),
            code_url: None,
            tags: &["IoT", "Robotics", "Solar Energy", "Sensors", "Embedded Systems"],
            images: &[
                "/images/Smart%20Ship/1771736775304-9oyg31.jpg",
                "/images/Smart%20Ship/1771736790169-42hiwb.jpg",
                "/images/Smart%20Ship/1771736804546-qwd0v1.jpg",
                "/images/Smart%20Ship/1771736835523-ndovxf.jpg",
                "/images/Smart%20Ship/1771736858742-dmwcut.jpg",
            ],
        },

        Project {
            id: "9-lives",
            title: "9 Lives",
            summary: "A game jam puzzle-platformer where sacrificing lives unlocks abilities to solve levels.",
            description: r#"
<p>A 2D puzzle platformer built in 48 hours for the Delta Time game jam at PES University EC Campus, centered around the theme "Death is not the end."</p>

<h3>Mechanics</h3>
<p>You play as a cat navigating hazards. Instead of ending the run, each death grants a specific modifier or ability (such as altered gravity or vision changes) necessary to bypass environmental obstacles.</p>

<h3>Outcome</h3>
<p>Programmed gameplay mechanics, level layouts, and physics under the 48-hour deadline. The project placed 1st in Art and 3rd place overall in the competition.</p>
            "#,
            live_url: None,
            code_url: None,
            tags: &["Game Development", "Game Programming", "Level Design"],
            images: &[
                "/images/9%20Lives/1776928935687.jpeg",
                "/images/9%20Lives/1776928935761.jpeg",
                "/images/9%20Lives/1776928936063.jpeg",
                "/images/9%20Lives/1776928936329.jpeg",
                "/images/9%20Lives/1776928936677.jpeg",
            ],
        },

        Project {
            id: "notes-app",
            title: "Notes App",
            summary: "A clean note-taking web application for organizing, tagging, and searching personal notes.",
            description: r#"
<p>A lightweight web application for creating, managing, and categorizing personal notes and documentation.</p>

<h3>Features</h3>
<ul style="margin-left: 20px; margin-bottom: 16px;">
    <li>Clean note editor with fast text entry and updating.</li>
    <li>Tag-based categorization to organize notes across subjects and projects.</li>
    <li>Chronological view and keyword search for quick retrieval.</li>
    <li>Export options for saving notes locally.</li>
</ul>
            "#,
            live_url: None,
            code_url: None,
            tags: &["Web Development", "UI Design", "Productivity"],
            images: &[
                "/images/Notes%20App/1771657347988-vspt6p.png",
                "/images/Notes%20App/1771657361227-ji23fq.png",
                "/images/Notes%20App/1771657364352-zslxpp.png",
                "/images/Notes%20App/1771657366485-zazp5m.png",
            ],
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
    provide_meta_context();

    view! {
        <Title text="Sagar Hutagi | Developer & Systems Engineer"/>
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
                        href="#"
                        on:click=move |ev| {
                            ev.prevent_default();
                            if let Some(win) = web_sys::window() {
                                let _ = win.alert_with_message("Resume is temporarily removed for privacy reasons.");
                            }
                        }
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

                <a
                    href="#"
                    on:click=move |ev| {
                        ev.prevent_default();
                        if let Some(win) = web_sys::window() {
                            let _ = win.alert_with_message("Resume is temporarily removed for privacy reasons.");
                        }
                    }
                    class="contact-row"
                >
                    <span>"Resume"</span>
                    <span class="contact-value">"Removed for privacy"</span>
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
                        let title = format!("{} | Sagar Hutagi", p.title);
                        view! {
                            <div>
                                <Title text=title/>
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
                                
                                {((p.live_url.is_some() || p.code_url.is_some()))
                                    .then(|| view! {
                                        <div class="project-actions">
                                            {p.live_url.map(|url| {
                                                let label = if url.contains("drive.google.com") { "Watch Demo" } else { "View Project" };
                                                view! {
                                                    <a
                                                        href=url
                                                        target="_blank"
                                                        rel="noopener noreferrer"
                                                        class="btn"
                                                    >
                                                        {label}
                                                    </a>
                                                }
                                            })}
                                            {p.code_url.map(|url| {
                                                view! {
                                                    <a
                                                        href=url
                                                        target="_blank"
                                                        rel="noopener noreferrer"
                                                        class="btn"
                                                    >
                                                        "Code"
                                                    </a>
                                                }
                                            })}
                                        </div>
                                    })}

                                {(!p.images.is_empty())
                                    .then(|| view! {
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
                                                            <img src=src_owned alt=p.title loading="lazy"/>
                                                        </button>
                                                    }
                                                })
                                                .collect_view()}
                                        </div>
                                    })}
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