use yew::prelude::*;

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <section class="flex items-center justify-center h-screen w-screen">
            <p class="text-5xl font-light py-5 px-10 border border-gray-400 rounded-xl">
                {"👋🏻 Hi from Rust & Tailwind!"}
            </p>
        </section>
    }
}
