use bytes::{Buf, BytesMut};
use serde::{Deserialize, Serialize};
use std::str;

#[derive(Debug, Serialize, Deserialize)]
struct Header {
    version: u8,
    message_id: u64,
}

#[derive(Debug, Serialize, Deserialize)]
struct Metadata {
    file_name: String,
    chunk_count: u32,
}

fn main() {
    // Example binary data with header, metadata, and file chunk
    let header = Header {
        version: 1,
        message_id: 1234567890,
    };
    let metadata = Metadata {
        file_name: "test_file.txt".to_string(),
        chunk_count: 3,
    };

    // 将header和metadata转换为JSON字符串
    let header_str = serde_json::to_string(&header).unwrap();
    let metadata_str = serde_json::to_string(&metadata).unwrap();


    let file_chunk = b"This is a tests file content.";

    // 合并所有部分，并在每部分之间添加'|'作为分隔符
    let mut binary_data = Vec::new();
    binary_data.extend_from_slice(header_str.as_bytes());
    binary_data.push(b'|');
    binary_data.extend_from_slice(metadata_str.as_bytes());
    binary_data.push(b'|');
    binary_data.extend_from_slice(file_chunk);


    // Split the binary data based on '|'
    let parts: Vec<&[u8]> = binary_data.split(|&b| b == b'|').collect();

    // Convert header bytes to string
    let header_bytes = parts[0];
    let header_str = str::from_utf8(header_bytes).expect("Failed to convert header bytes to string");
    let header: Header = serde_json::from_str(header_str).expect("Failed to deserialize header");

    // Convert metadata bytes to string
    let metadata_bytes = parts[1];
    let metadata_str =
        str::from_utf8(metadata_bytes).expect("Failed to convert metadata bytes to string");
    let metadata: Metadata =
        serde_json::from_str(metadata_str).expect("Failed to deserialize metadata");

    // Collect file chunk bytes into BytesMut
    let mut file_chunk = BytesMut::new();
    for i in 2..parts.len() {
        file_chunk.extend_from_slice(parts[i]);
    }

    // Example: Print the deserialized header, metadata, and file chunk
    println!("Header: {:?}", header);
    println!("Metadata: {:?}", metadata);

    let file_chunk_str = str::from_utf8(&file_chunk).expect("Failed to convert file chunk bytes to string");
    println!("File Chunk: {:?}", file_chunk_str);
}