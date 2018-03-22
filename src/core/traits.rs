
use std::collections::HashMap;

pub struct WSHandle;

pub trait Core {
    fn allocate_code(&mut self) -> ();
    fn set_code(&mut self, code: &str) -> ();
    fn derive_key(&mut self, purpose: &str, length: u8) -> Vec<u8>;
    fn close(&mut self) -> ();

    fn get_action(&mut self) -> Option<Action>;

    fn websocket_connection_made(&mut self, handle: WSHandle) -> ();
    fn websocket_message_received(&mut self, handle: WSHandle, message: &Vec<u8>) -> ();
    fn websocket_connection_lost(&mut self, handle: WSHandle) -> ();
}

pub enum Action {
    GotWelcome(HashMap<String, String>), // actually anything JSON-able
    GotCode(String), // must be easy to canonically encode into UTF-8 bytes
    GotUnverifiedKey(Vec<u8>),
    GotVerifier(Vec<u8>),
    GotVersions(HashMap<String, String>), // actually anything JSON-able
    GotMessage(Vec<u8>),
    GotClosed(Result),

    WebSocketOpen(WSHandle, String),
    WebSocketSendMessage(WSHandle, Vec<u8>),
    WebSocketClose(WSHandle),
}

pub enum Result {
    Happy,
    Error
}