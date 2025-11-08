use yew::prelude::*;
use crate::style::style::*;
mod components;
mod assets;
mod app;
mod style;

use crate::components::{
    about::About,
    gallery::Gallery,
    welcome::Welcome,
};
#[function_component]
fn App() -> Html {
    let theme = DARK.clone();
    let mut style = String::from("flex flex-col font-sans text-slate-900 bg-slate-200 min-h-screen ");
    style.push_str(&theme.colours);
    html! {
        <div class={classes!(String::from(style))}>
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