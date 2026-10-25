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
            link: "https://github.com/sagarhutagi/pesu-helper-extension",
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
            link: "https://github.com/sagarhutagi/blackhole",
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
            id: "encrypted-chat",
            title: "PGP Encrypted Chat App",
            summary: "A privacy-focused messaging application on the Dark Web using PGP encryption and Tor hidden services.",
            description: r#"
<p>Built a privacy-first messaging application using <b>PGP encryption</b> (Pretty-Good-Privacy) operating on the <b>Dark Web 🧅</b>, ensuring communications remain entirely private from end to end.</p>

<h3>🧅 Tor Hidden Services & Vanity .onion</h3>
<p>Hosted locally using Tor hidden services. Used <code>mkp224o</code> to generate a custom vanity v3 <code>.onion</code> address with <code>sagarh</code> at the start—computed in roughly 5 minutes on a 4-core CPU.</p>

<h3>🔒 End-to-End PGP Encryption</h3>
<p>Inspired by an <i>IAmLucid</i> video exploring PGP cryptography. Messages are encrypted client-side using <b>OpenPGP.js</b> before transmission—the backend only stores encrypted ciphertext, never plaintext.</p>

<h3>✨ Key Features & Architecture</h3>
<ul style="margin-left: 20px; margin-bottom: 16px;">
    <li><b>Tor-Native Routing:</b> Runs as a Tor hidden service reachable via custom .onion addresses.</li>
    <li><b>Secure Key Management:</b> Public keys are synchronized via Supabase; private keys remain exclusively in local browser storage.</li>
    <li><b>Supabase Backend:</b> Real-time messaging with strict Row-Level Security (RLS) policies.</li>
    <li><b>Zero Metadata Tracking:</b> Designed from the ground up to protect user privacy and avoid message retention.</li>
</ul>

<p>This project was an exciting journey into <i>asymmetric encryption, Tor networking, OpenPGP, and zero-trust web architectures</i>.</p>
            "#,
            link: "https://github.com/sagarhutagi/tor-chat-web",
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
            id: "smart-ship",
            title: "Smart Ship",
            summary: "An automatic, eco-friendly, solar-powered vessel designed to prevent maritime collisions and reduce emissions.",
            description: r#"
<p>Smart Ship is an automatic, secure, eco-friendly, and cost-effective marine vessel designed to make shipping more efficient, reduce carbon emissions, and prevent maritime accidents that occur each year.</p>

<h3>☀️ 100% Electric & Solar Powered</h3>
<p>Eliminates fossil fuel dependency completely by running purely on electricity. Equipped with high-efficiency solar panels capable of fully recharging the onboard battery system within an hour.</p>

<h3>🛡️ Automated Safety & Sensor Systems</h3>
<ul style="margin-left: 20px; margin-bottom: 16px;">
    <li><b>Ultrasonic Obstacle Detection:</b> Scans ahead for objects and hazards, automatically stopping the ship immediately upon detection to avoid collisions and save lives.</li>
    <li><b>Smart Bilge Pump:</b> Integrated moisture sensors detect any accidental water ingress inside the hull and trigger a motorized pump to eject the water automatically.</li>
    <li><b>Autonomous LDR Lighting:</b> Built-in Light Dependent Resistor (LDR) sensor turns navigation lights on automatically at night and turns them off at sunrise to conserve energy.</li>
    <li><b>Functional Directional Steering:</b> Features an active electromechanical steering mechanism that turns the ship's rudder precisely with steering commands.</li>
</ul>

<p>Developed with green technologies to dramatically reduce environmental impact and provide autonomous safety at sea.</p>
            "#,
            link: "https://drive.google.com/file/d/1BZz6glL9o3w-IC6s5pwm-dUTCaBAwCi-/view",
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
            id: "notes-app",
            title: "Notes App (Daily Diary)",
            summary: "A personal diary and note-taking platform featuring mood analytics, tagging, and secure export.",
            description: r#"
<p>Daily Diary is a personal journal and note-taking platform designed to help users document thoughts, track emotional wellness, and gain insights into daily habits.</p>

<h3>📊 Mood Tracking & Analytics</h3>
<p>Every note and entry tracks mood ratings alongside custom tags, allowing users to visualize emotional trends across an interactive calendar interface.</p>

<h3>✨ Key Features</h3>
<ul style="margin-left: 20px; margin-bottom: 16px;">
    <li><b>Rich Diary Entries:</b> Create, edit, and categorize entries with mood ratings and custom tags.</li>
    <li><b>Interactive Calendar View:</b> Browse past reflections chronologically with at-a-glance status indicators.</li>
    <li><b>Search & Filtering:</b> Quickly locate past entries by keywords, mood ratings, or dates.</li>
    <li><b>PDF & CSV Export:</b> Generate clean PDF reports and data backups directly from the app.</li>
    <li><b>User Authentication:</b> Multi-user login with password hashing and session management.</li>
</ul>

<p>Built with <i>Python, Flask, SQLAlchemy, SQLite, and Matplotlib</i> for data visualization and report generation.</p>
            "#,
            link: "https://github.com/sagarhutagi/dailydiary",
            tags: &["Python", "Flask", "SQLite", "Data Analytics"],
            images: &[
                "/images/Notes%20App/1771657347988-vspt6p.png",
                "/images/Notes%20App/1771657361227-ji23fq.png",
                "/images/Notes%20App/1771657364352-zslxpp.png",
                "/images/Notes%20App/1771657366485-zazp5m.png",
            ],
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
            link: "https://github.com/sagarhutagi/deltatime",
            tags: &["Game Development", "Game Programming", "Level Design"],
            images: &[],
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
                                
                                <div class="project-actions">
                                    <a
                                        href=p.link
                                        target="_blank"
                                        rel="noopener noreferrer"
                                        class="btn"
                                    >
                                        {if p.link.contains("drive.google.com") { "Watch Video Demo" } else { "View Project" }}
                                    </a>
                                    {(!p.link.contains("drive.google.com"))
                                        .then(|| view! {
                                            <a
                                                href=p.link
                                                target="_blank"
                                                rel="noopener noreferrer"
                                                class="btn"
                                            >
                                                "Code"
                                            </a>
                                        })}
                                </div>

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