use bus::{
    demo, publish, publish_bot, redact_for_egress, send_to, submit, Bus, Envelope, Publish,
    BUS_CAP, MAX_BOT_EVENTS,
};

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
    let scrubbed = redact_for_egress("Authorization: Bearer sk-live-0123456789abcdef now");
    assert!(!scrubbed.contains("sk-live"));
    assert!(scrubbed.contains("Bearer [redacted]"));
    let mut bus = Bus::new();
    let rx = bus.subscribe("peer", "work");
    submit(&mut bus, "work", "see sk-live-0123456789abcdef");
    let mail = rx.recv().unwrap();
    assert!(!mail.body.contains("sk-live"));
    assert!(mail.body.contains("[redacted]"));
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
