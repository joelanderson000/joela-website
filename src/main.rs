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
        <>
            <app::Header />
            // <app::Nav />
            <Welcome />
            <About />
            <Gallery />
            <app::Footer />
        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}