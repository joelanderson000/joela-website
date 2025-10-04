use yew::prelude::*;

use crate::components::{
    links::Socials,
    links::NavLinks,
    links::ContactLink
};

#[function_component]
pub fn Header() -> Html {
    html! {
            <div class={classes!("header__container")}>
                <Nav />
                <header>
                    <h1>{"Joel Anderson"}</h1>
                </header>
                <NavSocials />
                <Hamburger />
                <button class={classes!("nav__toggle-btn")} id="nav__toggle-open"><i class={classes!("uil", "uil-bars")}></i></button>
                <button class={classes!("nav__toggle-btn")} id="nav__toggle-close"><i class={classes!("uil", "uil-multiply")}></i></button>
            </div>
    }
}

#[function_component]
pub fn Nav() -> Html {
    html! {
                <nav>
                    <ul class={classes!("nav__links")}>
                        <NavLinks />
                    </ul>
                </nav>
    }
}


#[function_component]
pub fn NavSocials() -> Html {
        html! {
                <div class={classes!("nav__socials")}>
                    <ul class={classes!("nav__links")}>
                        <Socials />
                    </ul>
                </div>
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
        <ul class={classes!("footer__links")}>
            <NavLinks />
            <ContactLink />
        </ul>
    }
}

#[function_component]
fn Copyright() -> Html {
    html! {
            <div class={classes!("footer__copyright")}>
                <h3>{"© Joel Anderson 2025"}</h3>
            </div>
    }
}
#[function_component]
fn FooterTitle() -> Html {
    html! {
            <div class={classes!("footer__head")}>
                <h2 class="footer__title">{"More to Come!"}</h2>
            </div>
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
