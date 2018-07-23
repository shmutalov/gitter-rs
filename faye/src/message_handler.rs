use bayeux::{Message, MessageType};

pub trait MessageHandler {
    fn on_connect(&self, msg: &Message) {}
    fn on_disconnect(&self, msg: &Message) {}
    fn on_error(&self, msg: &Message) {}
    fn on_handshake(&self, msg: &Message) {}
    fn on_subscribe(&self, msg: &Message) {}
    fn on_unsubscribe(&self, msg: &Message) {}
    fn on_message(&self, msg: &Message) {}

    fn process_message(&self, msg: &Message) {
        let is_meta = msg.is_meta();

        if is_meta {
            match msg.get_type() {
                MessageType::Handshake => self.on_handshake(msg),
                MessageType::Connect => self.on_connect(msg),
                MessageType::Disconnect => self.on_connect(msg),
                MessageType::Subscribe => self.on_subscribe(msg),
                MessageType::Unsubscribe => self.on_unsubscribe(msg),
                _ => self.on_error(&msg),
            }

            return;
        }

        self.on_message(msg);
    }
}

pub trait FayeHandler {
    fn on_connect(&self);
    fn on_disconnect(&self);
    fn on_message(&self);
    fn on_error(&self);
}
