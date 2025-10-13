use yew::prelude::*;
use crate::assets::styles::tailwind;

// #[function_component]
// pub fn Welcome() -> Html {
//     html! {
//         <section id="welcome" class={classes!("flex", "flex-col", "items-center", "justify-center", "h-screen", "text-center", "p-4")}>
//             <h2>{"Creativity and Technology"}</h2>
//         </section>
//     }
// }

#[function_component]
pub fn Welcome() -> Html {
    html! {
        <section id="welcome" class={String::from("flex flex-col items-center justify-center h-screen text-center p-4")}>
            <h2>{"Creativity and Technology"}</h2>
        </section>
    }
}