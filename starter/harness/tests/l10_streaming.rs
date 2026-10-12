use harness::l02_message::ToolCall;
use harness::l07_loop::ModelReply;
use harness::l10_streaming::*;
#[test]
fn parse_lines() {
    assert_eq!(parse_sse_line(""), Ok(None));
    assert_eq!(parse_sse_line(": keep-alive"), Ok(None));
    assert_eq!(parse_sse_line("data: [DONE]"), Ok(Some(vec![Delta::Done])));
    assert_eq!(
        parse_sse_line(r#"data: {"choices":[{"delta":{"content":"Hel"}}]}"#),
        Ok(Some(vec![Delta::Text("Hel".into())]))
    );
    assert_eq!(
        parse_sse_line(
            r#"data: {"choices":[{"delta":{"tool_calls":[{"index":0,"id":"c1","function":{"name":"add","arguments":"{\"a"}}]}}]}"#
        ),
        Ok(Some(vec![
            Delta::ToolCallStart {
                index: 0,
                id: "c1".into(),
                name: "add".into()
            },
            Delta::ToolCallArgs {
                index: 0,
                fragment: "{\"a".into()
            }
        ]))
    );
    assert!(parse_sse_line("data: {oops").is_err());
}
#[test]
fn accumulate_tool_call() {
    let mut acc = Accumulator::default();
    for d in [
        Delta::ToolCallStart {
            index: 0,
            id: "c1".into(),
            name: "add".into(),
        },
        Delta::ToolCallArgs {
            index: 0,
            fragment: "{\"a\":".into(),
        },
        Delta::ToolCallArgs {
            index: 0,
            fragment: "1}".into(),
        },
    ] {
        acc.apply(&d);
    }
    assert_eq!(
        acc.finish(),
        ModelReply::ToolCalls(vec![ToolCall {
            id: "c1".into(),
            name: "add".into(),
            arguments: "{\"a\":1}".into()
        }])
    );
}
#[tokio::test]
async fn consume_channel() {
    let (tx, rx) = tokio::sync::mpsc::channel(4);
    tokio::spawn(async move {
        for d in [
            Delta::Text("He".into()),
            Delta::Text("llo".into()),
            Delta::Done,
            Delta::Text("ignored".into()),
        ] {
            let _ = tx.send(d).await;
        }
    });
    let mut seen = Vec::new();
    let reply = consume(rx, |t| seen.push(t.to_string())).await;
    assert_eq!(reply, ModelReply::Text("Hello".into()));
    assert_eq!(seen, vec!["He", "llo"]);
}
