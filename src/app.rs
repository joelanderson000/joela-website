use yew::prelude::*;

use crate::assets::styles::tailwind;

use crate::components::{
    links::Socials,
    links::NavLinks,
    links::ContactLink
};

#[function_component]
pub fn Header() -> Html {
    html! {
            <div class={classes!(String::from("flex flex-row justify-between top-0 bg-slate-300 sticky shadow-md h-12 max-h-12"))}>
                <Nav />
                <header>
                    <h1 class={classes!("flex", "flex-row", "gap-4", "justify-center", "p-2")}>{"Joel Anderson"}</h1>
                </header>
                <NavSocials />
                // <Hamburger />
                <button class={classes!("nav__toggle-btn")} id="nav__toggle-open"><i class={classes!("uil", "uil-bars")}></i></button>
                <button class={classes!("nav__toggle-btn")} id="nav__toggle-close"><i class={classes!("uil", "uil-multiply")}></i></button>
            </div>
    }
}

#[function_component]
pub fn Nav() -> Html {
    html! {
                <nav>
                    <ul class={classes!("flex", "flex-row", "gap-4", "justify-center", "p-1")}>
                        <NavLinks />
                    </ul>
                </nav>
    }
}


#[function_component]
pub fn NavSocials() -> Html {
        html! {
            <ul class={classes!("object-scale-down", "flex", "flex-row", "gap-4", "p-1", "justify-center")}>
                <Socials />
            </ul>
        }
}




#[function_component]
pub fn Hamburger() -> Html {
    html! {
        <>
                <div class={classes!("nav__hamburger")}>
                    <ul>
                        <NavLinks />
                        <li>
                            <ul>
                                <Socials />
                            </ul>
                        </li>
                    </ul>
                </div>
        </>
    }
}

#[function_component]
fn FooterLinks() -> Html {
    html! {
        <ul class={classes!("flex",  "p-2", "justify-center", "gap-4")}>
            <NavLinks />
            <ContactLink />
        </ul>
    }
}

#[function_component]
fn Copyright() -> Html {
    html! {
            <div class={classes!("flex", "justify-center", "gap-4", "p-2")}>
                <h3>{"© Joel Anderson 2025"}</h3>
            </div>
    }
}
#[function_component]
fn FooterTitle() -> Html {
    html! {
        <h2 class={classes!("flex", "justify-center", "gap-4")}>{"More to Come!"}</h2>
    }
}

#[function_component]
pub fn Footer() -> Html {
    html! {
            <footer>
                <div class={classes!("footer__container")}>
                    <FooterTitle />
                    <FooterLinks />
                    <NavSocials />
                    <Copyright />
                </div>
            </footer>
    }
}
