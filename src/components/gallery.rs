use yew::prelude::*;
use yew::Properties;
use crate::assets::images;

#[function_component]
fn ImageCards(images::ImageListProps { images }: &images::ImageListProps) -> Html {
    images.iter().map(|image| html! {
        <div class={classes!("flex", "aspect-square", "bg-white", "justify-center", "items-center", "rounded-lg", "shadow-md", "hover:shadow-xl", "transition-shadow", "duration-300", "ease-in-out")}>
            <img src={image.src.clone()} loading={image.loading.clone()} class={classes!("object-scale-down", "max-h-full", "m-auto", "p-4")}/>
        </div>
    }).collect()
}

#[function_component]
pub fn Gallery() -> Html {
    html! {
        <section id="gallery" class={classes!(String::from("break-after"))}>
            <div class={classes!(String::from("max-w-fit content-center p-14"))}>
                <GalleryModal />
                <h2>{"Gallery"}</h2>
                <div class={classes!("flex", "flex-grow")}>
                    <ImageList />
                </div>
            </div>
        </section>
    }
}

#[function_component]
pub fn GalleryModal() -> Html {
    html! {
        <div id="gallery-modal" class={classes!("gallery-modal")}>
            <img class={classes!("gallery-modal-content")} id="gallery-modal-img" />
        </div>
    }
}
#[function_component]
pub fn ImageList() -> Html {
    let images = images::get_images();
    html! {
        <div class={classes!("grid", "grid-cols-1", "lg:grid-cols-3", "md:grid-cols-2", "gap-4")}>
            <ImageCards images={images} />
        </div>
    }
}