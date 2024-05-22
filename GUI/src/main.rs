use sycamore::prelude::*;

fn main() {
    sycamore::render(|cx| view! { cx,
        div (style="background-color: #f8f9fa; padding: 10px; text-align: center;") {
            h1 { "Welcome to My WebAssembly App!" }
        }
        div (style="font-family: Arial, sans-serif;") {
            main (style="padding: 15px;") {
                h2 { "Discover WebAssembly with Rust" }
                p { "This is a simple app to demonstrate how Rust can interact with web technologies." }
                div (style="height: 800px;") {
                    div (style="position: relative; width: 100%; margin-bottom: 50px;") {
                        img(src="data/matrix.gif", style="width: 100%; position: absolute; left: 0; top: 0; z-index: 1;")
                        img(src="data/wasm-ferris.png", style="width: 60%; height: auto; position: absolute; left: 50%; top: 50%; transform: translate(-50%, +40%); z-index: 2;")
                    }
                }
            }
        }
        div (style="background-color: #f8f9fa; padding: 10px; text-align: center;") {
            p { "Thank you for visiting!" }
        }
    });
}
