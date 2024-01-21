use serde::{Deserialize, Serialize};
use std::str;
use bytes::BytesMut;
use crate::model::task_type::TaskType;

/// 本类型实现客户端分块传参数据流到服务端
/// 第一个消息 header+metaData+businessData
/// 后续消息 header+metaData+stream（分块）
#[derive(Debug, Serialize, Deserialize)]
pub struct Header {
    pub app_id: String,
    pub msg_id: String,
    pub session_id: String,
    pub version: u8,  // 在实际后台维护一个  数字和对应的版本名称，方便判断版本的大小处理一些兼容性问题
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Metadata {
    pub name: String,
    pub stream_type: String,
    pub stream_length: u32,
    pub chunk_total: u32,
    pub chunk_index: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BusinessData {
    /// 任务类型  函数调用function（最好用前缀区分，比如操作redis，redis.getValue(),参数 name）  脚本执行script， 消息订阅 Subscription
    task_type: TaskType,
    /// 每种任务类型不同，在这里可以定义响应的参数，这个是一个json字符串，可以根据任务类型来确定不同的结构体
    task_params: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RequestData {
    header: Header,
    metadata: Metadata,
    business_data: Option<BusinessData>,
    // 业务数据,第一次有，第二次没有，使用Option类型，防止json序列化出错，没有这个key就是None值
    #[serde(skip)]
    stream: BytesMut,
}

/// 解析第一个发来的消息体，包含业务数据，不包含流数据
pub fn parser_first_request_payload(json_data: &str) -> Result<RequestData, Box<dyn std::error::Error>> {
    let parts: RequestData = serde_json::from_str(json_data).unwrap();
    Ok(parts)
}

/// 将header部分自动解析成字符串，然后将字符串序列化为对应的struc类型
/// static 告诉编译器这个类型的反序列化不依赖于任何特定的作用域或生命周期,在反序列化时更加灵活，不需要关注具体的生命周期限制
/// 用静态生命周期并不意味着该类型在内存中永远存在，而是表示它的生命周期与整个程序的生命周期相同。如果该类型被释放或销毁，它的生命周期也会结束。
fn deserialize_from_json<T: Deserialize<'static>>(bytes: &[u8]) -> Result<T, serde_json::Error> {
    let json_str = str::from_utf8(bytes).expect("Failed to convert struct bytes to string");
    serde_json::from_str(json_str).expect("Failed to deserialize header")
}

/// 解析后续的流数据
pub fn parser_request_payload<'a>(exist_request_data: &'a mut RequestData, binary_data: &'a Vec<u8>) -> Result<&'a mut RequestData, Box<dyn std::error::Error>> {
    // 按照分隔符 |  收集所有片段部分内容， 第一个是json字符串， 第二个是流数据
    let parts: Vec<&[u8]> = binary_data.split(|&b| b == b'|').collect();
    let part_request_data: RequestData = deserialize_from_json(parts[0]).unwrap();
    println!("part_request_data: {:?}", part_request_data);
    /// 加入一些debug信息
    println!("本次分块传输的index:{}", part_request_data.metadata.chunk_index);
    /// 分块传输完毕
    if part_request_data.metadata.chunk_total == part_request_data.metadata.chunk_index {
        println!("本次分块传输完毕，本次分块传输的index:{}", part_request_data.metadata.chunk_index);
    }
    exist_request_data.header = part_request_data.header;
    exist_request_data.metadata = part_request_data.metadata;
    if let Some(business_data) = part_request_data.business_data {
        exist_request_data.business_data = Some(business_data);
    }
    /// 追加字节流
    exist_request_data.stream.extend_from_slice(parts[1]);
    // 返回最新的请求数据，合并了分块数据， 解析完成之后，判断当前分块是否结束 index==total  删除map的key
    return Ok(exist_request_data);
}


/// 后续的消息体，需要对流数据进行聚合，直到流完毕
pub fn combine_stream<'a>(request_data: &'a mut RequestData, binary_data: &'a Vec<u8>) -> Result<&'a mut RequestData, Box<dyn std::error::Error>> {
    let mut stream = request_data.stream.clone();
    stream.extend_from_slice(binary_data);
    request_data.stream = stream;
    return Ok(request_data);
}