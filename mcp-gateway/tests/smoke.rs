use mcp_gateway::Gateway;

#[test]
fn a_new_gateway_has_an_empty_catalog_and_offers_no_tools() {
    let gateway = Gateway::new();
    assert!(gateway.catalog().is_empty());
    assert_eq!(gateway.catalog().len(), 0);

    let view = gateway.session_tools("session-1");
    assert_eq!(view.session_id, "session-1");
    assert!(view.tools.is_empty());
}
