use yew::prelude::*;
use serde::Deserialize;
#[derive(Properties, PartialEq, Clone, Debug,Deserialize )]
pub struct LongText  {
   pub difficulty : String ,
   pub body : String,
   pub source :  String 
}

impl LongText {
    pub fn new( difficulty : String , body : String, source :  String) -> LongText {
        LongText {
            difficulty,
            body,
            source 
        }
    }
}

#[derive(PartialEq,Debug)]
pub enum WordState {
    PARTIAL,
    INCORRECT,
    COMPLETE,
    EMPTY
}

pub struct LongTextWithId {

}
