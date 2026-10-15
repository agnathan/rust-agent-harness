window.BENCHMARK_DATA = {
  "lastUpdate": 1792032492619,
  "repoUrl": "https://github.com/agnathan/rust-agent-harness",
  "entries": {
    "Harness criterion": [
      {
        "commit": {
          "author": {
            "name": "agnathan",
            "username": "agnathan"
          },
          "committer": {
            "name": "agnathan",
            "username": "agnathan"
          },
          "id": "2e14708ffb864799884d2806e217dea2b0fb3751",
          "message": "Agent loop written; MCP gateway outlined and sketched",
          "timestamp": "2026-10-12T06:07:26Z",
          "url": "https://github.com/agnathan/rust-agent-harness/pull/1/commits/2e14708ffb864799884d2806e217dea2b0fb3751"
        },
        "date": 1791937630614,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "1.1/serde_50_message_request",
            "value": 10575.289030636899,
            "unit": "ns"
          },
          {
            "name": "1.2/mock_100_concurrent",
            "value": 78938.59689922481,
            "unit": "ns"
          },
          {
            "name": "1.2/mock_turn_overhead",
            "value": 250.85483166308254,
            "unit": "ns"
          },
          {
            "name": "1.3/build_request_and_parse_response",
            "value": 3020.706532559104,
            "unit": "ns"
          },
          {
            "name": "1.4/parse_response_edges",
            "value": 1094.6393566914276,
            "unit": "ns"
          },
          {
            "name": "2/chat_turn_overhead",
            "value": 691.3528130671507,
            "unit": "ns"
          },
          {
            "name": "2/inbox_submit",
            "value": 42.45154232424677,
            "unit": "ns"
          },
          {
            "name": "3.1/registry_lookup",
            "value": 25.692328651552,
            "unit": "ns"
          },
          {
            "name": "3.2/available_tools_50",
            "value": 30396.745190972222,
            "unit": "ns"
          },
          {
            "name": "3.3/dispatch_8_reads_concurrent",
            "value": 2132188.688235294,
            "unit": "ns"
          },
          {
            "name": "3.3/dispatch_8_reads_sequential",
            "value": 16698092.123809524,
            "unit": "ns"
          },
          {
            "name": "4.1/stdio_echo_round_trip",
            "value": 203430.26769662922,
            "unit": "ns"
          },
          {
            "name": "4.2/http_echo_round_trip",
            "value": 415470.0053661616,
            "unit": "ns"
          },
          {
            "name": "4.3/mcp_50_concurrent",
            "value": 5000531.476388888,
            "unit": "ns"
          },
          {
            "name": "4.3/mcp_50_sequential",
            "value": 10241973.016666666,
            "unit": "ns"
          },
          {
            "name": "4.4/mcp_list_cold",
            "value": 216617.65833333333,
            "unit": "ns"
          },
          {
            "name": "4.4/mcp_list_hit",
            "value": 252.2066330274563,
            "unit": "ns"
          },
          {
            "name": "5.1/schema_tokens_1000",
            "value": 1256634.5229885057,
            "unit": "ns"
          },
          {
            "name": "5.1/schema_tokens_10000",
            "value": 42790552.97619048,
            "unit": "ns"
          },
          {
            "name": "5.2/index_build_10000",
            "value": 21525128.611111112,
            "unit": "ns"
          },
          {
            "name": "5.2/index_search_10000",
            "value": 335666.97247706424,
            "unit": "ns"
          },
          {
            "name": "5.3/full_schema_tokens_10000",
            "value": 22842661.506944444,
            "unit": "ns"
          },
          {
            "name": "5.3/lazy_visible_prompt_10000",
            "value": 15467.858113207549,
            "unit": "ns"
          },
          {
            "name": "5.4/routed_search_10000",
            "value": 25860.90234282325,
            "unit": "ns"
          },
          {
            "name": "5.4/shrink_schemas_10000",
            "value": 62906371.125,
            "unit": "ns"
          },
          {
            "name": "5.5/agent_filter_10000",
            "value": 125707.00274884258,
            "unit": "ns"
          },
          {
            "name": "5.5/cache_cold_10000",
            "value": 26255120.82142857,
            "unit": "ns"
          },
          {
            "name": "5.5/cache_hit_10000",
            "value": 13.794524402651088,
            "unit": "ns"
          },
          {
            "name": "6.1/gate_check",
            "value": 3.424786259850721,
            "unit": "ns"
          },
          {
            "name": "6.1/live_tool_body",
            "value": 1808.576408825093,
            "unit": "ns"
          },
          {
            "name": "6.1/loop_8_reads",
            "value": 2152350.480392157,
            "unit": "ns"
          },
          {
            "name": "6.1/loop_iteration",
            "value": 368.6432860007461,
            "unit": "ns"
          },
          {
            "name": "6.10/budget_notice",
            "value": 76.11677838349391,
            "unit": "ns"
          },
          {
            "name": "6.10/cancel_select",
            "value": 94.91435756369117,
            "unit": "ns"
          },
          {
            "name": "6.10/canonical_args",
            "value": 1043.9485080284253,
            "unit": "ns"
          },
          {
            "name": "6.10/gate_check",
            "value": 7.476678696479782,
            "unit": "ns"
          },
          {
            "name": "6.10/live_tool_body",
            "value": 1765.070834550945,
            "unit": "ns"
          },
          {
            "name": "6.10/loop_8_reads",
            "value": 2188402.7088235295,
            "unit": "ns"
          },
          {
            "name": "6.10/loop_iteration",
            "value": 788.6592335820997,
            "unit": "ns"
          },
          {
            "name": "6.10/outcome_label",
            "value": 8.39900487865047,
            "unit": "ns"
          },
          {
            "name": "6.10/stall_observe",
            "value": 137.83821683467102,
            "unit": "ns"
          },
          {
            "name": "6.10/steer_append",
            "value": 58.26075857673158,
            "unit": "ns"
          },
          {
            "name": "6.10/stress_repair",
            "value": 52028.19505319149,
            "unit": "ns"
          },
          {
            "name": "6.10/truncate_output",
            "value": 4596.232150081271,
            "unit": "ns"
          },
          {
            "name": "6.10/validate_repair",
            "value": 258.5554364352884,
            "unit": "ns"
          },
          {
            "name": "6.2/cancel_select",
            "value": 96.01133636758277,
            "unit": "ns"
          },
          {
            "name": "6.2/gate_check",
            "value": 7.471831107344042,
            "unit": "ns"
          },
          {
            "name": "6.2/live_tool_body",
            "value": 1735.7636355500808,
            "unit": "ns"
          },
          {
            "name": "6.2/loop_8_reads",
            "value": 2157508.4235294117,
            "unit": "ns"
          },
          {
            "name": "6.2/loop_iteration",
            "value": 508.55122796579997,
            "unit": "ns"
          },
          {
            "name": "6.3/cancel_select",
            "value": 120.87110379107295,
            "unit": "ns"
          },
          {
            "name": "6.3/gate_check",
            "value": 7.477630158532056,
            "unit": "ns"
          },
          {
            "name": "6.3/live_tool_body",
            "value": 1738.8278360891445,
            "unit": "ns"
          },
          {
            "name": "6.3/loop_8_reads",
            "value": 2176056.330882353,
            "unit": "ns"
          },
          {
            "name": "6.3/loop_iteration",
            "value": 526.7284586167966,
            "unit": "ns"
          },
          {
            "name": "6.3/validate_repair",
            "value": 259.07875308250425,
            "unit": "ns"
          },
          {
            "name": "6.4/cancel_select",
            "value": 96.46117040751457,
            "unit": "ns"
          },
          {
            "name": "6.4/gate_check",
            "value": 7.468813866067999,
            "unit": "ns"
          },
          {
            "name": "6.4/live_tool_body",
            "value": 1727.0368472999648,
            "unit": "ns"
          },
          {
            "name": "6.4/loop_8_reads",
            "value": 2182976.923611111,
            "unit": "ns"
          },
          {
            "name": "6.4/loop_iteration",
            "value": 570.2206157141036,
            "unit": "ns"
          },
          {
            "name": "6.4/stall_observe",
            "value": 138.5824948969675,
            "unit": "ns"
          },
          {
            "name": "6.4/validate_repair",
            "value": 265.20692086687666,
            "unit": "ns"
          },
          {
            "name": "6.5/budget_notice",
            "value": 81.67645379075157,
            "unit": "ns"
          },
          {
            "name": "6.5/cancel_select",
            "value": 94.78699969937871,
            "unit": "ns"
          },
          {
            "name": "6.5/gate_check",
            "value": 7.480261341202521,
            "unit": "ns"
          },
          {
            "name": "6.5/live_tool_body",
            "value": 1708.163894980507,
            "unit": "ns"
          },
          {
            "name": "6.5/loop_8_reads",
            "value": 2178029.1701680673,
            "unit": "ns"
          },
          {
            "name": "6.5/loop_iteration",
            "value": 613.5804144153128,
            "unit": "ns"
          },
          {
            "name": "6.5/stall_observe",
            "value": 138.47827177277935,
            "unit": "ns"
          },
          {
            "name": "6.5/validate_repair",
            "value": 274.507002653149,
            "unit": "ns"
          },
          {
            "name": "6.6/budget_notice",
            "value": 76.04317096645167,
            "unit": "ns"
          },
          {
            "name": "6.6/cancel_select",
            "value": 95.4287939930926,
            "unit": "ns"
          },
          {
            "name": "6.6/gate_check",
            "value": 7.479265219921982,
            "unit": "ns"
          },
          {
            "name": "6.6/live_tool_body",
            "value": 1735.3196738025904,
            "unit": "ns"
          },
          {
            "name": "6.6/loop_8_reads",
            "value": 2180303.6357142855,
            "unit": "ns"
          },
          {
            "name": "6.6/loop_iteration",
            "value": 620.5661559098618,
            "unit": "ns"
          },
          {
            "name": "6.6/stall_observe",
            "value": 138.54256322053874,
            "unit": "ns"
          },
          {
            "name": "6.6/truncate_output",
            "value": 4585.396232181057,
            "unit": "ns"
          },
          {
            "name": "6.6/validate_repair",
            "value": 264.03195291584,
            "unit": "ns"
          },
          {
            "name": "6.7/budget_notice",
            "value": 77.59114197121826,
            "unit": "ns"
          },
          {
            "name": "6.7/cancel_select",
            "value": 100.59436565863513,
            "unit": "ns"
          },
          {
            "name": "6.7/gate_check",
            "value": 7.472172097973943,
            "unit": "ns"
          },
          {
            "name": "6.7/live_tool_body",
            "value": 1676.734339754134,
            "unit": "ns"
          },
          {
            "name": "6.7/loop_8_reads",
            "value": 2178992.2584033613,
            "unit": "ns"
          },
          {
            "name": "6.7/loop_iteration",
            "value": 609.8543360090841,
            "unit": "ns"
          },
          {
            "name": "6.7/stall_observe",
            "value": 136.5861574084505,
            "unit": "ns"
          },
          {
            "name": "6.7/steer_append",
            "value": 58.29715817841234,
            "unit": "ns"
          },
          {
            "name": "6.7/truncate_output",
            "value": 4616.487471964792,
            "unit": "ns"
          },
          {
            "name": "6.7/validate_repair",
            "value": 259.9597431703547,
            "unit": "ns"
          },
          {
            "name": "6.8/budget_notice",
            "value": 76.83974769251499,
            "unit": "ns"
          },
          {
            "name": "6.8/cancel_select",
            "value": 103.27952877218047,
            "unit": "ns"
          },
          {
            "name": "6.8/canonical_args",
            "value": 1053.2850766412503,
            "unit": "ns"
          },
          {
            "name": "6.8/gate_check",
            "value": 7.47913931928634,
            "unit": "ns"
          },
          {
            "name": "6.8/live_tool_body",
            "value": 1740.5599639922593,
            "unit": "ns"
          },
          {
            "name": "6.8/loop_8_reads",
            "value": 2178440.6019607843,
            "unit": "ns"
          },
          {
            "name": "6.8/loop_iteration",
            "value": 713.3989640522875,
            "unit": "ns"
          },
          {
            "name": "6.8/stall_observe",
            "value": 140.77314194422928,
            "unit": "ns"
          },
          {
            "name": "6.8/steer_append",
            "value": 58.625836963809675,
            "unit": "ns"
          },
          {
            "name": "6.8/truncate_output",
            "value": 5010.73786128477,
            "unit": "ns"
          },
          {
            "name": "6.8/validate_repair",
            "value": 266.825268946366,
            "unit": "ns"
          },
          {
            "name": "6.9/budget_notice",
            "value": 76.36470546932588,
            "unit": "ns"
          },
          {
            "name": "6.9/cancel_select",
            "value": 96.77393495112656,
            "unit": "ns"
          },
          {
            "name": "6.9/canonical_args",
            "value": 1048.6210227599495,
            "unit": "ns"
          },
          {
            "name": "6.9/gate_check",
            "value": 7.474039312041333,
            "unit": "ns"
          },
          {
            "name": "6.9/live_tool_body",
            "value": 1743.7697800002284,
            "unit": "ns"
          },
          {
            "name": "6.9/loop_8_reads",
            "value": 2183097.7264705882,
            "unit": "ns"
          },
          {
            "name": "6.9/loop_iteration",
            "value": 756.8077622536634,
            "unit": "ns"
          },
          {
            "name": "6.9/outcome_label",
            "value": 8.396777803795294,
            "unit": "ns"
          },
          {
            "name": "6.9/stall_observe",
            "value": 139.63508263087013,
            "unit": "ns"
          },
          {
            "name": "6.9/steer_append",
            "value": 58.312355129925585,
            "unit": "ns"
          },
          {
            "name": "6.9/truncate_output",
            "value": 4647.331996896017,
            "unit": "ns"
          },
          {
            "name": "6.9/validate_repair",
            "value": 277.2711693416594,
            "unit": "ns"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "agnathan@gmail.com",
            "name": "Daniel Holmlund",
            "username": "agnathan"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "16672db633928d794d0866806daad8b629c9f555",
          "message": "Merge pull request #1 from agnathan/cursor/toc-v4-mcp-chapters-4d24\n\nAgent loop written; MCP gateway outlined; benchmark dashboard",
          "timestamp": "2026-10-13T17:34:54-07:00",
          "tree_id": "cb24976f03054ca64c0db24b0c538a4ef985a67e",
          "url": "https://github.com/agnathan/rust-agent-harness/commit/16672db633928d794d0866806daad8b629c9f555"
        },
        "date": 1791939130932,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "1.1/serde_50_message_request",
            "value": 8443.637409979423,
            "unit": "ns"
          },
          {
            "name": "1.2/mock_100_concurrent",
            "value": 70214.8016722408,
            "unit": "ns"
          },
          {
            "name": "1.2/mock_turn_overhead",
            "value": 207.5059171690329,
            "unit": "ns"
          },
          {
            "name": "1.3/build_request_and_parse_response",
            "value": 2608.4404786150712,
            "unit": "ns"
          },
          {
            "name": "1.4/parse_response_edges",
            "value": 947.7757021670637,
            "unit": "ns"
          },
          {
            "name": "2/chat_turn_overhead",
            "value": 528.070344333388,
            "unit": "ns"
          },
          {
            "name": "2/inbox_submit",
            "value": 34.007413510852956,
            "unit": "ns"
          },
          {
            "name": "3.1/registry_lookup",
            "value": 17.854036572135023,
            "unit": "ns"
          },
          {
            "name": "3.2/available_tools_50",
            "value": 24412.750926549867,
            "unit": "ns"
          },
          {
            "name": "3.3/dispatch_8_reads_concurrent",
            "value": 2148247.9215686275,
            "unit": "ns"
          },
          {
            "name": "3.3/dispatch_8_reads_sequential",
            "value": 16834311.394444443,
            "unit": "ns"
          },
          {
            "name": "4.1/stdio_echo_round_trip",
            "value": 131208.6454008954,
            "unit": "ns"
          },
          {
            "name": "4.2/http_echo_round_trip",
            "value": 311760.8619658119,
            "unit": "ns"
          },
          {
            "name": "4.3/mcp_50_concurrent",
            "value": 3349873.8652777774,
            "unit": "ns"
          },
          {
            "name": "4.3/mcp_50_sequential",
            "value": 6677265.722222222,
            "unit": "ns"
          },
          {
            "name": "4.4/mcp_list_cold",
            "value": 137055.52617924527,
            "unit": "ns"
          },
          {
            "name": "4.4/mcp_list_hit",
            "value": 181.75333624884385,
            "unit": "ns"
          },
          {
            "name": "5.1/schema_tokens_1000",
            "value": 1009203.9222972973,
            "unit": "ns"
          },
          {
            "name": "5.1/schema_tokens_10000",
            "value": 40730233.66666667,
            "unit": "ns"
          },
          {
            "name": "5.2/index_build_10000",
            "value": 17542384.083333336,
            "unit": "ns"
          },
          {
            "name": "5.2/index_search_10000",
            "value": 189463.74557291667,
            "unit": "ns"
          },
          {
            "name": "5.3/full_schema_tokens_10000",
            "value": 21911775.993055556,
            "unit": "ns"
          },
          {
            "name": "5.3/lazy_visible_prompt_10000",
            "value": 12685.261794362686,
            "unit": "ns"
          },
          {
            "name": "5.4/routed_search_10000",
            "value": 18734.827022404654,
            "unit": "ns"
          },
          {
            "name": "5.4/shrink_schemas_10000",
            "value": 59638039.33928572,
            "unit": "ns"
          },
          {
            "name": "5.5/agent_filter_10000",
            "value": 86259.59247822643,
            "unit": "ns"
          },
          {
            "name": "5.5/cache_cold_10000",
            "value": 22661065.666666664,
            "unit": "ns"
          },
          {
            "name": "5.5/cache_hit_10000",
            "value": 12.312313996174897,
            "unit": "ns"
          },
          {
            "name": "6.1/gate_check",
            "value": 2.455813803838886,
            "unit": "ns"
          },
          {
            "name": "6.1/live_tool_body",
            "value": 1470.397467969272,
            "unit": "ns"
          },
          {
            "name": "6.1/loop_8_reads",
            "value": 2165532.9960784316,
            "unit": "ns"
          },
          {
            "name": "6.1/loop_iteration",
            "value": 299.84817257707755,
            "unit": "ns"
          },
          {
            "name": "6.10/budget_notice",
            "value": 65.92715245892819,
            "unit": "ns"
          },
          {
            "name": "6.10/cancel_select",
            "value": 85.81149983482359,
            "unit": "ns"
          },
          {
            "name": "6.10/canonical_args",
            "value": 857.4567861408398,
            "unit": "ns"
          },
          {
            "name": "6.10/gate_check",
            "value": 5.9408280379024765,
            "unit": "ns"
          },
          {
            "name": "6.10/live_tool_body",
            "value": 1522.9364665107732,
            "unit": "ns"
          },
          {
            "name": "6.10/loop_8_reads",
            "value": 2198456.859477124,
            "unit": "ns"
          },
          {
            "name": "6.10/loop_iteration",
            "value": 580.7792131144633,
            "unit": "ns"
          },
          {
            "name": "6.10/outcome_label",
            "value": 7.65691899846332,
            "unit": "ns"
          },
          {
            "name": "6.10/stall_observe",
            "value": 114.87319758038873,
            "unit": "ns"
          },
          {
            "name": "6.10/steer_append",
            "value": 45.84171802487119,
            "unit": "ns"
          },
          {
            "name": "6.10/stress_repair",
            "value": 31616.539796312387,
            "unit": "ns"
          },
          {
            "name": "6.10/truncate_output",
            "value": 3508.579640019667,
            "unit": "ns"
          },
          {
            "name": "6.10/validate_repair",
            "value": 204.23035132033687,
            "unit": "ns"
          },
          {
            "name": "6.2/cancel_select",
            "value": 90.45262008719715,
            "unit": "ns"
          },
          {
            "name": "6.2/gate_check",
            "value": 5.663379842678822,
            "unit": "ns"
          },
          {
            "name": "6.2/live_tool_body",
            "value": 1464.0036996373892,
            "unit": "ns"
          },
          {
            "name": "6.2/loop_8_reads",
            "value": 2170907.665266107,
            "unit": "ns"
          },
          {
            "name": "6.2/loop_iteration",
            "value": 407.6372360194223,
            "unit": "ns"
          },
          {
            "name": "6.3/cancel_select",
            "value": 90.56771898464338,
            "unit": "ns"
          },
          {
            "name": "6.3/gate_check",
            "value": 5.727250516410313,
            "unit": "ns"
          },
          {
            "name": "6.3/live_tool_body",
            "value": 1520.1696559855736,
            "unit": "ns"
          },
          {
            "name": "6.3/loop_8_reads",
            "value": 2184392.302287582,
            "unit": "ns"
          },
          {
            "name": "6.3/loop_iteration",
            "value": 416.494295389117,
            "unit": "ns"
          },
          {
            "name": "6.3/validate_repair",
            "value": 204.5139710825253,
            "unit": "ns"
          },
          {
            "name": "6.4/cancel_select",
            "value": 90.66426058094905,
            "unit": "ns"
          },
          {
            "name": "6.4/gate_check",
            "value": 5.65582606744868,
            "unit": "ns"
          },
          {
            "name": "6.4/live_tool_body",
            "value": 1529.5177416181884,
            "unit": "ns"
          },
          {
            "name": "6.4/loop_8_reads",
            "value": 2186598.3302521007,
            "unit": "ns"
          },
          {
            "name": "6.4/loop_iteration",
            "value": 428.07291664195304,
            "unit": "ns"
          },
          {
            "name": "6.4/stall_observe",
            "value": 115.28915310903477,
            "unit": "ns"
          },
          {
            "name": "6.4/validate_repair",
            "value": 207.90375035237076,
            "unit": "ns"
          },
          {
            "name": "6.5/budget_notice",
            "value": 61.487857652428545,
            "unit": "ns"
          },
          {
            "name": "6.5/cancel_select",
            "value": 90.37728045668335,
            "unit": "ns"
          },
          {
            "name": "6.5/gate_check",
            "value": 5.667988405103635,
            "unit": "ns"
          },
          {
            "name": "6.5/live_tool_body",
            "value": 1506.6152389975227,
            "unit": "ns"
          },
          {
            "name": "6.5/loop_8_reads",
            "value": 2190618.848272642,
            "unit": "ns"
          },
          {
            "name": "6.5/loop_iteration",
            "value": 479.53055631119804,
            "unit": "ns"
          },
          {
            "name": "6.5/stall_observe",
            "value": 113.2218837231987,
            "unit": "ns"
          },
          {
            "name": "6.5/validate_repair",
            "value": 201.66252664860485,
            "unit": "ns"
          },
          {
            "name": "6.6/budget_notice",
            "value": 65.04275407845671,
            "unit": "ns"
          },
          {
            "name": "6.6/cancel_select",
            "value": 92.11266109065186,
            "unit": "ns"
          },
          {
            "name": "6.6/gate_check",
            "value": 5.6760450907123765,
            "unit": "ns"
          },
          {
            "name": "6.6/live_tool_body",
            "value": 1591.985223385837,
            "unit": "ns"
          },
          {
            "name": "6.6/loop_8_reads",
            "value": 2185340.0588235296,
            "unit": "ns"
          },
          {
            "name": "6.6/loop_iteration",
            "value": 463.3199493603977,
            "unit": "ns"
          },
          {
            "name": "6.6/stall_observe",
            "value": 115.22169075941297,
            "unit": "ns"
          },
          {
            "name": "6.6/truncate_output",
            "value": 3529.7716304136725,
            "unit": "ns"
          },
          {
            "name": "6.6/validate_repair",
            "value": 206.54880314629443,
            "unit": "ns"
          },
          {
            "name": "6.7/budget_notice",
            "value": 61.60943649383837,
            "unit": "ns"
          },
          {
            "name": "6.7/cancel_select",
            "value": 92.4106999065892,
            "unit": "ns"
          },
          {
            "name": "6.7/gate_check",
            "value": 5.678999413561057,
            "unit": "ns"
          },
          {
            "name": "6.7/live_tool_body",
            "value": 1410.2456806656141,
            "unit": "ns"
          },
          {
            "name": "6.7/loop_8_reads",
            "value": 2189812.095588235,
            "unit": "ns"
          },
          {
            "name": "6.7/loop_iteration",
            "value": 453.38941061107244,
            "unit": "ns"
          },
          {
            "name": "6.7/stall_observe",
            "value": 113.53685331364251,
            "unit": "ns"
          },
          {
            "name": "6.7/steer_append",
            "value": 45.601231431824836,
            "unit": "ns"
          },
          {
            "name": "6.7/truncate_output",
            "value": 3423.771807269264,
            "unit": "ns"
          },
          {
            "name": "6.7/validate_repair",
            "value": 205.36406472111352,
            "unit": "ns"
          },
          {
            "name": "6.8/budget_notice",
            "value": 65.03377488334031,
            "unit": "ns"
          },
          {
            "name": "6.8/cancel_select",
            "value": 91.96111297291628,
            "unit": "ns"
          },
          {
            "name": "6.8/canonical_args",
            "value": 871.2107152179588,
            "unit": "ns"
          },
          {
            "name": "6.8/gate_check",
            "value": 7.335440675553308,
            "unit": "ns"
          },
          {
            "name": "6.8/live_tool_body",
            "value": 1568.5382104647274,
            "unit": "ns"
          },
          {
            "name": "6.8/loop_8_reads",
            "value": 2189388.7463235296,
            "unit": "ns"
          },
          {
            "name": "6.8/loop_iteration",
            "value": 527.5779699820068,
            "unit": "ns"
          },
          {
            "name": "6.8/stall_observe",
            "value": 114.54732393758887,
            "unit": "ns"
          },
          {
            "name": "6.8/steer_append",
            "value": 45.60096713603266,
            "unit": "ns"
          },
          {
            "name": "6.8/truncate_output",
            "value": 3470.123874259277,
            "unit": "ns"
          },
          {
            "name": "6.8/validate_repair",
            "value": 201.5828817420027,
            "unit": "ns"
          },
          {
            "name": "6.9/budget_notice",
            "value": 63.914378042097795,
            "unit": "ns"
          },
          {
            "name": "6.9/cancel_select",
            "value": 93.27227764513937,
            "unit": "ns"
          },
          {
            "name": "6.9/canonical_args",
            "value": 870.1481833493744,
            "unit": "ns"
          },
          {
            "name": "6.9/gate_check",
            "value": 5.659031505935159,
            "unit": "ns"
          },
          {
            "name": "6.9/live_tool_body",
            "value": 1558.705658607309,
            "unit": "ns"
          },
          {
            "name": "6.9/loop_8_reads",
            "value": 2178216.876470588,
            "unit": "ns"
          },
          {
            "name": "6.9/loop_iteration",
            "value": 631.8090084005735,
            "unit": "ns"
          },
          {
            "name": "6.9/outcome_label",
            "value": 7.655923261478652,
            "unit": "ns"
          },
          {
            "name": "6.9/stall_observe",
            "value": 112.06097320388054,
            "unit": "ns"
          },
          {
            "name": "6.9/steer_append",
            "value": 45.57698276289729,
            "unit": "ns"
          },
          {
            "name": "6.9/truncate_output",
            "value": 3444.1663413306833,
            "unit": "ns"
          },
          {
            "name": "6.9/validate_repair",
            "value": 205.8254381106326,
            "unit": "ns"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "name": "agnathan",
            "username": "agnathan"
          },
          "committer": {
            "name": "agnathan",
            "username": "agnathan"
          },
          "id": "89ec85097d72349d249f1fdf90d960cc7c9d5fed",
          "message": "Fix My harness workflow parse error",
          "timestamp": "2026-10-14T00:35:05Z",
          "url": "https://github.com/agnathan/rust-agent-harness/pull/2/commits/89ec85097d72349d249f1fdf90d960cc7c9d5fed"
        },
        "date": 1791940200561,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "1.1/serde_50_message_request",
            "value": 10362.406611648643,
            "unit": "ns"
          },
          {
            "name": "1.2/mock_100_concurrent",
            "value": 81771.62589480236,
            "unit": "ns"
          },
          {
            "name": "1.2/mock_turn_overhead",
            "value": 247.52432952605392,
            "unit": "ns"
          },
          {
            "name": "1.3/build_request_and_parse_response",
            "value": 2953.8266606717843,
            "unit": "ns"
          },
          {
            "name": "1.4/parse_response_edges",
            "value": 1141.6385386682018,
            "unit": "ns"
          },
          {
            "name": "2/chat_turn_overhead",
            "value": 656.1963476941852,
            "unit": "ns"
          },
          {
            "name": "2/inbox_submit",
            "value": 40.20805165220211,
            "unit": "ns"
          },
          {
            "name": "3.1/registry_lookup",
            "value": 25.91433267292029,
            "unit": "ns"
          },
          {
            "name": "3.2/available_tools_50",
            "value": 30377.748141116914,
            "unit": "ns"
          },
          {
            "name": "3.3/dispatch_8_reads_concurrent",
            "value": 2138098.213888889,
            "unit": "ns"
          },
          {
            "name": "3.3/dispatch_8_reads_sequential",
            "value": 16706239.908333333,
            "unit": "ns"
          },
          {
            "name": "4.1/stdio_echo_round_trip",
            "value": 200207.1014571949,
            "unit": "ns"
          },
          {
            "name": "4.2/http_echo_round_trip",
            "value": 402891.0041208791,
            "unit": "ns"
          },
          {
            "name": "4.3/mcp_50_concurrent",
            "value": 4959551.609375,
            "unit": "ns"
          },
          {
            "name": "4.3/mcp_50_sequential",
            "value": 10031475.281746032,
            "unit": "ns"
          },
          {
            "name": "4.4/mcp_list_cold",
            "value": 212161.15736994217,
            "unit": "ns"
          },
          {
            "name": "4.4/mcp_list_hit",
            "value": 257.4304924981293,
            "unit": "ns"
          },
          {
            "name": "5.1/schema_tokens_1000",
            "value": 1270299.9302681992,
            "unit": "ns"
          },
          {
            "name": "5.1/schema_tokens_10000",
            "value": 38682535.79166667,
            "unit": "ns"
          },
          {
            "name": "5.2/index_build_10000",
            "value": 20487153.291666664,
            "unit": "ns"
          },
          {
            "name": "5.2/index_search_10000",
            "value": 328109.5245535714,
            "unit": "ns"
          },
          {
            "name": "5.3/full_schema_tokens_10000",
            "value": 19511997.3375,
            "unit": "ns"
          },
          {
            "name": "5.3/lazy_visible_prompt_10000",
            "value": 15365.43469991547,
            "unit": "ns"
          },
          {
            "name": "5.4/routed_search_10000",
            "value": 26269.0527677497,
            "unit": "ns"
          },
          {
            "name": "5.4/shrink_schemas_10000",
            "value": 57801788.08333333,
            "unit": "ns"
          },
          {
            "name": "5.5/agent_filter_10000",
            "value": 124566.61274750449,
            "unit": "ns"
          },
          {
            "name": "5.5/cache_cold_10000",
            "value": 23761629.848214284,
            "unit": "ns"
          },
          {
            "name": "5.5/cache_hit_10000",
            "value": 13.80168539335324,
            "unit": "ns"
          },
          {
            "name": "6.1/gate_check",
            "value": 3.424572611096239,
            "unit": "ns"
          },
          {
            "name": "6.1/live_tool_body",
            "value": 1824.3790517719412,
            "unit": "ns"
          },
          {
            "name": "6.1/loop_8_reads",
            "value": 2155223.8235294116,
            "unit": "ns"
          },
          {
            "name": "6.1/loop_iteration",
            "value": 404.78086566474605,
            "unit": "ns"
          },
          {
            "name": "6.10/budget_notice",
            "value": 75.88667521360391,
            "unit": "ns"
          },
          {
            "name": "6.10/cancel_select",
            "value": 95.0066687546892,
            "unit": "ns"
          },
          {
            "name": "6.10/canonical_args",
            "value": 1060.9177427339973,
            "unit": "ns"
          },
          {
            "name": "6.10/gate_check",
            "value": 7.471687746764693,
            "unit": "ns"
          },
          {
            "name": "6.10/live_tool_body",
            "value": 1723.1072382587838,
            "unit": "ns"
          },
          {
            "name": "6.10/loop_8_reads",
            "value": 2178702.2147058826,
            "unit": "ns"
          },
          {
            "name": "6.10/loop_iteration",
            "value": 761.3752298814129,
            "unit": "ns"
          },
          {
            "name": "6.10/outcome_label",
            "value": 8.40001248766352,
            "unit": "ns"
          },
          {
            "name": "6.10/stall_observe",
            "value": 137.76555576131886,
            "unit": "ns"
          },
          {
            "name": "6.10/steer_append",
            "value": 58.31756583917742,
            "unit": "ns"
          },
          {
            "name": "6.10/stress_repair",
            "value": 50203.07946346582,
            "unit": "ns"
          },
          {
            "name": "6.10/truncate_output",
            "value": 4641.927404832095,
            "unit": "ns"
          },
          {
            "name": "6.10/validate_repair",
            "value": 263.86141615979795,
            "unit": "ns"
          },
          {
            "name": "6.2/cancel_select",
            "value": 96.4154861406628,
            "unit": "ns"
          },
          {
            "name": "6.2/gate_check",
            "value": 7.468944005958227,
            "unit": "ns"
          },
          {
            "name": "6.2/live_tool_body",
            "value": 1792.4640407325721,
            "unit": "ns"
          },
          {
            "name": "6.2/loop_8_reads",
            "value": 2161797.6911764704,
            "unit": "ns"
          },
          {
            "name": "6.2/loop_iteration",
            "value": 532.4859591112152,
            "unit": "ns"
          },
          {
            "name": "6.3/cancel_select",
            "value": 99.41746475144453,
            "unit": "ns"
          },
          {
            "name": "6.3/gate_check",
            "value": 7.476795114782657,
            "unit": "ns"
          },
          {
            "name": "6.3/live_tool_body",
            "value": 1768.7564158188359,
            "unit": "ns"
          },
          {
            "name": "6.3/loop_8_reads",
            "value": 2162722.8030462186,
            "unit": "ns"
          },
          {
            "name": "6.3/loop_iteration",
            "value": 543.0704632628072,
            "unit": "ns"
          },
          {
            "name": "6.3/validate_repair",
            "value": 263.9540915274961,
            "unit": "ns"
          },
          {
            "name": "6.4/cancel_select",
            "value": 100.74433537235905,
            "unit": "ns"
          },
          {
            "name": "6.4/gate_check",
            "value": 7.474392225433693,
            "unit": "ns"
          },
          {
            "name": "6.4/live_tool_body",
            "value": 1793.3233032624814,
            "unit": "ns"
          },
          {
            "name": "6.4/loop_8_reads",
            "value": 2177320.0161764706,
            "unit": "ns"
          },
          {
            "name": "6.4/loop_iteration",
            "value": 567.9282102381327,
            "unit": "ns"
          },
          {
            "name": "6.4/stall_observe",
            "value": 138.45163313883802,
            "unit": "ns"
          },
          {
            "name": "6.4/validate_repair",
            "value": 274.4048004685101,
            "unit": "ns"
          },
          {
            "name": "6.5/budget_notice",
            "value": 76.25617201198949,
            "unit": "ns"
          },
          {
            "name": "6.5/cancel_select",
            "value": 94.75654053614238,
            "unit": "ns"
          },
          {
            "name": "6.5/gate_check",
            "value": 7.482174643519117,
            "unit": "ns"
          },
          {
            "name": "6.5/live_tool_body",
            "value": 1713.684504443086,
            "unit": "ns"
          },
          {
            "name": "6.5/loop_8_reads",
            "value": 2178699.3907563025,
            "unit": "ns"
          },
          {
            "name": "6.5/loop_iteration",
            "value": 567.0910546594832,
            "unit": "ns"
          },
          {
            "name": "6.5/stall_observe",
            "value": 138.40456241962366,
            "unit": "ns"
          },
          {
            "name": "6.5/validate_repair",
            "value": 274.228239717022,
            "unit": "ns"
          },
          {
            "name": "6.6/budget_notice",
            "value": 77.7605340409306,
            "unit": "ns"
          },
          {
            "name": "6.6/cancel_select",
            "value": 94.85464584708099,
            "unit": "ns"
          },
          {
            "name": "6.6/gate_check",
            "value": 7.4824909996831295,
            "unit": "ns"
          },
          {
            "name": "6.6/live_tool_body",
            "value": 1733.6822074458198,
            "unit": "ns"
          },
          {
            "name": "6.6/loop_8_reads",
            "value": 2177713.7720588236,
            "unit": "ns"
          },
          {
            "name": "6.6/loop_iteration",
            "value": 603.2758049162651,
            "unit": "ns"
          },
          {
            "name": "6.6/stall_observe",
            "value": 138.52655665432303,
            "unit": "ns"
          },
          {
            "name": "6.6/truncate_output",
            "value": 4675.976903047447,
            "unit": "ns"
          },
          {
            "name": "6.6/validate_repair",
            "value": 256.49521967472333,
            "unit": "ns"
          },
          {
            "name": "6.7/budget_notice",
            "value": 77.0070326509379,
            "unit": "ns"
          },
          {
            "name": "6.7/cancel_select",
            "value": 96.59816855248421,
            "unit": "ns"
          },
          {
            "name": "6.7/gate_check",
            "value": 7.4815842885680635,
            "unit": "ns"
          },
          {
            "name": "6.7/live_tool_body",
            "value": 1655.3033401826483,
            "unit": "ns"
          },
          {
            "name": "6.7/loop_8_reads",
            "value": 2175715.111111111,
            "unit": "ns"
          },
          {
            "name": "6.7/loop_iteration",
            "value": 596.3849696016257,
            "unit": "ns"
          },
          {
            "name": "6.7/stall_observe",
            "value": 136.76842510845734,
            "unit": "ns"
          },
          {
            "name": "6.7/steer_append",
            "value": 58.97784954142705,
            "unit": "ns"
          },
          {
            "name": "6.7/truncate_output",
            "value": 4699.494490030886,
            "unit": "ns"
          },
          {
            "name": "6.7/validate_repair",
            "value": 267.86053734685913,
            "unit": "ns"
          },
          {
            "name": "6.8/budget_notice",
            "value": 77.63889764364097,
            "unit": "ns"
          },
          {
            "name": "6.8/cancel_select",
            "value": 95.74825640563128,
            "unit": "ns"
          },
          {
            "name": "6.8/canonical_args",
            "value": 1050.4032487502038,
            "unit": "ns"
          },
          {
            "name": "6.8/gate_check",
            "value": 7.472961346641389,
            "unit": "ns"
          },
          {
            "name": "6.8/live_tool_body",
            "value": 1716.3569723647663,
            "unit": "ns"
          },
          {
            "name": "6.8/loop_8_reads",
            "value": 2180560.1470588236,
            "unit": "ns"
          },
          {
            "name": "6.8/loop_iteration",
            "value": 669.6719138049576,
            "unit": "ns"
          },
          {
            "name": "6.8/stall_observe",
            "value": 139.70301394304937,
            "unit": "ns"
          },
          {
            "name": "6.8/steer_append",
            "value": 58.647436459476026,
            "unit": "ns"
          },
          {
            "name": "6.8/truncate_output",
            "value": 4617.987822304078,
            "unit": "ns"
          },
          {
            "name": "6.8/validate_repair",
            "value": 266.46499955990436,
            "unit": "ns"
          },
          {
            "name": "6.9/budget_notice",
            "value": 76.5965136088152,
            "unit": "ns"
          },
          {
            "name": "6.9/cancel_select",
            "value": 97.40738762340891,
            "unit": "ns"
          },
          {
            "name": "6.9/canonical_args",
            "value": 1059.767796978747,
            "unit": "ns"
          },
          {
            "name": "6.9/gate_check",
            "value": 7.472968915823924,
            "unit": "ns"
          },
          {
            "name": "6.9/live_tool_body",
            "value": 1738.0270045057364,
            "unit": "ns"
          },
          {
            "name": "6.9/loop_8_reads",
            "value": 2182846.285947712,
            "unit": "ns"
          },
          {
            "name": "6.9/loop_iteration",
            "value": 749.3220993294881,
            "unit": "ns"
          },
          {
            "name": "6.9/outcome_label",
            "value": 8.397299406813845,
            "unit": "ns"
          },
          {
            "name": "6.9/stall_observe",
            "value": 137.90481367098263,
            "unit": "ns"
          },
          {
            "name": "6.9/steer_append",
            "value": 58.43259949106215,
            "unit": "ns"
          },
          {
            "name": "6.9/truncate_output",
            "value": 4735.561155197086,
            "unit": "ns"
          },
          {
            "name": "6.9/validate_repair",
            "value": 278.64645285173077,
            "unit": "ns"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "name": "agnathan",
            "username": "agnathan"
          },
          "committer": {
            "name": "agnathan",
            "username": "agnathan"
          },
          "id": "e957f31405a7eda15cb0fd38b7d345189428aa74",
          "message": "Record Hermes guardrails against the written chapters",
          "timestamp": "2026-10-14T00:35:05Z",
          "url": "https://github.com/agnathan/rust-agent-harness/pull/3/commits/e957f31405a7eda15cb0fd38b7d345189428aa74"
        },
        "date": 1792032492223,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "1.1/serde_50_message_request",
            "value": 10258.027589110301,
            "unit": "ns"
          },
          {
            "name": "1.2/mock_100_concurrent",
            "value": 80062.9315538847,
            "unit": "ns"
          },
          {
            "name": "1.2/mock_turn_overhead",
            "value": 250.92044938402734,
            "unit": "ns"
          },
          {
            "name": "1.3/build_request_and_parse_response",
            "value": 2941.7408080480286,
            "unit": "ns"
          },
          {
            "name": "1.4/parse_response_edges",
            "value": 1093.7650104113227,
            "unit": "ns"
          },
          {
            "name": "2/chat_turn_overhead",
            "value": 872.8849016057247,
            "unit": "ns"
          },
          {
            "name": "2/inbox_submit",
            "value": 40.33116604857217,
            "unit": "ns"
          },
          {
            "name": "3.1/registry_lookup",
            "value": 25.73129215915441,
            "unit": "ns"
          },
          {
            "name": "3.2/available_tools_50",
            "value": 30332.37600083403,
            "unit": "ns"
          },
          {
            "name": "3.3/dispatch_8_reads_concurrent",
            "value": 2135647.755882353,
            "unit": "ns"
          },
          {
            "name": "3.3/dispatch_8_reads_sequential",
            "value": 16753932.00925926,
            "unit": "ns"
          },
          {
            "name": "4.1/stdio_echo_round_trip",
            "value": 200466.25635359116,
            "unit": "ns"
          },
          {
            "name": "4.2/http_echo_round_trip",
            "value": 414248.24986471864,
            "unit": "ns"
          },
          {
            "name": "4.3/mcp_50_concurrent",
            "value": 4993026.864583334,
            "unit": "ns"
          },
          {
            "name": "4.3/mcp_50_sequential",
            "value": 10265547.055555556,
            "unit": "ns"
          },
          {
            "name": "4.4/mcp_list_cold",
            "value": 214388.91136801543,
            "unit": "ns"
          },
          {
            "name": "4.4/mcp_list_hit",
            "value": 310.49582943941925,
            "unit": "ns"
          },
          {
            "name": "5.1/schema_tokens_1000",
            "value": 1270051.392142857,
            "unit": "ns"
          },
          {
            "name": "5.1/schema_tokens_10000",
            "value": 40370890.25,
            "unit": "ns"
          },
          {
            "name": "5.2/index_build_10000",
            "value": 20991806.44047619,
            "unit": "ns"
          },
          {
            "name": "5.2/index_search_10000",
            "value": 333655.9503030303,
            "unit": "ns"
          },
          {
            "name": "5.3/full_schema_tokens_10000",
            "value": 19687428.214285716,
            "unit": "ns"
          },
          {
            "name": "5.3/lazy_visible_prompt_10000",
            "value": 15410.836289294817,
            "unit": "ns"
          },
          {
            "name": "5.4/routed_search_10000",
            "value": 26006.68622876286,
            "unit": "ns"
          },
          {
            "name": "5.4/shrink_schemas_10000",
            "value": 59189890.22916667,
            "unit": "ns"
          },
          {
            "name": "5.5/agent_filter_10000",
            "value": 126389.9664351852,
            "unit": "ns"
          },
          {
            "name": "5.5/cache_cold_10000",
            "value": 24245048,
            "unit": "ns"
          },
          {
            "name": "5.5/cache_hit_10000",
            "value": 13.803458717719614,
            "unit": "ns"
          },
          {
            "name": "6.1/gate_check",
            "value": 3.4263130981220757,
            "unit": "ns"
          },
          {
            "name": "6.1/live_tool_body",
            "value": 1705.5789785274633,
            "unit": "ns"
          },
          {
            "name": "6.1/loop_8_reads",
            "value": 2152111.271323529,
            "unit": "ns"
          },
          {
            "name": "6.1/loop_iteration",
            "value": 363.4938452507531,
            "unit": "ns"
          },
          {
            "name": "6.10/budget_notice",
            "value": 75.92593996585981,
            "unit": "ns"
          },
          {
            "name": "6.10/cancel_select",
            "value": 95.2703110140256,
            "unit": "ns"
          },
          {
            "name": "6.10/canonical_args",
            "value": 1050.355306268828,
            "unit": "ns"
          },
          {
            "name": "6.10/gate_check",
            "value": 7.479072387672966,
            "unit": "ns"
          },
          {
            "name": "6.10/live_tool_body",
            "value": 1813.8182946145096,
            "unit": "ns"
          },
          {
            "name": "6.10/loop_8_reads",
            "value": 2182395.9100840334,
            "unit": "ns"
          },
          {
            "name": "6.10/loop_iteration",
            "value": 754.8536364139726,
            "unit": "ns"
          },
          {
            "name": "6.10/outcome_label",
            "value": 8.40133525078974,
            "unit": "ns"
          },
          {
            "name": "6.10/stall_observe",
            "value": 139.26050529825113,
            "unit": "ns"
          },
          {
            "name": "6.10/steer_append",
            "value": 58.29207544115266,
            "unit": "ns"
          },
          {
            "name": "6.10/stress_repair",
            "value": 53003.73200757576,
            "unit": "ns"
          },
          {
            "name": "6.10/truncate_output",
            "value": 4868.493999821556,
            "unit": "ns"
          },
          {
            "name": "6.10/validate_repair",
            "value": 264.7259070893013,
            "unit": "ns"
          },
          {
            "name": "6.2/cancel_select",
            "value": 96.19559097301433,
            "unit": "ns"
          },
          {
            "name": "6.2/gate_check",
            "value": 7.477452369843107,
            "unit": "ns"
          },
          {
            "name": "6.2/live_tool_body",
            "value": 1763.2754880170082,
            "unit": "ns"
          },
          {
            "name": "6.2/loop_8_reads",
            "value": 2158242.011764706,
            "unit": "ns"
          },
          {
            "name": "6.2/loop_iteration",
            "value": 544.1977758522171,
            "unit": "ns"
          },
          {
            "name": "6.3/cancel_select",
            "value": 96.36203480302754,
            "unit": "ns"
          },
          {
            "name": "6.3/gate_check",
            "value": 7.471964250112887,
            "unit": "ns"
          },
          {
            "name": "6.3/live_tool_body",
            "value": 1782.4019327492497,
            "unit": "ns"
          },
          {
            "name": "6.3/loop_8_reads",
            "value": 2170693.3571428573,
            "unit": "ns"
          },
          {
            "name": "6.3/loop_iteration",
            "value": 549.5655038993257,
            "unit": "ns"
          },
          {
            "name": "6.3/validate_repair",
            "value": 264.0097723819754,
            "unit": "ns"
          },
          {
            "name": "6.4/cancel_select",
            "value": 96.60129484030165,
            "unit": "ns"
          },
          {
            "name": "6.4/gate_check",
            "value": 7.468403659077431,
            "unit": "ns"
          },
          {
            "name": "6.4/live_tool_body",
            "value": 1798.342072612631,
            "unit": "ns"
          },
          {
            "name": "6.4/loop_8_reads",
            "value": 2173801.740196078,
            "unit": "ns"
          },
          {
            "name": "6.4/loop_iteration",
            "value": 562.0058647292908,
            "unit": "ns"
          },
          {
            "name": "6.4/stall_observe",
            "value": 138.73624810246318,
            "unit": "ns"
          },
          {
            "name": "6.4/validate_repair",
            "value": 265.30074882053907,
            "unit": "ns"
          },
          {
            "name": "6.5/budget_notice",
            "value": 78.24021012818409,
            "unit": "ns"
          },
          {
            "name": "6.5/cancel_select",
            "value": 94.63886365499604,
            "unit": "ns"
          },
          {
            "name": "6.5/gate_check",
            "value": 7.481803908694021,
            "unit": "ns"
          },
          {
            "name": "6.5/live_tool_body",
            "value": 1731.1712752555266,
            "unit": "ns"
          },
          {
            "name": "6.5/loop_8_reads",
            "value": 2177056.0483193276,
            "unit": "ns"
          },
          {
            "name": "6.5/loop_iteration",
            "value": 560.418743095436,
            "unit": "ns"
          },
          {
            "name": "6.5/stall_observe",
            "value": 136.58214807118912,
            "unit": "ns"
          },
          {
            "name": "6.5/validate_repair",
            "value": 271.4553578951328,
            "unit": "ns"
          },
          {
            "name": "6.6/budget_notice",
            "value": 75.97884347777685,
            "unit": "ns"
          },
          {
            "name": "6.6/cancel_select",
            "value": 94.93830385616914,
            "unit": "ns"
          },
          {
            "name": "6.6/gate_check",
            "value": 7.478933943232073,
            "unit": "ns"
          },
          {
            "name": "6.6/live_tool_body",
            "value": 1727.5563662324175,
            "unit": "ns"
          },
          {
            "name": "6.6/loop_8_reads",
            "value": 2181988.8161764704,
            "unit": "ns"
          },
          {
            "name": "6.6/loop_iteration",
            "value": 564.8897367183617,
            "unit": "ns"
          },
          {
            "name": "6.6/stall_observe",
            "value": 137.88197624444715,
            "unit": "ns"
          },
          {
            "name": "6.6/truncate_output",
            "value": 4593.271046320839,
            "unit": "ns"
          },
          {
            "name": "6.6/validate_repair",
            "value": 264.0520805338085,
            "unit": "ns"
          },
          {
            "name": "6.7/budget_notice",
            "value": 75.91730502214608,
            "unit": "ns"
          },
          {
            "name": "6.7/cancel_select",
            "value": 96.7525994115903,
            "unit": "ns"
          },
          {
            "name": "6.7/gate_check",
            "value": 7.482659233494914,
            "unit": "ns"
          },
          {
            "name": "6.7/live_tool_body",
            "value": 1651.1992194570134,
            "unit": "ns"
          },
          {
            "name": "6.7/loop_8_reads",
            "value": 2177387.3284313725,
            "unit": "ns"
          },
          {
            "name": "6.7/loop_iteration",
            "value": 657.1744514383599,
            "unit": "ns"
          },
          {
            "name": "6.7/stall_observe",
            "value": 136.5421681599516,
            "unit": "ns"
          },
          {
            "name": "6.7/steer_append",
            "value": 58.28326721266265,
            "unit": "ns"
          },
          {
            "name": "6.7/truncate_output",
            "value": 4591.289940063091,
            "unit": "ns"
          },
          {
            "name": "6.7/validate_repair",
            "value": 266.3926283108638,
            "unit": "ns"
          },
          {
            "name": "6.8/budget_notice",
            "value": 78.03275049467004,
            "unit": "ns"
          },
          {
            "name": "6.8/cancel_select",
            "value": 96.97715837782313,
            "unit": "ns"
          },
          {
            "name": "6.8/canonical_args",
            "value": 1053.0336672611684,
            "unit": "ns"
          },
          {
            "name": "6.8/gate_check",
            "value": 7.4749739085610525,
            "unit": "ns"
          },
          {
            "name": "6.8/live_tool_body",
            "value": 1735.5452280364857,
            "unit": "ns"
          },
          {
            "name": "6.8/loop_8_reads",
            "value": 2179444.637254902,
            "unit": "ns"
          },
          {
            "name": "6.8/loop_iteration",
            "value": 686.6760751740178,
            "unit": "ns"
          },
          {
            "name": "6.8/stall_observe",
            "value": 139.52225815665668,
            "unit": "ns"
          },
          {
            "name": "6.8/steer_append",
            "value": 58.5836555831237,
            "unit": "ns"
          },
          {
            "name": "6.8/truncate_output",
            "value": 4608.9023655217115,
            "unit": "ns"
          },
          {
            "name": "6.8/validate_repair",
            "value": 260.2503218832802,
            "unit": "ns"
          },
          {
            "name": "6.9/budget_notice",
            "value": 76.37112303882816,
            "unit": "ns"
          },
          {
            "name": "6.9/cancel_select",
            "value": 96.7747162599053,
            "unit": "ns"
          },
          {
            "name": "6.9/canonical_args",
            "value": 1051.796871440583,
            "unit": "ns"
          },
          {
            "name": "6.9/gate_check",
            "value": 7.473419877801443,
            "unit": "ns"
          },
          {
            "name": "6.9/live_tool_body",
            "value": 1729.2573108526449,
            "unit": "ns"
          },
          {
            "name": "6.9/loop_8_reads",
            "value": 2182034.5588235296,
            "unit": "ns"
          },
          {
            "name": "6.9/loop_iteration",
            "value": 714.8419232201297,
            "unit": "ns"
          },
          {
            "name": "6.9/outcome_label",
            "value": 8.39628484705133,
            "unit": "ns"
          },
          {
            "name": "6.9/stall_observe",
            "value": 141.74812057371668,
            "unit": "ns"
          },
          {
            "name": "6.9/steer_append",
            "value": 58.326023616451124,
            "unit": "ns"
          },
          {
            "name": "6.9/truncate_output",
            "value": 4614.838465350169,
            "unit": "ns"
          },
          {
            "name": "6.9/validate_repair",
            "value": 276.5792562966578,
            "unit": "ns"
          }
        ]
      }
    ]
  }
}