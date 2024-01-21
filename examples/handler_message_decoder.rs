use parking_lot::RwLock;
use std::collections::HashMap;
use std::sync::Arc;

type MessageChunks = Vec<u8>;
type GlobalMessageStorage = HashMap<String, HashMap<String, Arc<RwLock<MessageState>>>>;

struct MessageState {
    chunks: MessageChunks, // Chunked data for this message
}

// Define your own message decoding function based on your protocol
fn decode_message(chunks: &[u8]) -> Result<String, std::string::FromUtf8Error> {
    // Example decode implementation+
    // 解码消息的逻辑
    String::from_utf8(chunks.to_vec())
}


fn handle_message(storage: &Arc<RwLock<GlobalMessageStorage>>, app_id: &str, task_id: &str, data: Vec<u8>) -> Option<String> {
    // 定义一个用于插入的新 MessageState， 不要在接收的消息中定义，这里应该是一个全局的固定常量
    let new_task_state = || Arc::new(RwLock::new(MessageState { chunks: vec![] }));

    // 下面代码中使用了 代码块 相当于定义了一块作用域，会自动释放对应的锁，比如这里的读锁
    let task_state_arc_option = {
        // 获取全局存储的读锁来尝试找到任务状态
        let r_storage = storage.read();

        // 如果可以获取到app_id，那么尝试继续获取task_id
        if let Some(app_storage) = r_storage.get(app_id) {
            // 获取应用级存储的读锁来尝试找到任务状态
            // 在 Rust 中，当你从一个 HashMap 获取一个值时，你得到的是一个对存储在 HashMap 中值的引用。如果你想要为之后的使用保留这个值的所有权，你需要获取该值的一个副本，而不是引用。这就是 cloned() 方法的用途：它会克隆 Option 中的值（如果存在的话）。
            // 这里的clone，实际是对map中引用值的克隆，获取副本拿到所有权，是对 &Arc<RwLock<MessageState>>的克隆，获取Arc<RwLock<MessageState>>所有权
            app_storage.get(task_id).cloned()
        } else {
            None
        }
    };

    let task_state_arc = if let Some(task_state) = task_state_arc_option {
        task_state
    } else {
        // 获取全局存储的写锁来更新或插入任务状态
        let mut w_storage = storage.write();
        // 获取全局w_storage中应用id对应的hashmap，如果不存在，会执行or_default，创建一个空的map，存在不会创建，直接返回存在的map
        let app_storage = w_storage.entry(app_id.to_string()).or_default();

        // 尝试插入一个新的任务状态
        let task_state = app_storage.entry(task_id.to_string())
            .or_insert_with(new_task_state)
            .clone();
        task_state
    };

    let mut task_state = task_state_arc.write();
    task_state.chunks.extend(data);

    // 检查消息是否完整的逻辑
    /* if is_message_complete(&task_state.chunks) {
         let msg_data = std::mem::take(&mut task_state.chunks);
         return decode_message(&msg_data).ok();
     }*/

    None
}

fn is_message_complete(chunks: &[u8]) -> bool {
    // 实现一些逻辑来检查消息是否完整
    // 例如：检查最后一个字符是否是某个特定的分隔符，或者数据已经到达了特定长度
    true // 这里仅为示例
}


pub struct MessageHandler {
    storage: Arc<RwLock<GlobalMessageStorage>>,
}

impl MessageHandler {
    pub fn new() -> Self {
        Self {
            storage: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub fn handle_incoming_data(&self, app_id: &str, task_id: &str, data: Vec<u8>) -> Option<String> {
        handle_message(&self.storage, app_id, task_id, data)
    }
}

fn print_state(storage: &Arc<RwLock<GlobalMessageStorage>>) {
    for (app_id, app_storage) in storage.read().iter() {
        for (task_id, task_state) in app_storage.iter() {
            let msg = task_state.read();
            let result = String::from_utf8(msg.chunks.to_vec()).unwrap();
            println!("app_id:{}   task_id: {} payload:{}", app_id, task_id,result);
        }
    }
}

fn main() {
    let handler = MessageHandler::new();

    let app_id = "app1".into();
    let task_id = "task1".into();
    handler.handle_incoming_data(app_id, task_id, b"Hello, ".to_vec());

    handler.handle_incoming_data(app_id, task_id, b"World!".to_vec());

    handler.handle_incoming_data(app_id, task_id, b", we are family!".to_vec());

    print_state(&handler.storage);
}