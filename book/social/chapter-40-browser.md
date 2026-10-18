# Social posts: Chapter 40, "Browser Tools"

## X thread
1/
Chapter 40 is up: Browser Tools.

A page is a trait. FakeBrowser never dials. navigate checks the secret, then is_safe_url with allow_private false, and only then calls open. The final URL is checked again. 🧵

2/
Hermes Agent: tools/browser_tool.py sets DEFAULT_SNAPSHOT_THRESHOLD to 15000 and MAX_STORED_SNAPSHOT_CHARS to 2_000_000. evaluate_url_safety runs the secret check, then the policy check, before browser_navigate opens the page. Metadata stays blocked.

3/
agent/tool_dispatch_helpers.py wraps names that start with browser_. The floor is 32 characters. A shorter click or type note stays plain text. plugins/browser/ (browserbase, browser_use, firecrawl) stays deferred.

4/
One canned navigation measured @@MEAN:40/fake_navigate@@. The id is new. The demo prints "browser: Notes".

[personal note]

5/
Next is vision: magic bytes, a 50 MB download cap, a 20 MB inline cap, and a caption marked untrusted.

Read Chapter 40: [link]

#rustlang #AIagents

## LinkedIn
Chapter 40 is out: Browser Tools.

Hermes Agent opens a page only after the URL survives a secret check and a safety check, then looks at the final URL again. The snapshot the model sees defaults to 15,000 characters. The stored copy stops at 2,000,000. Tool results whose names start with browser_ are wrapped as untrusted data once they reach 32 characters.

This chapter puts that behind PageBackend. Tests use FakeBrowser, which returns a canned page and does not dial. A metadata address is refused before open. The result JSON carries wrap_untrusted("browser_navigate", ...). Browserbase, browser-use, and Firecrawl stay deferred.

One canned navigation measured @@MEAN:40/fake_navigate@@. The demo prints "browser: Notes".

[personal note: one takeaway from writing this chapter]

Next: vision. Read Chapter 40: [link]

#Rust #AIAgents #LearningInPublic #SoftwareEngineering
