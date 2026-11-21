use harness_core::bus::{
    demo, publish, publish_bot, send_to, Bus, Envelope, Publish, BUS_CAP, MAX_BOT_EVENTS,
};
use harness_core::redact_for_egress;

#[test]
fn a_full_subscriber_is_slow_and_a_direct_send_names_it() {
    let mut bus = Bus::new();
    let rx = bus.subscribe("slow", "work");
    for n in 0..BUS_CAP {
        assert_eq!(
            publish(
                &mut bus,
                Envelope {
                    topic: "work".into(),
                    body: n.to_string(),
                }
            ),
            Publish::Delivered(1)
        );
    }
    assert_eq!(
        publish(
            &mut bus,
            Envelope {
                topic: "work".into(),
                body: "more".into(),
            }
        ),
        Publish::Slow("slow".into())
    );
    assert_eq!(
        send_to(
            &bus,
            "slow",
            Envelope {
                topic: "work".into(),
                body: "direct".into(),
            }
        ),
        Publish::Slow("slow".into())
    );
    assert_eq!(rx.try_recv().unwrap().body, "0");
}

#[test]
fn the_twenty_first_bot_message_cools_down() {
    let mut bus = Bus::new();
    let rx = bus.subscribe("peer", "work");
    for _ in 0..MAX_BOT_EVENTS {
        assert!(matches!(
            publish_bot(
                &mut bus,
                Envelope {
                    topic: "work".into(),
                    body: "ping".into(),
                }
            ),
            Publish::Delivered(1)
        ));
    }
    assert_eq!(
        publish_bot(
            &mut bus,
            Envelope {
                topic: "work".into(),
                body: "ping".into(),
            }
        ),
        Publish::Cooldown
    );
    assert_eq!(bus.bot_events(), MAX_BOT_EVENTS);
    let _ = rx;
}

#[test]
fn egress_scrubs_a_bearer_and_an_sk_token() {
    let header = redact_for_egress("Authorization: Bearer sk-live-0123456789abcdef now");
    assert!(!header.contains("sk-live"), "{header}");
    assert!(header.contains("Authorization: ***"), "{header}");
    let bearer = redact_for_egress("see Bearer abcdefghijklmnopqrstuvwxyz012345 now");
    assert!(bearer.contains("Bearer [redacted]"), "{bearer}");
    assert!(
        !bearer.contains("abcdefghijklmnopqrstuvwxyz012345"),
        "{bearer}"
    );
    let assigned = redact_for_egress("ghp_abcdefghijklmnopqrst OPENAI_API_KEY=supersecret");
    assert!(!assigned.contains("ghp_abcdefghijklmnopqrst"), "{assigned}");
    assert!(!assigned.contains("supersecret"), "{assigned}");
    assert!(assigned.contains("OPENAI_API_KEY=***"), "{assigned}");

    let mut bus = Bus::new();
    let rx = bus.subscribe("peer", "work");
    let published = publish(
        &mut bus,
        Envelope {
            topic: "work".into(),
            body: "see sk-live-0123456789abcdef and OPENAI_API_KEY=supersecret".into(),
        },
    );
    assert_eq!(published, Publish::Delivered(1));
    let mail = rx.recv().unwrap();
    assert!(!mail.body.contains("sk-live"), "{}", mail.body);
    assert!(!mail.body.contains("supersecret"), "{}", mail.body);
    assert!(mail.body.contains("sk-***"), "{}", mail.body);
    assert!(mail.body.contains("OPENAI_API_KEY=***"), "{}", mail.body);
}

#[test]
fn a_different_topic_is_not_delivered() {
    let mut bus = Bus::new();
    let rx = bus.subscribe("peer", "other");
    assert_eq!(
        publish(
            &mut bus,
            Envelope {
                topic: "work".into(),
                body: "ping".into(),
            }
        ),
        Publish::Delivered(0)
    );
    assert!(rx.try_recv().is_err());
}

#[test]
fn demo_delivers_ping() {
    assert_eq!(demo(), "bus: ping ok");
}
