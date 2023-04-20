use yew::prelude::*;

struct Model {
    value: i64,
    input: String
}

#[function_component(App)]
fn app() -> Html {
    let state = use_state(|| Model {
        value: 0,
        input: String::from("hello")
    });

    let increment = {
        let state = state.clone();

        Callback::from(move |_| {
            state.set(Model {
                value: state.value + 1,
                input: state.input.clone()
            })
        })
    };

    let decrement = {
        let state = state.clone();
        Callback::from(move |_| {
            state.set(Model {
                value: state.value - 1,
                input: state.input.clone()
            })
        })
    };
    let onchange = {
        let state = state.clone();
        Callback::from(move |_| {
            state.set(Model {
                value: state.value,
                input: String::from("changed")
            })
        })
    };

    html! {
        <div>
            <div>
                <button onclick={onchange}>{"change"}</button>
                <p>{ state.input.clone() }</p>
            </div>
            <div>
                <div>{"Increment / Decrement"}</div>
                <button onclick={increment}>
                    {"+1"}
                </button>
                <p>{ state.value }</p>
                <button onclick={decrement}>
                    {"-1"}
                </button>
            </div>
        </div>
    }
}


fn main() {
    yew::start_app::<App>();
}
