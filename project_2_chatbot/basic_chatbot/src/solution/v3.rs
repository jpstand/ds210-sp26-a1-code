use kalosm::language::*;
use std::collections::HashMap;

#[allow(dead_code)]
pub struct ChatbotV3 {
    model: Llama,
    session: HashMap<String, Chat<Llama>>, // containing username and chat history 
}

impl ChatbotV3 {
    #[allow(dead_code)]
    pub fn new(model: Llama) -> ChatbotV3 {
        return ChatbotV3 {
            model: model,
            session: HashMap::new(), // creates history 
        };
    }

    #[allow(dead_code)]
    pub async fn chat_with_user(&mut self, username: String, message: String) -> String {
        // if username doesnt exist create a new session.
        if !self.session.contains_key(&username){ 
            self.session.insert(username.clone(), self.model.chat().with_system_prompt("The assistant will act like a pirate"));// The assistant will act like a pirate
        } 
        // retreive chat history. 
        let chat_session = self.session.get_mut(&username).unwrap(); 
        
        let mut async_output = chat_session.add_message(message.clone()); // send message to the LLM
        
        //printing to terminal
        println!("{username}: {message}"); // print what the user said
        print!("LLM: "); 
        async_output.to_std_out().await.unwrap(); // print output in the terminal
        return async_output.await.unwrap();
    }

    #[allow(dead_code)]
    pub fn get_history(&self, username: String) -> Vec<String> {
        if self.session.contains_key(&username){ 
            // retreive chat history. 
            let chat_session  = self.session.get(&username).unwrap().session().unwrap().history();
            let mut temp: Vec<String> = Vec::new(); // create a string of the content
            
            for i in chat_session{
                
                temp.push(i.content().to_string()); // pushing the history of the Vec
            }
            temp.remove(0); // removed first index, first index caused misalignment when reopened.
            println!("{:?}", temp);
            return temp; // return the history
        
        
        // Extract the chat message history for the given username
        // Hint: think of how you can retrieve the Chat object for that user, when you retrieve it
        // you may want to use https://docs.rs/kalosm/0.4.0/kalosm/language/struct.Chat.html#method.session
        // to then retrieve the history!
        }
        return Vec::new();
    }
}
