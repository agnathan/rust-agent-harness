# Social posts: Chapter 38, "Profiles"

## X thread
1/
Chapter 38 is up: Profiles.

Several personas share one process. Each profile has its own secret map. The prompt is the persona. Multiplex with no scope fails closed. The secret does not go into the prompt. 🧵

2/
Hermes Agent at 9e01b44, agent/secret_scope.py. This is E4. get_secret raises UnscopedSecretError when multiplexing is on and no scope is installed. A scope reads that profile's map. Under multiplex a miss does not fall through to os.environ. Global names such as PATH still read the process env. API_KEY is not one of those names.

3/
The harness returns Result instead of raising. The maps are HashMaps. There is no ContextVar, and there is no global-env allowlist, so an unscoped multiplex read fails closed for every name. resolve keeps a bool for "the secret was present" and drops the string. The persona Arc is the prompt.

4/
The bench is resolve_profile, the scoped hit. The mean is 96.7 ns, mean.point_estimate from [run 39128817573](https://github.com/agnathan/rust-agent-harness/actions/runs/39128817573). The demo prints "profile: writer". The key stays in the map.

[personal note]

5/
Next is chapter 39, the chat gateway. A stranger is denied. A pairing code is eight characters and expires. A platform message lands on the session inbox.

Read Chapter 38: [link]

#rustlang #AIagents

## LinkedIn
Chapter 38 is out: Profiles.

One process can hold more than one persona. Hermes Agent's get_secret fails closed when the multiplex gateway has no profile scope installed: UnscopedSecretError, rather than reading another profile's environment. I kept that. Writer and reader each have an API_KEY. The scope selects one map. The prompt is the persona, "you write", and the secret is not copied into it. An unscoped read is the error. The demo prints "profile: writer" and does not print the key.

The mean for resolve_profile is 96.7 ns.

[personal note]

Next: the chat gateway. Who is allowed to speak, a pairing code, and a platform message on the inbox. Read Chapter 38: [link]

#Rust #AIAgents #LearningInPublic #SoftwareEngineering
