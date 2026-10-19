# Social posts: Chapter 51, "Load and Token Capstone"

## X thread
1/
Chapter 51 is up: Load and Token Capstone.

Thirty-two tool cards. Every card, the first eight, or the one named tool_07. The schema is 18 bytes, five tokens. The arms are 160, 40, and 5. No socket. 🧵

2/
I read tools/tool_search_catalog.py on Hermes at 9e01b44. CHARS_PER_TOKEN is 4.0, and the comment says it slightly underestimates. math.ceil of the length over that constant decides whether a catalog listing fits.

3/
This crate uses integer div_ceil. tool_07 sits inside the first eight, so all three arms succeed. One hundred SessionGateway calls run as 13 waves of at most 8. p99 is computed and left off the line. oha is not run.

4/
The demo prints "capstone: every=160 static=40 dynamic=5 success=1/1/1 task=1/1/1 load=100/100 waves=13". The bench ids are 51/dynamic_tokens and 51/load_100. The old ids 49/dynamic_tokens, 47/dynamic_tokens, 49/load_100, and 47/load_100 are aliases. The means are 264.8 ns and 122.77 µs, mean.point_estimate from [run 39137469196](https://github.com/agnathan/rust-agent-harness/actions/runs/39137469196).

[personal note]

5/
Next is chapter 52, evals: chapter 9's saved tasks on the full harness. That chapter copies the tracing tree, not this server crate.

Read Chapter 51: [link]

#rustlang #AIagents

## LinkedIn
Chapter 51 is out: Load and Token Capstone.

The server track ends by comparing what a completed task costs in schema tokens. Hermes Agent at 9e01b44 sets CHARS_PER_TOKEN to 4.0 in tools/tool_search_catalog.py, with a comment that the rule slightly underestimates, and it uses math.ceil of the length over that constant. This chapter builds 32 cards, tool_00 through tool_31. Each schema is {"name":"tool_NN"}, 18 bytes. approx_tokens is integer div_ceil of the length by 4, so each card is 5 tokens. The arms are every card (160), the first eight (40), and the one card named tool_07 (5). tool_07 is inside the first eight, so all three arms succeed when there is no deadline. A deadline equal to the dynamic total fails the wide arms and keeps the dynamic arm.

The load runs 100 calls through SessionGateway in memory. CALL_BOUND is 8. The wave counter increments when the in-flight count returns to 0, so 100 calls are 13 waves. Nothing listens. oha is not invoked. p99 is computed on the samples and is not printed.

The bench ids are 51/dynamic_tokens and 51/load_100. Aliases: 49/dynamic_tokens, 47/dynamic_tokens, 49/load_100, and 47/load_100. The means are 264.8 ns and 122.77 µs, mean.point_estimate from [run 39137469196](https://github.com/agnathan/rust-agent-harness/actions/runs/39137469196). The demo does not need an API key.

[personal note: one takeaway from writing this chapter]

Next: chapter 52 re-runs the saved tasks on the harness. Read Chapter 51: [link]

#Rust #AIAgents #LearningInPublic #SoftwareEngineering
