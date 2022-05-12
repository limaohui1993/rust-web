use actix_web::{error,http::StatusCode,HttpResponse};
use serde::Serialize;
use std::fmt;

#[allow(dead_code)]
#[derive(Debug,Serialize)]
pub enum MyError{
    ActixError(String),
    NotFound(String),
    TeraError(String),
}

#[derive(Debug,Serialize)]
pub struct MyErrorResponse{
    error_message:String,
}

impl std::error::Error for MyError{}

impl MyError{
    fn error_response(&self)->String{
        match self{
            MyError::ActixError(msg)=>{
                println!("Server error Occured:{:?}",msg);
                "Internal Server error".into()
            },
            MyError::TeraError(msg)=>{
                println!("Error in rendering the template:{:?}",msg);
                msg.into()
            },
            MyError::NotFound(msg)=>{
                println!("Not found error occurred:{:?}",msg);
                msg.into()
            }
        }
    }
}

impl fmt::Display for MyError {
    fn fmt(&self,f:&mut fmt::Formatter)->Result<(),fmt::Error>{
        write!(f,"{}",self)
    }
}

impl From<actix_web::error::Error> for MyError {
    fn from(err: actix_web::error::Error)-> Self{
        MyError::ActixError(err.to_string())
    }
}