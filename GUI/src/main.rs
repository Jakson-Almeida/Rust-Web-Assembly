use sycamore::prelude::*;

fn main() {
    sycamore::render(|| view! 
        {
            div (style="background-color: #f8f9fa; padding: 10px; text-align: center;") 
            {
                h1 { "Welcome to My WebAssembly App!" }
            }
            div (style="font-family: Arial, sans-serif;") 
            {
                main (style="padding: 15px;")
                {
                    h2 { "Discover WebAssembly with Rust" }
                    p { "This is a simple app to demonstrate how Rust can interact with web technologies." }
                    div(style="height: 330px;") 
                    {
                        div(style="position: relative; width: 100%; margin-bottom: 50px;") 
                        {
                            img(src="https://i.makeagif.com/media/9-02-2016/ij0LMG.gif", style="width: 100%; height: 300px; position: absolute; left: 0; top: 0; z-index: 1;")  
                            img(src="https://rustwasm.github.io/wasm-pack/public/img/wasm-ferris.png", style="width: 30%; height: auto; position: absolute; left: 50%; top: 50%; transform: translate(-50%, +10%); z-index: 2;")      
                        }
                    }
                }
            }
            div (style="background-color: #f8f9fa; padding: 10px; text-align: center;") 
            {
                p { "Thank you for visiting!" }
            }
    });
}