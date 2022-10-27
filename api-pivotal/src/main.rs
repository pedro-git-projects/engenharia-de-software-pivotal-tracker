use yew::prelude::*;

#[derive(Clone, PartialEq)]
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
    let pictures = vec![
        CatPicture {
            id: "8ji".to_string(),
            url: "https://cdn2.thecatapi.com/images/8ji.jpg".to_string(),
            width: 500,
            height: 334,
        },
        CatPicture {
            id: "OeOUzmQIk".to_string(),
            url: "https://cdn2.thecatapi.com/images/OeOUzmQIk.jpg".to_string(),
            width: 1080,
            height: 1080,
        },
        CatPicture {
            id: "22d".to_string(),
            url: "https://cdn2.thecatapi.com/images/22d.jpg".to_string(),
            width: 500,
            height: 313,
        },
    ];

    html! {
        <>
            <h1>{ "Trabalho Pivotal Tracker + API" }</h1>
            <div>
            <h3>{ "Recarregue para ver uma nova imagem:" }</h3>
            <CatPictureList pictures={pictures}/>
            </div>
            </>
    }
}

fn main() {
    yew::start_app::<App>();
}
