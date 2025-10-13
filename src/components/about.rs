use yew::prelude::*;

#[function_component]
pub fn About() -> Html {
    html! {
            <section id="about">
                <div class={classes!("flex-row", "gap-8", "justify-center", "align-center", "items-center", "p-14")}>
                    <AboutBio />
                    <AboutImage />
                </div>
            </section>
    }
}

#[function_component]
pub fn AboutBio() -> Html {
    html! {
        <div class={classes!("max-w-2xl", "mx-auto", "flex", "flex-col", "items-center")}>
            <h2>{"Joel Anderson – Engineer, Photographer, Technology Enthusiast"}</h2>
                <p class={classes!("paragraph")}>
                    {"Welcome! I'm Joel Anderson. With a background in electrical engineering and computer science, I love the creative outlet that photography gives me."}
                </p>
                <p class={classes!("paragraph")}>
                    {"Photography for me is more than just a hobby; it's a way of seeing the world, telling stories, and connecting with people and places. My portfolio showcases a diverse range of photography styles including travel, lifestyle, street, landscape, and the occational event."}
                </p>
                <p class={classes!("paragraph")}>
                    {"Thank you for visiting. Dive in to explore my work, discover new perspectives, and maybe even find inspiration for your next project or adventure. For any questions or requests, "}
                    <a href="mailto:contact@joela.ca">{"Contact Me"}</a>
                    {"!"}
                </p>
        </div>
    }
}

#[function_component]
pub fn AboutImage() -> Html {
    html! {
        <div class={classes!("items-center")}>
            <img src="images/portrait.jpg" alt="about me portrait" height="400" />
        </div>
    }
}