use mcp_gateway::Gateway;

#[test]
fn the_minimal_server_offers_an_empty_catalog() {
    let gateway = Gateway::new();
    assert!(gateway.catalog().is_empty());
    let view = gateway.session_tools("session-1");
    assert!(view.tools.is_empty());
}
