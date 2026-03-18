use kalosm::language::*;

#[allow(dead_code)]
pub struct ChatbotV2 {
    // What should you store inside your Chatbot type?
    // The model? The chat_session?
    model: Llama,
    chat: Option<Chat<Llama>>, 
    
}

impl ChatbotV2 {
    #[allow(dead_code)]
    pub fn new(model: Llama) -> ChatbotV2 {
        return ChatbotV2 {
            model:model,
            chat: None,
        };
    }

    #[allow(dead_code)]
    pub async fn chat_with_user(&mut self, message: String) -> String {
        if self.chat.is_none() { // if there is no chat history make it
            self.chat = Some(self.model.chat().with_system_prompt("The assistant will act like a pirate"));
        }
            
        let mut chat_session = self.chat.clone().unwrap(); // if there is a chat history clone it 
        
        let mut async_output = chat_session.add_message(message.clone()); // send message to the LLM
       
        async_output.to_std_out().await.unwrap(); // print output in the terminal
        let output = async_output.await.unwrap(); // 
        
        self.chat = Some(chat_session);
        

        return output;
    }
}