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
    html! {
        <div class={classes!(String::from("flex flex-col font-sans text-gray-800 bg-slate-200 min-h-screen"))}>
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