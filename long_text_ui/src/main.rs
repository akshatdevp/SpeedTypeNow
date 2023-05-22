mod models;
use log::info;
use models::{LongText, WordState};
use web_sys::HtmlInputElement;
use yew::prelude::*;
// use log::info;
use wasm_bindgen::JsCast;
// use yew::web_sys::HtmlTextAreaElement;
fn main() {
    yew::Renderer::<App>::new().render();
}


#[function_component]
fn TextArea(text : &LongText) -> Html { 
    html! {
        <div class = "main_text_area"> 
        <h2>
        {&text.body}
        </h2>
        </div>
    }
    
}

/// checks word against typed word.
fn validate(typed_word : &str, actual_word : &str) -> WordState {
    info!("{:?}",typed_word);
    info!("{:?}",actual_word);
    if typed_word == actual_word {
        return WordState::COMPLETE;
    }
    
    if actual_word.contains(typed_word) {
        return WordState::PARTIAL;
    }
    WordState::INCORRECT
}

fn get_typed_word_state(e : KeyboardEvent, current_word : String) -> WordState {
    let target =  e.target();
    // Events can bubble so this listener might catch events from child elements which are not of type HtmlInputElement
    let maybe_input = target.and_then(|t| t.dyn_into::<HtmlInputElement>().ok());
    match maybe_input {
        Some(input) => {
            // let current_word = get_current_word_from_state();
            return validate(&input.value(), &current_word);
        }
        None => {
            return WordState::EMPTY;
        }
    }
}

fn get_current_word_from_state() -> String{
    String::from("some_word") //TODO
}


#[function_component(App)]
fn app() -> Html {
    let long_text = LongText::new(String::from( "easy" ) , String::from ( "Random text here for trial" ) ,String::from("source"));
    let state = use_state(|| MyComponent::new(long_text.clone())); // looks like "or" but is lambda :
    let input_value = "";
    wasm_logger::init(wasm_logger::Config::default());

  let on_key_press: Callback<web_sys::KeyboardEvent> = {
        let state = state.clone();
        Callback::from(move |event| {
            // let state = state.clone();
            let current_word = state.words_list.clone().get(state.current_index as usize).unwrap().clone();
            let typed_word = get_typed_word_state(event,current_word);
            if typed_word == WordState::COMPLETE {
                state.set(
                        MyComponent {
                            words_list: state.words_list.clone(),
                            current_index: (state.current_index+1)%state.words_list.len() as i32,
                            word_state: WordState::EMPTY, 
                            text: state.text.clone()
                        }
                    )
                // state.set(state_clone);
            }
        })
    };

    html! {
        <div style = "background:black;"> 
        <h1>{"typing text"}</h1>
        <TextArea difficulty = { state.text.difficulty.clone() }
                  source = { state.text.source.clone() }
                  body = { state.text.body.clone() }
                  />
        <input onkeyup = {on_key_press} value = {input_value}/> 
        <h2 id="abc"> {state.words_list.clone().get(state.current_index as usize)} </h2>

        </div>
    }
}

struct MyComponent {
    
    words_list : Vec<String>,
    current_index : i32,
    word_state : WordState,
    // current_word : String,
    text : LongText 
}
impl MyComponent {
    pub fn new(long_text : LongText) -> Self {
        let words_list: Vec<String> = long_text.body.split_whitespace().map(|s| s.to_string()).collect();
         MyComponent {
            words_list,
            current_index : 0,
            word_state :WordState::EMPTY,
            text : long_text
        }
    }
}


 
