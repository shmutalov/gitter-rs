pub trait MessageHandler {
    fn on_connected(&self) {}
    // fn on_connect_timeout(&self);
    fn on_disconnected(&self) {}

    fn on_error(&self) {}

    fn on_handshake(&self) {}
    // pub fn on_publish();
    fn on_subscribe(&self) {}
    fn on_unsubscribe(&self) {}
    // fn on_connect(&self);

    fn on_message(&self) {}

    fn process_message<S>(&self, data: S)
    where
        S: AsRef<str>,
    {
        let data = data.as_ref();
        let channel = data.split('/');
    }
}
