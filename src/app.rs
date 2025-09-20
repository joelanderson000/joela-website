use yew::prelude::*;

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

#[function_component(Footer)]
pub fn footer() -> Html {
    html! {
        <>
            <footer>
                <div class="footer__container">
                    <div class="footer__head">
                        <h2 class="footer__title">{"More to Come!"}</h2>
                    </div>
                    <ul class="footer__links">
                        <li><a href="#">{"Home"}</a></li>
                        <li><a href="#about">{"About"}</a></li>
                        <li><a href="#gallery">{"Gallery"}</a></li>
                        <li><a href="mailto:saskaperture@gmail.com">{"Contact Me"}</a></li>
                    </ul>
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
                    <div class="footer__copyright">
                        <h3>{"© Joel Anderson 2024"}</h3>
                    </div>
                </div>
            </footer>
        </>
    }
}
