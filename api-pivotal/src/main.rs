use gloo_net::http::Request;
use serde::Deserialize;
use yew::prelude::*;

#[derive(Clone, PartialEq, Deserialize)]
pub struct CatPicture {
    pub id: String,
    pub url: String,
    pub width: usize,
    pub height: usize,
}

#[derive(Properties, PartialEq)]
struct CatPictureListProps {
    pictures: Vec<CatPicture>,
}

#[function_component(CatPictureList)]
fn cat_picture_list(CatPictureListProps { pictures }: &CatPictureListProps) -> Html {
    pictures
        .iter()
        .map(|picture| {
            html! {
                <img src= {format!("{}", picture.url)}/>
            }
        })
        .collect()
}

#[function_component(App)]
fn app() -> Html {
    let pictures = use_state(|| vec![]);
    {
        let pictures = pictures.clone();
        use_effect_with_deps(
            move |_| {
                let pictures = pictures.clone();
                wasm_bindgen_futures::spawn_local(async move {
                    let fetched_pictures: Vec<CatPicture> =
                        Request::get("https://api.thecatapi.com/v1/images/search")
                            .send()
                            .await
                            .unwrap()
                            .json()
                            .await
                            .unwrap();
                    pictures.set(fetched_pictures);
                });
                || ()
            },
            (),
        );
    }

    html! {
        <>
            <h1>{ "Trabalho Pivotal Tracker + API" }</h1>
            <div>
            <h3>{ "Recarregue para ver uma nova imagem:" }</h3>
            <CatPictureList pictures={(*pictures).clone()}/>
            </div>
            </>
    }
}

fn main() {
    yew::start_app::<App>();
}
