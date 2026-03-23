use kalosm::language::*;
use crate::solution::file_library;

pub struct ChatbotV4 {
    model: Llama,
}

impl ChatbotV4 {
    pub fn new(model: Llama) -> ChatbotV4 {
        return ChatbotV4 {
            model: model,
        };
    }

    pub async fn chat_with_user(&mut self, username: String, message: String) -> String {
        let filename = &format!("{}.txt", username);
        let loaded_session = file_library::load_chat_session_from_file(filename);
        match loaded_session { // checks if loaded session exists
            Some(existing_session) => { 
                let mut chat_session = self.model
                .chat()
                .with_system_prompt("respond with 5 words max") //"The assistant will act like a pirate"
                .with_session(existing_session); //if it does exist then we use existing session
                println!("{message}");
                let mut async_output = chat_session.add_message(message); // send message to the LLM
                let _stream = async_output.to_std_out().await.unwrap(); // print output in the terminal
                let output = async_output.await;
                let session_for_writing = chat_session.session().unwrap();
                file_library::save_chat_session_to_file(filename, &session_for_writing); // we write a file for this most recent convo (or overwrite if one already exists)
                return output.unwrap();
            },
            None => { // if it does not exist then we make a new session
                let mut chat_session: Chat<Llama> = self.model
                .chat()
                .with_system_prompt("respond with 5 words max"); //"The assistant will act like a pirate"
                println!("{message}");
                let mut async_output = chat_session.add_message(message); // send message to the LLM
                let _stream = async_output.to_std_out().await.unwrap(); // print output in the terminal
                let output = async_output.await;
                let session_for_writing = chat_session.session().unwrap();
                file_library::save_chat_session_to_file(filename, &session_for_writing); //we write a file for this most 
                return output.unwrap();
            },
        }
        // TODO: You have to implement the rest:
        // You need to load the chat session from the file using file_library::load_chat_session_from_file(...).
        // Think about what needs to happen if the function returns None vs Some(session).
        // Hint: look at https://docs.rs/kalosm/latest/kalosm/language/struct.Chat.html#method.with_session

        
    }

    pub fn get_history(&self, username: String) -> Vec<String> {
        let filename = &format!("{}.txt", username);

        match file_library::load_chat_session_from_file(&filename) {
            None => {
                return Vec::new();
            },
            Some(session) => {
            let history = session.history();
            let mut temp = Vec::new();
            for i in history{
                temp.push(i.content().to_string()); // pushing the history of the Vec
            }
            temp.remove(0); // removed first index, first index caused misalignment when reopened.             
            println!("{:?}", temp);
            return temp; // return the history
            }
        }
    }
}