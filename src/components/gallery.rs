use yew::prelude::*;
use yew::Properties;
use crate::components::gallery;
use crate::assets::images;

#[function_component]
fn ImageCards(images::ImageListProps { images }: &images::ImageListProps) -> Html {
    images.iter().map(|image| html! {
        <div class={classes!("flex", "size-80", "bg-white", "justify-center", "items-center", "rounded-lg", "shadow-md", "hover:shadow-xl", "transition-shadow", "duration-300", "ease-in-out")}>
            <img src={image.src.clone()} loading={image.loading.clone()} class={classes!("object-scale-down", "max-h-80", "p-3")}/>
        </div>
    }).collect()
}

#[function_component]
pub fn Gallery() -> Html {
    html! {
        <section id="gallery">
            <div class={classes!("max-w-fit", "content-center", "p-14")}>
                <GalleryModal />
                <h2>{"Gallery"}</h2>
                <div class={classes!("image-gallery", "flex", "flex-wrap", "gap-4")}>
                    <gallery::ImageList />
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
        <div class={classes!("image-gallery", "flex", "flex-wrap", "gap-4")}>
            <ImageCards images={images} />
        </div>
    }
}