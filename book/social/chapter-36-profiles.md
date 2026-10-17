# Social posts: Chapter 36, "Profiles"

## X thread
1/
Chapter 36 is up: Profiles.

Several personas in one config. Each one has its own secrets. The prompt is the persona. Multiplex with no scope fails closed. 🧵

2/
Hermes Agent raises UnscopedSecretError from agent/secret_scope.py when multiplexing is on and get_secret has no profile scope. A miss under multiplex does not fall through to another profile's environment.

3/
The maps are in memory. resolve clones the persona Arc and reports whether the secret is present. Fake replies. No key is printed.

4/
One resolve measured 59.49 ns. The demo prints "profile: writer".

[personal note]

5/
Next is the chat gateway: who may speak, a pairing code, and a message onto the session inbox.

Read Chapter 36: [link]

#rustlang #AIagents

## LinkedIn
Chapter 36 is out: Profiles.

Hermes Agent serves more than one profile from one process. A secret read has to know which profile it is in. get_secret in agent/secret_scope.py raises UnscopedSecretError when multiplexing is on and no scope is installed. With a scope, the value comes from that profile. A miss does not borrow the next profile's environment.

This chapter stores each persona and each secret map in process. The prompt handed to the model is the persona. The secret is a lookup, and the lookup result is not written into that string.

One resolve measured 59.49 ns. The demo does not need an API key.

[personal note: one takeaway from writing this chapter]

Next: the chat gateway. Read Chapter 36: [link]

#Rust #AIAgents #LearningInPublic #SoftwareEngineering
