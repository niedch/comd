use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender};
use crate::actions::{StartRequest, StreamResult, StreamType};

pub fn spawn_worker(
    tx: UnboundedSender<StreamResult>,
    mut rx: UnboundedReceiver<StartRequest>,
) {
    tokio::spawn(async move {
        tokio::select! {
        Some(_action) = rx.recv() => {
                let result = StreamResult {
                    action_type: StreamType::StreamResult,
                    result: "Result 1".to_string()
                };


                match tx.send(result) {
                    Ok(_) => {},
                    Err(e) => {
                        println!("err {:?}", e);
                    }
                };
                tokio::time::sleep(std::time::Duration::from_millis(500)).await;

                let result = StreamResult {
                    action_type: StreamType::StreamResult,
                    result: "Result 2".to_string()
                };

                match tx.send(result) {
                    Ok(_) => {},
                    Err(e) => {
                        println!("err {:?}", e);
                    }
                };
                tokio::time::sleep(std::time::Duration::from_millis(500)).await;

                let result = StreamResult {
                    action_type: StreamType::StreamResult,
                    result: "Result 3".to_string()
                };

                match tx.send(result) {
                    Ok(_) => {},
                    Err(e) => {
                        println!("err {:?}", e);
                    }
                };
                tokio::time::sleep(std::time::Duration::from_millis(500)).await;

                let result = StreamResult {
                    action_type: StreamType::StreamEnd,
                    result: "StreamEnd".to_string()
                };

                match tx.send(result) {
                    Ok(_) => {},
                    Err(e) => {
                        println!("err {:?}", e);
                    }
                };
        }
        }
    });
}
