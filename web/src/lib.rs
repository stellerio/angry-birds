use yew::prelude::*;

#[function_component(App)]
fn app() -> Html {
    let launches = use_state(|| 0_u32);
    let score = use_state(|| 0_u32);
    let launched = use_state(|| false);

    let on_launch = {
        let launches = launches.clone();
        let score = score.clone();
        let launched = launched.clone();
        Callback::from(move |_| {
            launches.set(*launches + 1);
            score.set(*score + 100);
            launched.set(true);
        })
    };

    let on_reset = {
        let launches = launches.clone();
        let score = score.clone();
        let launched = launched.clone();
        Callback::from(move |_| {
            launches.set(0);
            score.set(0);
            launched.set(false);
        })
    };

    html! {
        <main class="page-shell">
            <nav class="nav">
                <a class="brand" href="#top"><span class="brand-mark">{"A"}</span>{"ANGRY / RUST"}</a>
                <div class="nav-links">
                    <a href="#about">{"ABOUT"}</a>
                    <a href="#demo">{"DEMO"}</a>
                    <a class="github-link" href="https://github.com/stellerio/angry-birds" target="_blank" rel="noreferrer">{"GITHUB ↗"}</a>
                </div>
            </nav>

            <section class="hero" id="top">
                <div class="hero-copy">
                    <div class="eyebrow"><span class="status-dot"></span>{" OPEN-SOURCE • BUILT WITH RUST"}</div>
                    <h1>{"A LITTLE "}<span>{"CHAOS."}</span><br />{"A LOT OF "}<span>{"RUST."}</span></h1>
                    <p class="hero-text">{"An original slingshot game project exploring physics, playful interactions, and Rust running right in your browser."}</p>
                    <div class="hero-actions">
                        <a class="primary-button" href="#demo">{"TRY THE DEMO"}<span>{"↘"}</span></a>
                        <a class="text-button" href="https://github.com/stellerio/angry-birds" target="_blank" rel="noreferrer">{"VIEW SOURCE ↗"}</a>
                    </div>
                    <div class="hero-note"><span>{"01"}</span>{" NO INSTALL. JUST YOUR BROWSER."}</div>
                </div>
                <div class="hero-art" aria-label="Stylized slingshot game illustration">
                    <div class="orbit orbit-one"></div>
                    <div class="orbit orbit-two"></div>
                    <div class="sun"></div>
                    <div class="platform"></div>
                    <div class="tower tower-back"><i></i><i></i></div>
                    <div class="tower tower-front"><i></i><i></i><i></i></div>
                    <div class="bird"><span class="bird-eye"></span><span class="bird-beak"></span></div>
                    <div class="slingshot"><i></i><b></b><i></i></div>
                    <div class="spark spark-a">{"✳"}</div>
                    <div class="spark spark-b">{"+"}</div>
                    <div class="art-label">{"KINETIC PLAY / 001"}</div>
                </div>
            </section>

            <section class="stats" id="about">
                <div><span class="stat-number">{"01"}</span><span class="stat-label">{"RUST-FIRST"}</span></div>
                <div><span class="stat-number">{"WASM"}</span><span class="stat-label">{"BROWSER POWERED"}</span></div>
                <div><span class="stat-number">{"∞"}</span><span class="stat-label">{"THINGS TO BUILD"}</span></div>
            </section>

            <section class="demo-section" id="demo">
                <div class="section-heading">
                    <div><div class="eyebrow">{"// INTERACTIVE PLAYGROUND"}</div><h2>{"TAKE A SHOT."}</h2></div>
                    <p>{"A tiny interaction demo for the site. The full physics game is still in development."}</p>
                </div>
                <div class="demo-card">
                    <div class="demo-scene">
                        <div class="demo-skyline"></div>
                        <div class={classes!("demo-bird", if *launched { "launched" } else { "" })}><span></span></div>
                        <div class="demo-target"><i></i><i></i><i></i></div>
                        <div class="demo-ground"></div>
                        <div class="demo-hint">{if *launched { "NICE SHOT. READY FOR ANOTHER?" } else { "READY... AIM... CLICK!" }}</div>
                    </div>
                    <div class="demo-panel">
                        <div class="score-label">{"SESSION SCORE"}</div>
                        <div class="score-value">{format!("{:04}", *score)}</div>
                        <div class="launch-count"><span>{"LAUNCHES"}</span><strong>{format!("{:02}", *launches)}</strong></div>
                        <button class="launch-button" onclick={on_launch}>{"LAUNCH BIRD"}<span>{"↗"}</span></button>
                        <button class="reset-button" onclick={on_reset}>{"RESET SESSION"}</button>
                        <p class="demo-disclaimer">{"Demo points are for fun — no external services or accounts."}</p>
                    </div>
                </div>
            </section>

            <section class="build-strip">
                <div><div class="eyebrow">{"// THE STACK"}</div><h2>{"RUST AT THE CORE."}</h2></div>
                <div class="stack-items"><span>{"RUST"}</span><b>{"+"}</b><span>{"WASM"}</span><b>{"+"}</b><span>{"Yew"}</span><b>{"+"}</b><span>{"GitHub Pages"}</span></div>
            </section>

            <footer><a class="brand" href="#top"><span class="brand-mark">{"A"}</span>{"ANGRY / RUST"}</a><span>{"AN EXPERIMENT IN BROWSER-BASED RUST."}</span><a href="https://github.com/stellerio/angry-birds" target="_blank" rel="noreferrer">{"SOURCE CODE ↗"}</a></footer>
        </main>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
