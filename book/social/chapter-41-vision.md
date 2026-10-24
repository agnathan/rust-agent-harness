# Social posts: Chapter 41, "Vision"

## X thread
1/
Chapter 41 is up: Vision.

Bytes are sniffed. Empty, over 50 MB, over 20 MB, or not jpeg/png/gif/webp is refused. A PNG over 100,000,000 pixels is refused. The caption is untrusted. 🧵

2/
Hermes Agent: tools/vision_tools.py sets _VISION_MAX_DOWNLOAD_BYTES to 50*1024*1024 and _MAX_BASE64_BYTES to 20*1024*1024. tools/vision_tools_image_prep.py accepts image/jpeg, image/png, image/gif, and image/webp, and caps validated pixels at 100_000_000. The sniff reads magic bytes.

3/
A remote URL uses the same secret and SSRF checks as the browser. data: URLs skip that remote check and still pass prepare. FakeImages does not download. There is no vision model in this crate.

4/
One sniff of a 1x1 PNG measured 11.2 ns, mean.point_estimate from [run 39128817573](https://github.com/agnathan/rust-agent-harness/actions/runs/39128817573). The id is new. The demo prints "vision: png".

5/
Next is the MCP gateway service: replicas, a drain, and the client checks around sampling and malware advisories.

Read Chapter 41: [link]

#rustlang #AIagents

## LinkedIn
Chapter 41 is out: Vision.

Hermes Agent refuses a vision download over 50 MB and an inline image over 20 MB. The types it will embed are jpeg, png, gif, and webp, decided from magic bytes rather than a file extension. A raster whose decoded pixels would pass 100,000,000 is rejected.

This chapter sniffs the header in process. prepare refuses an empty buffer, either size cap, a foreign type, and a PNG whose IHDR width times height exceeds that pixel cap. analyze checks the URL for secrets and for private or metadata addresses before it loads bytes, then wraps the caption as vision_analyze. The 1x1 fixture is a PNG header. The bench sniffs that header.

One sniff measured 11.2 ns, mean.point_estimate from [run 39128817573](https://github.com/agnathan/rust-agent-harness/actions/runs/39128817573). The demo prints "vision: png".

Next: the MCP gateway service. Read Chapter 41: [link]

#Rust #AIAgents #LearningInPublic #SoftwareEngineering
