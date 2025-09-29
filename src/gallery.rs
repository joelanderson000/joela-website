use yew::prelude::*;
use yew::Properties;
use crate::{gallery, images};

#[function_component(ImageEntry)]
fn image_list(images::ImageListProps { images }: &images::ImageListProps) -> Html {
    images.iter().map(|image| html! {
        <li><img src={image.src.clone()} loading={image.loading.clone()}/></li>
    }).collect()
}
#[function_component(Gallery)]
pub fn gallery() -> Html {
    html! {
        <>
            <section id="gallery">
                <div class={classes!("container", "gallery__container")}>
                    <GalleryModal />
                    <h2>{"Gallery"}</h2>
                    <gallery::ImageList />
                </div>
            </section>
        </>
    }
}

#[function_component(GalleryModal)]
pub fn gallery_modal() -> Html {
    html! {
        <div id="gallery-modal" class={classes!("gallery-modal")}>
            <img class={classes!("gallery-modal-content")} id="gallery-modal-img" />
        </div>
    }
}
#[function_component(ImageList)]
pub fn image_list() -> Html {
    let images = images::get_images();
    html! {
        <>
            <ul class={classes!("image-gallery")}>
                <ImageEntry images={images} />
            </ul>
        </>
    }
}