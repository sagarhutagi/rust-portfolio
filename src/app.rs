use leptos::prelude::*;
use leptos_meta::{provide_meta_context, Link, Meta, Title};
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


#[derive(Clone, Copy)]
pub struct ToastContext(pub WriteSignal<Option<String>>);

impl ToastContext {
    pub fn show(&self, msg: &str) {
        let set_toast = self.0;
        set_toast.set(Some(msg.to_string()));
        leptos::prelude::set_timeout(
            move || {
                set_toast.set(None);
            },
            std::time::Duration::from_millis(3800),
        );
    }
}

// ─────────────────────────────
// BLOG POSTS
// ─────────────────────────────

#[derive(Clone)]
pub struct BlogPost {
    pub id: &'static str,
    pub title: &'static str,
    pub date: &'static str,
    pub read_time: &'static str,
    pub summary: &'static str,
    pub medium_url: &'static str,
    pub tags: &'static [&'static str],
    pub content_html: &'static str,
}

pub fn blog_posts() -> Vec<BlogPost> {
    vec![
        BlogPost {
            id: "my-first-foss-event-indiafoss-26",
            title: "My First FOSS Event: IndiaFOSS ‘26",
            date: "Oct 2, 2026",
            read_time: "3 min read",
            summary: "Reflections on attending IndiaFOSS '26 at NIMHANS Convention Centre: open-source hardware hacks, side-channel AES key extraction with ChipWhisperer, and meeting Kailash Nadh.",
            medium_url: "https://medium.com/@sagarmh6364/my-first-foss-event-indiafoss-26-1271809c0335",
            tags: &["FOSS", "Linux", "Hardware Security", "Community"],
            content_html: r#"
<p>I’ve been using Linux and Open Source Software for years now and I love ‘Distro Hopping’. Since I’m a student I can’t always afford proprietary software from the pocket money I get. I always try to find the free alternative to paid software. That’s why loved the concept of FOSS from the start.</p>

<p>I am also in a college community called OSIRIS at PES University which has really cool people who also love Open Source. And they had planned to attend this event together and were discussing it on the official WhatsApp group. That’s how I got to know about the event and purchased a ticket.</p>

<p>Since it was my first Tech/FOSS event I barely knew anything. My plan was simple — attend the talks in the domains that I’m interested in and talk to as many people as possible. <em>(Spoiler: Everything went as planned)</em></p>

<figure class="blog-figure">
    <img src="/images/blogs/indiafoss-26/inauguration.webp" alt="India FOSS ’26 Inauguration" loading="lazy" decoding="async" />
    <figcaption>India FOSS ’26 Inauguration</figcaption>
</figure>

<p>The event was 2 days long (Sept 26th and 27th) from 9am to 5pm. It featured talks from prominent people and there were many hardware and software stalls in the venue.</p>

<div class="blog-gallery-grid">
    <figure class="blog-figure">
        <img src="/images/blogs/indiafoss-26/scanner.webp" alt="Document scanner" loading="lazy" decoding="async" />
        <figcaption>Document scanner</figcaption>
    </figure>
    <figure class="blog-figure">
        <img src="/images/blogs/indiafoss-26/streamdeck.webp" alt="A custom built stream deck" loading="lazy" decoding="async" />
        <figcaption>A custom built stream deck</figcaption>
    </figure>
    <figure class="blog-figure">
        <img src="/images/blogs/indiafoss-26/flipper.webp" alt="Flipper Zero" loading="lazy" decoding="async" />
        <figcaption>Flipper Zero</figcaption>
    </figure>
</div>

<h3>Extracting AES Keys from Embedded Devices</h3>
<p>One of the talks that caught my attention was <strong>‘Extracting AES Keys from Embedded Devices Using Open Source Hardware’</strong>. I got to learn how a processor’s power consumption changes depending on the operations it performs and how we can capture that information using a device called <strong>ChipWhisperer</strong> and extract the exact <strong>256-bit AES Key</strong> which can be used to decrypt the encrypted data. The demonstration was really fun.</p>

<h3>Meeting Industry Leaders & Open Source Devs</h3>
<p>I also got to meet <strong>Kailash Nadh, the CTO of Zerodha</strong>, and had a chance to talk to him in person which was really cool. I talked to people who never went to college and bagged good positions in their career, the developer of <strong>EndeavourOS</strong>, and people who worked at quant companies, Cloudflare, KDE, and many more.</p>

<figure class="blog-figure">
    <img src="/images/blogs/indiafoss-26/team_kailash.webp" alt="Picture of our team with Kailash Nadh" loading="lazy" decoding="async" />
    <figcaption>Picture of our team with Kailash Nadh (CTO, Zerodha)</figcaption>
</figure>

<h3>Key Takeaways</h3>
<p>Overall, the event was very interesting and informative. I learnt about a lot of stuff, connected with many, and some of them also guided me on how to get better at what I’m doing. The people there were very approachable. It was the BEST.</p>
"#,
        }
    ]
}

pub fn find_blog_post(id: &str) -> Option<BlogPost> {
    blog_posts().into_iter().find(|b| b.id == id)
}

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
                "/images/PGP%20Encrypted%20Chat%20App/1777299784909.webp",
                "/images/PGP%20Encrypted%20Chat%20App/1777299784982.webp",
                "/images/PGP%20Encrypted%20Chat%20App/1777299785426.webp",
                "/images/PGP%20Encrypted%20Chat%20App/1777299785849.webp",
                "/images/PGP%20Encrypted%20Chat%20App/1777299785934.webp",
                "/images/PGP%20Encrypted%20Chat%20App/1777299787105.webp",
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
                "/images/Black%20Hole/1771654373806-n1cotm.webp",
                "/images/Black%20Hole/1771654385387-yarprl.webp",
                "/images/Black%20Hole/1771654391907-u6cu9h.webp",
                "/images/Black%20Hole/1771654399070-ioz2fj.webp",
                "/images/Black%20Hole/1771654409129-zwpnbv.webp",
                "/images/Black%20Hole/1771654436350-0wejnw.webp",
                "/images/Black%20Hole/1771654474900-papx30.webp",
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
                "/images/AI%20Code%20Doctor/1773744840366-kmcus1.webp",
                "/images/AI%20Code%20Doctor/1773744842814-doykvj.webp",
                "/images/AI%20Code%20Doctor/1773744845082-n0wcoq.webp",
                "/images/AI%20Code%20Doctor/1773744847680-im0a0v.webp",
                "/images/AI%20Code%20Doctor/1773744850597-0slx8t.webp",
                "/images/AI%20Code%20Doctor/1773744870085-nruvv3.webp",
                "/images/AI%20Code%20Doctor/1773744874943-3b8wuq.webp",
                "/images/AI%20Code%20Doctor/1773744879999-wli1t5.webp",
                "/images/AI%20Code%20Doctor/1773744886862-et53tt.webp",
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
                "/images/PESU%20Helper/1771655197396-wvw2zz.webp",
                "/images/PESU%20Helper/1771655199298-as38p7.webp",
                "/images/PESU%20Helper/1771655201098-n5mej3.webp",
                "/images/PESU%20Helper/1771655203019-6ua7yw.webp",
                "/images/PESU%20Helper/1771655222324-dkaw5n.webp",
                "/images/PESU%20Helper/1771655229985-cmkkgt.webp",
                "/images/PESU%20Helper/1771655248448-flx81o.webp",
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
                "/images/Smart%20Ship/1771736775304-9oyg31.webp",
                "/images/Smart%20Ship/1771736790169-42hiwb.webp",
                "/images/Smart%20Ship/1771736804546-qwd0v1.webp",
                "/images/Smart%20Ship/1771736835523-ndovxf.webp",
                "/images/Smart%20Ship/1771736858742-dmwcut.webp",
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
                "/images/9%20Lives/1776928935687.webp",
                "/images/9%20Lives/1776928935761.webp",
                "/images/9%20Lives/1776928936063.webp",
                "/images/9%20Lives/1776928936329.webp",
                "/images/9%20Lives/1776928936677.webp",
            ],
        },

        Project {
            id: "notes-app",
            title: "Notes App",
            summary: "A subject-categorized study notes platform for publishing Obsidian-authored notes as PDFs with discussion sections.",
            description: r#"
<p>A web platform built to organize and publish study notes. Notes were authored locally in Obsidian, exported and uploaded as PDFs, organized by subjects, and included interactive discussion threads.</p>

<h3>Obsidian Workflow</h3>
<p>Notes were written and maintained in Obsidian with Markdown and formatting, then uploaded to the platform as readable PDFs for students to access across devices.</p>

<h3>Features</h3>
<ul style="margin-left: 20px; margin-bottom: 16px;">
    <li>Subject Categorization: Notes organized cleanly by academic subjects for streamlined access.</li>
    <li>Embedded PDF Viewing: View notes directly in-browser without having to download files first.</li>
    <li>Comment Sections: Integrated discussion area beneath each note for asking questions, sharing feedback, and collaborating with peers.</li>
</ul>
            "#,
            live_url: None,
            code_url: None,
            tags: &["Web Development", "Obsidian", "PDF", "Education"],
            images: &[
                "/images/Notes%20App/1771657347988-vspt6p.webp",
                "/images/Notes%20App/1771657361227-ji23fq.webp",
                "/images/Notes%20App/1771657364352-zslxpp.webp",
                "/images/Notes%20App/1771657366485-zazp5m.webp",
            ],
        },
    ]
}

fn find_project(id: &str) -> Option<Project> {
    projects().into_iter().find(|p| p.id == id)
}

fn get_adjacent_projects(id: &str) -> (Option<Project>, Option<Project>) {
    let list = projects();
    if let Some(pos) = list.iter().position(|p| p.id == id) {
        let prev = if pos > 0 {
            Some(list[pos - 1].clone())
        } else {
            None
        };
        let next = if pos + 1 < list.len() {
            Some(list[pos + 1].clone())
        } else {
            None
        };
        (prev, next)
    } else {
        (None, None)
    }
}

// ─────────────────────────────
// APP / ROUTER
// ─────────────────────────────

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    let (toast, set_toast) = signal::<Option<String>>(None);
    provide_context(ToastContext(set_toast));

    view! {
        <Title text="Sagar Hutagi | Portfolio"/>
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
                        <Route
                            path=(StaticSegment("blog"), ParamSegment("id"))
                            view=BlogPage
                        />
                    </Routes>
                </main>
                <Footer/>
            </div>
        </Router>

        {move || toast.get().map(|msg| {
            view! {
                <div class="toast-wrapper" role="status" aria-live="polite">
                    <div class="toast-card">
                        <span class="toast-icon">"ℹ"</span>
                        <span class="toast-text">{msg}</span>
                        <button
                            type="button"
                            class="toast-close"
                            aria-label="Close notification"
                            on:click=move |_| set_toast.set(None)
                        >
                            "✕"
                        </button>
                    </div>
                </div>
            }
        })}
    }
}

#[component]
fn Header() -> impl IntoView {
    view! {
        <header class="navbar">
            <a href="/" class="nav-name" aria-label="Sagar Hutagi - Home">
                <svg class="logo-mark" width="42" height="36" viewBox="0 0 318 274" fill="none">
                    <path fill="currentColor" fill-rule="evenodd" d="M 317,30 L 290,42 L 269,112 C 266.71,114.27 263.73,113.71 261.12,114.39 C 260.00,114.68 258.96,115.20 257.87,115.56 C 256.35,116.06 254.75,116.27 253.18,116.58 C 252.05,116.81 250.94,117.09 249.81,117.40 C 243.55,119.11 237.02,121.58 230.83,123.45 C 229.60,123.82 228.38,124.17 227.16,124.54 C 225.15,125.15 223.14,125.80 221.14,126.45 C 219.13,127.09 217.12,127.73 215.13,128.46 C 214.05,128.85 212.97,129.28 211.88,129.63 C 209.88,130.28 207.85,130.70 205.84,131.39 C 201.93,132.73 198.11,135.09 193.95,135.76 C 191.81,136.10 189.58,136.00 188.00,138.00 C 187.32,135.53 188.25,132.72 188.72,129.81 C 189.75,123.36 188.49,116.38 184.98,111.02 C 179.11,102.03 166.95,97.59 156.48,94.20 C 149.14,91.83 142.64,89.97 135.11,87.68 C 126.80,85.16 117.24,82.13 113.02,74.95 C 112.30,73.71 111.73,72.34 111.37,70.92 C 110.82,68.69 110.81,66.31 111.27,64.11 C 112.53,58.15 117.31,53.46 122.20,49.14 C 133.42,39.23 145.23,31.26 157.94,25.02 C 162.56,22.75 167.31,20.71 172.08,18.67 C 185.88,12.80 199.97,6.99 214.18,3.56 C 219.74,2.21 225.33,1.24 231.00,0.00 L 211,0 C 195.05,2.32 177.65,5.73 161.83,10.51 C 148.14,14.64 135.63,19.80 122.39,26.27 C 103.57,35.47 83.26,47.34 77.65,66.83 C 76.56,70.60 76.03,74.67 77.13,78.26 C 78.16,81.64 80.64,84.61 83.65,86.92 C 88.66,90.77 95.12,92.80 101.24,94.60 C 103.95,95.39 106.58,96.13 109.23,96.74 C 120.50,99.33 131.91,99.44 144.03,103.27 C 152.64,105.99 161.61,110.60 163.26,118.09 C 163.88,120.89 163.47,124.10 162.45,126.99 C 158.18,139.13 143.10,145.76 131.49,151.71 C 127.10,153.96 123.21,156.11 119.16,158.28 C 73.30,182.86 7.67,209.40 0.00,270.00 C 19.36,237.82 52.51,217.88 84.84,201.54 C 94.93,196.44 104.93,191.69 115.21,187.10 C 143.92,174.28 174.74,162.71 204.11,151.21 C 226.08,142.60 247.23,134.04 267.00,118.00 L 244,221 L 270,207 Z M 182,172 C 170.20,176.74 159.76,186.63 152.00,197.00 L 134,273 L 162,257 Z M 205,63 L 177,76 C 173.69,80.50 172.54,85.35 174.46,88.58 C 176.24,91.57 180.65,93.17 184.63,95.32 C 187.77,97.02 190.64,99.07 192.37,101.78 C 193.35,103.31 193.96,105.06 194.00,107.00 Z"/>
                </svg>
            </a>

            <nav>
                <a href="/#work">"Work"</a>
                <a href="/#writing">"Writing"</a>
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
    let toast = use_context::<ToastContext>();

    view! {
        <section id="home" class="hero">
            <p class="hero-greeting">"Sup, I'm"</p>
            <h1>"Sagar Hutagi"</h1>

            <div class="hero-bottom">
                <p class="tagline">"I go where the rabbit holes lead."</p>

                <p class="intro">
                    "I build software across web, systems, and open source — usually starting with a simple question and ending somewhere I didn't expect."
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
                            if let Some(t) = toast {
                                t.show("Resume is temporarily removed for privacy reasons.");
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

        <section id="writing" class="work">
            <div class="section-title">"Writing"</div>

            <div class="projects">
                {blog_posts()
                    .into_iter()
                    .map(|b| {
                        let href = format!("/blog/{}", b.id);
                        view! {
                            <A href=href attr:class="project">
                                <div class="project-header-row">
                                    <h2>{b.title}</h2>
                                    <span class="project-meta-date">{b.date}</span>
                                </div>
                                <p>{b.summary}</p>
                                <div class="project-tags">
                                    {b.tags.iter().map(|t| view! { <span class="tag">{*t}</span> }).collect_view()}
                                </div>
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
                        if let Some(t) = toast {
                            t.show("Resume is temporarily removed for privacy reasons.");
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
                        let page_title = format!("{} | Sagar Hutagi", p.title);
                        let canonical_url = format!("https://sagarhutagi.tech/project/{}", p.id);
                        let og_image = p.images.first().copied().unwrap_or("/favicon.svg");
                        let full_image_url = format!("https://sagarhutagi.tech{}", og_image);
                        let (prev_proj, next_proj) = get_adjacent_projects(&p.id);
                        view! {
                            <div>
                                <Title text=page_title.clone()/>
                                <Meta name="description" content=p.summary/>
                                <Meta property="og:title" content=page_title.clone()/>
                                <Meta property="og:description" content=p.summary/>
                                <Meta property="og:url" content=canonical_url.clone()/>
                                <Meta property="og:image" content=full_image_url.clone()/>
                                <Meta name="twitter:title" content=page_title/>
                                <Meta name="twitter:description" content=p.summary/>
                                <Meta name="twitter:image" content=full_image_url/>
                                <Link rel="canonical" href=canonical_url/>

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
                                                .enumerate()
                                                .map(|(idx, src)| {
                                                    let src_owned = src.to_string();
                                                    let src_click = src_owned.clone();
                                                    let alt_text = format!("{} screenshot {}", p.title, idx + 1);
                                                    view! {
                                                        <button
                                                            class="gallery-thumb"
                                                            on:click=move |_| set_lightbox.set(Some(src_click.clone()))
                                                            aria-label=format!("View full size screenshot {} for {}", idx + 1, p.title)
                                                        >
                                                            <img src=src_owned alt=alt_text loading="lazy" decoding="async"/>
                                                        </button>
                                                    }
                                                })
                                                .collect_view()}
                                        </div>
                                    })}

                                <nav class="project-pagination" aria-label="Project navigation">
                                    <div class="pagination-prev">
                                        {prev_proj.map(|prev| view! {
                                            <A href=format!("/project/{}", prev.id) attr:class="pagination-link prev">
                                                <span class="pagination-label">"← Previous"</span>
                                                <span class="pagination-title">{prev.title}</span>
                                            </A>
                                        })}
                                    </div>
                                    <div class="pagination-all">
                                        <A href="/#work" attr:class="pagination-all-link">
                                            "All Projects"
                                        </A>
                                    </div>
                                    <div class="pagination-next">
                                        {next_proj.map(|next| view! {
                                            <A href=format!("/project/{}", next.id) attr:class="pagination-link next">
                                                <span class="pagination-label">"Next →"</span>
                                                <span class="pagination-title">{next.title}</span>
                                            </A>
                                        })}
                                    </div>
                                </nav>
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
// ─────────────────────────────
// BLOG PAGE
// ─────────────────────────────

#[component]
fn BlogPage() -> impl IntoView {
    let params = use_params_map();

    view! {
        <section class="project-page blog-page">
            {move || {
                let id = params.read().get("id").unwrap_or_default();
                match find_blog_post(&id) {
                    Some(b) => {
                        let page_title = format!("{} | Sagar Hutagi", b.title);
                        let canonical_url = format!("https://sagarhutagi.tech/blog/{}", b.id);
                        let og_image = "https://sagarhutagi.tech/images/blogs/indiafoss-26/team_kailash.webp";
                        view! {
                            <div>
                                <Title text=page_title.clone()/>
                                <Meta name="description" content=b.summary/>
                                <Meta property="og:title" content=page_title.clone()/>
                                <Meta property="og:description" content=b.summary/>
                                <Meta property="og:url" content=canonical_url.clone()/>
                                <Meta property="og:image" content=og_image/>
                                <Meta property="og:type" content="article"/>
                                <Meta name="twitter:title" content=page_title/>
                                <Meta name="twitter:description" content=b.summary/>
                                <Meta name="twitter:image" content=og_image/>
                                <Link rel="canonical" href=canonical_url/>

                                <div class="back-nav">
                                    <A href="/#writing" attr:class="back-arrow" attr:aria-label="Back to writing">
                                        "←"
                                    </A>
                                </div>

                                <div class="blog-header">
                                    <div class="blog-meta-top">
                                        <span class="blog-date">{b.date}</span>
                                        <span class="blog-bullet">"•"</span>
                                        <span class="blog-read-time">{b.read_time}</span>
                                    </div>

                                    <h1>{b.title}</h1>

                                    <div class="project-tags center-tags">
                                        {b.tags.iter().map(|t| view! { <span class="tag">{*t}</span> }).collect_view()}
                                    </div>

                                    <div class="blog-callout">
                                        <span>"Originally published on Medium"</span>
                                        <a href=b.medium_url target="_blank" rel="noopener noreferrer" class="blog-callout-link">
                                            "Read on Medium ↗"
                                        </a>
                                    </div>
                                </div>

                                <article class="project-description blog-body" inner_html=b.content_html></article>

                                <div class="project-actions" style="margin-top: 40px;">
                                    <A href="/#writing" attr:class="btn">
                                        "← All Writing"
                                    </A>
                                    <a
                                        href=b.medium_url
                                        target="_blank"
                                        rel="noopener noreferrer"
                                        class="btn"
                                    >
                                        "Discuss on Medium ↗"
                                    </a>
                                </div>
                            </div>
                        }.into_any()
                    }
                    None => view! { <NotFound/> }.into_any(),
                }
            }}
        </section>
    }
}
