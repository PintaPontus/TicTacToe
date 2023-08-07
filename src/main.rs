use leptos::*;

use tictactoe::*;

mod tictactoe;



fn main() {
    mount_to_body(|cx| view! { cx, <App/> })
}

#[component]
fn App(cx: Scope) -> impl IntoView {

    view! { cx,
        <div class="app">
            <Board/>
        </div>
    }
}
