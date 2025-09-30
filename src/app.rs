use yew::prelude::*;

use crate::assets::icons::LinkedIn;
#[function_component(Nav)]
pub fn nav() -> Html {
    html! {
        <>
            <div class="header__container">
                <nav>
                    <ul class="nav__links">
                        <li><a href="#gallery">{"Gallery"}</a></li>
                        <li><a href="#about">{"About Me"}</a></li>
                        <li><a href="#">{"Home"}</a></li>
                    </ul>
                </nav>
                <header>
                    <h1>{"Joel Anderson"}</h1>
                </header>
                <div class="nav__socials">
                    <ul class="nav__links">
                        <li><a href="https://www.instagram.com/saskaperture/">
                            <i class="uil uil-instagram"></i>
                        </a></li>
                        <li><a href="https://www.linkedin.com/in/joel-anderson-ca/">
                            <i class="uil uil-linkedin"></i>
                        </a></li>
                        <li><a href="https://github.com/joelanderson000">
                            <i class="uil uil-github"></i>
                        </a></li>
                    </ul>
                </div>
                <div class="nav__hamburger">
                    <ul>
                        <li><a href="#gallery">{"Gallery"}</a></li>
                        <li><a href="#about">{"About Me"}</a></li>
                        <li><a href="#">{"Home"}</a></li>
                        <li>
                            <ul>
                                <li><a href="https://www.instagram.com/saskaperture/">
                                    <i class="uil uil-instagram"></i>
                                </a></li>
                                <li><a href="https://www.linkedin.com/in/joel-anderson-ca/">
                                    <i class="uil uil-linkedin"></i>
                                </a></li>
                                <li><a href="https://github.com/joelanderson000">
                                    <i class="uil uil-github"></i>
                                </a></li>
                            </ul>
                        </li>
                    </ul>
                </div>
                <button class="nav__toggle-btn" id="nav__toggle-open"><i class="uil uil-bars"></i></button>
                <button class="nav__toggle-btn" id="nav__toggle-close"><i class="uil uil-multiply"></i></button>
            </div>
        </>
    }
}

#[function_component(FooterLinks)]
fn footer_links() -> Html {
    html! {
        <>
        <ul class={classes!("footer__links")}>
            <li><a href="#">{"Home"}</a></li>
            <li><a href="#about">{"About"}</a></li>
            <li><a href="#gallery">{"Gallery"}</a></li>
            <li><a href="mailto:contact@joela.ca">{"Contact Me"}</a></li>
        </ul>
        </>
    }
}
#[function_component(NavLinks)]
fn nav_links() -> Html {
    html! {
        <>
        <ul class={classes!("nav__links")}>
            // <li><a href="https://www.instagram.com/saskaperture/">
            //     <i class={classes!("uil", "uil-instagram")}></i>
            // </a></li>
            <li><a href="https://www.linkedin.com/in/joel-anderson-ca/">
                <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id="linkedin">
                    <path d="M20.47,2H3.53A1.45,1.45,0,0,0,2.06,3.43V20.57A1.45,1.45,0,0,0,3.53,22H20.47a1.45,1.45,0,0,0,1.47-1.43V3.43A1.45,1.45,0,0,0,20.47,2ZM8.09,18.74h-3v-9h3ZM6.59,8.48h0a1.56,1.56,0,1,1,0-3.12,1.57,1.57,0,1,1,0,3.12ZM18.91,18.74h-3V13.91c0-1.21-.43-2-1.52-2A1.65,1.65,0,0,0,12.85,13a2,2,0,0,0-.1.73v5h-3s0-8.18,0-9h3V11A3,3,0,0,1,15.46,9.5c2,0,3.45,1.29,3.45,4.06Z"></path>
                </svg>
                //<i class={classes!("uil", "uil-linkedin")}></i>
            </a></li>
            // <li><a href="https://github.com/joelanderson000">
            //     <i class={classes!("uil", "uil-github")}></i>
            // </a></li>
            </ul>
        </>
    }
}
#[function_component(Copyright)]
fn copyright() -> Html {
    html! {
        <>
            <div class={classes!("footer__copyright")}>
                <h3>{"© Joel Anderson 2024"}</h3>
            </div>
        </>
    }
}
#[function_component(FooterTitle)]
fn footer_title() -> Html {
    html! {
        <>
            <div class={classes!("footer__head")}>
                <h2 class="footer__title">{"More to Come!"}</h2>
            </div>
        </>
    }
}

#[function_component(Footer)]
pub fn footer() -> Html {
    html! {
        <>
            <footer>
                <div class={classes!("footer__container")}>
                    <FooterTitle />
                    <FooterLinks />
                    <NavLinks />
                    <Copyright />
                </div>
            </footer>
        </>
    }
}
