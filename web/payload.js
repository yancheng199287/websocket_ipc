const PROTOCOL_VERSION = "v1.0.0"

const DELIMITER = '|'; // 分隔符

const DELIMITER_BLOB = new Blob([DELIMITER]);


const CHUNK_SIZE = 5; // 512KB

///const CHUNK_SIZE = 512 * 1024; // 512KB


class Header {
    constructor(app_id, session_id) {
        /// 应用id，从应用的配置文件读取，变为全局常量
        this.app_id = app_id;
        /// 会话id，服务端返回生成的会话id，这个保存到当前客户端的会话配置参数里
        this.session_id = session_id;
        // 每次请求和响应的唯一标识
        this.msg_id = app_id + "_" + this.getUuid();
        /// 协议版本，随时升级，需要记录每次变化，每次升级做兼容性处理
        this.version = PROTOCOL_VERSION;
        /// 请求的时间
        this.datetime = new Date().toString();
    }

    // 生成唯一性id
    getUuid() {
        return Number(Math.random().toString().slice(2, 10) + Date.now()).toString(36)
    }
}

class MetaData {
    constructor(name, type, length, chunkTotal, chunkIndex) {
        // "流的名称"  可以是文件名称，图片名称，如果是脚本，文本 取个默认的名称接口
        this.name = name;
        //  "word/excel/img/text" 流的具体类型
        this.type = type;
        // 字节数组长度， 单纯的  流的数据
        this.length = length;
        // 总共的分块数量  小于或者等于0 代表不需要分块， 用分块总量除以总长度可以得到每块的大小
        this.chunkTotal = chunkTotal;
        // 分块的索引     1  代表第一块 ， 如果当前分块索引等于chunkTotal则分块传输完毕
        this.chunkIndex = chunkIndex;
    }

}

/// 任务类型，执行函数，执行脚本，执行消息订阅
const TASK_TYPE = {
    Function: "Function",
    Script: "Script",
    Subscription: "Subscription",
}

class BusinessData {

    constructor(task_type, task_params) {
        // 任务执行类型
        this.task_type = task_type;
        // 任务执行的json参数，每个任务不一样，具体自己后面再定义
        this.task_params = task_params;
    }

}


function sendPayload(ws, header, metaData, businessData, dataBlob) {
    if (!dataBlob) {
        metaData.length = 0;
    } else {
        metaData.length = dataBlob.size;
    }
    let stream = spiltStream(dataBlob);
    metaData.chunkTotal = stream.length;
    metaData.chunkIndex = 0;
    let payload = {
        header: header,
        metaData: metaData,
        businessData: businessData
    };
    let payloadStr = JSON.stringify(payload);

    // 发送第一个事件
    ws.send(payloadStr);

    if (stream.length <= 0) {
        return;
    }
    // 循环发送后续的分块数据流消息
    buildChunkPartPayload(header, metaData, stream).forEach((item) => {
        ws.send(item);
    });
}

/*第一个消息,直接使用文本消息，大多数情况够用
header+metaData+businessData*/

function buildFirstPayload() {
    let header = new Header("com.miniapp", "1as46a54sda6daasdasd");
    let metaData = new MetaData("test", "word", 1000, 10, 1);
    let businessData = new BusinessData(TASK_TYPE.Function, "{ \"name\":\"test\" }");

    let payload = {
        header: header,
        metaData: metaData,
        businessData: businessData
    };
    // let payloadStr = JSON.stringify(payload);
    //return new Blob([payloadStr], {type: "text/plain;charset=utf-8"});
    return JSON.stringify(payload);
}


/*
第二个消息 分块流数据
header+metaData+stream（分块）
*/
function buildChunkPartPayload(header, metaData, stream) {
    metaData.chunkTotal = stream.length;
    const chunks = [];
    for (let i = 0; i < stream.length; i++) {
        metaData.chunkIndex = i + 1;
        metaData.length = stream[i].size;
        let payload = {
            header: header,
            metaData: metaData
        };
        let chunkPartBlob = new Blob([JSON.stringify(payload), DELIMITER_BLOB, stream[i]])
        chunks.push(chunkPartBlob);
    }
    return chunks;
}

// 按 CHUNK_SIZE分块大小将 data blob进行分割，返回一个分块数组，数组的容量就是分块的总数量
function spiltStream(data) {
    if (!data) {
        return [];
    }
    const dataBlob = (data instanceof Blob) ? data : new Blob([data]);
    if (dataBlob.size > CHUNK_SIZE) {
        const chunkSize = CHUNK_SIZE;
        const chunkCount = Math.ceil(dataBlob.size / chunkSize);
        const chunks = [];
        for (let i = 0; i < chunkCount; i++) {
            const chunk = dataBlob.slice(i * chunkSize, (i + 1) * chunkSize);
            chunks.push(chunk);
        }
        return chunks;
    } else {
        return [dataBlob];
    }
}


class WS {

    arrayBufferToString(buffer) {
        const decoder = new TextDecoder('utf-8');
        return decoder.decode(buffer);
    }

    send(data) {
        if (data instanceof Blob) {
            data.arrayBuffer().then((arrayBuffer) => {
                let str = this.arrayBufferToString(arrayBuffer);
                console.log("\n")
                console.log("模拟发送消息 Blob Blob as String", data.size);
                console.log(str);
                console.log("\n")
            });
        } else {
            console.log("模拟发送消息", data.length);
            console.log(data);
        }
    }
}


function buildFirstPayloadTest() {
    // 创建一个10MB的Blob
    const blobSizeInMB = 10;
    // const blobSizeInBytes = blobSizeInMB * 1024 * 1024; // 10 * 1024 * 1024
    const blobSizeInBytes = blobSizeInMB * 5 + 27; // 10 * 1024 * 1024
    const randomData = generateRandomData(blobSizeInBytes);
    const testBlob = new Blob([randomData], {type: 'application/octet-stream'});
    const length = testBlob.size;
    console.log("testBlob.size:", length); // 应该接近10MB


    let header = new Header("com.miniapp", "1as46a54sda6daasdasd");
    let metaData = new MetaData("test", "word", 0, 0, 0);
    let businessData = new BusinessData(TASK_TYPE.Function, "{ \"name\":\"test\" }");


    sendPayload(new WS(), header, metaData, businessData, testBlob);
}


function generateRandomData(sizeInBytes) {
    let data = [];
    const characters = 'ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789';
    const charactersLength = characters.length;
    for (let i = 0; i < sizeInBytes; i++) {
        data.push(characters.charAt(Math.floor(Math.random() * charactersLength)));
    }
    return new TextEncoder().encode(data.join(''));
}


buildFirstPayloadTest();