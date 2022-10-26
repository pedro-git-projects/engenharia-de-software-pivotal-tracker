use yew::prelude::*;

#[function_component(App)]
fn app() -> Html {
    html! {
    <>
        <h1>{ "Trabalho Pivotal Tracker + API" }</h1>
       <div>
            <h3>{ "Recarregue para ver uma nova imagem:" }</h3>
            <img src="https://picsum.photos/200/300" alt="Lorem Picsum" />
        </div>
    </>
    }
}

fn main() {
    yew::start_app::<App>();
}
