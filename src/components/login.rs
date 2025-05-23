use web_sys::{HtmlInputElement, InputEvent};
use yew::functional::*;
use yew::prelude::*;
use yew_router::prelude::*;

use crate::Route;
use crate::User;

#[function_component(Login)]
pub fn login() -> Html {
    let username = use_state(|| String::new());
    let user = use_context::<User>().expect("No context found.");

    let oninput = {
        let current_username = username.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            current_username.set(input.value());
        })
    };

    let onclick = {
        let username = username.clone();
        let user = user.clone();
        Callback::from(move |_| *user.username.borrow_mut() = (*username).clone())
    };

    html! {
      <div class="bg-gradient-to-r from-purple-600 via-pink-500 to-red-500 flex w-screen h-screen items-center justify-center">
        <div class="bg-white bg-opacity-20 backdrop-blur-md p-8 rounded-xl shadow-lg">
          <h1 class="text-3xl font-bold text-white mb-6">{"Welcome to YewChat!"}</h1>
          <form class="m-4 flex">
            <input {oninput}
                   class="bg-white rounded-l-lg p-4 border-t mr-0 border-b border-l text-gray-800 border-gray-200 outline-none focus:ring-2 focus:ring-purple-400 transition"
                   placeholder="Username" />
            <Link<Route> to={Route::Chat}>
                <button {onclick}
                        disabled={username.len() < 1}
                        class="
                            px-8 rounded-r-lg bg-violet-600 text-white font-bold p-4 uppercase
                            transition duration-300 ease-in-out transform
                            hover:bg-violet-700 hover:shadow-lg hover:-translate-y-1
                            disabled:opacity-50
                        ">
                    {"Go Chatting!"}
                </button>
            </Link<Route>>
          </form>
        </div>
      </div>
    }
}