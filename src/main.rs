mod about;
mod gallery;
mod welcome;
mod app;
mod images;

use yew::prelude::*;

#[function_component(App)]
fn app() -> Html {
    html! {
        <>
            <app::Nav />
            <welcome::Welcome />
            <about::About />
            <gallery::Gallery />
            <app::Footer />
        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}