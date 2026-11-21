use harness_core::hooks::{
    scan_plugin_shape, should_allow_plugin_install, InstallDecision, PluginFile, PluginVerdict,
    MAX_PLUGIN_FILE_COUNT, MAX_PLUGIN_SINGLE_FILE_BYTES, MAX_PLUGIN_TOTAL_BYTES,
};

#[test]
fn an_oversized_plugin_is_caution_and_force_cannot_pass_dangerous() {
    let fat = vec![PluginFile {
        path: "blob.bin".into(),
        bytes: MAX_PLUGIN_SINGLE_FILE_BYTES + 1,
    }];
    assert_eq!(scan_plugin_shape(&fat), PluginVerdict::Caution);
    let many = vec![
        PluginFile {
            path: "a.txt".into(),
            bytes: 1
        };
        MAX_PLUGIN_FILE_COUNT + 1
    ];
    assert_eq!(scan_plugin_shape(&many), PluginVerdict::Caution);
    let huge = vec![PluginFile {
        path: "sum.bin".into(),
        bytes: MAX_PLUGIN_TOTAL_BYTES + 1,
    }];
    assert_eq!(scan_plugin_shape(&huge), PluginVerdict::Caution);
    let small = vec![PluginFile {
        path: "ok.txt".into(),
        bytes: 32,
    }];
    assert_eq!(scan_plugin_shape(&small), PluginVerdict::Safe);
    assert_eq!(
        should_allow_plugin_install(PluginVerdict::Caution, false),
        InstallDecision::Confirm
    );
    assert_eq!(
        should_allow_plugin_install(PluginVerdict::Caution, true),
        InstallDecision::Allow
    );
    assert_eq!(
        should_allow_plugin_install(PluginVerdict::Dangerous, true),
        InstallDecision::Block
    );
}
