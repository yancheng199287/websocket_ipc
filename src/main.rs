mod payload_decoder;
mod model;

mod lib;

use fastwebsockets::{Frame, OpCode, Payload, upgrade, WebSocketError};
use hyper::{Body, Request, Response};
use hyper::server::conn::Http;
use hyper::service::service_fn;
use tokio::net::TcpListener;


async fn handle_client(fut: upgrade::UpgradeFut) -> Result<(), WebSocketError> {
    let websocket = fut.await?;
    let mut ws = fastwebsockets::FragmentCollector::new(websocket);
    loop {
        let frame = ws.read_frame().await?;
       let sss =  &frame.payload.to_vec();
        let s = String::from_utf8(sss.to_vec()).unwrap();
        println!("服务端接收到消息内容1111: {}", s);
        match frame.opcode {
            OpCode::Close => break,
            OpCode::Text | OpCode::Binary => {
               // 使用utf8库的decode方法对frame的payload进行解码
                let result = utf8::decode(&frame.payload);
                match result {
                    Ok(msg) => {
                        println!("服务端接收到消息内容: {}", msg);
                    }
                    Err(error) => {
                        println!("解码消息错误: {:?}", error);
                    }
                }


                ws.write_frame(frame).await?;
                let content = "hello, I am from websocket";
                let json = serde_json::to_string(content).unwrap();
                let frame = Frame::text(Payload::from(json.as_bytes()));
                ws.write_frame(frame).await?;
                // 跳出循环则是关闭此连接通道
                //  break;
            }
            _ => {}
        }
    }

    Ok(())
}

async fn server_upgrade(mut req: Request<Body>) -> Result<Response<Body>, WebSocketError> {
    // 对request对象进行升级websocket
    let (response, fut) = upgrade::upgrade(&mut req)?;

    // 这个地方继续开启异步任务去处理流的相关操作
    tokio::task::spawn(async move {
        // unconstrained代表一个不受tokio运行时限制的异步任务，tokio默认有最大线程数和最大等待时间，而这个方法不受这个限制
        // 请确保处理不太耗时和资源的操作
        if let Err(e) = tokio::task::unconstrained(handle_client(fut)).await {
            eprintln!("Error in websocket connection: {:?}", e);
        }
    });
    // let response = Response::new("hello world");
    //  let response = Response::builder().status(200).body(Body::from("hello world")).unwrap();
    // 升级完成立即返回，注意这里返回之后，上面的异步任务还在继续执行
    Ok(response)
}

fn main() -> Result<(), WebSocketError> {
    // 启动一个运行时，用来执行异步事件任务
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_io()
        .build()
        .unwrap();

    // 阻塞一个异步任务，阻塞使用的是loop来完成
    rt.block_on(async move {
        // 启动服务端监听端口
        let listener = TcpListener::bind("127.0.0.1:8080").await?;
        println!("Server started, listening on {}", "127.0.0.1:8080");

        // 循环获取客户端连接事件，这个循环是阻塞的
        loop {
            // 获取一个TCP客户端连接
            let (stream, _) = listener.accept().await?;
            println!("Client connected");

            // 处理客户端连接请求
            tokio::spawn(async move {
                let conn_fut = Http::new()
                    .serve_connection(stream, service_fn(server_upgrade))
                    .with_upgrades();
                if let Err(e) = conn_fut.await {
                    println!("An error occurred: {:?}", e);
                }
            });
        }
    })
}
