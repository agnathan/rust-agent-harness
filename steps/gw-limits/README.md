# Chapter 15: Limits and Breakers

`gw-limits` wraps the chapter 8 session gateway. Each tool accepts 32 calls and runs 8. A retry or a hedge runs only when the tool is idempotent. Three failures open a breaker. `web_search` and `delegate_task` stop at 50. The demo needs no API key.
