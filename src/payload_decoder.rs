use serde::{Deserialize, Serialize};
use serde_json::Result;



#[derive(Serialize, Deserialize)]
enum MsgType {
    // 文本消息，主要是json数据信息
    Text,
    // 订阅消息类型，一般是需要持续的由服务端向客户端发送消息
    Subscription,
    // 脚本和函数调用，一般是一次性调用
    Script,
    // 文件流传输，传输文件使用
    Stream,
}

/// websocket中每个数据包的发送是固定
#[derive(Serialize, Deserialize)]
struct WSPayload {
    // 协议版本，用于后续版本的升级做兼容性判断，必须每次记录每次协议的升级内容，以便做兼容性处理
    msg_version: String,
    // 应用id名称
    app_id: String,
    // 消息类型，不同消息对应不同的处理方式
    msg_type: MsgType,
    // 消息id，通过 app_id+uuid+时间戳 去生成，用于每个消息的标识， 服务端识别同一种消息，客户端可以根据消息id正确处理结果回调给客户端前端函数
    msg_id: String,
    // 如果字节流的话，body就是base64, 需要进行分块传输，fastWebsocket设置最大的消息是64mb，大文件需要分块传输，分块传输从0开始，-1结束
    chunk: u8,
    // 如果字节流的话，body就是base64,否则就是普通json字符串
    body: String,
}

impl WSPayload {
    fn new() -> WSPayload {
        return WSPayload {
            app_id: "".to_string(),
            msg_type: MsgType::Text,
            msg_id: "".to_string(),
            msg_version: "".to_string(),
            chunk: 0,
            body: "".to_string(),
        };
    }
}


#[derive(Serialize, Deserialize)]
struct WSPayloadResponse {
    // 状态码 200 成功
    code: u8,
    // 错误消息，成功为空
    msg: String,
    // 响应体body
    data: WSPayload,
    // 开启debug模式，响应的时候把请求体也带上，这里序列号json字符串
}

#[derive(Serialize, Deserialize)]
struct WSPayloadResponseBody {
    // 开启debug模式，响应的时候把请求体也带上，这里序列号json字符串
    request_pay_load: String,
    // 响应结果
    body: String,
}