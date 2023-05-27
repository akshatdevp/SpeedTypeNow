mod models;
use log::info;
use models::{LongText, WordState};
use web_sys::{HtmlInputElement};
use yew::prelude::*;
// use log::info;
use wasm_bindgen::JsCast;
use serde::Deserialize;
// use yew::web_sys::HtmlTextAreaElement;
//
fn main() {
    wasm_logger::init(wasm_logger::Config::default());
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

fn get_current_time() -> f64 {
    let date = js_sys::Date::new_0();
    date.get_time()
}

fn time_difference(old_time : &f64 , new_time : &f64 ) -> f64 { 
    return new_time - old_time;
}

/// checks word against typed word.
fn validate(typed_word : &str, actual_word : &str) -> WordState {
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

            return validate(&input.value(), &current_word);
        }
        None => {
            return WordState::EMPTY;
        }
    }
}



//TDOO : refactor so that  state is set in child to avoid constant re render
#[function_component(App)]
fn app() -> Html {
    info!("refreshing app!");
    let long_text = LongText::new(String::from( "easy" ) , String::from ( "Random text here for trial" ) ,String::from("source"));
    let state = use_state(|| MyComponent::new(long_text.clone())); // looks like "or" but is lambda :
    fetch_call(state.clone());
    let input_value = "";

    let on_key_press: Callback<web_sys::KeyboardEvent> = {
        let state = state.clone();
        Callback::from(move |event| {
            // let state = state.clone();
            let current_word = state.words_list.clone().get(state.current_index as usize).unwrap().clone();
            let typed_word = get_typed_word_state(event,current_word);
            if typed_word == WordState::COMPLETE {
                let time;
                if state.current_index  + 1 == state.words_list.len() as i32{
                    time = get_current_time() 
                }
                else {
                    time = state.time.clone()
                }
                state.set(
                    MyComponent {
                        words_list: state.words_list.clone(),
                        current_index: (state.current_index+1)%state.words_list.len() as i32,
                        // word_state: WordState::EMPTY, 
                        text: state.text.clone(),
                        time : time.clone(),
                        time_elapsed_in_seconds : time_difference(&time.clone(), &get_current_time().clone())
                    }
                    )
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
            <h2> {state.words_list.clone().get(state.current_index as usize)} </h2>
            <h6>  { (state.current_index*60) as f64/state.time_elapsed_in_seconds} </h6>
            </div>
    }
}


#[derive(Deserialize)]
struct MyComponent {

    words_list : Vec<String>,
    current_index : i32,
    // word_state : WordState,
    text : LongText ,
    time : f64 ,
    time_elapsed_in_seconds : f64
}
impl MyComponent {
    pub fn new(long_text : LongText) -> Self {
        let words_list: Vec<String> = long_text.body.split_whitespace().map(|s| s.to_string()).collect();
        MyComponent {
            words_list,
            current_index : 0,
            // word_state :WordState::EMPTY,
            text : long_text,
            time : get_current_time(),
            time_elapsed_in_seconds : 0.0
        }
    }
}



fn fetch_call(state : UseStateHandle<MyComponent>){
    wasm_bindgen_futures::spawn_local(
        async move {
            let request_url = format!("http://localhost:8080/texts/random");

            // let resp = client.get(request_url)
                             // .header(key, value)
            match reqwest::get(request_url)
                            .await {
                Ok( re) => {
                        let resp = re.json::<Vec<LongText>>().await;
                        match resp {
                            Ok(r) => {
                                let cur_r = r.get(0).unwrap();
                                state.set(
                                    MyComponent::new(cur_r.clone())
                                    );
                                info!("got response from backend! {:?}",cur_r);
                                
                                ()
                            },
                            Err(e) => {
                                info!("{:?}",e);
                                ()
                            },
                        }

                },
                Err(_) => {
                    info!("response obtain");
                    ()},
            }
            ()
        })
}


/*
 * 
 */
