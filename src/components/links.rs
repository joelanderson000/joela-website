use yew::{function_component, html, Html};

use crate::assets::{icons};
struct SocialIcon {
    href: &'static str,
    icon: Html,
}
#[function_component]
pub fn Socials() -> Html {
    let socials: Vec<SocialIcon> = vec![
        SocialIcon {
            href: "https://www.linkedin.com/in/joel-anderson-ca/",
            icon: html! { <icons::Linkedin /> },
        },
        SocialIcon {
            href: "https://github.com/joelanderson000",
            icon: html! { <icons::Github /> },
        },
        SocialIcon {
            href: "https://www.instagram.com/saskaperture/",
            icon: html! { <icons::Instagram /> },
        }
    ];
    html! {
                    {socials.iter().map(|social| html! {
                        <li><a href={social.href}>
                            {social.icon.clone()}
                        </a></li>
                        }).collect::<Html>()}
        }

}

#[function_component]
pub fn NavLinks() -> Html {
    html! {
        <>
            <li><a href="#gallery">{"Gallery"}</a></li>
            <li><a href="#about">{"About Me"}</a></li>
            <li><a href="#">{"Home"}</a></li>
        </>
    }
}



#[function_component]
pub fn ContactLink() -> Html {
    html! {
        <>
            <li><a href="mailto:contact@joela.ca">{"Contact Me"}</a></li>
        </>
    }
}