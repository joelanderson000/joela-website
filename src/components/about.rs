use yew::prelude::*;

#[function_component(About)]
pub fn about() -> Html {
    html! {
        <>
            <section id="about">
                <div class="container aboutMe__container">
                    <div class="aboutMe_bio">
                        <h2>{"Joel Anderson – Engineer, Photographer, Technology Enthusiast"}</h2>
                        <p class="paragraph">
                            {"Welcome! I'm Joel Anderson. With a background in electrical engineering and computer science, I love the creative outlet that photography gives me."}
                        </p>
                        <p class="paragraph">
                            {"Photography for me is more than just a hobby; it's a way of seeing the world, telling stories, and connecting with people and places. My portfolio showcases a diverse range of photography styles including travel, lifestyle, street, landscape, and the occational event."}
                        </p>
                        <p class="paragraph">
                            {"Thank you for visiting. Dive in to explore my work, discover new perspectives, and maybe even find inspiration for your next project or adventure. For any questions or requests, "}
                            <a href="mailto:saskaperture@gmail.com">{"Contact Me"}</a>
                            {"!"}
                        </p>
                    </div>
                    <div class="standaloneImage">
                        <img src="images/portrait.jpg" alt="about me portrait" height="400" />
                    </div>
                </div>
            </section>
        </>
    }
}