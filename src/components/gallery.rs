use yew::prelude::*;
use yew::Properties;
use crate::components::gallery;
use crate::assets::images;

#[function_component]
fn ImageEntry(images::ImageListProps { images }: &images::ImageListProps) -> Html {
    images.iter().map(|image| html! {
        <li><img src={image.src.clone()} loading={image.loading.clone()}/></li>
    }).collect()
}
#[function_component]
pub fn Gallery() -> Html {
    html! {
        <section id="gallery">
            <div class={classes!("container", "gallery__container")}>
                <GalleryModal />
                <h2>{"Gallery"}</h2>
                <gallery::ImageList />
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
        <ul class={classes!("image-gallery")}>
            <ImageEntry images={images} />
        </ul>
    }
}