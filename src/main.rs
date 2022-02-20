/* The use statement at the top of our app imports everything from the the prelude module.
 use-ing the prelude imports the right traits, types, and macros needed for working with Dioxus. */
use dioxus::prelude::*;
use dioxus_core::UiEvent;
use dioxus::events::FormData;

#[path = "./components/Button.rs"] mod Button; // importing button


/*  This initialization code launches a Tokio runtime on a helper thread where your code will run. 
  Then, the WebView renderer will be launched on the main-thread. Due to platform requirements, 
  the main thread is blocked by your app's event loop.
*/
fn main() {
    dioxus::desktop::launch(App);
}

/* Finally, our app. Every component in Dioxus is a function that takes in Context and Props and returns an Element. */
fn App(cx: Scope) -> Element {
  let (value, set_value) = use_state(&cx, String::new);
  cx.render(rsx!( 
    style { [include_str!("./main.css")] }
    div {
      class:  "hello",
      "{value}",
    }
    Button::render()
    input {
      "type": "text",
      value: "{value}",
      oninput: move |evt| set_value(on_input(evt))
    }
  ))
}

fn on_input(evt: UiEvent<FormData>) -> String {
  let new_value = evt.value.clone();
  println!("current value: {}", new_value);

  return new_value;
}
