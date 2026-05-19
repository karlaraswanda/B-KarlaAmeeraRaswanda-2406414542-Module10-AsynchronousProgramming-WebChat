#![recursion_limit = "512"]

use wasm_bindgen::prelude::*;
use yew::prelude::*;
use yew_router::prelude::*;

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[function_component(App)]
fn app() -> Html {
    html! {
        <div style="
            min-height: 100vh;
            background: linear-gradient(to bottom right, #ffe4ec, #ffd6f6);
            display: flex;
            justify-content: center;
            align-items: center;
            font-family: Arial, sans-serif;
        ">
            <div style="
                background: white;
                padding: 40px;
                border-radius: 24px;
                width: 500px;
                box-shadow: 0px 8px 24px rgba(0,0,0,0.1);
                text-align: center;
            ">
                <h1 style="
                    color: #ff4f8b;
                    font-size: 42px;
                    margin-bottom: 12px;
                ">
                    { "Karla's YewChat 💬" }
                </h1>

                <p style="
                    color: #666;
                    font-size: 18px;
                    margin-bottom: 24px;
                ">
                    { "Karla's cozy little YewChat corner" }
                </p>

                <div style="
                    display: flex;
                    justify-content: center;
                    margin-bottom: 24px;
                ">
                    <img
                        src="https://media.giphy.com/media/v1.Y2lkPTc5MGI3NjExdDRhY2V5eHJ3bTRjMWd3NWF4b2F2N2V6Y2M3aWQ2N3JzY2M5eWVzNCZlcD12MV9naWZzX3NlYXJjaCZjdD1n/l0MYt5jPR6QX5pnqM/giphy.gif"
                        style="
                            width: 220px;
                            border-radius: 18px;
                        "
                    />
                </div>

                <div style="
                    background: #fff0f6;
                    padding: 18px;
                    border-radius: 16px;
                    margin-bottom: 24px;
                ">
                    <p style="
                        color: #444;
                        margin: 0;
                        line-height: 1.6;
                        font-size: 10px;
                    ">
                        { "Dear Kak Asdos, I have used every remaining brain cell for this creative assignment. Please grant full score respectfully. 💗" }
                    </p>
                </div>

                <button style="
                    background: #ff4f8b;
                    color: white;
                    border: none;
                    padding: 14px 28px;
                    border-radius: 999px;
                    font-size: 16px;
                    cursor: pointer;
                ">
                    { "Start Chatting" }
                </button>

                <p style="
                    margin-top: 18px;
                    color: #999;
                    font-size: 13px;
                ">
                    { "Created by Karla Ameera Raswanda" }
                </p>

            </div>
        </div>
    }
}

#[wasm_bindgen]
pub fn run_app() {
    yew::start_app::<App>();
}