use yew::prelude::*;

#[function_component(App)]
fn app() -> Html {
    html! {
    <>
        <h1>{ "Trabalho Pivotal Tracker + API" }</h1>
        <div>
            <h3>{"Lorem Picsum:"}</h3>
            <p>{ "Descrição da imagem aleatória 1" }</p>
            <p>{ "Descrição da imagem aleatória 2" }</p>
            <p>{ "Descrição da imagem aleatória 3" }</p>
            <p>{ "Descrição da imagem aleatória 4" }</p>
        </div>
        <div>
            <h3>{ "Imagem atual" }</h3>
            <img src="https://picsum.photos/200/300" alt="Lorem Picsum" />
        </div>
    </>
    }
}

fn main() {
    yew::start_app::<App>();
}
