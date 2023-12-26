mod model;
use yew::prelude::*;

#[function_component]
fn App() -> Html {
    let counter = use_state(|| 0);
    let onclick = {
        let counter = counter.clone();
        move |_| {
            let value = *counter + 1;
            counter.set(value);
        }
    };



    html! {
        <div class="align-center justify-center">
        <div class="justify-center align-center flex">
            <button {onclick} class="hover:bg-green-400 border">{ "+1" }</button>
            <p>{ *counter }</p>
        </div>
        <form class="justify-center align-center flex">
            <label for="username">{"Username"}</label>
            <input type="text" id="username" name="username" />
            <label for="password">{"Password"}</label>
            <input type="password" id="password" name="password" />
            <button class="hover:bg-green-400 border">{"Login"}</button>
        </form>
        <form class="justify-center align-center flex">
            <label for="res_name">{"Name of Restaurant"}</label>
            <input type="text" id="res_name" name="res_name" />
            <label for="address">{"Address"}</label>
            <input type="text" id="address" name="address" />
            <label for="email">{"Email"}</label>
            <input type="text" id="email" name="email" />
            <label for="description">{"Description"}</label>
            <input type="text" id="description" name="description" />
            <label for="rating">{"Rating"}</label>
            <input type="text" id="rating" name="rating" />
            <label for="price">{"Price"}</label>
            <input type="text" id="price" name="price" />
            <button class="hover:bg-green-400 border">{"Add Restaurant"}</button>
        </form>
        <form class="justify-center align-center flex">
            <label for="res_name">{"Name of Restaurant"}</label>
            <input type="text" id="res_name_rev" name="res_name_rev" />
            <label for="username">{"Username"}</label>
            <input type="text" id="username_rev" name="username_rev" />
            <label for="rating">{"Rating"}</label>
            <input type="text" id="rating_rev" name="rating_rev" />
            <label for="comment">{"Comment"}</label>
            <input type="text" id="comment" name="comment" />
            <button class="hover:bg-green-400 border">{"Add Review"}</button>
        </form>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
