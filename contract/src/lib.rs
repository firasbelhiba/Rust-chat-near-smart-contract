use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::serde::Serialize;
use near_sdk::{env, near_bindgen};

#[derive(BorshDeserialize, BorshSerialize, Serialize)]
#[serde(crate = "near_sdk::serde")]
#[derive(Clone, Debug)]
pub struct Message {
    content: String,
    owner_id: String,
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Chat {
    messages: Vec<Message>,
}

impl Default for Chat {
    fn default() -> Self {
        Self {
            messages: Vec::new(),
        }
    }
}

#[near_bindgen]
impl Chat {
    pub fn get_total_messages(&self) -> usize {
        return self.messages.len();
    }

    pub fn add_message(&mut self, message_content: String) {
        let account_id = env::signer_account_id();
        let message_object = Message {
            content: message_content,
            owner_id: account_id.to_string(),
        };
        self.messages.push(message_object);
    }

    pub fn get_messages(&self) -> Vec<Message> {
        return self.messages.clone();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_total_messages() {
        // #1 step : instansiate the contract by its method "default"
        let contract = Chat::default();

        // #2 step : Assert and verify the result that we expact
        // By default total messages will be 0 so we expect to get 0 ( By default the Vector of messages is empty so the length of it is 0)
        assert_eq!(contract.get_total_messages(), 0);
    }

    #[test]
    fn add_message() {
        // #1 step : We have to create a mutable reference to our contract because we are going to update some value to the contract
        let mut contract = Chat::default();
        // #2 step : We need to execute the task which is adding a message to the vector by calling its appropriate method
        let message_content = "Hey Lightency peeps !".to_string(); // Don't forget to convert it to the string format
        contract.add_message(message_content);
        // #3 step : Now finally we need to check if the execution of the task went to success or not by calling the assert method
        assert_eq!(contract.get_total_messages(), 1); // Now by adding a new message to the vector, the length of it should be equal to 1
    }

    // Adding 2 message test
    // In case of you wondered how can we add 2 messages and test them , here is the function of it
    #[test]
    fn add_two_messages() {
        // Message 1 :
        let mut contract = Chat::default();
        let message_content = "Hey Lightency peeps !".to_string();
        contract.add_message(message_content);
        assert_eq!(contract.get_total_messages(), 1);
        // Message 2 :
        let message_content_2 = "How is it going folks ?".to_string();
        contract.add_message(message_content_2);
        assert_eq!(contract.get_total_messages(), 2);
    }
}
