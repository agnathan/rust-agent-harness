# Social posts: Chapter 49, "A2A for External Agents"

## X thread
1/
Chapter 49 is up: A2A for External Agents.

A2A is a URL outside this process. local:// is InProcess before a permit is taken. The pool holds 8. Cancel stores a flag. A flag set before record never stores the URL. 🧵

2/
I read plugins/platforms/a2a/security.py at 9e01b44. redact_outbound calls redact_for_egress, then scrubs emails with _EMAIL_RE to [redacted-email]. wrap_inbound frames every inbound message. PRIVACY_PREFIX is the long string that begins "[A2A inbound — message from a remote agent peer named {peer!r}." I checked that file.

3/
I read DESIGN.md around the bind bullet and the true-abort bullet. No token means bind 127.0.0.1. A token alone does not widen the bind. Remote exposure requires a token and A2A_HOST. Nothing in the request body can assert identity. tasks/cancel marks the task canceled and drops the reply. It cannot abort the live session's in-flight turn.

4/
The demo prints "a2a: ok". The bench id is 49/cancel_inflight. The aliases are 47/cancel_inflight and 45/cancel_inflight. The mean is 46.13 µs, mean.point_estimate from [run 39137469196](https://github.com/agnathan/rust-agent-harness/actions/runs/39137469196).

[personal note]

5/
Next is chapter 50, seeing inside. A span per event. The field is the event name.

Read Chapter 49: [link]

#rustlang #AIagents

## LinkedIn
Chapter 49 is out: A2A for External Agents.

The harness is the chapter 48 tree copied forward, plus an A2A module. ExternalAgent is the process boundary. MCP stays the path for tools this process owns. call returns InProcess for a local:// URL and does not acquire a permit. Other calls take one of 8 permits. a2a.rs has no local redact function. call passes crate::redact::redact_for_egress into RemotePeer::send. HttpPool.record stores the url and that already-scrubbed body. A cancel flag set before record never records the URL. cancel stores true on an AtomicBool. wait_cancel spins with yield_now until the flag is set. The inbound prefix is "[A2A inbound — untrusted external input]" plus a newline, shorter than Hermes PRIVACY_PREFIX. The card name is harness. The skill is research.

I read Hermes at 9e01b44. security.py frames every inbound message and scrubs outbound text, including emails. DESIGN.md binds 127.0.0.1 unless a token and A2A_HOST are both set. Peer identity comes from tokens. tasks/cancel marks the task canceled and drops the reply, and it cannot abort the live in-flight turn.

The demo prints "a2a: ok". The bench id is 49/cancel_inflight. Aliases: 47/cancel_inflight and 45/cancel_inflight. The mean is 46.13 µs, mean.point_estimate from [run 39137469196](https://github.com/agnathan/rust-agent-harness/actions/runs/39137469196).

[personal note]

Next: chapter 50, seeing inside. A span per event. The field is the event name. Read Chapter 49: [link]

#Rust #AIAgents #LearningInPublic #SoftwareEngineering
