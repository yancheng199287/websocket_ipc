use parking_lot::RwLock;
use std::collections::HashMap;
use std::sync::Arc;

// 假设您的消息片段能转换为这个结构体
#[derive(Debug, Clone)]
struct MessageChunk {
    app_id: u64,
    task_id: u64,
    sequence: u64, // 消息序列号
    data: String,  // 片段的数据
    is_last: bool, // 是否是最后一块数据
}

// 用来存储和合并消息的结构体
#[derive(Default)]
struct MessageAccumulator {
    messages: HashMap<u64, HashMap<u64, String>>, // 每个appid包含一个任务ID到消息的映射
}

// 此结构是线程安全的消息解码器，包含应用程序id和任务id
struct MessageDecoder {
    accumulator: RwLock<MessageAccumulator>,
}

impl MessageAccumulator {
    fn add_chunk(&mut self, chunk: MessageChunk) -> Option<String> {
        let task_map = self.messages.entry(chunk.app_id).or_insert_with(HashMap::new);
        let message = task_map.entry(chunk.task_id).or_insert_with(String::new);

        message.push_str(&chunk.data);

        if chunk.is_last {
            task_map.remove(&chunk.task_id)
        } else {
            None
        }
    }
}

impl MessageDecoder {
    pub fn new() -> Self {
        MessageDecoder {
            accumulator: RwLock::new(MessageAccumulator::default()),
        }
    }

    pub fn process_chunk(&self, chunk: MessageChunk) {
        // 获取写锁
        let mut accumulator = self.accumulator.write();

        // 添加消息片段到合适的位置，并检查是否构成了完整的消息
        if let Some(complete_message) = accumulator.add_chunk(chunk) {
            // 在这里可以进行消息的进一步处理，比如调用更多任务或者查询数据库
            self.handle_message(complete_message);
        }
        // 锁在这里自动释放
    }

    fn handle_message(&self, message: String) {
        println!("处理完整的消息: {}", message);
        // TODO: 在这里实现消息处理逻辑，比如执行任务或查询数据库
    }
}

// 在您的应用中使用MessageDecoder
fn main() {
    let decoder = MessageDecoder::new();
    let chunks = vec![
        MessageChunk { app_id: 1, task_id: 1, sequence: 1, data: "Hello ".into(), is_last: false },
        MessageChunk { app_id: 1, task_id: 1, sequence: 2, data: "World".into(), is_last: false },
        MessageChunk { app_id: 1, task_id: 1, sequence: 3, data: "!".into(), is_last: true },
        // ... 其他消息片段
    ];

    for chunk in chunks {
        decoder.process_chunk(chunk);
    }
}