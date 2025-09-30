use yew::prelude::*;

#[function_component(Welcome)]
pub fn welcome() -> Html {
    html! {
        <>
            <section id="welcome">
                <div class="container welcome__container">
                    <div class="welcome__text">
                        <h2>{"Creativity and Technology"}</h2>
                        <p class="paragraph"></p>
                    </div>
                </div>
            </section>
        </>
    }
}