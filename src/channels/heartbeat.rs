use std::cell::RefCell;
use std::io::Write;
use std::rc::Rc;

use message_manager::MessageManager;
use cast::cast_channel;

const CHANNEL_NAMESPACE: &'static str = "urn:x-cast:com.google.cast.tp.heartbeat";

const MESSAGE_TYPE_PING: &'static str = "PING";
const MESSAGE_TYPE_PONG: &'static str = "PONG";

#[derive(Serialize, Debug)]
struct HeartBeatRequest {
    #[serde(rename="type")]
    pub typ: String,
}

#[derive(Deserialize, Debug)]
pub struct HeartbeatResponse {
    #[serde(rename="type")]
    pub typ: String,
}

pub struct HeartbeatChannel<W>
    where W: Write
{
    sender: String,
    receiver: String,
    writer: Rc<RefCell<W>>,
}

impl<W> HeartbeatChannel<W>
    where W: Write
{
    pub fn new(sender: String, receiver: String, writer: Rc<RefCell<W>>) -> HeartbeatChannel<W> {
        HeartbeatChannel {
            sender: sender,
            receiver: receiver,
            writer: writer,
        }
    }

    pub fn try_handle(&self, message: &cast_channel::CastMessage) -> Result<HeartbeatResponse, ()> {
        if message.get_namespace() != CHANNEL_NAMESPACE {
            return Err(());
        }

        Ok(MessageManager::parse_payload(message))
    }

    pub fn ping(&self) {
        let message = MessageManager::create(CHANNEL_NAMESPACE.to_owned(),
                                             self.sender.clone(),
                                             self.receiver.clone(),
                                             Some(HeartBeatRequest {
                                                 typ: MESSAGE_TYPE_PING.to_owned(),
                                             }));
        MessageManager::send(&mut *self.writer.borrow_mut(), message);
    }

    pub fn pong(&self) {
        let message = MessageManager::create(CHANNEL_NAMESPACE.to_owned(),
                                             self.sender.clone(),
                                             self.receiver.clone(),
                                             Some(HeartBeatRequest {
                                                 typ: MESSAGE_TYPE_PONG.to_owned(),
                                             }));
        MessageManager::send(&mut *self.writer.borrow_mut(), message);
    }
}
