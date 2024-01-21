const APP_ID = 'com.mini';

const MSG_TYPE = {
    Text: 'Text',
    Subscription: 'Subscription',
    Script: 'Script',
    Stream: 'Stream',
};

class WSPayload {

    constructor(app_id, msg_type, chunk, body) {
        this.app_id = app_id;
        this.msg_type = msg_type;
        this.chunk = chunk;
        this.body = body;
        this.msg_id = this.app_id + "_" + this.getUuid();
    }

    generateUUID() {
        this.msg_id = this.app_id + "_" + this.getUuid();
    }

    getUuid() {
        return Number(Math.random().toString().slice(2, 10) + Date.now()).toString(36)
    }
}

function buildTextWSPayload(body) {
    return new WSPayload(APP_ID, MSG_TYPE.Text, -1, body);
}


// 发送分块数据
function sendChunks2(metadata, data) {
    const CHUNK_SIZE = 512 * 1024; // 512KB
    const delimiter = '|'; // 分隔符
    const metadataJson = JSON.stringify(metadata);
    const delimiterBlob = new Blob([delimiter]);
    const dataBlob = (data instanceof Blob) ? data : new Blob([data]);



    dataBlob.arrayBuffer().then((arrayBuffer) => {
        for (let start = 0; start < arrayBuffer.byteLength; start += CHUNK_SIZE) {
            const fullBlob = new Blob([metadataJson, delimiterBlob, dataBlob]);
            const chunk = arrayBuffer.slice(start, Math.min(start + CHUNK_SIZE, arrayBuffer.byteLength));
            ws.send(chunk);
        }
    });
}


// 发送分块数据
function sendChunks1(metadata, data) {
    const CHUNK_SIZE = 512 * 1024; // 512KB
    const delimiter = '|'; // 分隔符
    const metadataJson = JSON.stringify(metadata);
    const delimiterBlob = new Blob([delimiter]);
    const dataBlob = (data instanceof Blob) ? data : new Blob([data]);
    const fullBlob = new Blob([metadataJson, delimiterBlob, dataBlob]);

    fullBlob.arrayBuffer().then((arrayBuffer) => {
        for (let start = 0; start < arrayBuffer.byteLength; start += CHUNK_SIZE) {
            const chunk = arrayBuffer.slice(start, Math.min(start + CHUNK_SIZE, arrayBuffer.byteLength));
            ws.send(chunk);
        }
    });
}



// 用于发送数据的函数
function sendChunks(ws, data, metadata) {
    const CHUNK_SIZE = 512 * 1024; // 512KB
    const encoder = new TextEncoder();
    const jsonMetadata = JSON.stringify(metadata);
    const delimiter = encoder.encode('|'); // 分隔符编码，自定义分隔符

    // 创建Blob元素：JSON metadata, 分隔符 以及 data
    const combinedDataBlob = new Blob([jsonMetadata, delimiter, data]);

    // 读取Blob并发送分块数据
    const reader = new FileReader();
    reader.onload = function (e) {
        const arrayBuffer = e.target.result;
        const uint8Array = new Uint8Array(arrayBuffer);

        // 把数据分块发送
        for (let offset = 0; offset < uint8Array.byteLength; offset += CHUNK_SIZE) {
            const chunk = uint8Array.slice(offset, Math.min(uint8Array.byteLength, offset + CHUNK_SIZE));
            ws.send(chunk);
        }
    };
    reader.readAsArrayBuffer(combinedDataBlob);
}