use yew::prelude::*;

mod components;
mod assets;
mod app;

use crate::components::{
    about::About,
    gallery::Gallery,
    welcome::Welcome,
};
#[function_component(App)]
fn app() -> Html {
    html! {
        <>
            <app::Nav />
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