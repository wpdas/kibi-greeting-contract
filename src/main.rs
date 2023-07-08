use std::{env};
mod kibi_lib;
use kibi_lib::types::{KibContract};
use serde::{Serialize, Deserialize};


// Contract
#[derive(Debug, Serialize, Deserialize)]
struct GreetingContract {
    greeting: String,
}

impl GreetingContract {
    async fn new(contract_id: String) -> Self {
        let contract_stored_content = KibContract::load(contract_id).await;
        
        // Return stored contract's content
        if contract_stored_content.is_some() {
            let stored_content = contract_stored_content.unwrap();
            let stored_content_json: GreetingContract = serde_json::from_str(&stored_content).unwrap();
            return stored_content_json;
        }

        // or, brand new one
        Self {
            greeting: "Default".to_string()
        }
    }

    pub fn set_greeting(&mut self, greeting: String) {
        self.greeting = greeting;
    }
}

#[tokio::main]
async fn main () {
    let args: Vec<String> = env::args().collect();

    if args.len() < 4 {
        println!("You must provide the params");
        return
    }

    // Contract flow
    let no_value = "".to_string();
    let action = args.get(1).unwrap_or(&no_value); // call
    let contract_id = args.get(2).unwrap_or(&no_value);
    let method = args.get(3).unwrap_or(&no_value); // contract's method
    let method_params = args.get(4).unwrap_or(&no_value); // method's params JSON

    let mut greeting_contract = GreetingContract::new(contract_id.to_owned()).await;

    // Methods
    // Callers
    if action.to_owned() == "call".to_string() {
        if method.to_owned() == "set_greeting".to_string() {
            greeting_contract.set_greeting(method_params.to_owned());
        }

        KibContract::persist(contract_id.to_owned(), &greeting_contract).await
    }

    if action.to_owned() == "view".to_string() {
        if method.to_owned() == "greeting".to_string() {
            println!("{:?}", greeting_contract.greeting);
        }
    }
}