# Social posts: Chapter 18, "Providers"

## X thread
1/
Chapter 18 is up: Providers.

OpenRouter is still the default. This chapter builds the Anthropic Messages body and the Bedrock Converse body beside it, plus an OpenRouter route. Rotating a key keeps the same client id. 🧵

2/
Hermes Agent's build_anthropic_kwargs sends model, messages, and max_tokens, and sets system only when it is non-empty (agent/anthropic_adapter.py). build_converse_kwargs uses modelId and inferenceConfig (agent/bedrock_adapter.py). Provider sort is throughput, latency, or price. Anything else is dropped.

3/
Duplicate tool-call ids become id_d2, not a random suffix (agent/message_sanitization.py). A transcript that ends on a tool row is closed so the next user message is not tool-then-user. ssl_verify false is refused unless a dev flag is set.

4/
Choosing the next route measured 5.990 ns. Rotating a key measured 3.465 ns. The demo is a fake fallback. No HTTP. The client id stays 1.

[personal note]

5/
Next is streaming: fold the deltas, strip think blocks as they arrive, and let a read-only tool start before the stream finishes.

Read Chapter 18: [link]

#rustlang #AIagents

## LinkedIn
Chapter 18 is out: Providers.

The first client in this book speaks OpenRouter. Anthropic and Bedrock want different JSON. I built those bodies without opening a socket: model, messages, and max_tokens for Anthropic; modelId and inferenceConfig for Bedrock. An OpenRouter route carries a models list and a provider object. Invalid sort values are dropped.

Key rotation advances an index inside one pool. The client id does not change. A cheaper side model stays off the user turn unless that turn asked for a summary. Choosing the next route measured 5.990 ns.

[personal note: one takeaway from writing this chapter]

Next: streaming the same events as the tokens arrive. Read Chapter 18: [link]

#Rust #AIAgents #LearningInPublic #SoftwareEngineering
