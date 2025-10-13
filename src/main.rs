use yew::prelude::*;

mod components;
mod assets;
mod app;

use crate::components::{
    about::About,
    gallery::Gallery,
    welcome::Welcome,
};
#[function_component]
fn App() -> Html {
    let style: Vec<&str> = vec![
        "font-sans",
        "text-gray-800",
        "bg-slate-200",
        "min-h-screen",
    ];
    html! {
        <div class={classes!(style)}>
            <app::Header />
            // <app::Nav />
            <Welcome />
            <About />
            <Gallery />
            <app::Footer />
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}