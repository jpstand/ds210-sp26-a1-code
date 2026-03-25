use kalosm::language::*;
use file_chatbot::solution::file_library::{self, save_chat_session_to_file};
use rocket::response;

use crate::solution::Cache;

pub struct ChatbotV5 {
    model: Llama,
    cache: Cache<Chat<Llama>>,
}

impl ChatbotV5 {
    pub fn new(model: Llama) -> ChatbotV5 {
        return ChatbotV5 {
            model: model,
            cache: Cache::new(3),
        };
    }

    pub async fn chat_with_user(&mut self, username: String, message: String) -> String {
        let filename = &format!("{}.txt", username);

        let cached_chat = self.cache.get_chat(&username);

        match cached_chat {
            None => {
                println!("chat_with_user: {username} is not in the cache!");
                let file = file_library::load_chat_session_from_file(filename); // load chat from file if not in cache
                let mut chat_session: Chat<Llama> = self.model.chat(); //variable used by both cases
                match file { // checks if the file exists
                    None => { // if it does not exist then we make a new file 
                        let mut new_session = chat_session.with_system_prompt("The assistant will act like a pirate"); //"The assistant will act like a pirate"
                        let mut async_output = new_session.add_message(message.clone()); // send message to the LLM
                        println!("{username}: {message}"); // print user message to terminal
                        async_output.to_std_out().await.unwrap(); // print output in the terminal
                        let output = async_output.await.unwrap();
                        let session_for_writing = new_session.session().unwrap();
                        file_library::save_chat_session_to_file(filename, &session_for_writing); // we write a file  
                        self.cache.insert_chat(username, self.model.chat());// cache the chat session
                        return output;
                    },
                    Some(existing_session) => { 
                        chat_session = chat_session.with_session(existing_session); //if it does exist then we use existing session
                        let mut async_output = chat_session.add_message(message.clone()); // send message to the LLM
                        println!("{username}: {message}"); // print user message to terminal
                        async_output.to_std_out().await.unwrap(); // print output in the terminal
                        let output = async_output.await.unwrap();
                        let session_for_writing = chat_session.session().unwrap();
                        file_library::save_chat_session_to_file(filename, &session_for_writing); // we write a file for this most recent convo (or overwrite if one already exists)
                        self.cache.insert_chat(username, self.model.chat());// cache the chat session
                        return output;
                    },
                }
                // The cache does not have the chat. What should you do?
            }
            Some(chat_session) => {
                println!("chat_with_user: {username} is in the cache! Nice!");
                
                let mut response = chat_session.add_message(message); // add message to end of chain of chat and send to bot to get a response.
                response.to_std_out().await.unwrap();// print to terminal
                
                let output = response.await.unwrap();
                let temp  = chat_session.session();
                let session_for_writing= temp.as_ref().unwrap();

                file_library::save_chat_session_to_file(filename, session_for_writing); // save the history to file

                //no need to insert_chat here because we are directly updating the cache. 
                return output; //return response
            }
        }
    }

    pub fn get_history(&mut self, username: String) -> Vec<String> {
        let filename = &format!("{}.txt", username);
        let cached_chat = self.cache.get_chat(&username);

        match cached_chat {
            None => {
                println!("get_history: {username} is not in the cache!");
                // TODO: The cache does not have the chat. What should you do?
                match file_library::load_chat_session_from_file(filename) {
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
            Some(chat_session) => {
                println!("get_history: {username} is in the cache! Nice!");
                // TODO: The cache has this chat. What should you do?
                let user_session = chat_session.session().unwrap(); // get the session 
                let history = user_session.history(); // get the sesion history
                let mut temp = Vec::new();
                for i in &history{
                    temp.push(i.content().to_string()); // pushing the history of the Vec
                }
                temp.remove(0); // removed first index, first index caused misalignment when reopened.             
                println!("{:?}", temp);
                    
                return temp; // return the history

            }
        }
    }
}