use gloo_storage::{LocalStorage, Storage};
use gloo_timers::callback::Interval;
use js_sys::Date;
use web_sys::HtmlInputElement;
use yew::prelude::*;

const NOTES_KEY: &str = "orbitdesk-notes-v1";
const TASKS_KEY: &str = "orbitdesk-tasks-v1";

#[derive(Clone, PartialEq)]
enum Page {
    Home,
    Notes,
    Tasks,
    Focus,
    Tools,
}

fn clock_text() -> String {
    Date::new_0().to_locale_time_string("en-US").into()
}

#[function_component(App)]
fn app() -> Html {
    let page = use_state(|| Page::Home);
    let clock = use_state(clock_text);
    let notes = use_state(|| LocalStorage::get::<String>(NOTES_KEY).unwrap_or_default());
    let note_saved = use_state(|| false);
    let task_input = use_state(String::new);
    let tasks = use_state(|| {
        LocalStorage::get::<String>(TASKS_KEY)
            .unwrap_or_default()
            .lines()
            .filter(|line| !line.trim().is_empty())
            .map(str::to_owned)
            .collect::<Vec<String>>()
    });
    let focus_seconds = use_state(|| 25 * 60_u32);
    let focus_running = use_state(|| false);
    let focus_sessions = use_state(|| 0_u32);
    let calc_a = use_state(|| "12".to_string());
    let calc_b = use_state(|| "8".to_string());
    let calc_result = use_state(|| "20".to_string());
    let launches = use_state(|| 0_u32);

    {
        let clock = clock.clone();
        use_effect_with((), move |_| {
            let interval = Interval::new(1000, move || clock.set(clock_text()));
            || drop(interval)
        });
    }
    {
        let focus_seconds = focus_seconds.clone();
        let focus_running = focus_running.clone();
        let focus_sessions = focus_sessions.clone();
        use_effect_with((), move |_| {
            let interval = Interval::new(1000, move || {
                if *focus_running && *focus_seconds > 0 {
                    let next = *focus_seconds - 1;
                    focus_seconds.set(next);
                    if next == 0 {
                        focus_running.set(false);
                        focus_sessions.set(*focus_sessions + 1);
                    }
                }
            });
            || drop(interval)
        });
    }

    let nav = |label: &'static str, target: Page, active: bool, page: UseStateHandle<Page>| {
        html! { <button class={classes!("nav-item", active.then_some("active"))} onclick={Callback::from(move |_| page.set(target.clone()))}><span class="nav-icon">{match label { "Home" => "⌂", "Notes" => "▤", "Tasks" => "✓", "Focus" => "◷", _ => "⌘" }}</span>{label}</button> }
    };

    let save_notes = {
        let notes = notes.clone();
        let note_saved = note_saved.clone();
        Callback::from(move |_| {
            let _ = LocalStorage::set(NOTES_KEY, (*notes).clone());
            note_saved.set(true);
        })
    };
    let add_task = {
        let tasks = tasks.clone();
        let task_input = task_input.clone();
        Callback::from(move |_| {
            let item = task_input.trim().to_string();
            if !item.is_empty() {
                let mut next = (*tasks).clone();
                next.push(item);
                let _ = LocalStorage::set(TASKS_KEY, next.join("\n"));
                tasks.set(next);
                task_input.set(String::new());
            }
        })
    };
    let clear_tasks = {
        let tasks = tasks.clone();
        Callback::from(move |_| {
            tasks.set(Vec::new());
            let _ = LocalStorage::set(TASKS_KEY, "");
        })
    };
    let toggle_focus = {
        let focus_running = focus_running.clone();
        let focus_seconds = focus_seconds.clone();
        let focus_sessions = focus_sessions.clone();
        Callback::from(move |_| {
            if *focus_seconds == 0 {
                focus_seconds.set(25 * 60);
                focus_sessions.set(*focus_sessions);
            }
            focus_running.set(!*focus_running);
        })
    };
    let reset_focus = {
        let focus_seconds = focus_seconds.clone();
        let focus_running = focus_running.clone();
        Callback::from(move |_| {
            focus_seconds.set(25 * 60);
            focus_running.set(false);
        })
    };
    let calculate = {
        let calc_a = calc_a.clone();
        let calc_b = calc_b.clone();
        let calc_result = calc_result.clone();
        Callback::from(move |_| {
            let a = calc_a.parse::<f64>().unwrap_or(0.0);
            let b = calc_b.parse::<f64>().unwrap_or(0.0);
            calc_result.set(format!("{}", a + b));
        })
    };
    let launch = {
        let launches = launches.clone();
        Callback::from(move |_| launches.set(*launches + 1))
    };

    let time = (*clock).clone();
    let focus_display = format!("{:02}:{:02}", *focus_seconds / 60, *focus_seconds % 60);
    let tasks_count = tasks.len();
    let note_count = notes.split_whitespace().count();
    let active_page = (*page).clone();

    html! {
        <div class="app-frame">
            <aside class="sidebar">
                <a class="logo" href="#top"><span class="logo-mark">{"O"}</span><span>{"orbit"}<small>{"DESK / 01"}</small></span></a>
                <div class="side-label">{"WORKSPACE"}</div>
                {nav("Home", Page::Home, active_page == Page::Home, page.clone())}
                {nav("Notes", Page::Notes, active_page == Page::Notes, page.clone())}
                {nav("Tasks", Page::Tasks, active_page == Page::Tasks, page.clone())}
                {nav("Focus", Page::Focus, active_page == Page::Focus, page.clone())}
                {nav("Tools", Page::Tools, active_page == Page::Tools, page.clone())}
                <div class="sidebar-bottom"><div class="live-pill"><i></i>{" ALL SYSTEMS LOCAL"}</div><p>{"Your little corner of the internet."}</p><span>{"ORBIT DESK · RUST + WASM"}</span></div>
            </aside>
            <main class="main">
                <header class="topbar"><div><span class="crumb">{"WORKSPACE"}</span><span class="crumb-slash">{"/"}</span><strong>{match active_page { Page::Home => "Overview", Page::Notes => "Notes", Page::Tasks => "Task board", Page::Focus => "Focus room", Page::Tools => "Toolbox" }}</strong></div><div class="top-right"><span class="date-pill">{"● LIVE CLOCK"}</span><span class="avatar">{"O"}</span></div></header>
                {if active_page == Page::Home {
                    html! {
                        <>
                            <section class="welcome"><div><div class="eyebrow"><span class="pulse"></span>{" YOUR PERSONAL COMMAND CENTER"}</div><h1>{"Make room for "} <em>{"big ideas."}</em></h1><p>{"A focused little dashboard for notes, tasks, time and everything you're building."}</p><div class="welcome-actions"><button class="btn-primary" onclick={Callback::from({let page=page.clone(); move |_| page.set(Page::Notes)})}>{"WRITE SOMETHING"}<span>{"↗"}</span></button><button class="btn-quiet" onclick={Callback::from({let page=page.clone(); move |_| page.set(Page::Focus)})}>{"START A FOCUS SESSION"}</button></div></div><div class="orb-art"><div class="orb-ring ring-a"></div><div class="orb-ring ring-b"></div><div class="orb-core"></div><span class="orb-caption">{"LOCAL / ONLINE"}</span><span class="orb-star star-a">{"✳"}</span><span class="orb-star star-b">{"+"}</span></div></section>
                            <section class="metric-grid">
                                <article class="metric"><div class="metric-top"><span>{"LOCAL TIME"}</span><span class="metric-icon">{"◷"}</span></div><div class="metric-value clock-value">{time.clone()}</div><div class="metric-foot"><i></i>{" TICKING LIVE IN YOUR BROWSER"}</div></article>
                                <article class="metric"><div class="metric-top"><span>{"NOTES WORDS"}</span><span class="metric-icon">{"▤"}</span></div><div class="metric-value">{note_count}</div><div class="metric-foot">{" SAVED IN THIS BROWSER"}</div></article>
                                <article class="metric"><div class="metric-top"><span>{"OPEN TASKS"}</span><span class="metric-icon">{"✓"}</span></div><div class="metric-value">{tasks_count}</div><div class="metric-foot">{" YOUR PERSONAL CHECKLIST"}</div></article>
                            </section>
                            <section class="content-grid"><article class="panel note-preview"><div class="panel-head"><div><span class="panel-kicker">{"QUICK CAPTURE"}</span><h2>{"Thoughts, meet notes."}</h2></div><button class="text-link" onclick={Callback::from({let page=page.clone(); move |_| page.set(Page::Notes)})}>{"OPEN NOTES ↗"}</button></div><p class="muted">{"Keep ideas close. Your notes save locally in this browser."}</p><div class="preview-note">{if notes.is_empty() {"Your next great idea goes here..."} else {(*notes).clone()}}</div><button class="btn-outline" onclick={Callback::from({let page=page.clone(); move |_| page.set(Page::Notes)})}>{"+ NEW NOTE"}</button></article><article class="panel focus-preview"><div class="panel-head"><div><span class="panel-kicker">{"FOCUS TIMER"}</span><h2>{"Find your flow."}</h2></div><span class="round-icon">{"◴"}</span></div><div class="mini-timer">{focus_display.clone()}</div><p class="muted">{"25 minutes. One thing at a time."}</p><button class="btn-primary full" onclick={toggle_focus.clone()}>{if *focus_running {"PAUSE SESSION"} else {"START FOCUS"} }<span>{"↗"}</span></button></article></section>
                            <section class="bottom-strip"><span class="strip-mark">{"✳"}</span><div><strong>{"Small steps count."}</strong><p>{"You've completed "}{*focus_sessions}{" focus session(s). Keep showing up."}</p></div><button class="text-link" onclick={Callback::from({let page=page.clone(); move |_| page.set(Page::Tasks)})}>{"VIEW TASKS ↗"}</button></section>
                        </>
                    }
                } else if active_page == Page::Notes {
                    html! { <section class="tool-page"><div class="eyebrow">{"// YOUR IDEAS, KEPT CLOSE"}</div><h1>{"The notes <em>space.</em>"}</h1><p class="muted">{"Draft freely. Hit save to keep it in this browser."}</p><article class="panel editor-panel"><div class="panel-head"><div><span class="panel-kicker">{"UNTITLED NOTE"}</span><h2>{"Scratchpad"}</h2></div><span class="saved-badge">{"● LOCAL STORAGE"}</span></div><textarea class="notes-editor" placeholder="Start typing an idea, a plan, a reminder..." value={(*notes).clone()} oninput={Callback::from({let notes=notes.clone();let note_saved=note_saved.clone();move |e:InputEvent| {let input: web_sys::HtmlTextAreaElement=e.target_unchecked_into();notes.set(input.value());note_saved.set(false);}})}></textarea><div class="editor-footer"><span class="muted">{note_count}{" words · saved on this device only"}</span><button class="btn-primary" onclick={save_notes.clone()}>{if *note_saved {"SAVED ✓"} else {"SAVE NOTE ↗"}}</button></div></article><p class="privacy-note">{"No account or server is used. Clearing this browser's site data can erase locally saved notes."}</p></section> }
                } else if active_page == Page::Tasks {
                    html! { <section class="tool-page"><div class="eyebrow">{"// ONE THING AT A TIME"}</div><h1>{"The task <em>board.</em>"}</h1><p class="muted">{"Get it out of your head and onto the list."}</p><article class="panel task-panel"><form class="task-add" onsubmit={Callback::from({let add_task=add_task.clone();move |e:SubmitEvent| {e.prevent_default();add_task.emit(());}})}><input placeholder="Add a task to your list..." value={(*task_input).clone()} oninput={Callback::from({let task_input=task_input.clone();move |e:InputEvent| {let input:HtmlInputElement=e.target_unchecked_into();task_input.set(input.value());}})} /><button class="btn-primary" type="submit">{"ADD TASK +"}</button></form><div class="task-list">{for tasks.iter().enumerate().map(|(i,task)| html!{<div class="task-row" key={i}><span class="task-number">{format!("{:02}",i+1)}</span><span>{task}</span></div>})}{if tasks.is_empty(){html!{<div class="empty-state">{"Nothing on the list yet. Add a task above and get rolling."}</div>}}else{html!{}}}</div><div class="editor-footer"><span class="muted">{tasks_count}{" task(s) · saved on this device"}</span><button class="btn-outline" onclick={clear_tasks}>{"CLEAR LIST"}</button></div></article></section> }
                } else if active_page == Page::Focus {
                    html! { <section class="tool-page"><div class="eyebrow">{"// LESS SCROLLING, MORE DOING"}</div><h1>{"Enter the <em>flow.</em>"}</h1><p class="muted">{"A simple Pomodoro-style focus timer."}</p><article class="panel focus-room"><div class="timer-ring"><div class="timer-inner"><span>{"FOCUS SESSION"}</span><strong>{focus_display}</strong><small>{if *focus_running {"SESSION IN PROGRESS"} else {"READY WHEN YOU ARE"}}</small></div></div><div class="focus-controls"><button class="btn-primary" onclick={toggle_focus.clone()}>{if *focus_running {"PAUSE TIMER"} else if *focus_seconds == 0 {"START AGAIN"} else {"START TIMER"}}</button><button class="btn-outline" onclick={reset_focus.clone()}>{"RESET"}</button></div><p class="muted">{"Completed sessions: "}{*focus_sessions}{" · Timer runs while this page is open."}</p></article></section> }
                } else {
                    html! { <section class="tool-page"><div class="eyebrow">{"// SMALL TOOLS, ZERO FUSS"}</div><h1>{"Your <em>toolbox.</em>"}</h1><p class="muted">{"A few handy utilities built right into your browser dashboard."}</p><div class="tools-grid"><article class="panel utility-card"><span class="panel-kicker">{"01 / CLOCK"}</span><div class="utility-icon">{"◷"}</div><h2>{"Live clock"}</h2><div class="tool-clock">{time.clone()}</div><p class="muted">{"Updates every second using your device's local time."}</p></article><article class="panel utility-card"><span class="panel-kicker">{"02 / CALCULATOR"}</span><div class="utility-icon">{"＋"}</div><h2>{"Quick add"}</h2><div class="calc-inputs"><input type="number" value={(*calc_a).clone()} oninput={Callback::from({let calc_a=calc_a.clone();move |e:InputEvent| {let input:HtmlInputElement=e.target_unchecked_into();calc_a.set(input.value());}})}/><span>{"+"}</span><input type="number" value={(*calc_b).clone()} oninput={Callback::from({let calc_b=calc_b.clone();move |e:InputEvent| {let input:HtmlInputElement=e.target_unchecked_into();calc_b.set(input.value());}})}/></div><button class="btn-primary full" onclick={calculate}>{"CALCULATE"}</button><div class="calc-result">{"RESULT  /  "}{(*calc_result).clone()}</div></article><article class="panel utility-card"><span class="panel-kicker">{"03 / MINI GAME"}</span><div class="utility-icon">{"↗"}</div><h2>{"Launch counter"}</h2><p class="muted">{"A tiny callback demo from the original project."}</p><div class="tool-clock">{*launches}{" launches"}</div><button class="btn-primary full" onclick={launch}>{"LAUNCH +1"}</button></article><article class="panel utility-card"><span class="panel-kicker">{"04 / QUICK LINKS"}</span><div class="utility-icon">{"⌘"}</div><h2>{"Jump back in"}</h2><p class="muted">{"Pick up where you left off."}</p><button class="btn-outline full" onclick={Callback::from({let page=page.clone();move |_| page.set(Page::Notes)})}>{"OPEN NOTES ↗"}</button><button class="btn-outline full" onclick={Callback::from({let page=page.clone();move |_| page.set(Page::Tasks)})}>{"OPEN TASKS ↗"}</button></article></div></section> }
                }}
                <footer class="footer"><span>{"ORBIT DESK / PERSONAL SPACE"}</span><span>{"MADE WITH RUST + YEW + WEBASSEMBLY"}</span><span>{"YOUR BROWSER. YOUR WORKSPACE."}</span></footer>
            </main>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
