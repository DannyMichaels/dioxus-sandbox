use dioxus::prelude::*;

// #[derive(PartialEq, Props)]
// struct ButtonProps { title: String }

pub fn render(cx: Scope) -> Element {
  cx.render(rsx!(
    style { [include_str!("./button.css")] }

    button {
      class: "button",
      "click me"
    }
  ))
}
