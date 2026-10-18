//! Keyless demo. Builds a tiny catalog and searches it. No socket and no API key.

use gw_catalog::{Gateway, Load, ToolCard, DEFAULT_SEARCH_LIMIT};

fn card(name: &str, description: &str, server: &str) -> ToolCard {
    ToolCard {
        name: name.to_string(),
        description: description.to_string(),
        server: server.to_string(),
    }
}

fn main() {
    let gateway = Gateway::new();
    gateway
        .reload(Load {
            native: vec![card(
                "files_rename_file",
                "Rename a file on the files server.",
                "files",
            )],
            plugins: vec![],
            upstreams: vec![gw_catalog::Upstream {
                name: "mail".into(),
                tools: vec![card(
                    "mail_search_message",
                    "Search a mail message.",
                    "mail",
                )],
            }],
        })
        .expect("reload");
    let hits = gateway.search("rename a file", DEFAULT_SEARCH_LIMIT);
    println!("tools={}", gateway.len());
    for hit in hits {
        println!("{} ({})", hit.name, hit.server);
    }
}
