
use std::sync::Arc;


pub fn get_secret_string(filename: &str) -> Arc<String> { 
    //arc is just a smart pointer that doesnt drop out of scope while any thread is still accessing it
    // eventually make this read from local file on runtime
    // let secret_string = Arc::new(Mutex::new(String::from("coconut"))); 
    // don't need mutex here because Read Only string, just need smart pointer across threads
    Arc::new(String::from("coconut"))
}
