use std::sync::mpsc;

pub enum ThreadMessageType {
    // iterate through command list and
    // call the relevant function accordingly
    CommandExternal(String),
}

pub struct ThreadCommunicator {
    pub receiver: mpsc::Receiver<ThreadMessageType>,
}

impl ThreadCommunicator {
    pub fn new(receiver: mpsc::Receiver<ThreadMessageType>) -> ThreadCommunicator {
        ThreadCommunicator { receiver }
    }

    pub fn listen(&self) {
        loop {
            match self.receiver.recv() {
                Ok(message) => match message {
                    ThreadMessageType::CommandExternal(command) => {
                        println!("Received command: {}", command);
                    }
                },
                Err(_) => {
                    println!("Error receiving message");
                }
            }
        }
    }
}
