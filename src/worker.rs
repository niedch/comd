use crate::{
    actions::{StartRequest, StreamResult},
    config::Settings,
};
use futures::StreamExt;
use rig::{
    agent::MultiTurnStreamItem,
    client::CompletionClient,
    providers::gemini,
    streaming::{StreamedAssistantContent, StreamingPrompt},
};
use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender};

pub fn spawn_worker(
    settings: Settings,
    _tx: UnboundedSender<StreamResult>,
    mut rx: UnboundedReceiver<StartRequest>,
) {
    tokio::spawn(async move {
        while let Some(action) = rx.recv().await {
                let client = gemini::Client::new(&settings.global.gemini_api_key).unwrap();
                let agent = client.agent("gemini-2.5-flash")
                    .preamble(&settings.global.system_prompt)
                    .build();

                let mut response_stream = agent.stream_prompt(&action.prompt).await;

                while let Some(item) = response_stream.next().await {
                    match item {
                        Ok(MultiTurnStreamItem::StreamAssistantItem(StreamedAssistantContent::Text(result))) => {
                            let _ = _tx.send(StreamResult{
                                action_type: crate::actions::StreamType::StreamResult,
                                result: result.to_string(),
                            });
                        }
                        Ok(MultiTurnStreamItem::FinalResponse(response)) => {
                            let _ = _tx.send(StreamResult{
                                action_type: crate::actions::StreamType::StreamEnd,
                                result: response.response().to_string()
                            });
                        }
                        Err(e) => {
                            eprintln!("Error in stream: {}", e);
                            break;
                        }
                        Ok(_) => {}
                    };
                }

        }
    });
}
