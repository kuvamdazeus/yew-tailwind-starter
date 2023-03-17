use yew::prelude::*;

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <section class="flex items-center justify-center h-screen w-screen">
            <p class="text-5xl font-bold">{"👋🏻 Hi from Tailwind!"}</p>
        </section>
    }
}
