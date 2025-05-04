mod comp;

use comp::App;

fn main() {
    yew::Renderer::<App>::new().render();
}