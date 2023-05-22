use diesel::prelude::*;
use serde::{Serialize,Deserialize};
use crate::schema::long_text;

#[derive(Queryable,Serialize)] 
pub struct LongText  {
   pub id : i32,
   pub difficulty : String ,
   pub body : String,
   pub source :  String 
}

// insertable is kept separate so id can be auto-inserted.
#[derive(Serialize,Insertable,Deserialize)] 
#[diesel(table_name = long_text)]
pub struct LongTextInsertor  {
   pub difficulty : String ,
   pub body : String,
   pub source :  String 
}


