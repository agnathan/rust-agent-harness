window.BENCHMARK_DATA = {
  "lastUpdate": 1792211222983,
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
          "id": "9002c03a3b2b2d0feeb34a42b181a2c4e2bd07b0",
          "message": "Merge pull request #2 from agnathan/cursor/fix-my-harness-workflow-4d24\n\nFix My harness workflow parse error",
          "timestamp": "2026-10-14T21:43:58-07:00",
          "tree_id": "69d26dcecc17276c30a8e3cca25bc81905d6ca09",
          "url": "https://github.com/agnathan/rust-agent-harness/commit/9002c03a3b2b2d0feeb34a42b181a2c4e2bd07b0"
        },
        "date": 1792040407739,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "1.1/serde_50_message_request",
            "value": 10279.772291666666,
            "unit": "ns"
          },
          {
            "name": "1.2/mock_100_concurrent",
            "value": 82125.58318965517,
            "unit": "ns"
          },
          {
            "name": "1.2/mock_turn_overhead",
            "value": 251.45450988524817,
            "unit": "ns"
          },
          {
            "name": "1.3/build_request_and_parse_response",
            "value": 2944.596521325727,
            "unit": "ns"
          },
          {
            "name": "1.4/parse_response_edges",
            "value": 1113.1826053437642,
            "unit": "ns"
          },
          {
            "name": "2/chat_turn_overhead",
            "value": 696.8907083799829,
            "unit": "ns"
          },
          {
            "name": "2/inbox_submit",
            "value": 40.216274154440676,
            "unit": "ns"
          },
          {
            "name": "3.1/registry_lookup",
            "value": 30.465094215235915,
            "unit": "ns"
          },
          {
            "name": "3.2/available_tools_50",
            "value": 30209.441666666666,
            "unit": "ns"
          },
          {
            "name": "3.3/dispatch_8_reads_concurrent",
            "value": 2139440.852941177,
            "unit": "ns"
          },
          {
            "name": "3.3/dispatch_8_reads_sequential",
            "value": 16817151.791666668,
            "unit": "ns"
          },
          {
            "name": "4.1/stdio_echo_round_trip",
            "value": 201729.98667582418,
            "unit": "ns"
          },
          {
            "name": "4.2/http_echo_round_trip",
            "value": 411739.95168539323,
            "unit": "ns"
          },
          {
            "name": "4.3/mcp_50_concurrent",
            "value": 5001903.770833334,
            "unit": "ns"
          },
          {
            "name": "4.3/mcp_50_sequential",
            "value": 10080692.35625,
            "unit": "ns"
          },
          {
            "name": "4.4/mcp_list_cold",
            "value": 212754.0549132948,
            "unit": "ns"
          },
          {
            "name": "4.4/mcp_list_hit",
            "value": 256.93056677409095,
            "unit": "ns"
          },
          {
            "name": "5.1/schema_tokens_1000",
            "value": 1260357.066091954,
            "unit": "ns"
          },
          {
            "name": "5.1/schema_tokens_10000",
            "value": 42102106.25,
            "unit": "ns"
          },
          {
            "name": "5.2/index_build_10000",
            "value": 21315905.027777776,
            "unit": "ns"
          },
          {
            "name": "5.2/index_search_10000",
            "value": 329749.27106060606,
            "unit": "ns"
          },
          {
            "name": "5.3/full_schema_tokens_10000",
            "value": 22239360.833333336,
            "unit": "ns"
          },
          {
            "name": "5.3/lazy_visible_prompt_10000",
            "value": 15566.998904903208,
            "unit": "ns"
          },
          {
            "name": "5.4/routed_search_10000",
            "value": 25845.40015443098,
            "unit": "ns"
          },
          {
            "name": "5.4/shrink_schemas_10000",
            "value": 61465274.483333334,
            "unit": "ns"
          },
          {
            "name": "5.5/agent_filter_10000",
            "value": 123506.14211864406,
            "unit": "ns"
          },
          {
            "name": "5.5/cache_cold_10000",
            "value": 24562693.433333334,
            "unit": "ns"
          },
          {
            "name": "5.5/cache_hit_10000",
            "value": 13.80444872027977,
            "unit": "ns"
          },
          {
            "name": "6.1/gate_check",
            "value": 3.428432889379013,
            "unit": "ns"
          },
          {
            "name": "6.1/live_tool_body",
            "value": 1808.0271549883382,
            "unit": "ns"
          },
          {
            "name": "6.1/loop_8_reads",
            "value": 2159048.1647058823,
            "unit": "ns"
          },
          {
            "name": "6.1/loop_iteration",
            "value": 363.228087835289,
            "unit": "ns"
          },
          {
            "name": "6.10/budget_notice",
            "value": 75.99470247859433,
            "unit": "ns"
          },
          {
            "name": "6.10/cancel_select",
            "value": 95.26072257280089,
            "unit": "ns"
          },
          {
            "name": "6.10/canonical_args",
            "value": 1053.8103028377805,
            "unit": "ns"
          },
          {
            "name": "6.10/gate_check",
            "value": 7.470861694636881,
            "unit": "ns"
          },
          {
            "name": "6.10/live_tool_body",
            "value": 1786.8931102426714,
            "unit": "ns"
          },
          {
            "name": "6.10/loop_8_reads",
            "value": 2181481.7841386553,
            "unit": "ns"
          },
          {
            "name": "6.10/loop_iteration",
            "value": 764.5321432731023,
            "unit": "ns"
          },
          {
            "name": "6.10/outcome_label",
            "value": 8.394091421517203,
            "unit": "ns"
          },
          {
            "name": "6.10/stall_observe",
            "value": 138.3667316502029,
            "unit": "ns"
          },
          {
            "name": "6.10/steer_append",
            "value": 58.35359566063539,
            "unit": "ns"
          },
          {
            "name": "6.10/stress_repair",
            "value": 49109.04546052632,
            "unit": "ns"
          },
          {
            "name": "6.10/truncate_output",
            "value": 4602.345993427705,
            "unit": "ns"
          },
          {
            "name": "6.10/validate_repair",
            "value": 258.94206004234223,
            "unit": "ns"
          },
          {
            "name": "6.2/cancel_select",
            "value": 101.99283103633209,
            "unit": "ns"
          },
          {
            "name": "6.2/gate_check",
            "value": 7.47352021110307,
            "unit": "ns"
          },
          {
            "name": "6.2/live_tool_body",
            "value": 1785.9911205511467,
            "unit": "ns"
          },
          {
            "name": "6.2/loop_8_reads",
            "value": 2160795.461764706,
            "unit": "ns"
          },
          {
            "name": "6.2/loop_iteration",
            "value": 537.6461912133932,
            "unit": "ns"
          },
          {
            "name": "6.3/cancel_select",
            "value": 96.16788186141814,
            "unit": "ns"
          },
          {
            "name": "6.3/gate_check",
            "value": 7.480816708872709,
            "unit": "ns"
          },
          {
            "name": "6.3/live_tool_body",
            "value": 1779.3825972743675,
            "unit": "ns"
          },
          {
            "name": "6.3/loop_8_reads",
            "value": 2174824.0470588235,
            "unit": "ns"
          },
          {
            "name": "6.3/loop_iteration",
            "value": 586.9948262962722,
            "unit": "ns"
          },
          {
            "name": "6.3/validate_repair",
            "value": 256.35499530377183,
            "unit": "ns"
          },
          {
            "name": "6.4/cancel_select",
            "value": 99.92865855798223,
            "unit": "ns"
          },
          {
            "name": "6.4/gate_check",
            "value": 7.47370417759747,
            "unit": "ns"
          },
          {
            "name": "6.4/live_tool_body",
            "value": 1730.1857022196964,
            "unit": "ns"
          },
          {
            "name": "6.4/loop_8_reads",
            "value": 2175765.755718954,
            "unit": "ns"
          },
          {
            "name": "6.4/loop_iteration",
            "value": 588.692899050483,
            "unit": "ns"
          },
          {
            "name": "6.4/stall_observe",
            "value": 138.93544076467074,
            "unit": "ns"
          },
          {
            "name": "6.4/validate_repair",
            "value": 265.29977412723497,
            "unit": "ns"
          },
          {
            "name": "6.5/budget_notice",
            "value": 75.29996109791404,
            "unit": "ns"
          },
          {
            "name": "6.5/cancel_select",
            "value": 94.79147265591061,
            "unit": "ns"
          },
          {
            "name": "6.5/gate_check",
            "value": 7.480401966452211,
            "unit": "ns"
          },
          {
            "name": "6.5/live_tool_body",
            "value": 1743.6021454112038,
            "unit": "ns"
          },
          {
            "name": "6.5/loop_8_reads",
            "value": 2178837.400653595,
            "unit": "ns"
          },
          {
            "name": "6.5/loop_iteration",
            "value": 602.190096769702,
            "unit": "ns"
          },
          {
            "name": "6.5/stall_observe",
            "value": 136.09949579264475,
            "unit": "ns"
          },
          {
            "name": "6.5/validate_repair",
            "value": 275.92511214199203,
            "unit": "ns"
          },
          {
            "name": "6.6/budget_notice",
            "value": 76.27710814727203,
            "unit": "ns"
          },
          {
            "name": "6.6/cancel_select",
            "value": 99.06883025791231,
            "unit": "ns"
          },
          {
            "name": "6.6/gate_check",
            "value": 7.478082464476513,
            "unit": "ns"
          },
          {
            "name": "6.6/live_tool_body",
            "value": 1721.4441816235967,
            "unit": "ns"
          },
          {
            "name": "6.6/loop_8_reads",
            "value": 2181938.4673202615,
            "unit": "ns"
          },
          {
            "name": "6.6/loop_iteration",
            "value": 614.5043989827003,
            "unit": "ns"
          },
          {
            "name": "6.6/stall_observe",
            "value": 137.31550577811873,
            "unit": "ns"
          },
          {
            "name": "6.6/truncate_output",
            "value": 4777.808613114754,
            "unit": "ns"
          },
          {
            "name": "6.6/validate_repair",
            "value": 266.77594441810425,
            "unit": "ns"
          },
          {
            "name": "6.7/budget_notice",
            "value": 77.37294398639996,
            "unit": "ns"
          },
          {
            "name": "6.7/cancel_select",
            "value": 96.5181326380082,
            "unit": "ns"
          },
          {
            "name": "6.7/gate_check",
            "value": 7.473626683066568,
            "unit": "ns"
          },
          {
            "name": "6.7/live_tool_body",
            "value": 1696.9111316465771,
            "unit": "ns"
          },
          {
            "name": "6.7/loop_8_reads",
            "value": 2183712.0764705883,
            "unit": "ns"
          },
          {
            "name": "6.7/loop_iteration",
            "value": 605.6487152992754,
            "unit": "ns"
          },
          {
            "name": "6.7/stall_observe",
            "value": 135.0182621975468,
            "unit": "ns"
          },
          {
            "name": "6.7/steer_append",
            "value": 58.29211846163078,
            "unit": "ns"
          },
          {
            "name": "6.7/truncate_output",
            "value": 4770.520043590123,
            "unit": "ns"
          },
          {
            "name": "6.7/validate_repair",
            "value": 259.36120673819016,
            "unit": "ns"
          },
          {
            "name": "6.8/budget_notice",
            "value": 76.29567010756523,
            "unit": "ns"
          },
          {
            "name": "6.8/cancel_select",
            "value": 95.03260528230636,
            "unit": "ns"
          },
          {
            "name": "6.8/canonical_args",
            "value": 1043.7039711705552,
            "unit": "ns"
          },
          {
            "name": "6.8/gate_check",
            "value": 7.474369186794235,
            "unit": "ns"
          },
          {
            "name": "6.8/live_tool_body",
            "value": 1706.3787952597313,
            "unit": "ns"
          },
          {
            "name": "6.8/loop_8_reads",
            "value": 2188493.843137255,
            "unit": "ns"
          },
          {
            "name": "6.8/loop_iteration",
            "value": 706.5018913232548,
            "unit": "ns"
          },
          {
            "name": "6.8/stall_observe",
            "value": 136.0627135442221,
            "unit": "ns"
          },
          {
            "name": "6.8/steer_append",
            "value": 58.57951192074434,
            "unit": "ns"
          },
          {
            "name": "6.8/truncate_output",
            "value": 4561.592309622381,
            "unit": "ns"
          },
          {
            "name": "6.8/validate_repair",
            "value": 268.66418280590096,
            "unit": "ns"
          },
          {
            "name": "6.9/budget_notice",
            "value": 78.14380385178029,
            "unit": "ns"
          },
          {
            "name": "6.9/cancel_select",
            "value": 96.7426241445354,
            "unit": "ns"
          },
          {
            "name": "6.9/canonical_args",
            "value": 1075.2771013438014,
            "unit": "ns"
          },
          {
            "name": "6.9/gate_check",
            "value": 7.473300678781284,
            "unit": "ns"
          },
          {
            "name": "6.9/live_tool_body",
            "value": 1759.3484750619202,
            "unit": "ns"
          },
          {
            "name": "6.9/loop_8_reads",
            "value": 2190076.981372549,
            "unit": "ns"
          },
          {
            "name": "6.9/loop_iteration",
            "value": 708.5040340604327,
            "unit": "ns"
          },
          {
            "name": "6.9/outcome_label",
            "value": 8.397131172812063,
            "unit": "ns"
          },
          {
            "name": "6.9/stall_observe",
            "value": 138.71671442810865,
            "unit": "ns"
          },
          {
            "name": "6.9/steer_append",
            "value": 58.27641322239871,
            "unit": "ns"
          },
          {
            "name": "6.9/truncate_output",
            "value": 4754.882122414934,
            "unit": "ns"
          },
          {
            "name": "6.9/validate_repair",
            "value": 270.7003314038594,
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
          "id": "4307f3d976977969cf8f32d4649da0974ee6a0c2",
          "message": "Merge pull request #3 from agnathan/cursor/hermes-guardrails-notes-4d24\n\nRecord Hermes guardrails against the written chapters",
          "timestamp": "2026-10-14T21:44:08-07:00",
          "tree_id": "b0dc440bfedf056647264d9535b33a5e906738d5",
          "url": "https://github.com/agnathan/rust-agent-harness/commit/4307f3d976977969cf8f32d4649da0974ee6a0c2"
        },
        "date": 1792040414118,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "1.1/serde_50_message_request",
            "value": 10303.499431979551,
            "unit": "ns"
          },
          {
            "name": "1.2/mock_100_concurrent",
            "value": 78410.95656779662,
            "unit": "ns"
          },
          {
            "name": "1.2/mock_turn_overhead",
            "value": 246.8461732684168,
            "unit": "ns"
          },
          {
            "name": "1.3/build_request_and_parse_response",
            "value": 2975.9901597099743,
            "unit": "ns"
          },
          {
            "name": "1.4/parse_response_edges",
            "value": 1114.7215283118837,
            "unit": "ns"
          },
          {
            "name": "2/chat_turn_overhead",
            "value": 683.2366783030398,
            "unit": "ns"
          },
          {
            "name": "2/inbox_submit",
            "value": 40.1997713865119,
            "unit": "ns"
          },
          {
            "name": "3.1/registry_lookup",
            "value": 25.69310387872936,
            "unit": "ns"
          },
          {
            "name": "3.2/available_tools_50",
            "value": 30293.327081604424,
            "unit": "ns"
          },
          {
            "name": "3.3/dispatch_8_reads_concurrent",
            "value": 2135160.662698413,
            "unit": "ns"
          },
          {
            "name": "3.3/dispatch_8_reads_sequential",
            "value": 16757741.155555556,
            "unit": "ns"
          },
          {
            "name": "4.1/stdio_echo_round_trip",
            "value": 201211.7573369565,
            "unit": "ns"
          },
          {
            "name": "4.2/http_echo_round_trip",
            "value": 403140.27111111116,
            "unit": "ns"
          },
          {
            "name": "4.3/mcp_50_concurrent",
            "value": 4934066.5,
            "unit": "ns"
          },
          {
            "name": "4.3/mcp_50_sequential",
            "value": 9945135.0125,
            "unit": "ns"
          },
          {
            "name": "4.4/mcp_list_cold",
            "value": 209181.9290229885,
            "unit": "ns"
          },
          {
            "name": "4.4/mcp_list_hit",
            "value": 260.81729044462224,
            "unit": "ns"
          },
          {
            "name": "5.1/schema_tokens_1000",
            "value": 1257730.7112068967,
            "unit": "ns"
          },
          {
            "name": "5.1/schema_tokens_10000",
            "value": 40232959.5,
            "unit": "ns"
          },
          {
            "name": "5.2/index_build_10000",
            "value": 20998891.666666664,
            "unit": "ns"
          },
          {
            "name": "5.2/index_search_10000",
            "value": 335307.5194954128,
            "unit": "ns"
          },
          {
            "name": "5.3/full_schema_tokens_10000",
            "value": 20063351.35,
            "unit": "ns"
          },
          {
            "name": "5.3/lazy_visible_prompt_10000",
            "value": 15693.520641163792,
            "unit": "ns"
          },
          {
            "name": "5.4/routed_search_10000",
            "value": 25667.0806227758,
            "unit": "ns"
          },
          {
            "name": "5.4/shrink_schemas_10000",
            "value": 58901073.69444445,
            "unit": "ns"
          },
          {
            "name": "5.5/agent_filter_10000",
            "value": 123890.97147732813,
            "unit": "ns"
          },
          {
            "name": "5.5/cache_cold_10000",
            "value": 24221528.5125,
            "unit": "ns"
          },
          {
            "name": "5.5/cache_hit_10000",
            "value": 13.787751759281727,
            "unit": "ns"
          },
          {
            "name": "6.1/gate_check",
            "value": 3.424898500972953,
            "unit": "ns"
          },
          {
            "name": "6.1/live_tool_body",
            "value": 1768.1495938983667,
            "unit": "ns"
          },
          {
            "name": "6.1/loop_8_reads",
            "value": 2152736.521848739,
            "unit": "ns"
          },
          {
            "name": "6.1/loop_iteration",
            "value": 334.3658486922782,
            "unit": "ns"
          },
          {
            "name": "6.10/budget_notice",
            "value": 77.78322097713647,
            "unit": "ns"
          },
          {
            "name": "6.10/cancel_select",
            "value": 95.0435255402102,
            "unit": "ns"
          },
          {
            "name": "6.10/canonical_args",
            "value": 1046.9425794051224,
            "unit": "ns"
          },
          {
            "name": "6.10/gate_check",
            "value": 7.470760145478455,
            "unit": "ns"
          },
          {
            "name": "6.10/live_tool_body",
            "value": 1717.5887890662334,
            "unit": "ns"
          },
          {
            "name": "6.10/loop_8_reads",
            "value": 2178513.506302521,
            "unit": "ns"
          },
          {
            "name": "6.10/loop_iteration",
            "value": 759.9650228041246,
            "unit": "ns"
          },
          {
            "name": "6.10/outcome_label",
            "value": 8.39959006501959,
            "unit": "ns"
          },
          {
            "name": "6.10/stall_observe",
            "value": 137.80271676743655,
            "unit": "ns"
          },
          {
            "name": "6.10/steer_append",
            "value": 58.69242395636161,
            "unit": "ns"
          },
          {
            "name": "6.10/stress_repair",
            "value": 50614.85060060061,
            "unit": "ns"
          },
          {
            "name": "6.10/truncate_output",
            "value": 4771.171949646825,
            "unit": "ns"
          },
          {
            "name": "6.10/validate_repair",
            "value": 265.6944219133878,
            "unit": "ns"
          },
          {
            "name": "6.2/cancel_select",
            "value": 123.91819976220839,
            "unit": "ns"
          },
          {
            "name": "6.2/gate_check",
            "value": 7.4641976696238626,
            "unit": "ns"
          },
          {
            "name": "6.2/live_tool_body",
            "value": 1780.8949990062606,
            "unit": "ns"
          },
          {
            "name": "6.2/loop_8_reads",
            "value": 2149892.2955882354,
            "unit": "ns"
          },
          {
            "name": "6.2/loop_iteration",
            "value": 486.3273449880576,
            "unit": "ns"
          },
          {
            "name": "6.3/cancel_select",
            "value": 96.18460025491447,
            "unit": "ns"
          },
          {
            "name": "6.3/gate_check",
            "value": 7.469619331878832,
            "unit": "ns"
          },
          {
            "name": "6.3/live_tool_body",
            "value": 1756.1861668062465,
            "unit": "ns"
          },
          {
            "name": "6.3/loop_8_reads",
            "value": 2168808.93627451,
            "unit": "ns"
          },
          {
            "name": "6.3/loop_iteration",
            "value": 574.4139610137361,
            "unit": "ns"
          },
          {
            "name": "6.3/validate_repair",
            "value": 256.7834605315591,
            "unit": "ns"
          },
          {
            "name": "6.4/cancel_select",
            "value": 94.99459899677333,
            "unit": "ns"
          },
          {
            "name": "6.4/gate_check",
            "value": 7.470064814674103,
            "unit": "ns"
          },
          {
            "name": "6.4/live_tool_body",
            "value": 1778.7193758163967,
            "unit": "ns"
          },
          {
            "name": "6.4/loop_8_reads",
            "value": 2174257.237254902,
            "unit": "ns"
          },
          {
            "name": "6.4/loop_iteration",
            "value": 564.3511268118771,
            "unit": "ns"
          },
          {
            "name": "6.4/stall_observe",
            "value": 138.5506899467236,
            "unit": "ns"
          },
          {
            "name": "6.4/validate_repair",
            "value": 244.48335424011154,
            "unit": "ns"
          },
          {
            "name": "6.5/budget_notice",
            "value": 80.73053955422253,
            "unit": "ns"
          },
          {
            "name": "6.5/cancel_select",
            "value": 95.63289382085792,
            "unit": "ns"
          },
          {
            "name": "6.5/gate_check",
            "value": 7.49982306728941,
            "unit": "ns"
          },
          {
            "name": "6.5/live_tool_body",
            "value": 1782.4752589602685,
            "unit": "ns"
          },
          {
            "name": "6.5/loop_8_reads",
            "value": 2174111.2034313725,
            "unit": "ns"
          },
          {
            "name": "6.5/loop_iteration",
            "value": 610.3548095192677,
            "unit": "ns"
          },
          {
            "name": "6.5/stall_observe",
            "value": 139.14412554727053,
            "unit": "ns"
          },
          {
            "name": "6.5/validate_repair",
            "value": 277.5888794473466,
            "unit": "ns"
          },
          {
            "name": "6.6/budget_notice",
            "value": 75.98142518039144,
            "unit": "ns"
          },
          {
            "name": "6.6/cancel_select",
            "value": 95.0102066566831,
            "unit": "ns"
          },
          {
            "name": "6.6/gate_check",
            "value": 7.48839259224716,
            "unit": "ns"
          },
          {
            "name": "6.6/live_tool_body",
            "value": 1762.930595843835,
            "unit": "ns"
          },
          {
            "name": "6.6/loop_8_reads",
            "value": 2172550.6411764706,
            "unit": "ns"
          },
          {
            "name": "6.6/loop_iteration",
            "value": 600.0070451989914,
            "unit": "ns"
          },
          {
            "name": "6.6/stall_observe",
            "value": 138.60884358543845,
            "unit": "ns"
          },
          {
            "name": "6.6/truncate_output",
            "value": 4758.09491449481,
            "unit": "ns"
          },
          {
            "name": "6.6/validate_repair",
            "value": 256.4699624202135,
            "unit": "ns"
          },
          {
            "name": "6.7/budget_notice",
            "value": 76.2359786541017,
            "unit": "ns"
          },
          {
            "name": "6.7/cancel_select",
            "value": 97.16305572064398,
            "unit": "ns"
          },
          {
            "name": "6.7/gate_check",
            "value": 7.475659356482627,
            "unit": "ns"
          },
          {
            "name": "6.7/live_tool_body",
            "value": 1626.9922443953933,
            "unit": "ns"
          },
          {
            "name": "6.7/loop_8_reads",
            "value": 2174933.869607843,
            "unit": "ns"
          },
          {
            "name": "6.7/loop_iteration",
            "value": 602.443856725826,
            "unit": "ns"
          },
          {
            "name": "6.7/stall_observe",
            "value": 136.5731305932133,
            "unit": "ns"
          },
          {
            "name": "6.7/steer_append",
            "value": 58.66133144900695,
            "unit": "ns"
          },
          {
            "name": "6.7/truncate_output",
            "value": 4671.14930211902,
            "unit": "ns"
          },
          {
            "name": "6.7/validate_repair",
            "value": 266.7872738006417,
            "unit": "ns"
          },
          {
            "name": "6.8/budget_notice",
            "value": 81.57320658953975,
            "unit": "ns"
          },
          {
            "name": "6.8/cancel_select",
            "value": 97.20516211136497,
            "unit": "ns"
          },
          {
            "name": "6.8/canonical_args",
            "value": 1058.7747319716427,
            "unit": "ns"
          },
          {
            "name": "6.8/gate_check",
            "value": 7.471859584172866,
            "unit": "ns"
          },
          {
            "name": "6.8/live_tool_body",
            "value": 1772.9721817500285,
            "unit": "ns"
          },
          {
            "name": "6.8/loop_8_reads",
            "value": 2179249.6794117647,
            "unit": "ns"
          },
          {
            "name": "6.8/loop_iteration",
            "value": 698.8217058898562,
            "unit": "ns"
          },
          {
            "name": "6.8/stall_observe",
            "value": 138.33018384555635,
            "unit": "ns"
          },
          {
            "name": "6.8/steer_append",
            "value": 58.898746298132366,
            "unit": "ns"
          },
          {
            "name": "6.8/truncate_output",
            "value": 4582.389748110831,
            "unit": "ns"
          },
          {
            "name": "6.8/validate_repair",
            "value": 260.5337981631397,
            "unit": "ns"
          },
          {
            "name": "6.9/budget_notice",
            "value": 77.59619630801168,
            "unit": "ns"
          },
          {
            "name": "6.9/cancel_select",
            "value": 97.01845494616624,
            "unit": "ns"
          },
          {
            "name": "6.9/canonical_args",
            "value": 1058.341342394005,
            "unit": "ns"
          },
          {
            "name": "6.9/gate_check",
            "value": 7.466516592532247,
            "unit": "ns"
          },
          {
            "name": "6.9/live_tool_body",
            "value": 1714.6308444907584,
            "unit": "ns"
          },
          {
            "name": "6.9/loop_8_reads",
            "value": 2180062.4764705882,
            "unit": "ns"
          },
          {
            "name": "6.9/loop_iteration",
            "value": 744.8857118208657,
            "unit": "ns"
          },
          {
            "name": "6.9/outcome_label",
            "value": 8.397074494760265,
            "unit": "ns"
          },
          {
            "name": "6.9/stall_observe",
            "value": 137.73863307755363,
            "unit": "ns"
          },
          {
            "name": "6.9/steer_append",
            "value": 58.3767820090342,
            "unit": "ns"
          },
          {
            "name": "6.9/truncate_output",
            "value": 4732.470920556494,
            "unit": "ns"
          },
          {
            "name": "6.9/validate_repair",
            "value": 270.72924637774526,
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
          "id": "c82fe4c8f4ce04f10ccc0fb384fd8d01c6d8b126",
          "message": "Write chapters 5, 7, 8, and 10",
          "timestamp": "2026-10-15T04:44:13Z",
          "url": "https://github.com/agnathan/rust-agent-harness/pull/4/commits/c82fe4c8f4ce04f10ccc0fb384fd8d01c6d8b126"
        },
        "date": 1792133297400,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "1.1/serde_50_message_request",
            "value": 5837.982183709637,
            "unit": "ns"
          },
          {
            "name": "1.2/mock_100_concurrent",
            "value": 58485.16236722306,
            "unit": "ns"
          },
          {
            "name": "1.2/mock_turn_overhead",
            "value": 174.54986733682912,
            "unit": "ns"
          },
          {
            "name": "1.3/build_request_and_parse_response",
            "value": 1439.7418965926158,
            "unit": "ns"
          },
          {
            "name": "1.4/parse_response_edges",
            "value": 641.2050859158926,
            "unit": "ns"
          },
          {
            "name": "10/scripted_turn",
            "value": 3.6803942313079947,
            "unit": "ns"
          },
          {
            "name": "10/turn_transition",
            "value": 0.6508149659423969,
            "unit": "ns"
          },
          {
            "name": "2/chat_turn_overhead",
            "value": 444.7803017905276,
            "unit": "ns"
          },
          {
            "name": "2/inbox_submit",
            "value": 29.605765050842688,
            "unit": "ns"
          },
          {
            "name": "3.1/registry_lookup",
            "value": 11.894101309927912,
            "unit": "ns"
          },
          {
            "name": "3.2/available_tools_50",
            "value": 17334.78084802287,
            "unit": "ns"
          },
          {
            "name": "3.3/dispatch_8_reads_concurrent",
            "value": 2114714.6296296297,
            "unit": "ns"
          },
          {
            "name": "3.3/dispatch_8_reads_sequential",
            "value": 16726885.404761905,
            "unit": "ns"
          },
          {
            "name": "4.1/stdio_echo_round_trip",
            "value": 81410.82028337062,
            "unit": "ns"
          },
          {
            "name": "4.2/http_echo_round_trip",
            "value": 217226.70644312951,
            "unit": "ns"
          },
          {
            "name": "4.3/mcp_50_concurrent",
            "value": 2184474.017973856,
            "unit": "ns"
          },
          {
            "name": "4.3/mcp_50_sequential",
            "value": 4146435.8353174604,
            "unit": "ns"
          },
          {
            "name": "4.4/mcp_list_cold",
            "value": 89046.54232723577,
            "unit": "ns"
          },
          {
            "name": "4.4/mcp_list_hit",
            "value": 182.39647305565586,
            "unit": "ns"
          },
          {
            "name": "5.1/schema_tokens_1000",
            "value": 662446.317442602,
            "unit": "ns"
          },
          {
            "name": "5.1/schema_tokens_10000",
            "value": 25917049.166666664,
            "unit": "ns"
          },
          {
            "name": "5.2/index_build_10000",
            "value": 12322512.708333332,
            "unit": "ns"
          },
          {
            "name": "5.2/index_search_10000",
            "value": 119856.3201320132,
            "unit": "ns"
          },
          {
            "name": "5.3/full_schema_tokens_10000",
            "value": 13262821.722222222,
            "unit": "ns"
          },
          {
            "name": "5.3/lazy_visible_prompt_10000",
            "value": 10258.113902198422,
            "unit": "ns"
          },
          {
            "name": "5.4/routed_search_10000",
            "value": 12348.758983050848,
            "unit": "ns"
          },
          {
            "name": "5.4/shrink_schemas_10000",
            "value": 43102519.72321428,
            "unit": "ns"
          },
          {
            "name": "5.5/agent_filter_10000",
            "value": 53421.81656482652,
            "unit": "ns"
          },
          {
            "name": "5.5/cache_cold_10000",
            "value": 12681938.775,
            "unit": "ns"
          },
          {
            "name": "5.5/cache_hit_10000",
            "value": 14.839435979362793,
            "unit": "ns"
          },
          {
            "name": "6.1/gate_check",
            "value": 1.8006935084448803,
            "unit": "ns"
          },
          {
            "name": "6.1/live_tool_body",
            "value": 1178.2808689817507,
            "unit": "ns"
          },
          {
            "name": "6.1/loop_8_reads",
            "value": 2124917.7142857146,
            "unit": "ns"
          },
          {
            "name": "6.1/loop_iteration",
            "value": 210.38618440778072,
            "unit": "ns"
          },
          {
            "name": "6.10/budget_notice",
            "value": 40.61884670615097,
            "unit": "ns"
          },
          {
            "name": "6.10/cancel_select",
            "value": 144.93382085771168,
            "unit": "ns"
          },
          {
            "name": "6.10/canonical_args",
            "value": 629.0872258629734,
            "unit": "ns"
          },
          {
            "name": "6.10/gate_check",
            "value": 11.308465584155908,
            "unit": "ns"
          },
          {
            "name": "6.10/live_tool_body",
            "value": 1194.5698978977994,
            "unit": "ns"
          },
          {
            "name": "6.10/loop_8_reads",
            "value": 2138000.9831932774,
            "unit": "ns"
          },
          {
            "name": "6.10/loop_iteration",
            "value": 577.1695187165776,
            "unit": "ns"
          },
          {
            "name": "6.10/outcome_label",
            "value": 5.092088171807188,
            "unit": "ns"
          },
          {
            "name": "6.10/stall_observe",
            "value": 65.22802292961455,
            "unit": "ns"
          },
          {
            "name": "6.10/steer_append",
            "value": 39.95422345412826,
            "unit": "ns"
          },
          {
            "name": "6.10/stress_repair",
            "value": 25035.394736842107,
            "unit": "ns"
          },
          {
            "name": "6.10/truncate_output",
            "value": 2111.3869483882672,
            "unit": "ns"
          },
          {
            "name": "6.10/validate_repair",
            "value": 172.50098900429026,
            "unit": "ns"
          },
          {
            "name": "6.2/cancel_select",
            "value": 142.78554297551395,
            "unit": "ns"
          },
          {
            "name": "6.2/gate_check",
            "value": 11.302581950160082,
            "unit": "ns"
          },
          {
            "name": "6.2/live_tool_body",
            "value": 1194.659817314291,
            "unit": "ns"
          },
          {
            "name": "6.2/loop_8_reads",
            "value": 2122522.1634920635,
            "unit": "ns"
          },
          {
            "name": "6.2/loop_iteration",
            "value": 395.23713167835945,
            "unit": "ns"
          },
          {
            "name": "6.3/cancel_select",
            "value": 141.46881043737312,
            "unit": "ns"
          },
          {
            "name": "6.3/gate_check",
            "value": 11.303025711772179,
            "unit": "ns"
          },
          {
            "name": "6.3/live_tool_body",
            "value": 1252.8641525550852,
            "unit": "ns"
          },
          {
            "name": "6.3/loop_8_reads",
            "value": 2134301.8303571427,
            "unit": "ns"
          },
          {
            "name": "6.3/loop_iteration",
            "value": 399.51503347053233,
            "unit": "ns"
          },
          {
            "name": "6.3/validate_repair",
            "value": 173.58310902361438,
            "unit": "ns"
          },
          {
            "name": "6.4/cancel_select",
            "value": 145.08502439327685,
            "unit": "ns"
          },
          {
            "name": "6.4/gate_check",
            "value": 11.29896034943981,
            "unit": "ns"
          },
          {
            "name": "6.4/live_tool_body",
            "value": 1198.9159020976708,
            "unit": "ns"
          },
          {
            "name": "6.4/loop_8_reads",
            "value": 2133758.0925925924,
            "unit": "ns"
          },
          {
            "name": "6.4/loop_iteration",
            "value": 420.4525483190181,
            "unit": "ns"
          },
          {
            "name": "6.4/stall_observe",
            "value": 63.6963805972111,
            "unit": "ns"
          },
          {
            "name": "6.4/validate_repair",
            "value": 172.5418678651763,
            "unit": "ns"
          },
          {
            "name": "6.5/budget_notice",
            "value": 39.973511906496874,
            "unit": "ns"
          },
          {
            "name": "6.5/cancel_select",
            "value": 143.97292875239617,
            "unit": "ns"
          },
          {
            "name": "6.5/gate_check",
            "value": 11.301330187876054,
            "unit": "ns"
          },
          {
            "name": "6.5/live_tool_body",
            "value": 1249.842394342195,
            "unit": "ns"
          },
          {
            "name": "6.5/loop_8_reads",
            "value": 2132957.7944444446,
            "unit": "ns"
          },
          {
            "name": "6.5/loop_iteration",
            "value": 444.4989072684567,
            "unit": "ns"
          },
          {
            "name": "6.5/stall_observe",
            "value": 65.56435518577109,
            "unit": "ns"
          },
          {
            "name": "6.5/validate_repair",
            "value": 164.112310341408,
            "unit": "ns"
          },
          {
            "name": "6.6/budget_notice",
            "value": 41.68166735236768,
            "unit": "ns"
          },
          {
            "name": "6.6/cancel_select",
            "value": 146.50130598478313,
            "unit": "ns"
          },
          {
            "name": "6.6/gate_check",
            "value": 11.298153076417798,
            "unit": "ns"
          },
          {
            "name": "6.6/live_tool_body",
            "value": 1181.523898520902,
            "unit": "ns"
          },
          {
            "name": "6.6/loop_8_reads",
            "value": 2140170.1574074076,
            "unit": "ns"
          },
          {
            "name": "6.6/loop_iteration",
            "value": 436.61164613817294,
            "unit": "ns"
          },
          {
            "name": "6.6/stall_observe",
            "value": 66.08614504311569,
            "unit": "ns"
          },
          {
            "name": "6.6/truncate_output",
            "value": 2181.3844373001293,
            "unit": "ns"
          },
          {
            "name": "6.6/validate_repair",
            "value": 172.6036183225359,
            "unit": "ns"
          },
          {
            "name": "6.7/budget_notice",
            "value": 41.80632706253269,
            "unit": "ns"
          },
          {
            "name": "6.7/cancel_select",
            "value": 139.12944431614113,
            "unit": "ns"
          },
          {
            "name": "6.7/gate_check",
            "value": 11.298000078629693,
            "unit": "ns"
          },
          {
            "name": "6.7/live_tool_body",
            "value": 1196.502684954281,
            "unit": "ns"
          },
          {
            "name": "6.7/loop_8_reads",
            "value": 2137726.811728395,
            "unit": "ns"
          },
          {
            "name": "6.7/loop_iteration",
            "value": 444.0571319180726,
            "unit": "ns"
          },
          {
            "name": "6.7/stall_observe",
            "value": 66.2381110781678,
            "unit": "ns"
          },
          {
            "name": "6.7/steer_append",
            "value": 39.91761221552643,
            "unit": "ns"
          },
          {
            "name": "6.7/truncate_output",
            "value": 2313.778962345364,
            "unit": "ns"
          },
          {
            "name": "6.7/validate_repair",
            "value": 162.43034231734293,
            "unit": "ns"
          },
          {
            "name": "6.8/budget_notice",
            "value": 42.05798264042669,
            "unit": "ns"
          },
          {
            "name": "6.8/cancel_select",
            "value": 141.44532227008187,
            "unit": "ns"
          },
          {
            "name": "6.8/canonical_args",
            "value": 632.3994005976663,
            "unit": "ns"
          },
          {
            "name": "6.8/gate_check",
            "value": 11.30542355974659,
            "unit": "ns"
          },
          {
            "name": "6.8/live_tool_body",
            "value": 1268.7202208502908,
            "unit": "ns"
          },
          {
            "name": "6.8/loop_8_reads",
            "value": 2142062.864197531,
            "unit": "ns"
          },
          {
            "name": "6.8/loop_iteration",
            "value": 541.6732341782674,
            "unit": "ns"
          },
          {
            "name": "6.8/stall_observe",
            "value": 65.38529754300438,
            "unit": "ns"
          },
          {
            "name": "6.8/steer_append",
            "value": 39.92392811020587,
            "unit": "ns"
          },
          {
            "name": "6.8/truncate_output",
            "value": 2672.2717630738625,
            "unit": "ns"
          },
          {
            "name": "6.8/validate_repair",
            "value": 164.73776297738323,
            "unit": "ns"
          },
          {
            "name": "6.9/budget_notice",
            "value": 40.67747381197276,
            "unit": "ns"
          },
          {
            "name": "6.9/cancel_select",
            "value": 139.44644814586076,
            "unit": "ns"
          },
          {
            "name": "6.9/canonical_args",
            "value": 627.4898971591797,
            "unit": "ns"
          },
          {
            "name": "6.9/gate_check",
            "value": 11.30071391206968,
            "unit": "ns"
          },
          {
            "name": "6.9/live_tool_body",
            "value": 1255.9334421875712,
            "unit": "ns"
          },
          {
            "name": "6.9/loop_8_reads",
            "value": 2139535.375,
            "unit": "ns"
          },
          {
            "name": "6.9/loop_iteration",
            "value": 546.7865358761619,
            "unit": "ns"
          },
          {
            "name": "6.9/outcome_label",
            "value": 5.09209652412118,
            "unit": "ns"
          },
          {
            "name": "6.9/stall_observe",
            "value": 65.32547794736553,
            "unit": "ns"
          },
          {
            "name": "6.9/steer_append",
            "value": 39.89791719016263,
            "unit": "ns"
          },
          {
            "name": "6.9/truncate_output",
            "value": 2120.704458922427,
            "unit": "ns"
          },
          {
            "name": "6.9/validate_repair",
            "value": 172.45907678608643,
            "unit": "ns"
          },
          {
            "name": "gw5/echo_through_gate",
            "value": 69.89605877140414,
            "unit": "ns"
          },
          {
            "name": "gw5/eight_concurrent_sleeps",
            "value": 2110539.680555556,
            "unit": "ns"
          },
          {
            "name": "gw7/catalog_arc_swap",
            "value": 1493.8872749590835,
            "unit": "ns"
          },
          {
            "name": "gw7/catalog_reload_10k",
            "value": 15990649.198412698,
            "unit": "ns"
          },
          {
            "name": "gw7/catalog_search_10k",
            "value": 119791.94581958195,
            "unit": "ns"
          },
          {
            "name": "gw8/dynamic_rename_task",
            "value": 3375001.751262626,
            "unit": "ns"
          },
          {
            "name": "gw8/every_tool_rename_task",
            "value": 4805014.097098215,
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
          "id": "072e72c190315ed78bd384c3d57d2ca08b2b0185",
          "message": "Write chapters 5, 7, 8, and 10",
          "timestamp": "2026-10-15T04:44:13Z",
          "url": "https://github.com/agnathan/rust-agent-harness/pull/4/commits/072e72c190315ed78bd384c3d57d2ca08b2b0185"
        },
        "date": 1792134704360,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "1.1/serde_50_message_request",
            "value": 10211.59686450168,
            "unit": "ns"
          },
          {
            "name": "1.2/mock_100_concurrent",
            "value": 78139.20519713263,
            "unit": "ns"
          },
          {
            "name": "1.2/mock_turn_overhead",
            "value": 247.9035411879616,
            "unit": "ns"
          },
          {
            "name": "1.3/build_request_and_parse_response",
            "value": 2946.8712585868498,
            "unit": "ns"
          },
          {
            "name": "1.4/parse_response_edges",
            "value": 1214.6333796563063,
            "unit": "ns"
          },
          {
            "name": "10/scripted_turn",
            "value": 7.79607792083727,
            "unit": "ns"
          },
          {
            "name": "10/turn_transition",
            "value": 1.1029886092728363,
            "unit": "ns"
          },
          {
            "name": "2/chat_turn_overhead",
            "value": 698.9508165275997,
            "unit": "ns"
          },
          {
            "name": "2/inbox_submit",
            "value": 40.63273861844051,
            "unit": "ns"
          },
          {
            "name": "3.1/registry_lookup",
            "value": 25.996747048955662,
            "unit": "ns"
          },
          {
            "name": "3.2/available_tools_50",
            "value": 30697.379005059025,
            "unit": "ns"
          },
          {
            "name": "3.3/dispatch_8_reads_concurrent",
            "value": 2140163.189215686,
            "unit": "ns"
          },
          {
            "name": "3.3/dispatch_8_reads_sequential",
            "value": 16715467.583333332,
            "unit": "ns"
          },
          {
            "name": "4.1/stdio_echo_round_trip",
            "value": 204785.39682539683,
            "unit": "ns"
          },
          {
            "name": "4.2/http_echo_round_trip",
            "value": 413270.9095643939,
            "unit": "ns"
          },
          {
            "name": "4.3/mcp_50_concurrent",
            "value": 4992443.079166667,
            "unit": "ns"
          },
          {
            "name": "4.3/mcp_50_sequential",
            "value": 10193426.118055556,
            "unit": "ns"
          },
          {
            "name": "4.4/mcp_list_cold",
            "value": 206649.7962506842,
            "unit": "ns"
          },
          {
            "name": "4.4/mcp_list_hit",
            "value": 255.3868941686207,
            "unit": "ns"
          },
          {
            "name": "5.1/schema_tokens_1000",
            "value": 1261106.0275862068,
            "unit": "ns"
          },
          {
            "name": "5.1/schema_tokens_10000",
            "value": 40938500.26190476,
            "unit": "ns"
          },
          {
            "name": "5.2/index_build_10000",
            "value": 21055359.125,
            "unit": "ns"
          },
          {
            "name": "5.2/index_search_10000",
            "value": 331780.09618465125,
            "unit": "ns"
          },
          {
            "name": "5.3/full_schema_tokens_10000",
            "value": 21061367.175,
            "unit": "ns"
          },
          {
            "name": "5.3/lazy_visible_prompt_10000",
            "value": 15606.946060541006,
            "unit": "ns"
          },
          {
            "name": "5.4/routed_search_10000",
            "value": 25756.97355253753,
            "unit": "ns"
          },
          {
            "name": "5.4/shrink_schemas_10000",
            "value": 59630106.25,
            "unit": "ns"
          },
          {
            "name": "5.5/agent_filter_10000",
            "value": 124351.90320733104,
            "unit": "ns"
          },
          {
            "name": "5.5/cache_cold_10000",
            "value": 24015487.5,
            "unit": "ns"
          },
          {
            "name": "5.5/cache_hit_10000",
            "value": 13.795260559855155,
            "unit": "ns"
          },
          {
            "name": "6.1/gate_check",
            "value": 3.4288011231080695,
            "unit": "ns"
          },
          {
            "name": "6.1/live_tool_body",
            "value": 1872.7158031649685,
            "unit": "ns"
          },
          {
            "name": "6.1/loop_8_reads",
            "value": 2157004.5555555555,
            "unit": "ns"
          },
          {
            "name": "6.1/loop_iteration",
            "value": 364.08755068487625,
            "unit": "ns"
          },
          {
            "name": "6.10/budget_notice",
            "value": 77.61119698866906,
            "unit": "ns"
          },
          {
            "name": "6.10/cancel_select",
            "value": 95.23293915941787,
            "unit": "ns"
          },
          {
            "name": "6.10/canonical_args",
            "value": 1054.087128889661,
            "unit": "ns"
          },
          {
            "name": "6.10/gate_check",
            "value": 7.468664345530248,
            "unit": "ns"
          },
          {
            "name": "6.10/live_tool_body",
            "value": 1736.8119206561128,
            "unit": "ns"
          },
          {
            "name": "6.10/loop_8_reads",
            "value": 2183654.404411765,
            "unit": "ns"
          },
          {
            "name": "6.10/loop_iteration",
            "value": 731.8062128647618,
            "unit": "ns"
          },
          {
            "name": "6.10/outcome_label",
            "value": 8.397514288017607,
            "unit": "ns"
          },
          {
            "name": "6.10/stall_observe",
            "value": 138.18609740697315,
            "unit": "ns"
          },
          {
            "name": "6.10/steer_append",
            "value": 59.88087901939406,
            "unit": "ns"
          },
          {
            "name": "6.10/stress_repair",
            "value": 51261.954785247435,
            "unit": "ns"
          },
          {
            "name": "6.10/truncate_output",
            "value": 4810.7800423504505,
            "unit": "ns"
          },
          {
            "name": "6.10/validate_repair",
            "value": 270.48131126995224,
            "unit": "ns"
          },
          {
            "name": "6.2/cancel_select",
            "value": 95.92747514062265,
            "unit": "ns"
          },
          {
            "name": "6.2/gate_check",
            "value": 7.4727249662284585,
            "unit": "ns"
          },
          {
            "name": "6.2/live_tool_body",
            "value": 1851.3987330321827,
            "unit": "ns"
          },
          {
            "name": "6.2/loop_8_reads",
            "value": 2153293.4823529413,
            "unit": "ns"
          },
          {
            "name": "6.2/loop_iteration",
            "value": 532.0132889093138,
            "unit": "ns"
          },
          {
            "name": "6.3/cancel_select",
            "value": 96.84227648315672,
            "unit": "ns"
          },
          {
            "name": "6.3/gate_check",
            "value": 7.478463699985831,
            "unit": "ns"
          },
          {
            "name": "6.3/live_tool_body",
            "value": 1757.4674339239036,
            "unit": "ns"
          },
          {
            "name": "6.3/loop_8_reads",
            "value": 2171584.3578431373,
            "unit": "ns"
          },
          {
            "name": "6.3/loop_iteration",
            "value": 544.8246451408984,
            "unit": "ns"
          },
          {
            "name": "6.3/validate_repair",
            "value": 261.6948847333836,
            "unit": "ns"
          },
          {
            "name": "6.4/cancel_select",
            "value": 95.03999077963776,
            "unit": "ns"
          },
          {
            "name": "6.4/gate_check",
            "value": 7.4693742843829085,
            "unit": "ns"
          },
          {
            "name": "6.4/live_tool_body",
            "value": 1779.4633468170302,
            "unit": "ns"
          },
          {
            "name": "6.4/loop_8_reads",
            "value": 2176303.5992647056,
            "unit": "ns"
          },
          {
            "name": "6.4/loop_iteration",
            "value": 542.6841828077457,
            "unit": "ns"
          },
          {
            "name": "6.4/stall_observe",
            "value": 139.04170336739412,
            "unit": "ns"
          },
          {
            "name": "6.4/validate_repair",
            "value": 256.3011290100144,
            "unit": "ns"
          },
          {
            "name": "6.5/budget_notice",
            "value": 78.50597597294396,
            "unit": "ns"
          },
          {
            "name": "6.5/cancel_select",
            "value": 95.20212036325748,
            "unit": "ns"
          },
          {
            "name": "6.5/gate_check",
            "value": 7.475842134181773,
            "unit": "ns"
          },
          {
            "name": "6.5/live_tool_body",
            "value": 1727.773790533784,
            "unit": "ns"
          },
          {
            "name": "6.5/loop_8_reads",
            "value": 2176154.5375,
            "unit": "ns"
          },
          {
            "name": "6.5/loop_iteration",
            "value": 601.0465212479648,
            "unit": "ns"
          },
          {
            "name": "6.5/stall_observe",
            "value": 136.5488026606977,
            "unit": "ns"
          },
          {
            "name": "6.5/validate_repair",
            "value": 288.21442554691913,
            "unit": "ns"
          },
          {
            "name": "6.6/budget_notice",
            "value": 79.10691318740419,
            "unit": "ns"
          },
          {
            "name": "6.6/cancel_select",
            "value": 95.96050035363521,
            "unit": "ns"
          },
          {
            "name": "6.6/gate_check",
            "value": 7.473818395394812,
            "unit": "ns"
          },
          {
            "name": "6.6/live_tool_body",
            "value": 1803.9137095174328,
            "unit": "ns"
          },
          {
            "name": "6.6/loop_8_reads",
            "value": 2176444.3,
            "unit": "ns"
          },
          {
            "name": "6.6/loop_iteration",
            "value": 609.041005252142,
            "unit": "ns"
          },
          {
            "name": "6.6/stall_observe",
            "value": 140.5873408741959,
            "unit": "ns"
          },
          {
            "name": "6.6/truncate_output",
            "value": 4580.8638441952435,
            "unit": "ns"
          },
          {
            "name": "6.6/validate_repair",
            "value": 259.07113286899687,
            "unit": "ns"
          },
          {
            "name": "6.7/budget_notice",
            "value": 78.27051332110177,
            "unit": "ns"
          },
          {
            "name": "6.7/cancel_select",
            "value": 97.05749860085885,
            "unit": "ns"
          },
          {
            "name": "6.7/gate_check",
            "value": 7.477092329603605,
            "unit": "ns"
          },
          {
            "name": "6.7/live_tool_body",
            "value": 1699.1398198042057,
            "unit": "ns"
          },
          {
            "name": "6.7/loop_8_reads",
            "value": 2178813.3786764704,
            "unit": "ns"
          },
          {
            "name": "6.7/loop_iteration",
            "value": 612.8283324364386,
            "unit": "ns"
          },
          {
            "name": "6.7/stall_observe",
            "value": 138.26365083705934,
            "unit": "ns"
          },
          {
            "name": "6.7/steer_append",
            "value": 58.380410989916655,
            "unit": "ns"
          },
          {
            "name": "6.7/truncate_output",
            "value": 4673.357235751626,
            "unit": "ns"
          },
          {
            "name": "6.7/validate_repair",
            "value": 272.43933313260976,
            "unit": "ns"
          },
          {
            "name": "6.8/budget_notice",
            "value": 77.44862532631224,
            "unit": "ns"
          },
          {
            "name": "6.8/cancel_select",
            "value": 95.61285115919216,
            "unit": "ns"
          },
          {
            "name": "6.8/canonical_args",
            "value": 1079.0255994749755,
            "unit": "ns"
          },
          {
            "name": "6.8/gate_check",
            "value": 7.475033732361732,
            "unit": "ns"
          },
          {
            "name": "6.8/live_tool_body",
            "value": 1733.172903114712,
            "unit": "ns"
          },
          {
            "name": "6.8/loop_8_reads",
            "value": 2181857.411029412,
            "unit": "ns"
          },
          {
            "name": "6.8/loop_iteration",
            "value": 677.816821942312,
            "unit": "ns"
          },
          {
            "name": "6.8/stall_observe",
            "value": 137.9421099028596,
            "unit": "ns"
          },
          {
            "name": "6.8/steer_append",
            "value": 58.050106359864586,
            "unit": "ns"
          },
          {
            "name": "6.8/truncate_output",
            "value": 4577.504984068981,
            "unit": "ns"
          },
          {
            "name": "6.8/validate_repair",
            "value": 265.40776277747716,
            "unit": "ns"
          },
          {
            "name": "6.9/budget_notice",
            "value": 78.64741651976524,
            "unit": "ns"
          },
          {
            "name": "6.9/cancel_select",
            "value": 96.5259840442895,
            "unit": "ns"
          },
          {
            "name": "6.9/canonical_args",
            "value": 1050.5489222918252,
            "unit": "ns"
          },
          {
            "name": "6.9/gate_check",
            "value": 7.473943323791392,
            "unit": "ns"
          },
          {
            "name": "6.9/live_tool_body",
            "value": 1762.4302077363895,
            "unit": "ns"
          },
          {
            "name": "6.9/loop_8_reads",
            "value": 2182655.533006536,
            "unit": "ns"
          },
          {
            "name": "6.9/loop_iteration",
            "value": 758.3196669823233,
            "unit": "ns"
          },
          {
            "name": "6.9/outcome_label",
            "value": 8.39756080840386,
            "unit": "ns"
          },
          {
            "name": "6.9/stall_observe",
            "value": 136.1449941165087,
            "unit": "ns"
          },
          {
            "name": "6.9/steer_append",
            "value": 59.21242208413936,
            "unit": "ns"
          },
          {
            "name": "6.9/truncate_output",
            "value": 5064.025868007716,
            "unit": "ns"
          },
          {
            "name": "6.9/validate_repair",
            "value": 270.1664281093621,
            "unit": "ns"
          },
          {
            "name": "gw5/echo_through_gate",
            "value": 45.531932873584495,
            "unit": "ns"
          },
          {
            "name": "gw5/eight_concurrent_sleeps",
            "value": 2130108.9618055555,
            "unit": "ns"
          },
          {
            "name": "gw7/catalog_arc_swap",
            "value": 2465.5756806052823,
            "unit": "ns"
          },
          {
            "name": "gw7/catalog_reload_10k",
            "value": 27484795.388888888,
            "unit": "ns"
          },
          {
            "name": "gw7/catalog_search_10k",
            "value": 298159.49453551916,
            "unit": "ns"
          },
          {
            "name": "gw8/dynamic_rename_task",
            "value": 5684951.625,
            "unit": "ns"
          },
          {
            "name": "gw8/every_tool_rename_task",
            "value": 8357133.435714286,
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
          "id": "af12bfe94df25cd0ca2e926ee80d50db9872e192",
          "message": "Write chapters 5, 7, 8, and 10",
          "timestamp": "2026-10-15T04:44:13Z",
          "url": "https://github.com/agnathan/rust-agent-harness/pull/4/commits/af12bfe94df25cd0ca2e926ee80d50db9872e192"
        },
        "date": 1792135673389,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "1.1/serde_50_message_request",
            "value": 5971.68284725655,
            "unit": "ns"
          },
          {
            "name": "1.2/mock_100_concurrent",
            "value": 60331.429494598764,
            "unit": "ns"
          },
          {
            "name": "1.2/mock_turn_overhead",
            "value": 183.94900462458378,
            "unit": "ns"
          },
          {
            "name": "1.3/build_request_and_parse_response",
            "value": 1469.9226356899658,
            "unit": "ns"
          },
          {
            "name": "1.4/parse_response_edges",
            "value": 622.1771677340987,
            "unit": "ns"
          },
          {
            "name": "10/scripted_turn",
            "value": 3.9006506846948863,
            "unit": "ns"
          },
          {
            "name": "10/turn_transition",
            "value": 0.6850631051912559,
            "unit": "ns"
          },
          {
            "name": "2/chat_turn_overhead",
            "value": 483.7549845352265,
            "unit": "ns"
          },
          {
            "name": "2/inbox_submit",
            "value": 30.15189523545931,
            "unit": "ns"
          },
          {
            "name": "3.1/registry_lookup",
            "value": 11.599572771079377,
            "unit": "ns"
          },
          {
            "name": "3.2/available_tools_50",
            "value": 17308.75179021089,
            "unit": "ns"
          },
          {
            "name": "3.3/dispatch_8_reads_concurrent",
            "value": 2108293.3395061726,
            "unit": "ns"
          },
          {
            "name": "3.3/dispatch_8_reads_sequential",
            "value": 16754789.833333334,
            "unit": "ns"
          },
          {
            "name": "4.1/stdio_echo_round_trip",
            "value": 86926.02385124611,
            "unit": "ns"
          },
          {
            "name": "4.2/http_echo_round_trip",
            "value": 219954.74621212122,
            "unit": "ns"
          },
          {
            "name": "4.3/mcp_50_concurrent",
            "value": 2267006.8398109246,
            "unit": "ns"
          },
          {
            "name": "4.3/mcp_50_sequential",
            "value": 4188287.7333333334,
            "unit": "ns"
          },
          {
            "name": "4.4/mcp_list_cold",
            "value": 90978.10811859241,
            "unit": "ns"
          },
          {
            "name": "4.4/mcp_list_hit",
            "value": 190.75241669343768,
            "unit": "ns"
          },
          {
            "name": "5.1/schema_tokens_1000",
            "value": 668857.334090909,
            "unit": "ns"
          },
          {
            "name": "5.1/schema_tokens_10000",
            "value": 26910805.888888888,
            "unit": "ns"
          },
          {
            "name": "5.2/index_build_10000",
            "value": 13083766.8,
            "unit": "ns"
          },
          {
            "name": "5.2/index_search_10000",
            "value": 126047.17361111111,
            "unit": "ns"
          },
          {
            "name": "5.3/full_schema_tokens_10000",
            "value": 14082490.791666666,
            "unit": "ns"
          },
          {
            "name": "5.3/lazy_visible_prompt_10000",
            "value": 10208.891417723631,
            "unit": "ns"
          },
          {
            "name": "5.4/routed_search_10000",
            "value": 12417.383105413104,
            "unit": "ns"
          },
          {
            "name": "5.4/shrink_schemas_10000",
            "value": 45679943.57142857,
            "unit": "ns"
          },
          {
            "name": "5.5/agent_filter_10000",
            "value": 53335.1842808552,
            "unit": "ns"
          },
          {
            "name": "5.5/cache_cold_10000",
            "value": 13577121.882142857,
            "unit": "ns"
          },
          {
            "name": "5.5/cache_hit_10000",
            "value": 14.992267066901565,
            "unit": "ns"
          },
          {
            "name": "6.1/gate_check",
            "value": 1.6405520796865538,
            "unit": "ns"
          },
          {
            "name": "6.1/live_tool_body",
            "value": 1208.429034612177,
            "unit": "ns"
          },
          {
            "name": "6.1/loop_8_reads",
            "value": 2120041,
            "unit": "ns"
          },
          {
            "name": "6.1/loop_iteration",
            "value": 219.27136569169807,
            "unit": "ns"
          },
          {
            "name": "6.10/budget_notice",
            "value": 41.74158178203892,
            "unit": "ns"
          },
          {
            "name": "6.10/cancel_select",
            "value": 143.9600923633554,
            "unit": "ns"
          },
          {
            "name": "6.10/canonical_args",
            "value": 636.1058919294001,
            "unit": "ns"
          },
          {
            "name": "6.10/gate_check",
            "value": 11.732918111557439,
            "unit": "ns"
          },
          {
            "name": "6.10/live_tool_body",
            "value": 1355.601335890061,
            "unit": "ns"
          },
          {
            "name": "6.10/loop_8_reads",
            "value": 2132225.6601307187,
            "unit": "ns"
          },
          {
            "name": "6.10/loop_iteration",
            "value": 522.3821322018955,
            "unit": "ns"
          },
          {
            "name": "6.10/outcome_label",
            "value": 5.2820459936494295,
            "unit": "ns"
          },
          {
            "name": "6.10/stall_observe",
            "value": 68.21468558811642,
            "unit": "ns"
          },
          {
            "name": "6.10/steer_append",
            "value": 41.47639609318887,
            "unit": "ns"
          },
          {
            "name": "6.10/stress_repair",
            "value": 26133.02916363636,
            "unit": "ns"
          },
          {
            "name": "6.10/truncate_output",
            "value": 2198.95450716091,
            "unit": "ns"
          },
          {
            "name": "6.10/validate_repair",
            "value": 165.84961609592165,
            "unit": "ns"
          },
          {
            "name": "6.2/cancel_select",
            "value": 149.04164536825556,
            "unit": "ns"
          },
          {
            "name": "6.2/gate_check",
            "value": 11.333084976661144,
            "unit": "ns"
          },
          {
            "name": "6.2/live_tool_body",
            "value": 1192.8820775670597,
            "unit": "ns"
          },
          {
            "name": "6.2/loop_8_reads",
            "value": 2123873.949404762,
            "unit": "ns"
          },
          {
            "name": "6.2/loop_iteration",
            "value": 387.40680686373867,
            "unit": "ns"
          },
          {
            "name": "6.3/cancel_select",
            "value": 146.0311984025504,
            "unit": "ns"
          },
          {
            "name": "6.3/gate_check",
            "value": 11.757257263136996,
            "unit": "ns"
          },
          {
            "name": "6.3/live_tool_body",
            "value": 1285.6448282357374,
            "unit": "ns"
          },
          {
            "name": "6.3/loop_8_reads",
            "value": 2136510.4328703703,
            "unit": "ns"
          },
          {
            "name": "6.3/loop_iteration",
            "value": 428.9327094287763,
            "unit": "ns"
          },
          {
            "name": "6.3/validate_repair",
            "value": 177.493836060131,
            "unit": "ns"
          },
          {
            "name": "6.4/cancel_select",
            "value": 176.99498053618186,
            "unit": "ns"
          },
          {
            "name": "6.4/gate_check",
            "value": 11.501484677919905,
            "unit": "ns"
          },
          {
            "name": "6.4/live_tool_body",
            "value": 1359.4610758773383,
            "unit": "ns"
          },
          {
            "name": "6.4/loop_8_reads",
            "value": 2131703.636904762,
            "unit": "ns"
          },
          {
            "name": "6.4/loop_iteration",
            "value": 428.96901597161957,
            "unit": "ns"
          },
          {
            "name": "6.4/stall_observe",
            "value": 68.09395969167383,
            "unit": "ns"
          },
          {
            "name": "6.4/validate_repair",
            "value": 168.73286631613234,
            "unit": "ns"
          },
          {
            "name": "6.5/budget_notice",
            "value": 44.397082653166585,
            "unit": "ns"
          },
          {
            "name": "6.5/cancel_select",
            "value": 150.8410153462868,
            "unit": "ns"
          },
          {
            "name": "6.5/gate_check",
            "value": 11.900227521610098,
            "unit": "ns"
          },
          {
            "name": "6.5/live_tool_body",
            "value": 1221.317504260367,
            "unit": "ns"
          },
          {
            "name": "6.5/loop_8_reads",
            "value": 2140142.091666667,
            "unit": "ns"
          },
          {
            "name": "6.5/loop_iteration",
            "value": 467.8446534167421,
            "unit": "ns"
          },
          {
            "name": "6.5/stall_observe",
            "value": 68.05304409274217,
            "unit": "ns"
          },
          {
            "name": "6.5/validate_repair",
            "value": 178.86020280566396,
            "unit": "ns"
          },
          {
            "name": "6.6/budget_notice",
            "value": 43.33628287945392,
            "unit": "ns"
          },
          {
            "name": "6.6/cancel_select",
            "value": 148.08904910057697,
            "unit": "ns"
          },
          {
            "name": "6.6/gate_check",
            "value": 11.526975945966317,
            "unit": "ns"
          },
          {
            "name": "6.6/live_tool_body",
            "value": 1223.7881825341267,
            "unit": "ns"
          },
          {
            "name": "6.6/loop_8_reads",
            "value": 2142340.2977941176,
            "unit": "ns"
          },
          {
            "name": "6.6/loop_iteration",
            "value": 455.4642134155998,
            "unit": "ns"
          },
          {
            "name": "6.6/stall_observe",
            "value": 68.58846582246872,
            "unit": "ns"
          },
          {
            "name": "6.6/truncate_output",
            "value": 2321.4114020300563,
            "unit": "ns"
          },
          {
            "name": "6.6/validate_repair",
            "value": 175.95482177973253,
            "unit": "ns"
          },
          {
            "name": "6.7/budget_notice",
            "value": 44.08205864130474,
            "unit": "ns"
          },
          {
            "name": "6.7/cancel_select",
            "value": 144.97252907286605,
            "unit": "ns"
          },
          {
            "name": "6.7/gate_check",
            "value": 11.563671322964392,
            "unit": "ns"
          },
          {
            "name": "6.7/live_tool_body",
            "value": 1255.9397564519477,
            "unit": "ns"
          },
          {
            "name": "6.7/loop_8_reads",
            "value": 2142762.4990079366,
            "unit": "ns"
          },
          {
            "name": "6.7/loop_iteration",
            "value": 482.5967797076089,
            "unit": "ns"
          },
          {
            "name": "6.7/stall_observe",
            "value": 68.39109517587173,
            "unit": "ns"
          },
          {
            "name": "6.7/steer_append",
            "value": 40.090226480620714,
            "unit": "ns"
          },
          {
            "name": "6.7/truncate_output",
            "value": 2232.8087725712107,
            "unit": "ns"
          },
          {
            "name": "6.7/validate_repair",
            "value": 169.288181115212,
            "unit": "ns"
          },
          {
            "name": "6.8/budget_notice",
            "value": 42.05774718048458,
            "unit": "ns"
          },
          {
            "name": "6.8/cancel_select",
            "value": 140.9726014253203,
            "unit": "ns"
          },
          {
            "name": "6.8/canonical_args",
            "value": 660.5537798642533,
            "unit": "ns"
          },
          {
            "name": "6.8/gate_check",
            "value": 11.758293193305107,
            "unit": "ns"
          },
          {
            "name": "6.8/live_tool_body",
            "value": 1211.891366289478,
            "unit": "ns"
          },
          {
            "name": "6.8/loop_8_reads",
            "value": 2136704.0944444444,
            "unit": "ns"
          },
          {
            "name": "6.8/loop_iteration",
            "value": 545.4187529268623,
            "unit": "ns"
          },
          {
            "name": "6.8/stall_observe",
            "value": 67.76258394742342,
            "unit": "ns"
          },
          {
            "name": "6.8/steer_append",
            "value": 40.38301620395562,
            "unit": "ns"
          },
          {
            "name": "6.8/truncate_output",
            "value": 2727.1836896826,
            "unit": "ns"
          },
          {
            "name": "6.8/validate_repair",
            "value": 179.05199882961108,
            "unit": "ns"
          },
          {
            "name": "6.9/budget_notice",
            "value": 42.554126423113175,
            "unit": "ns"
          },
          {
            "name": "6.9/cancel_select",
            "value": 140.29024076149773,
            "unit": "ns"
          },
          {
            "name": "6.9/canonical_args",
            "value": 660.004485114419,
            "unit": "ns"
          },
          {
            "name": "6.9/gate_check",
            "value": 11.573990065347115,
            "unit": "ns"
          },
          {
            "name": "6.9/live_tool_body",
            "value": 1183.991449124806,
            "unit": "ns"
          },
          {
            "name": "6.9/loop_8_reads",
            "value": 2150254.5490196077,
            "unit": "ns"
          },
          {
            "name": "6.9/loop_iteration",
            "value": 581.240097896776,
            "unit": "ns"
          },
          {
            "name": "6.9/outcome_label",
            "value": 5.304998495011153,
            "unit": "ns"
          },
          {
            "name": "6.9/stall_observe",
            "value": 68.86097306301394,
            "unit": "ns"
          },
          {
            "name": "6.9/steer_append",
            "value": 40.56294199949122,
            "unit": "ns"
          },
          {
            "name": "6.9/truncate_output",
            "value": 2210.5318849699765,
            "unit": "ns"
          },
          {
            "name": "6.9/validate_repair",
            "value": 179.4514020048374,
            "unit": "ns"
          },
          {
            "name": "gw5/echo_through_gate",
            "value": 72.23027364452206,
            "unit": "ns"
          },
          {
            "name": "gw5/eight_concurrent_sleeps",
            "value": 2099819.8583333334,
            "unit": "ns"
          },
          {
            "name": "gw7/catalog_arc_swap",
            "value": 1572.8622387436226,
            "unit": "ns"
          },
          {
            "name": "gw7/catalog_reload_10k",
            "value": 16949675.569444444,
            "unit": "ns"
          },
          {
            "name": "gw7/catalog_search_10k",
            "value": 123506.67460653753,
            "unit": "ns"
          },
          {
            "name": "gw8/dynamic_rename_task",
            "value": 3537675.938131313,
            "unit": "ns"
          },
          {
            "name": "gw8/every_tool_rename_task",
            "value": 5004075.642857143,
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
          "id": "79742a99e267b2310b1b63dd8c7ce194ed27ee09",
          "message": "Write chapters 11 through 20",
          "timestamp": "2026-10-15T04:44:13Z",
          "url": "https://github.com/agnathan/rust-agent-harness/pull/5/commits/79742a99e267b2310b1b63dd8c7ce194ed27ee09"
        },
        "date": 1792184612231,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "1.1/serde_50_message_request",
            "value": 8951.968167689161,
            "unit": "ns"
          },
          {
            "name": "1.2/mock_100_concurrent",
            "value": 109877.30975609756,
            "unit": "ns"
          },
          {
            "name": "1.2/mock_turn_overhead",
            "value": 264.13428702472766,
            "unit": "ns"
          },
          {
            "name": "1.3/build_request_and_parse_response",
            "value": 2732.4130821535023,
            "unit": "ns"
          },
          {
            "name": "1.4/parse_response_edges",
            "value": 997.1416828002602,
            "unit": "ns"
          },
          {
            "name": "10/scripted_turn",
            "value": 8.964054440197586,
            "unit": "ns"
          },
          {
            "name": "10/turn_transition",
            "value": 1.3211589502743917,
            "unit": "ns"
          },
          {
            "name": "11/skip_decision",
            "value": 237.235827104048,
            "unit": "ns"
          },
          {
            "name": "2/chat_turn_overhead",
            "value": 772.3164136360353,
            "unit": "ns"
          },
          {
            "name": "2/inbox_submit",
            "value": 40.88757883248073,
            "unit": "ns"
          },
          {
            "name": "3.1/registry_lookup",
            "value": 18.01086234961867,
            "unit": "ns"
          },
          {
            "name": "3.2/available_tools_50",
            "value": 28490.817210567213,
            "unit": "ns"
          },
          {
            "name": "3.3/dispatch_8_reads_concurrent",
            "value": 2130934.5168067226,
            "unit": "ns"
          },
          {
            "name": "3.3/dispatch_8_reads_sequential",
            "value": 16622278.25,
            "unit": "ns"
          },
          {
            "name": "4.1/stdio_echo_round_trip",
            "value": 176357.9575569358,
            "unit": "ns"
          },
          {
            "name": "4.2/http_echo_round_trip",
            "value": 376093.706185567,
            "unit": "ns"
          },
          {
            "name": "4.3/mcp_50_concurrent",
            "value": 3423095.8045454547,
            "unit": "ns"
          },
          {
            "name": "4.3/mcp_50_sequential",
            "value": 7267682.75297619,
            "unit": "ns"
          },
          {
            "name": "4.4/mcp_list_cold",
            "value": 182449.75964912283,
            "unit": "ns"
          },
          {
            "name": "4.4/mcp_list_hit",
            "value": 224.5410399628559,
            "unit": "ns"
          },
          {
            "name": "5.1/schema_tokens_1000",
            "value": 1130155.6742424243,
            "unit": "ns"
          },
          {
            "name": "5.1/schema_tokens_10000",
            "value": 31850342.875,
            "unit": "ns"
          },
          {
            "name": "5.2/index_build_10000",
            "value": 19241821.489583336,
            "unit": "ns"
          },
          {
            "name": "5.2/index_search_10000",
            "value": 188909.51101190477,
            "unit": "ns"
          },
          {
            "name": "5.3/full_schema_tokens_10000",
            "value": 15195649.641666666,
            "unit": "ns"
          },
          {
            "name": "5.3/lazy_visible_prompt_10000",
            "value": 16597.75773727648,
            "unit": "ns"
          },
          {
            "name": "5.4/routed_search_10000",
            "value": 19526.89306078537,
            "unit": "ns"
          },
          {
            "name": "5.4/shrink_schemas_10000",
            "value": 55181199,
            "unit": "ns"
          },
          {
            "name": "5.5/agent_filter_10000",
            "value": 64649.07625,
            "unit": "ns"
          },
          {
            "name": "5.5/cache_cold_10000",
            "value": 19957097.75,
            "unit": "ns"
          },
          {
            "name": "5.5/cache_hit_10000",
            "value": 22.956771497079796,
            "unit": "ns"
          },
          {
            "name": "6.1/gate_check",
            "value": 3.4513281579489687,
            "unit": "ns"
          },
          {
            "name": "6.1/live_tool_body",
            "value": 1871.5283847653377,
            "unit": "ns"
          },
          {
            "name": "6.1/loop_8_reads",
            "value": 2143593.718487395,
            "unit": "ns"
          },
          {
            "name": "6.1/loop_iteration",
            "value": 328.0747239195971,
            "unit": "ns"
          },
          {
            "name": "6.10/budget_notice",
            "value": 72.56679645810055,
            "unit": "ns"
          },
          {
            "name": "6.10/cancel_select",
            "value": 208.75782422838705,
            "unit": "ns"
          },
          {
            "name": "6.10/canonical_args",
            "value": 1047.1717830245843,
            "unit": "ns"
          },
          {
            "name": "6.10/gate_check",
            "value": 19.219058324949028,
            "unit": "ns"
          },
          {
            "name": "6.10/live_tool_body",
            "value": 1918.3786790795612,
            "unit": "ns"
          },
          {
            "name": "6.10/loop_8_reads",
            "value": 2174098.645588235,
            "unit": "ns"
          },
          {
            "name": "6.10/loop_iteration",
            "value": 880.1356140225448,
            "unit": "ns"
          },
          {
            "name": "6.10/outcome_label",
            "value": 7.241421388613928,
            "unit": "ns"
          },
          {
            "name": "6.10/stall_observe",
            "value": 129.4215502376321,
            "unit": "ns"
          },
          {
            "name": "6.10/steer_append",
            "value": 63.205057146790175,
            "unit": "ns"
          },
          {
            "name": "6.10/stress_repair",
            "value": 47750.19769165608,
            "unit": "ns"
          },
          {
            "name": "6.10/truncate_output",
            "value": 4104.501758829523,
            "unit": "ns"
          },
          {
            "name": "6.10/validate_repair",
            "value": 293.4955426559146,
            "unit": "ns"
          },
          {
            "name": "6.2/cancel_select",
            "value": 209.10135368097093,
            "unit": "ns"
          },
          {
            "name": "6.2/gate_check",
            "value": 20.12000249008755,
            "unit": "ns"
          },
          {
            "name": "6.2/live_tool_body",
            "value": 1931.3021014043784,
            "unit": "ns"
          },
          {
            "name": "6.2/loop_8_reads",
            "value": 2152900.044117647,
            "unit": "ns"
          },
          {
            "name": "6.2/loop_iteration",
            "value": 621.7804682881142,
            "unit": "ns"
          },
          {
            "name": "6.3/cancel_select",
            "value": 209.47537549242554,
            "unit": "ns"
          },
          {
            "name": "6.3/gate_check",
            "value": 19.108043988177258,
            "unit": "ns"
          },
          {
            "name": "6.3/live_tool_body",
            "value": 2117.866279486979,
            "unit": "ns"
          },
          {
            "name": "6.3/loop_8_reads",
            "value": 2164803.5539215687,
            "unit": "ns"
          },
          {
            "name": "6.3/loop_iteration",
            "value": 628.2629025435924,
            "unit": "ns"
          },
          {
            "name": "6.3/validate_repair",
            "value": 290.99735275049824,
            "unit": "ns"
          },
          {
            "name": "6.4/cancel_select",
            "value": 212.43839741216587,
            "unit": "ns"
          },
          {
            "name": "6.4/gate_check",
            "value": 19.29735406298116,
            "unit": "ns"
          },
          {
            "name": "6.4/live_tool_body",
            "value": 2109.7377267222028,
            "unit": "ns"
          },
          {
            "name": "6.4/loop_8_reads",
            "value": 2170014.783088235,
            "unit": "ns"
          },
          {
            "name": "6.4/loop_iteration",
            "value": 637.3360063739487,
            "unit": "ns"
          },
          {
            "name": "6.4/stall_observe",
            "value": 130.54202641086994,
            "unit": "ns"
          },
          {
            "name": "6.4/validate_repair",
            "value": 291.1509663991468,
            "unit": "ns"
          },
          {
            "name": "6.5/budget_notice",
            "value": 74.69074668356814,
            "unit": "ns"
          },
          {
            "name": "6.5/cancel_select",
            "value": 213.74947276540405,
            "unit": "ns"
          },
          {
            "name": "6.5/gate_check",
            "value": 19.372512674844717,
            "unit": "ns"
          },
          {
            "name": "6.5/live_tool_body",
            "value": 2086.2292774831863,
            "unit": "ns"
          },
          {
            "name": "6.5/loop_8_reads",
            "value": 2170031.369747899,
            "unit": "ns"
          },
          {
            "name": "6.5/loop_iteration",
            "value": 686.3073820078557,
            "unit": "ns"
          },
          {
            "name": "6.5/stall_observe",
            "value": 130.1743402435797,
            "unit": "ns"
          },
          {
            "name": "6.5/validate_repair",
            "value": 287.5853660183148,
            "unit": "ns"
          },
          {
            "name": "6.6/budget_notice",
            "value": 76.419320210253,
            "unit": "ns"
          },
          {
            "name": "6.6/cancel_select",
            "value": 210.32144829577484,
            "unit": "ns"
          },
          {
            "name": "6.6/gate_check",
            "value": 19.4092196189582,
            "unit": "ns"
          },
          {
            "name": "6.6/live_tool_body",
            "value": 1911.7205869548363,
            "unit": "ns"
          },
          {
            "name": "6.6/loop_8_reads",
            "value": 2169320.2671568627,
            "unit": "ns"
          },
          {
            "name": "6.6/loop_iteration",
            "value": 686.7792136876592,
            "unit": "ns"
          },
          {
            "name": "6.6/stall_observe",
            "value": 136.20310263198695,
            "unit": "ns"
          },
          {
            "name": "6.6/truncate_output",
            "value": 4170.336603876058,
            "unit": "ns"
          },
          {
            "name": "6.6/validate_repair",
            "value": 298.858645843654,
            "unit": "ns"
          },
          {
            "name": "6.7/budget_notice",
            "value": 69.89201969998018,
            "unit": "ns"
          },
          {
            "name": "6.7/cancel_select",
            "value": 213.20342993181285,
            "unit": "ns"
          },
          {
            "name": "6.7/gate_check",
            "value": 19.226427912785063,
            "unit": "ns"
          },
          {
            "name": "6.7/live_tool_body",
            "value": 1835.3645744198923,
            "unit": "ns"
          },
          {
            "name": "6.7/loop_8_reads",
            "value": 2165346.805147059,
            "unit": "ns"
          },
          {
            "name": "6.7/loop_iteration",
            "value": 677.3550886531469,
            "unit": "ns"
          },
          {
            "name": "6.7/stall_observe",
            "value": 128.70617892211914,
            "unit": "ns"
          },
          {
            "name": "6.7/steer_append",
            "value": 63.459237245557325,
            "unit": "ns"
          },
          {
            "name": "6.7/truncate_output",
            "value": 4155.390796812749,
            "unit": "ns"
          },
          {
            "name": "6.7/validate_repair",
            "value": 297.17334238746236,
            "unit": "ns"
          },
          {
            "name": "6.8/budget_notice",
            "value": 70.93079628620481,
            "unit": "ns"
          },
          {
            "name": "6.8/cancel_select",
            "value": 210.20707175514377,
            "unit": "ns"
          },
          {
            "name": "6.8/canonical_args",
            "value": 1046.6408958811712,
            "unit": "ns"
          },
          {
            "name": "6.8/gate_check",
            "value": 19.196595712581615,
            "unit": "ns"
          },
          {
            "name": "6.8/live_tool_body",
            "value": 2106.90877132319,
            "unit": "ns"
          },
          {
            "name": "6.8/loop_8_reads",
            "value": 2167339.698529412,
            "unit": "ns"
          },
          {
            "name": "6.8/loop_iteration",
            "value": 818.0428923976875,
            "unit": "ns"
          },
          {
            "name": "6.8/stall_observe",
            "value": 134.60731971325635,
            "unit": "ns"
          },
          {
            "name": "6.8/steer_append",
            "value": 63.229034451755865,
            "unit": "ns"
          },
          {
            "name": "6.8/truncate_output",
            "value": 4127.800054112555,
            "unit": "ns"
          },
          {
            "name": "6.8/validate_repair",
            "value": 293.30138216705575,
            "unit": "ns"
          },
          {
            "name": "6.9/budget_notice",
            "value": 72.99335650408723,
            "unit": "ns"
          },
          {
            "name": "6.9/cancel_select",
            "value": 212.76036881655807,
            "unit": "ns"
          },
          {
            "name": "6.9/canonical_args",
            "value": 1052.93578235182,
            "unit": "ns"
          },
          {
            "name": "6.9/gate_check",
            "value": 19.262781051404577,
            "unit": "ns"
          },
          {
            "name": "6.9/live_tool_body",
            "value": 2232.1881905866553,
            "unit": "ns"
          },
          {
            "name": "6.9/loop_8_reads",
            "value": 2175686.636029412,
            "unit": "ns"
          },
          {
            "name": "6.9/loop_iteration",
            "value": 873.4808678524437,
            "unit": "ns"
          },
          {
            "name": "6.9/outcome_label",
            "value": 7.32230031274341,
            "unit": "ns"
          },
          {
            "name": "6.9/stall_observe",
            "value": 129.70574941166922,
            "unit": "ns"
          },
          {
            "name": "6.9/steer_append",
            "value": 63.26340537966172,
            "unit": "ns"
          },
          {
            "name": "6.9/truncate_output",
            "value": 4160.200281667168,
            "unit": "ns"
          },
          {
            "name": "6.9/validate_repair",
            "value": 247.67292029178353,
            "unit": "ns"
          },
          {
            "name": "gw5/echo_through_gate",
            "value": 109.70461310903796,
            "unit": "ns"
          },
          {
            "name": "gw5/eight_concurrent_sleeps",
            "value": 2089791.252469136,
            "unit": "ns"
          },
          {
            "name": "gw7/catalog_arc_swap",
            "value": 2384.210744996617,
            "unit": "ns"
          },
          {
            "name": "gw7/catalog_reload_10k",
            "value": 27826297.825,
            "unit": "ns"
          },
          {
            "name": "gw7/catalog_search_10k",
            "value": 194353.85170250898,
            "unit": "ns"
          },
          {
            "name": "gw8/dynamic_rename_task",
            "value": 5268580.102380952,
            "unit": "ns"
          },
          {
            "name": "gw8/every_tool_rename_task",
            "value": 7609274.75,
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
          "id": "0befa3d108cb271137b271e99fdf897e25441716",
          "message": "Write chapters 11 through 20",
          "timestamp": "2026-10-15T04:44:13Z",
          "url": "https://github.com/agnathan/rust-agent-harness/pull/5/commits/0befa3d108cb271137b271e99fdf897e25441716"
        },
        "date": 1792185367902,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "1.1/serde_50_message_request",
            "value": 10431.34201889711,
            "unit": "ns"
          },
          {
            "name": "1.2/mock_100_concurrent",
            "value": 79298.0305316092,
            "unit": "ns"
          },
          {
            "name": "1.2/mock_turn_overhead",
            "value": 254.24517043662337,
            "unit": "ns"
          },
          {
            "name": "1.3/build_request_and_parse_response",
            "value": 2939.674052223315,
            "unit": "ns"
          },
          {
            "name": "1.4/parse_response_edges",
            "value": 1107.4249584248132,
            "unit": "ns"
          },
          {
            "name": "10/scripted_turn",
            "value": 8.109279041508277,
            "unit": "ns"
          },
          {
            "name": "10/turn_transition",
            "value": 1.100736881034472,
            "unit": "ns"
          },
          {
            "name": "11/skip_decision",
            "value": 251.25565056834574,
            "unit": "ns"
          },
          {
            "name": "12/api_request_hook",
            "value": 2.179135290911578,
            "unit": "ns"
          },
          {
            "name": "12/empty_pre_post",
            "value": 10.901266964545538,
            "unit": "ns"
          },
          {
            "name": "12/memory_enqueue",
            "value": 54.18094198669634,
            "unit": "ns"
          },
          {
            "name": "2/chat_turn_overhead",
            "value": 763.8507227425273,
            "unit": "ns"
          },
          {
            "name": "2/inbox_submit",
            "value": 40.58775163623061,
            "unit": "ns"
          },
          {
            "name": "3.1/registry_lookup",
            "value": 25.69031575161395,
            "unit": "ns"
          },
          {
            "name": "3.2/available_tools_50",
            "value": 30710.21261574074,
            "unit": "ns"
          },
          {
            "name": "3.3/dispatch_8_reads_concurrent",
            "value": 2140599.1875,
            "unit": "ns"
          },
          {
            "name": "3.3/dispatch_8_reads_sequential",
            "value": 16817981.939814813,
            "unit": "ns"
          },
          {
            "name": "4.1/stdio_echo_round_trip",
            "value": 204495.35440074906,
            "unit": "ns"
          },
          {
            "name": "4.2/http_echo_round_trip",
            "value": 411253.8907615481,
            "unit": "ns"
          },
          {
            "name": "4.3/mcp_50_concurrent",
            "value": 4955197.7109375,
            "unit": "ns"
          },
          {
            "name": "4.3/mcp_50_sequential",
            "value": 10086221.2625,
            "unit": "ns"
          },
          {
            "name": "4.4/mcp_list_cold",
            "value": 214264.34690109183,
            "unit": "ns"
          },
          {
            "name": "4.4/mcp_list_hit",
            "value": 255.1816265045848,
            "unit": "ns"
          },
          {
            "name": "5.1/schema_tokens_1000",
            "value": 1253263.6551724137,
            "unit": "ns"
          },
          {
            "name": "5.1/schema_tokens_10000",
            "value": 43921185.6375,
            "unit": "ns"
          },
          {
            "name": "5.2/index_build_10000",
            "value": 21877393.266666666,
            "unit": "ns"
          },
          {
            "name": "5.2/index_search_10000",
            "value": 338368.5805555555,
            "unit": "ns"
          },
          {
            "name": "5.3/full_schema_tokens_10000",
            "value": 28206097.125,
            "unit": "ns"
          },
          {
            "name": "5.3/lazy_visible_prompt_10000",
            "value": 15273.144888514935,
            "unit": "ns"
          },
          {
            "name": "5.4/routed_search_10000",
            "value": 25881.686771353794,
            "unit": "ns"
          },
          {
            "name": "5.4/shrink_schemas_10000",
            "value": 66955537.81666666,
            "unit": "ns"
          },
          {
            "name": "5.5/agent_filter_10000",
            "value": 127150.86294955044,
            "unit": "ns"
          },
          {
            "name": "5.5/cache_cold_10000",
            "value": 25811206.925,
            "unit": "ns"
          },
          {
            "name": "5.5/cache_hit_10000",
            "value": 13.789137828017633,
            "unit": "ns"
          },
          {
            "name": "6.1/gate_check",
            "value": 3.4263629376812132,
            "unit": "ns"
          },
          {
            "name": "6.1/live_tool_body",
            "value": 1744.2151451558593,
            "unit": "ns"
          },
          {
            "name": "6.1/loop_8_reads",
            "value": 2147291.8333333335,
            "unit": "ns"
          },
          {
            "name": "6.1/loop_iteration",
            "value": 361.2090332241554,
            "unit": "ns"
          },
          {
            "name": "6.10/budget_notice",
            "value": 76.48885117965838,
            "unit": "ns"
          },
          {
            "name": "6.10/cancel_select",
            "value": 99.08352612292296,
            "unit": "ns"
          },
          {
            "name": "6.10/canonical_args",
            "value": 1055.3513654653266,
            "unit": "ns"
          },
          {
            "name": "6.10/gate_check",
            "value": 7.470015871238104,
            "unit": "ns"
          },
          {
            "name": "6.10/live_tool_body",
            "value": 1727.9628189953128,
            "unit": "ns"
          },
          {
            "name": "6.10/loop_8_reads",
            "value": 2178200.163398693,
            "unit": "ns"
          },
          {
            "name": "6.10/loop_iteration",
            "value": 777.255857331169,
            "unit": "ns"
          },
          {
            "name": "6.10/outcome_label",
            "value": 8.397224079591648,
            "unit": "ns"
          },
          {
            "name": "6.10/stall_observe",
            "value": 137.4059362262174,
            "unit": "ns"
          },
          {
            "name": "6.10/steer_append",
            "value": 59.845902589748725,
            "unit": "ns"
          },
          {
            "name": "6.10/stress_repair",
            "value": 50487.242163661576,
            "unit": "ns"
          },
          {
            "name": "6.10/truncate_output",
            "value": 4632.390500277457,
            "unit": "ns"
          },
          {
            "name": "6.10/validate_repair",
            "value": 273.1179189659682,
            "unit": "ns"
          },
          {
            "name": "6.2/cancel_select",
            "value": 96.44054528370594,
            "unit": "ns"
          },
          {
            "name": "6.2/gate_check",
            "value": 7.472726514128778,
            "unit": "ns"
          },
          {
            "name": "6.2/live_tool_body",
            "value": 1774.420406325994,
            "unit": "ns"
          },
          {
            "name": "6.2/loop_8_reads",
            "value": 2154806.0784313725,
            "unit": "ns"
          },
          {
            "name": "6.2/loop_iteration",
            "value": 533.8331283991488,
            "unit": "ns"
          },
          {
            "name": "6.3/cancel_select",
            "value": 94.64779526041295,
            "unit": "ns"
          },
          {
            "name": "6.3/gate_check",
            "value": 7.47451163922932,
            "unit": "ns"
          },
          {
            "name": "6.3/live_tool_body",
            "value": 1752.9043472936567,
            "unit": "ns"
          },
          {
            "name": "6.3/loop_8_reads",
            "value": 2167647.455882353,
            "unit": "ns"
          },
          {
            "name": "6.3/loop_iteration",
            "value": 542.2706208949405,
            "unit": "ns"
          },
          {
            "name": "6.3/validate_repair",
            "value": 244.3460633223077,
            "unit": "ns"
          },
          {
            "name": "6.4/cancel_select",
            "value": 95.31382909354157,
            "unit": "ns"
          },
          {
            "name": "6.4/gate_check",
            "value": 7.479186053038523,
            "unit": "ns"
          },
          {
            "name": "6.4/live_tool_body",
            "value": 1816.627860324425,
            "unit": "ns"
          },
          {
            "name": "6.4/loop_8_reads",
            "value": 2172743.95620915,
            "unit": "ns"
          },
          {
            "name": "6.4/loop_iteration",
            "value": 544.0074647339284,
            "unit": "ns"
          },
          {
            "name": "6.4/stall_observe",
            "value": 139.76317932603064,
            "unit": "ns"
          },
          {
            "name": "6.4/validate_repair",
            "value": 262.27897808674913,
            "unit": "ns"
          },
          {
            "name": "6.5/budget_notice",
            "value": 77.3074510659039,
            "unit": "ns"
          },
          {
            "name": "6.5/cancel_select",
            "value": 95.3220123415015,
            "unit": "ns"
          },
          {
            "name": "6.5/gate_check",
            "value": 7.468908864612977,
            "unit": "ns"
          },
          {
            "name": "6.5/live_tool_body",
            "value": 1707.7943435954367,
            "unit": "ns"
          },
          {
            "name": "6.5/loop_8_reads",
            "value": 2165352.245564893,
            "unit": "ns"
          },
          {
            "name": "6.5/loop_iteration",
            "value": 643.4273358058226,
            "unit": "ns"
          },
          {
            "name": "6.5/stall_observe",
            "value": 136.33853413692322,
            "unit": "ns"
          },
          {
            "name": "6.5/validate_repair",
            "value": 288.9271356186217,
            "unit": "ns"
          },
          {
            "name": "6.6/budget_notice",
            "value": 83.19000595774796,
            "unit": "ns"
          },
          {
            "name": "6.6/cancel_select",
            "value": 96.6055671938829,
            "unit": "ns"
          },
          {
            "name": "6.6/gate_check",
            "value": 7.474178086851013,
            "unit": "ns"
          },
          {
            "name": "6.6/live_tool_body",
            "value": 1697.9992027299518,
            "unit": "ns"
          },
          {
            "name": "6.6/loop_8_reads",
            "value": 2172061.290441177,
            "unit": "ns"
          },
          {
            "name": "6.6/loop_iteration",
            "value": 630.4216634045329,
            "unit": "ns"
          },
          {
            "name": "6.6/stall_observe",
            "value": 139.8422387631631,
            "unit": "ns"
          },
          {
            "name": "6.6/truncate_output",
            "value": 4610.402245325247,
            "unit": "ns"
          },
          {
            "name": "6.6/validate_repair",
            "value": 257.5371876808616,
            "unit": "ns"
          },
          {
            "name": "6.7/budget_notice",
            "value": 76.57601314977997,
            "unit": "ns"
          },
          {
            "name": "6.7/cancel_select",
            "value": 95.40756583159917,
            "unit": "ns"
          },
          {
            "name": "6.7/gate_check",
            "value": 7.486843561674885,
            "unit": "ns"
          },
          {
            "name": "6.7/live_tool_body",
            "value": 1667.513212552597,
            "unit": "ns"
          },
          {
            "name": "6.7/loop_8_reads",
            "value": 2175803.698039216,
            "unit": "ns"
          },
          {
            "name": "6.7/loop_iteration",
            "value": 632.4881109970925,
            "unit": "ns"
          },
          {
            "name": "6.7/stall_observe",
            "value": 140.54453860493413,
            "unit": "ns"
          },
          {
            "name": "6.7/steer_append",
            "value": 58.42410052437566,
            "unit": "ns"
          },
          {
            "name": "6.7/truncate_output",
            "value": 4589.849590086098,
            "unit": "ns"
          },
          {
            "name": "6.7/validate_repair",
            "value": 268.40079819798814,
            "unit": "ns"
          },
          {
            "name": "6.8/budget_notice",
            "value": 78.49849624313165,
            "unit": "ns"
          },
          {
            "name": "6.8/cancel_select",
            "value": 95.64449353491955,
            "unit": "ns"
          },
          {
            "name": "6.8/canonical_args",
            "value": 1143.0648559545548,
            "unit": "ns"
          },
          {
            "name": "6.8/gate_check",
            "value": 7.475442822960346,
            "unit": "ns"
          },
          {
            "name": "6.8/live_tool_body",
            "value": 1820.3181379674543,
            "unit": "ns"
          },
          {
            "name": "6.8/loop_8_reads",
            "value": 2172434.8159663863,
            "unit": "ns"
          },
          {
            "name": "6.8/loop_iteration",
            "value": 700.4494626719966,
            "unit": "ns"
          },
          {
            "name": "6.8/stall_observe",
            "value": 137.94240259592468,
            "unit": "ns"
          },
          {
            "name": "6.8/steer_append",
            "value": 57.99670481330566,
            "unit": "ns"
          },
          {
            "name": "6.8/truncate_output",
            "value": 4685.39889232517,
            "unit": "ns"
          },
          {
            "name": "6.8/validate_repair",
            "value": 265.5482830041301,
            "unit": "ns"
          },
          {
            "name": "6.9/budget_notice",
            "value": 79.2397239596772,
            "unit": "ns"
          },
          {
            "name": "6.9/cancel_select",
            "value": 96.57877756364667,
            "unit": "ns"
          },
          {
            "name": "6.9/canonical_args",
            "value": 1045.3847937923347,
            "unit": "ns"
          },
          {
            "name": "6.9/gate_check",
            "value": 7.485041602739403,
            "unit": "ns"
          },
          {
            "name": "6.9/live_tool_body",
            "value": 1726.98369495801,
            "unit": "ns"
          },
          {
            "name": "6.9/loop_8_reads",
            "value": 2177830.3999183006,
            "unit": "ns"
          },
          {
            "name": "6.9/loop_iteration",
            "value": 742.2696252465483,
            "unit": "ns"
          },
          {
            "name": "6.9/outcome_label",
            "value": 8.398672672952497,
            "unit": "ns"
          },
          {
            "name": "6.9/stall_observe",
            "value": 136.24448919863715,
            "unit": "ns"
          },
          {
            "name": "6.9/steer_append",
            "value": 59.220891264305294,
            "unit": "ns"
          },
          {
            "name": "6.9/truncate_output",
            "value": 4690.3164630266,
            "unit": "ns"
          },
          {
            "name": "6.9/validate_repair",
            "value": 267.7955737582709,
            "unit": "ns"
          },
          {
            "name": "gw5/echo_through_gate",
            "value": 45.578106453881034,
            "unit": "ns"
          },
          {
            "name": "gw5/eight_concurrent_sleeps",
            "value": 2118191.9305555555,
            "unit": "ns"
          },
          {
            "name": "gw7/catalog_arc_swap",
            "value": 2405.3678601054144,
            "unit": "ns"
          },
          {
            "name": "gw7/catalog_reload_10k",
            "value": 28586023.53125,
            "unit": "ns"
          },
          {
            "name": "gw7/catalog_search_10k",
            "value": 296623.4928377855,
            "unit": "ns"
          },
          {
            "name": "gw8/dynamic_rename_task",
            "value": 5756848.892857143,
            "unit": "ns"
          },
          {
            "name": "gw8/every_tool_rename_task",
            "value": 8440939.738095239,
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
          "id": "3250fa8f087229bc7272de9ea0eaf6a3f791400e",
          "message": "Write chapters 11 through 20",
          "timestamp": "2026-10-15T04:44:13Z",
          "url": "https://github.com/agnathan/rust-agent-harness/pull/5/commits/3250fa8f087229bc7272de9ea0eaf6a3f791400e"
        },
        "date": 1792185669157,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "1.1/serde_50_message_request",
            "value": 10387.002772182255,
            "unit": "ns"
          },
          {
            "name": "1.2/mock_100_concurrent",
            "value": 77803.52925531915,
            "unit": "ns"
          },
          {
            "name": "1.2/mock_turn_overhead",
            "value": 262.973268812212,
            "unit": "ns"
          },
          {
            "name": "1.3/build_request_and_parse_response",
            "value": 2735.6075591836734,
            "unit": "ns"
          },
          {
            "name": "1.4/parse_response_edges",
            "value": 1188.1988061079778,
            "unit": "ns"
          },
          {
            "name": "10/scripted_turn",
            "value": 6.95788253218668,
            "unit": "ns"
          },
          {
            "name": "10/turn_transition",
            "value": 1.1846385849800707,
            "unit": "ns"
          },
          {
            "name": "11/skip_decision",
            "value": 273.37297509837833,
            "unit": "ns"
          },
          {
            "name": "12/api_request_hook",
            "value": 2.4598567986397377,
            "unit": "ns"
          },
          {
            "name": "12/empty_pre_post",
            "value": 12.651921249097859,
            "unit": "ns"
          },
          {
            "name": "12/memory_enqueue",
            "value": 59.550120188310856,
            "unit": "ns"
          },
          {
            "name": "13/checkpoint_1000",
            "value": 53949.357421875,
            "unit": "ns"
          },
          {
            "name": "13/rewind_1000",
            "value": 80191.44584245078,
            "unit": "ns"
          },
          {
            "name": "13/warm_accept",
            "value": 22573.638876582278,
            "unit": "ns"
          },
          {
            "name": "2/chat_turn_overhead",
            "value": 740.0225605323701,
            "unit": "ns"
          },
          {
            "name": "2/inbox_submit",
            "value": 42.646500022206354,
            "unit": "ns"
          },
          {
            "name": "3.1/registry_lookup",
            "value": 22.70562373308231,
            "unit": "ns"
          },
          {
            "name": "3.2/available_tools_50",
            "value": 31046.297549909257,
            "unit": "ns"
          },
          {
            "name": "3.3/dispatch_8_reads_concurrent",
            "value": 2167079.918067227,
            "unit": "ns"
          },
          {
            "name": "3.3/dispatch_8_reads_sequential",
            "value": 17034000.597222224,
            "unit": "ns"
          },
          {
            "name": "4.1/stdio_echo_round_trip",
            "value": 168411.862080316,
            "unit": "ns"
          },
          {
            "name": "4.2/http_echo_round_trip",
            "value": 402478.5402930403,
            "unit": "ns"
          },
          {
            "name": "4.3/mcp_50_concurrent",
            "value": 4150470.25,
            "unit": "ns"
          },
          {
            "name": "4.3/mcp_50_sequential",
            "value": 8513397.433333334,
            "unit": "ns"
          },
          {
            "name": "4.4/mcp_list_cold",
            "value": 175600.12476741095,
            "unit": "ns"
          },
          {
            "name": "4.4/mcp_list_hit",
            "value": 233.8039355566296,
            "unit": "ns"
          },
          {
            "name": "5.1/schema_tokens_1000",
            "value": 1266970.2689655172,
            "unit": "ns"
          },
          {
            "name": "5.1/schema_tokens_10000",
            "value": 42500826.333333336,
            "unit": "ns"
          },
          {
            "name": "5.2/index_build_10000",
            "value": 21816632.541666664,
            "unit": "ns"
          },
          {
            "name": "5.2/index_search_10000",
            "value": 241924.11920529802,
            "unit": "ns"
          },
          {
            "name": "5.3/full_schema_tokens_10000",
            "value": 22223488.601190478,
            "unit": "ns"
          },
          {
            "name": "5.3/lazy_visible_prompt_10000",
            "value": 16071.968309859156,
            "unit": "ns"
          },
          {
            "name": "5.4/routed_search_10000",
            "value": 23931.61630506246,
            "unit": "ns"
          },
          {
            "name": "5.4/shrink_schemas_10000",
            "value": 65287945.7,
            "unit": "ns"
          },
          {
            "name": "5.5/agent_filter_10000",
            "value": 113806.12657986111,
            "unit": "ns"
          },
          {
            "name": "5.5/cache_cold_10000",
            "value": 25219516.833333332,
            "unit": "ns"
          },
          {
            "name": "5.5/cache_hit_10000",
            "value": 16.00052163756698,
            "unit": "ns"
          },
          {
            "name": "6.1/gate_check",
            "value": 3.167560373299595,
            "unit": "ns"
          },
          {
            "name": "6.1/live_tool_body",
            "value": 1899.9078014800907,
            "unit": "ns"
          },
          {
            "name": "6.1/loop_8_reads",
            "value": 2185448.946895425,
            "unit": "ns"
          },
          {
            "name": "6.1/loop_iteration",
            "value": 366.09951304584916,
            "unit": "ns"
          },
          {
            "name": "6.10/budget_notice",
            "value": 79.17455666097064,
            "unit": "ns"
          },
          {
            "name": "6.10/cancel_select",
            "value": 120.06093883396296,
            "unit": "ns"
          },
          {
            "name": "6.10/canonical_args",
            "value": 1082.0758929758108,
            "unit": "ns"
          },
          {
            "name": "6.10/gate_check",
            "value": 7.319993166629057,
            "unit": "ns"
          },
          {
            "name": "6.10/live_tool_body",
            "value": 1932.1876895854398,
            "unit": "ns"
          },
          {
            "name": "6.10/loop_8_reads",
            "value": 2227898.154411765,
            "unit": "ns"
          },
          {
            "name": "6.10/loop_iteration",
            "value": 745.4529028669263,
            "unit": "ns"
          },
          {
            "name": "6.10/outcome_label",
            "value": 9.868031618731038,
            "unit": "ns"
          },
          {
            "name": "6.10/stall_observe",
            "value": 145.76910553483887,
            "unit": "ns"
          },
          {
            "name": "6.10/steer_append",
            "value": 58.71567531207536,
            "unit": "ns"
          },
          {
            "name": "6.10/stress_repair",
            "value": 38919.23025963597,
            "unit": "ns"
          },
          {
            "name": "6.10/truncate_output",
            "value": 4437.4728125,
            "unit": "ns"
          },
          {
            "name": "6.10/validate_repair",
            "value": 263.6730948787166,
            "unit": "ns"
          },
          {
            "name": "6.2/cancel_select",
            "value": 118.97863507807841,
            "unit": "ns"
          },
          {
            "name": "6.2/gate_check",
            "value": 7.301045496491516,
            "unit": "ns"
          },
          {
            "name": "6.2/live_tool_body",
            "value": 1940.7192476338164,
            "unit": "ns"
          },
          {
            "name": "6.2/loop_8_reads",
            "value": 2167712.060504202,
            "unit": "ns"
          },
          {
            "name": "6.2/loop_iteration",
            "value": 523.7926372077479,
            "unit": "ns"
          },
          {
            "name": "6.3/cancel_select",
            "value": 116.65583743777316,
            "unit": "ns"
          },
          {
            "name": "6.3/gate_check",
            "value": 9.504787952099456,
            "unit": "ns"
          },
          {
            "name": "6.3/live_tool_body",
            "value": 1949.3710832286904,
            "unit": "ns"
          },
          {
            "name": "6.3/loop_8_reads",
            "value": 2199836.490196078,
            "unit": "ns"
          },
          {
            "name": "6.3/loop_iteration",
            "value": 527.173377129335,
            "unit": "ns"
          },
          {
            "name": "6.3/validate_repair",
            "value": 263.8326997357268,
            "unit": "ns"
          },
          {
            "name": "6.4/cancel_select",
            "value": 142.77351419937767,
            "unit": "ns"
          },
          {
            "name": "6.4/gate_check",
            "value": 7.134440353860056,
            "unit": "ns"
          },
          {
            "name": "6.4/live_tool_body",
            "value": 1956.455712118793,
            "unit": "ns"
          },
          {
            "name": "6.4/loop_8_reads",
            "value": 2210372.011764706,
            "unit": "ns"
          },
          {
            "name": "6.4/loop_iteration",
            "value": 546.264570524344,
            "unit": "ns"
          },
          {
            "name": "6.4/stall_observe",
            "value": 146.25442131136998,
            "unit": "ns"
          },
          {
            "name": "6.4/validate_repair",
            "value": 259.82723957551,
            "unit": "ns"
          },
          {
            "name": "6.5/budget_notice",
            "value": 80.55186909604402,
            "unit": "ns"
          },
          {
            "name": "6.5/cancel_select",
            "value": 117.83151806828857,
            "unit": "ns"
          },
          {
            "name": "6.5/gate_check",
            "value": 7.317504239098238,
            "unit": "ns"
          },
          {
            "name": "6.5/live_tool_body",
            "value": 1899.9470525913575,
            "unit": "ns"
          },
          {
            "name": "6.5/loop_8_reads",
            "value": 2177001.8118580766,
            "unit": "ns"
          },
          {
            "name": "6.5/loop_iteration",
            "value": 599.206599142586,
            "unit": "ns"
          },
          {
            "name": "6.5/stall_observe",
            "value": 139.92322335891407,
            "unit": "ns"
          },
          {
            "name": "6.5/validate_repair",
            "value": 266.5987750454037,
            "unit": "ns"
          },
          {
            "name": "6.6/budget_notice",
            "value": 82.59447641166776,
            "unit": "ns"
          },
          {
            "name": "6.6/cancel_select",
            "value": 117.79146116939796,
            "unit": "ns"
          },
          {
            "name": "6.6/gate_check",
            "value": 7.316934985158518,
            "unit": "ns"
          },
          {
            "name": "6.6/live_tool_body",
            "value": 1866.4136549193881,
            "unit": "ns"
          },
          {
            "name": "6.6/loop_8_reads",
            "value": 2230008.948319328,
            "unit": "ns"
          },
          {
            "name": "6.6/loop_iteration",
            "value": 587.4754778875749,
            "unit": "ns"
          },
          {
            "name": "6.6/stall_observe",
            "value": 145.23364283634953,
            "unit": "ns"
          },
          {
            "name": "6.6/truncate_output",
            "value": 4449.951342613265,
            "unit": "ns"
          },
          {
            "name": "6.6/validate_repair",
            "value": 265.2707117926612,
            "unit": "ns"
          },
          {
            "name": "6.7/budget_notice",
            "value": 81.29140938168526,
            "unit": "ns"
          },
          {
            "name": "6.7/cancel_select",
            "value": 119.75672951356236,
            "unit": "ns"
          },
          {
            "name": "6.7/gate_check",
            "value": 7.41143656930724,
            "unit": "ns"
          },
          {
            "name": "6.7/live_tool_body",
            "value": 1818.6574455955981,
            "unit": "ns"
          },
          {
            "name": "6.7/loop_8_reads",
            "value": 2228313.8259803923,
            "unit": "ns"
          },
          {
            "name": "6.7/loop_iteration",
            "value": 593.2252771946821,
            "unit": "ns"
          },
          {
            "name": "6.7/stall_observe",
            "value": 140.5203827149232,
            "unit": "ns"
          },
          {
            "name": "6.7/steer_append",
            "value": 58.715250698676385,
            "unit": "ns"
          },
          {
            "name": "6.7/truncate_output",
            "value": 4470.841548289656,
            "unit": "ns"
          },
          {
            "name": "6.7/validate_repair",
            "value": 264.42261184632355,
            "unit": "ns"
          },
          {
            "name": "6.8/budget_notice",
            "value": 82.79098937960393,
            "unit": "ns"
          },
          {
            "name": "6.8/cancel_select",
            "value": 115.82592063538472,
            "unit": "ns"
          },
          {
            "name": "6.8/canonical_args",
            "value": 1068.888790734197,
            "unit": "ns"
          },
          {
            "name": "6.8/gate_check",
            "value": 7.4018141919191915,
            "unit": "ns"
          },
          {
            "name": "6.8/live_tool_body",
            "value": 1920.574511479796,
            "unit": "ns"
          },
          {
            "name": "6.8/loop_8_reads",
            "value": 2237743.48496732,
            "unit": "ns"
          },
          {
            "name": "6.8/loop_iteration",
            "value": 703.5732602850265,
            "unit": "ns"
          },
          {
            "name": "6.8/stall_observe",
            "value": 146.18247322200116,
            "unit": "ns"
          },
          {
            "name": "6.8/steer_append",
            "value": 58.957268543385275,
            "unit": "ns"
          },
          {
            "name": "6.8/truncate_output",
            "value": 4400.094180276064,
            "unit": "ns"
          },
          {
            "name": "6.8/validate_repair",
            "value": 266.59050329308457,
            "unit": "ns"
          },
          {
            "name": "6.9/budget_notice",
            "value": 80.39754045814732,
            "unit": "ns"
          },
          {
            "name": "6.9/cancel_select",
            "value": 117.85023091850348,
            "unit": "ns"
          },
          {
            "name": "6.9/canonical_args",
            "value": 1060.4537380980369,
            "unit": "ns"
          },
          {
            "name": "6.9/gate_check",
            "value": 7.309580401225325,
            "unit": "ns"
          },
          {
            "name": "6.9/live_tool_body",
            "value": 1979.16796474359,
            "unit": "ns"
          },
          {
            "name": "6.9/loop_8_reads",
            "value": 2230503.411764706,
            "unit": "ns"
          },
          {
            "name": "6.9/loop_iteration",
            "value": 754.2483903989794,
            "unit": "ns"
          },
          {
            "name": "6.9/outcome_label",
            "value": 9.87041994692956,
            "unit": "ns"
          },
          {
            "name": "6.9/stall_observe",
            "value": 143.31362227944186,
            "unit": "ns"
          },
          {
            "name": "6.9/steer_append",
            "value": 59.09403208670129,
            "unit": "ns"
          },
          {
            "name": "6.9/truncate_output",
            "value": 4474.465484248092,
            "unit": "ns"
          },
          {
            "name": "6.9/validate_repair",
            "value": 270.44545181891306,
            "unit": "ns"
          },
          {
            "name": "gw5/echo_through_gate",
            "value": 49.422164169474705,
            "unit": "ns"
          },
          {
            "name": "gw5/eight_concurrent_sleeps",
            "value": 2157504.0970588233,
            "unit": "ns"
          },
          {
            "name": "gw7/catalog_arc_swap",
            "value": 2478.20333242159,
            "unit": "ns"
          },
          {
            "name": "gw7/catalog_reload_10k",
            "value": 29407496.35714286,
            "unit": "ns"
          },
          {
            "name": "gw7/catalog_search_10k",
            "value": 241315.86589403974,
            "unit": "ns"
          },
          {
            "name": "gw8/dynamic_rename_task",
            "value": 5936963.923611111,
            "unit": "ns"
          },
          {
            "name": "gw8/every_tool_rename_task",
            "value": 8743382.133333333,
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
          "id": "e3d10735e3af01f0768abefe15cfdd775e341c68",
          "message": "Write chapters 11 through 20",
          "timestamp": "2026-10-15T04:44:13Z",
          "url": "https://github.com/agnathan/rust-agent-harness/pull/5/commits/e3d10735e3af01f0768abefe15cfdd775e341c68"
        },
        "date": 1792185989297,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "1.1/serde_50_message_request",
            "value": 10067.355369693561,
            "unit": "ns"
          },
          {
            "name": "1.2/mock_100_concurrent",
            "value": 78733.8412356322,
            "unit": "ns"
          },
          {
            "name": "1.2/mock_turn_overhead",
            "value": 250.30554221047856,
            "unit": "ns"
          },
          {
            "name": "1.3/build_request_and_parse_response",
            "value": 3109.8884766508368,
            "unit": "ns"
          },
          {
            "name": "1.4/parse_response_edges",
            "value": 1114.0551679748824,
            "unit": "ns"
          },
          {
            "name": "10/scripted_turn",
            "value": 7.78872964075191,
            "unit": "ns"
          },
          {
            "name": "10/turn_transition",
            "value": 1.106402604415393,
            "unit": "ns"
          },
          {
            "name": "11/skip_decision",
            "value": 251.3185038831692,
            "unit": "ns"
          },
          {
            "name": "12/api_request_hook",
            "value": 2.1797526323363177,
            "unit": "ns"
          },
          {
            "name": "12/empty_pre_post",
            "value": 17.809967817956135,
            "unit": "ns"
          },
          {
            "name": "12/memory_enqueue",
            "value": 57.41530180540746,
            "unit": "ns"
          },
          {
            "name": "13/checkpoint_1000",
            "value": 50355.7238343254,
            "unit": "ns"
          },
          {
            "name": "13/rewind_1000",
            "value": 81092.51657770801,
            "unit": "ns"
          },
          {
            "name": "13/warm_accept",
            "value": 28418.37461023213,
            "unit": "ns"
          },
          {
            "name": "14/empty_budget",
            "value": 0.904327184930229,
            "unit": "ns"
          },
          {
            "name": "14/repetition_guard",
            "value": 2541.763861765937,
            "unit": "ns"
          },
          {
            "name": "2/chat_turn_overhead",
            "value": 725.197323981964,
            "unit": "ns"
          },
          {
            "name": "2/inbox_submit",
            "value": 40.455266831255145,
            "unit": "ns"
          },
          {
            "name": "3.1/registry_lookup",
            "value": 25.956160419376058,
            "unit": "ns"
          },
          {
            "name": "3.2/available_tools_50",
            "value": 30603.283301767675,
            "unit": "ns"
          },
          {
            "name": "3.3/dispatch_8_reads_concurrent",
            "value": 2139676.088235294,
            "unit": "ns"
          },
          {
            "name": "3.3/dispatch_8_reads_sequential",
            "value": 16679482.177777778,
            "unit": "ns"
          },
          {
            "name": "4.1/stdio_echo_round_trip",
            "value": 202407.90870786516,
            "unit": "ns"
          },
          {
            "name": "4.2/http_echo_round_trip",
            "value": 415581.9772727273,
            "unit": "ns"
          },
          {
            "name": "4.3/mcp_50_concurrent",
            "value": 4999685.71875,
            "unit": "ns"
          },
          {
            "name": "4.3/mcp_50_sequential",
            "value": 10100506.703571428,
            "unit": "ns"
          },
          {
            "name": "4.4/mcp_list_cold",
            "value": 216428.34181636726,
            "unit": "ns"
          },
          {
            "name": "4.4/mcp_list_hit",
            "value": 253.4479101291341,
            "unit": "ns"
          },
          {
            "name": "5.1/schema_tokens_1000",
            "value": 1274939.3555555556,
            "unit": "ns"
          },
          {
            "name": "5.1/schema_tokens_10000",
            "value": 43549883.69444445,
            "unit": "ns"
          },
          {
            "name": "5.2/index_build_10000",
            "value": 21475412.875,
            "unit": "ns"
          },
          {
            "name": "5.2/index_search_10000",
            "value": 334181.2119708995,
            "unit": "ns"
          },
          {
            "name": "5.3/full_schema_tokens_10000",
            "value": 22398704.083333336,
            "unit": "ns"
          },
          {
            "name": "5.3/lazy_visible_prompt_10000",
            "value": 15649.475373293515,
            "unit": "ns"
          },
          {
            "name": "5.4/routed_search_10000",
            "value": 25919.108827734097,
            "unit": "ns"
          },
          {
            "name": "5.4/shrink_schemas_10000",
            "value": 62102279.775,
            "unit": "ns"
          },
          {
            "name": "5.5/agent_filter_10000",
            "value": 125920.138177624,
            "unit": "ns"
          },
          {
            "name": "5.5/cache_cold_10000",
            "value": 26184547.55,
            "unit": "ns"
          },
          {
            "name": "5.5/cache_hit_10000",
            "value": 13.801803684942959,
            "unit": "ns"
          },
          {
            "name": "6.1/gate_check",
            "value": 3.4272516824126953,
            "unit": "ns"
          },
          {
            "name": "6.1/live_tool_body",
            "value": 1735.4523355710062,
            "unit": "ns"
          },
          {
            "name": "6.1/loop_8_reads",
            "value": 2158333.4598039216,
            "unit": "ns"
          },
          {
            "name": "6.1/loop_iteration",
            "value": 362.9410208851992,
            "unit": "ns"
          },
          {
            "name": "6.10/budget_notice",
            "value": 76.60075749310035,
            "unit": "ns"
          },
          {
            "name": "6.10/cancel_select",
            "value": 94.96699239382579,
            "unit": "ns"
          },
          {
            "name": "6.10/canonical_args",
            "value": 1062.3793491236243,
            "unit": "ns"
          },
          {
            "name": "6.10/gate_check",
            "value": 7.476249222966525,
            "unit": "ns"
          },
          {
            "name": "6.10/live_tool_body",
            "value": 1723.3712340425532,
            "unit": "ns"
          },
          {
            "name": "6.10/loop_8_reads",
            "value": 2180806.2826797385,
            "unit": "ns"
          },
          {
            "name": "6.10/loop_iteration",
            "value": 762.6818738572734,
            "unit": "ns"
          },
          {
            "name": "6.10/outcome_label",
            "value": 8.395247274819106,
            "unit": "ns"
          },
          {
            "name": "6.10/stall_observe",
            "value": 136.34717495698862,
            "unit": "ns"
          },
          {
            "name": "6.10/steer_append",
            "value": 59.82740468929405,
            "unit": "ns"
          },
          {
            "name": "6.10/stress_repair",
            "value": 50899.54816931217,
            "unit": "ns"
          },
          {
            "name": "6.10/truncate_output",
            "value": 4611.82508235145,
            "unit": "ns"
          },
          {
            "name": "6.10/validate_repair",
            "value": 271.6811458403052,
            "unit": "ns"
          },
          {
            "name": "6.2/cancel_select",
            "value": 94.8593727284761,
            "unit": "ns"
          },
          {
            "name": "6.2/gate_check",
            "value": 7.472425481187459,
            "unit": "ns"
          },
          {
            "name": "6.2/live_tool_body",
            "value": 1771.8692647763405,
            "unit": "ns"
          },
          {
            "name": "6.2/loop_8_reads",
            "value": 2160573.5514705884,
            "unit": "ns"
          },
          {
            "name": "6.2/loop_iteration",
            "value": 556.9488594227976,
            "unit": "ns"
          },
          {
            "name": "6.3/cancel_select",
            "value": 96.45349491731592,
            "unit": "ns"
          },
          {
            "name": "6.3/gate_check",
            "value": 7.471986056355142,
            "unit": "ns"
          },
          {
            "name": "6.3/live_tool_body",
            "value": 1740.1266454173683,
            "unit": "ns"
          },
          {
            "name": "6.3/loop_8_reads",
            "value": 2173352.154901961,
            "unit": "ns"
          },
          {
            "name": "6.3/loop_iteration",
            "value": 539.4656627664452,
            "unit": "ns"
          },
          {
            "name": "6.3/validate_repair",
            "value": 262.00128799775996,
            "unit": "ns"
          },
          {
            "name": "6.4/cancel_select",
            "value": 94.86942184931996,
            "unit": "ns"
          },
          {
            "name": "6.4/gate_check",
            "value": 7.482466871011965,
            "unit": "ns"
          },
          {
            "name": "6.4/live_tool_body",
            "value": 1858.964790442134,
            "unit": "ns"
          },
          {
            "name": "6.4/loop_8_reads",
            "value": 2166656.430147059,
            "unit": "ns"
          },
          {
            "name": "6.4/loop_iteration",
            "value": 524.3829522545216,
            "unit": "ns"
          },
          {
            "name": "6.4/stall_observe",
            "value": 139.26163477190917,
            "unit": "ns"
          },
          {
            "name": "6.4/validate_repair",
            "value": 264.55834964906023,
            "unit": "ns"
          },
          {
            "name": "6.5/budget_notice",
            "value": 75.65747114079542,
            "unit": "ns"
          },
          {
            "name": "6.5/cancel_select",
            "value": 97.29098526613927,
            "unit": "ns"
          },
          {
            "name": "6.5/gate_check",
            "value": 7.471749839347259,
            "unit": "ns"
          },
          {
            "name": "6.5/live_tool_body",
            "value": 1755.5276076578098,
            "unit": "ns"
          },
          {
            "name": "6.5/loop_8_reads",
            "value": 2177342.270774977,
            "unit": "ns"
          },
          {
            "name": "6.5/loop_iteration",
            "value": 613.1409193163886,
            "unit": "ns"
          },
          {
            "name": "6.5/stall_observe",
            "value": 146.27081807093455,
            "unit": "ns"
          },
          {
            "name": "6.5/validate_repair",
            "value": 269.89511254430613,
            "unit": "ns"
          },
          {
            "name": "6.6/budget_notice",
            "value": 77.72985534567243,
            "unit": "ns"
          },
          {
            "name": "6.6/cancel_select",
            "value": 95.31481061123782,
            "unit": "ns"
          },
          {
            "name": "6.6/gate_check",
            "value": 7.476528653008778,
            "unit": "ns"
          },
          {
            "name": "6.6/live_tool_body",
            "value": 1736.0199396221903,
            "unit": "ns"
          },
          {
            "name": "6.6/loop_8_reads",
            "value": 2179222.507352941,
            "unit": "ns"
          },
          {
            "name": "6.6/loop_iteration",
            "value": 611.7866475587689,
            "unit": "ns"
          },
          {
            "name": "6.6/stall_observe",
            "value": 140.93668969890535,
            "unit": "ns"
          },
          {
            "name": "6.6/truncate_output",
            "value": 4589.841179275716,
            "unit": "ns"
          },
          {
            "name": "6.6/validate_repair",
            "value": 257.04810371343854,
            "unit": "ns"
          },
          {
            "name": "6.7/budget_notice",
            "value": 76.90898047294161,
            "unit": "ns"
          },
          {
            "name": "6.7/cancel_select",
            "value": 95.78407052218003,
            "unit": "ns"
          },
          {
            "name": "6.7/gate_check",
            "value": 7.4739959091451595,
            "unit": "ns"
          },
          {
            "name": "6.7/live_tool_body",
            "value": 1659.2755206668799,
            "unit": "ns"
          },
          {
            "name": "6.7/loop_8_reads",
            "value": 2176299.2156862747,
            "unit": "ns"
          },
          {
            "name": "6.7/loop_iteration",
            "value": 610.135195843421,
            "unit": "ns"
          },
          {
            "name": "6.7/stall_observe",
            "value": 140.10397605521985,
            "unit": "ns"
          },
          {
            "name": "6.7/steer_append",
            "value": 58.35534439459579,
            "unit": "ns"
          },
          {
            "name": "6.7/truncate_output",
            "value": 4582.552419749975,
            "unit": "ns"
          },
          {
            "name": "6.7/validate_repair",
            "value": 265.73224803445,
            "unit": "ns"
          },
          {
            "name": "6.8/budget_notice",
            "value": 78.89725041604477,
            "unit": "ns"
          },
          {
            "name": "6.8/cancel_select",
            "value": 96.77227863119279,
            "unit": "ns"
          },
          {
            "name": "6.8/canonical_args",
            "value": 1057.0595039434502,
            "unit": "ns"
          },
          {
            "name": "6.8/gate_check",
            "value": 7.472407470131531,
            "unit": "ns"
          },
          {
            "name": "6.8/live_tool_body",
            "value": 1731.5600101078694,
            "unit": "ns"
          },
          {
            "name": "6.8/loop_8_reads",
            "value": 2182308.8209150326,
            "unit": "ns"
          },
          {
            "name": "6.8/loop_iteration",
            "value": 683.6203852177805,
            "unit": "ns"
          },
          {
            "name": "6.8/stall_observe",
            "value": 138.94788510614154,
            "unit": "ns"
          },
          {
            "name": "6.8/steer_append",
            "value": 58.13604719144568,
            "unit": "ns"
          },
          {
            "name": "6.8/truncate_output",
            "value": 4557.226000652677,
            "unit": "ns"
          },
          {
            "name": "6.8/validate_repair",
            "value": 265.7442371903061,
            "unit": "ns"
          },
          {
            "name": "6.9/budget_notice",
            "value": 78.08582222729899,
            "unit": "ns"
          },
          {
            "name": "6.9/cancel_select",
            "value": 102.24384647016169,
            "unit": "ns"
          },
          {
            "name": "6.9/canonical_args",
            "value": 1063.6666905786706,
            "unit": "ns"
          },
          {
            "name": "6.9/gate_check",
            "value": 7.478105074816623,
            "unit": "ns"
          },
          {
            "name": "6.9/live_tool_body",
            "value": 1748.281976369841,
            "unit": "ns"
          },
          {
            "name": "6.9/loop_8_reads",
            "value": 2183399.341503268,
            "unit": "ns"
          },
          {
            "name": "6.9/loop_iteration",
            "value": 750.1269935633368,
            "unit": "ns"
          },
          {
            "name": "6.9/outcome_label",
            "value": 8.395119855577846,
            "unit": "ns"
          },
          {
            "name": "6.9/stall_observe",
            "value": 137.04166127263923,
            "unit": "ns"
          },
          {
            "name": "6.9/steer_append",
            "value": 59.18152072287157,
            "unit": "ns"
          },
          {
            "name": "6.9/truncate_output",
            "value": 4673.261484288038,
            "unit": "ns"
          },
          {
            "name": "6.9/validate_repair",
            "value": 260.8592732070892,
            "unit": "ns"
          },
          {
            "name": "gw5/echo_through_gate",
            "value": 45.58644758000788,
            "unit": "ns"
          },
          {
            "name": "gw5/eight_concurrent_sleeps",
            "value": 2119886.0805555554,
            "unit": "ns"
          },
          {
            "name": "gw7/catalog_arc_swap",
            "value": 2418.9842917997867,
            "unit": "ns"
          },
          {
            "name": "gw7/catalog_reload_10k",
            "value": 29302029,
            "unit": "ns"
          },
          {
            "name": "gw7/catalog_search_10k",
            "value": 300684.7350140056,
            "unit": "ns"
          },
          {
            "name": "gw8/dynamic_rename_task",
            "value": 5746927.902380953,
            "unit": "ns"
          },
          {
            "name": "gw8/every_tool_rename_task",
            "value": 8564706.05,
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
          "id": "5820b4092b5e5ca85134133683f86d12fcf396fe",
          "message": "Write chapters 11 through 20",
          "timestamp": "2026-10-15T04:44:13Z",
          "url": "https://github.com/agnathan/rust-agent-harness/pull/5/commits/5820b4092b5e5ca85134133683f86d12fcf396fe"
        },
        "date": 1792186107410,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "1.1/serde_50_message_request",
            "value": 7737.424957537154,
            "unit": "ns"
          },
          {
            "name": "1.2/mock_100_concurrent",
            "value": 97132.2460399449,
            "unit": "ns"
          },
          {
            "name": "1.2/mock_turn_overhead",
            "value": 232.33133466249058,
            "unit": "ns"
          },
          {
            "name": "1.3/build_request_and_parse_response",
            "value": 2385.9902322092494,
            "unit": "ns"
          },
          {
            "name": "1.4/parse_response_edges",
            "value": 851.7252027655898,
            "unit": "ns"
          },
          {
            "name": "10/scripted_turn",
            "value": 7.631646324044712,
            "unit": "ns"
          },
          {
            "name": "10/turn_transition",
            "value": 1.1204823206843735,
            "unit": "ns"
          },
          {
            "name": "11/skip_decision",
            "value": 201.48084808955554,
            "unit": "ns"
          },
          {
            "name": "12/api_request_hook",
            "value": 1.958061590189831,
            "unit": "ns"
          },
          {
            "name": "12/empty_pre_post",
            "value": 8.967460087440724,
            "unit": "ns"
          },
          {
            "name": "12/memory_enqueue",
            "value": 46.584361376129735,
            "unit": "ns"
          },
          {
            "name": "13/checkpoint_1000",
            "value": 67646.62980173482,
            "unit": "ns"
          },
          {
            "name": "13/rewind_1000",
            "value": 101770.19071720025,
            "unit": "ns"
          },
          {
            "name": "13/warm_accept",
            "value": 27155.48907208589,
            "unit": "ns"
          },
          {
            "name": "14/empty_budget",
            "value": 0.7585076778607556,
            "unit": "ns"
          },
          {
            "name": "14/repetition_guard",
            "value": 1837.189282457829,
            "unit": "ns"
          },
          {
            "name": "15/open_breaker",
            "value": 24.34327198638875,
            "unit": "ns"
          },
          {
            "name": "15/permit_release",
            "value": 90.25607714087026,
            "unit": "ns"
          },
          {
            "name": "2/chat_turn_overhead",
            "value": 678.0192520549784,
            "unit": "ns"
          },
          {
            "name": "2/inbox_submit",
            "value": 35.47784185349738,
            "unit": "ns"
          },
          {
            "name": "3.1/registry_lookup",
            "value": 18.193449805654865,
            "unit": "ns"
          },
          {
            "name": "3.2/available_tools_50",
            "value": 24472.16868686869,
            "unit": "ns"
          },
          {
            "name": "3.3/dispatch_8_reads_concurrent",
            "value": 2133392.5357142854,
            "unit": "ns"
          },
          {
            "name": "3.3/dispatch_8_reads_sequential",
            "value": 16585632.116666667,
            "unit": "ns"
          },
          {
            "name": "4.1/stdio_echo_round_trip",
            "value": 160966.76364306785,
            "unit": "ns"
          },
          {
            "name": "4.2/http_echo_round_trip",
            "value": 342809.2693920336,
            "unit": "ns"
          },
          {
            "name": "4.3/mcp_50_concurrent",
            "value": 3016412.0865384615,
            "unit": "ns"
          },
          {
            "name": "4.3/mcp_50_sequential",
            "value": 6343580.994212963,
            "unit": "ns"
          },
          {
            "name": "4.4/mcp_list_cold",
            "value": 172755.66255868546,
            "unit": "ns"
          },
          {
            "name": "4.4/mcp_list_hit",
            "value": 193.7005491566046,
            "unit": "ns"
          },
          {
            "name": "5.1/schema_tokens_1000",
            "value": 996562.5820463321,
            "unit": "ns"
          },
          {
            "name": "5.1/schema_tokens_10000",
            "value": 29583404.263888888,
            "unit": "ns"
          },
          {
            "name": "5.2/index_build_10000",
            "value": 15791187.625,
            "unit": "ns"
          },
          {
            "name": "5.2/index_search_10000",
            "value": 167061.93550668727,
            "unit": "ns"
          },
          {
            "name": "5.3/full_schema_tokens_10000",
            "value": 11207574.75,
            "unit": "ns"
          },
          {
            "name": "5.3/lazy_visible_prompt_10000",
            "value": 14445.598114680946,
            "unit": "ns"
          },
          {
            "name": "5.4/routed_search_10000",
            "value": 16855.50165424045,
            "unit": "ns"
          },
          {
            "name": "5.4/shrink_schemas_10000",
            "value": 40833824.60416667,
            "unit": "ns"
          },
          {
            "name": "5.5/agent_filter_10000",
            "value": 56854.18506998445,
            "unit": "ns"
          },
          {
            "name": "5.5/cache_cold_10000",
            "value": 18399729.19375,
            "unit": "ns"
          },
          {
            "name": "5.5/cache_hit_10000",
            "value": 19.623520696281098,
            "unit": "ns"
          },
          {
            "name": "6.1/gate_check",
            "value": 2.9973853859437836,
            "unit": "ns"
          },
          {
            "name": "6.1/live_tool_body",
            "value": 1762.898746627506,
            "unit": "ns"
          },
          {
            "name": "6.1/loop_8_reads",
            "value": 2134553.7041666666,
            "unit": "ns"
          },
          {
            "name": "6.1/loop_iteration",
            "value": 275.9042763384617,
            "unit": "ns"
          },
          {
            "name": "6.10/budget_notice",
            "value": 62.80818055545958,
            "unit": "ns"
          },
          {
            "name": "6.10/cancel_select",
            "value": 176.82070828478868,
            "unit": "ns"
          },
          {
            "name": "6.10/canonical_args",
            "value": 882.1472264668073,
            "unit": "ns"
          },
          {
            "name": "6.10/gate_check",
            "value": 16.234162662750016,
            "unit": "ns"
          },
          {
            "name": "6.10/live_tool_body",
            "value": 1669.6743059029513,
            "unit": "ns"
          },
          {
            "name": "6.10/loop_8_reads",
            "value": 2154606.8848039214,
            "unit": "ns"
          },
          {
            "name": "6.10/loop_iteration",
            "value": 736.4259088830622,
            "unit": "ns"
          },
          {
            "name": "6.10/outcome_label",
            "value": 6.156837604233165,
            "unit": "ns"
          },
          {
            "name": "6.10/stall_observe",
            "value": 112.39465376255401,
            "unit": "ns"
          },
          {
            "name": "6.10/steer_append",
            "value": 53.34120467595788,
            "unit": "ns"
          },
          {
            "name": "6.10/stress_repair",
            "value": 42787.53723045822,
            "unit": "ns"
          },
          {
            "name": "6.10/truncate_output",
            "value": 3576.1012367534913,
            "unit": "ns"
          },
          {
            "name": "6.10/validate_repair",
            "value": 245.71498434390432,
            "unit": "ns"
          },
          {
            "name": "6.2/cancel_select",
            "value": 177.91695572493444,
            "unit": "ns"
          },
          {
            "name": "6.2/gate_check",
            "value": 16.25227976910568,
            "unit": "ns"
          },
          {
            "name": "6.2/live_tool_body",
            "value": 1748.921530305236,
            "unit": "ns"
          },
          {
            "name": "6.2/loop_8_reads",
            "value": 2141710.6397058824,
            "unit": "ns"
          },
          {
            "name": "6.2/loop_iteration",
            "value": 527.1049539404859,
            "unit": "ns"
          },
          {
            "name": "6.3/cancel_select",
            "value": 180.6394532345986,
            "unit": "ns"
          },
          {
            "name": "6.3/gate_check",
            "value": 16.305262521865423,
            "unit": "ns"
          },
          {
            "name": "6.3/live_tool_body",
            "value": 1842.4948606373728,
            "unit": "ns"
          },
          {
            "name": "6.3/loop_8_reads",
            "value": 2145302.4166666665,
            "unit": "ns"
          },
          {
            "name": "6.3/loop_iteration",
            "value": 527.663239767433,
            "unit": "ns"
          },
          {
            "name": "6.3/validate_repair",
            "value": 237.88803865062613,
            "unit": "ns"
          },
          {
            "name": "6.4/cancel_select",
            "value": 177.85019152818683,
            "unit": "ns"
          },
          {
            "name": "6.4/gate_check",
            "value": 16.243602933137186,
            "unit": "ns"
          },
          {
            "name": "6.4/live_tool_body",
            "value": 1716.9496735576445,
            "unit": "ns"
          },
          {
            "name": "6.4/loop_8_reads",
            "value": 2150527.166666667,
            "unit": "ns"
          },
          {
            "name": "6.4/loop_iteration",
            "value": 517.4920249965625,
            "unit": "ns"
          },
          {
            "name": "6.4/stall_observe",
            "value": 112.54270782397349,
            "unit": "ns"
          },
          {
            "name": "6.4/validate_repair",
            "value": 246.15747459596778,
            "unit": "ns"
          },
          {
            "name": "6.5/budget_notice",
            "value": 63.41013762031078,
            "unit": "ns"
          },
          {
            "name": "6.5/cancel_select",
            "value": 179.02450616267248,
            "unit": "ns"
          },
          {
            "name": "6.5/gate_check",
            "value": 16.307016496195107,
            "unit": "ns"
          },
          {
            "name": "6.5/live_tool_body",
            "value": 1613.982115058202,
            "unit": "ns"
          },
          {
            "name": "6.5/loop_8_reads",
            "value": 2154982.724264706,
            "unit": "ns"
          },
          {
            "name": "6.5/loop_iteration",
            "value": 581.2871418777222,
            "unit": "ns"
          },
          {
            "name": "6.5/stall_observe",
            "value": 110.20727143388399,
            "unit": "ns"
          },
          {
            "name": "6.5/validate_repair",
            "value": 251.93972312175293,
            "unit": "ns"
          },
          {
            "name": "6.6/budget_notice",
            "value": 64.76893911045141,
            "unit": "ns"
          },
          {
            "name": "6.6/cancel_select",
            "value": 178.38101772059116,
            "unit": "ns"
          },
          {
            "name": "6.6/gate_check",
            "value": 16.289149396301013,
            "unit": "ns"
          },
          {
            "name": "6.6/live_tool_body",
            "value": 1772.387544108998,
            "unit": "ns"
          },
          {
            "name": "6.6/loop_8_reads",
            "value": 2163042.9575163396,
            "unit": "ns"
          },
          {
            "name": "6.6/loop_iteration",
            "value": 667.0891341100371,
            "unit": "ns"
          },
          {
            "name": "6.6/stall_observe",
            "value": 115.68965690036853,
            "unit": "ns"
          },
          {
            "name": "6.6/truncate_output",
            "value": 3496.759983920245,
            "unit": "ns"
          },
          {
            "name": "6.6/validate_repair",
            "value": 241.3509989089654,
            "unit": "ns"
          },
          {
            "name": "6.7/budget_notice",
            "value": 60.30445801952813,
            "unit": "ns"
          },
          {
            "name": "6.7/cancel_select",
            "value": 177.90020814807824,
            "unit": "ns"
          },
          {
            "name": "6.7/gate_check",
            "value": 16.245903275583075,
            "unit": "ns"
          },
          {
            "name": "6.7/live_tool_body",
            "value": 1553.428495405001,
            "unit": "ns"
          },
          {
            "name": "6.7/loop_8_reads",
            "value": 2157533.1529411767,
            "unit": "ns"
          },
          {
            "name": "6.7/loop_iteration",
            "value": 581.3659036569907,
            "unit": "ns"
          },
          {
            "name": "6.7/stall_observe",
            "value": 113.08684656890273,
            "unit": "ns"
          },
          {
            "name": "6.7/steer_append",
            "value": 53.40420250963108,
            "unit": "ns"
          },
          {
            "name": "6.7/truncate_output",
            "value": 3454.2775904880145,
            "unit": "ns"
          },
          {
            "name": "6.7/validate_repair",
            "value": 246.1446320457797,
            "unit": "ns"
          },
          {
            "name": "6.8/budget_notice",
            "value": 60.11419394133836,
            "unit": "ns"
          },
          {
            "name": "6.8/cancel_select",
            "value": 176.66180159904633,
            "unit": "ns"
          },
          {
            "name": "6.8/canonical_args",
            "value": 879.104056756538,
            "unit": "ns"
          },
          {
            "name": "6.8/gate_check",
            "value": 16.241079485047095,
            "unit": "ns"
          },
          {
            "name": "6.8/live_tool_body",
            "value": 1720.9629602208113,
            "unit": "ns"
          },
          {
            "name": "6.8/loop_8_reads",
            "value": 2159462.550420168,
            "unit": "ns"
          },
          {
            "name": "6.8/loop_iteration",
            "value": 690.3434756246593,
            "unit": "ns"
          },
          {
            "name": "6.8/stall_observe",
            "value": 114.31028701127104,
            "unit": "ns"
          },
          {
            "name": "6.8/steer_append",
            "value": 53.47986800375874,
            "unit": "ns"
          },
          {
            "name": "6.8/truncate_output",
            "value": 3497.268390956594,
            "unit": "ns"
          },
          {
            "name": "6.8/validate_repair",
            "value": 248.42369087032344,
            "unit": "ns"
          },
          {
            "name": "6.9/budget_notice",
            "value": 61.30267247348711,
            "unit": "ns"
          },
          {
            "name": "6.9/cancel_select",
            "value": 177.89052485620857,
            "unit": "ns"
          },
          {
            "name": "6.9/canonical_args",
            "value": 869.956007682438,
            "unit": "ns"
          },
          {
            "name": "6.9/gate_check",
            "value": 16.248579512491183,
            "unit": "ns"
          },
          {
            "name": "6.9/live_tool_body",
            "value": 1818.5904813277612,
            "unit": "ns"
          },
          {
            "name": "6.9/loop_8_reads",
            "value": 2154180.833333333,
            "unit": "ns"
          },
          {
            "name": "6.9/loop_iteration",
            "value": 740.7223095569102,
            "unit": "ns"
          },
          {
            "name": "6.9/outcome_label",
            "value": 6.159174341484246,
            "unit": "ns"
          },
          {
            "name": "6.9/stall_observe",
            "value": 110.240402490935,
            "unit": "ns"
          },
          {
            "name": "6.9/steer_append",
            "value": 53.31711114286007,
            "unit": "ns"
          },
          {
            "name": "6.9/truncate_output",
            "value": 3448.2579203278756,
            "unit": "ns"
          },
          {
            "name": "6.9/validate_repair",
            "value": 234.37255059410654,
            "unit": "ns"
          },
          {
            "name": "gw5/echo_through_gate",
            "value": 92.94487264668533,
            "unit": "ns"
          },
          {
            "name": "gw5/eight_concurrent_sleeps",
            "value": 2101072.095238095,
            "unit": "ns"
          },
          {
            "name": "gw7/catalog_arc_swap",
            "value": 2075.735884295312,
            "unit": "ns"
          },
          {
            "name": "gw7/catalog_reload_10k",
            "value": 24074694.72857143,
            "unit": "ns"
          },
          {
            "name": "gw7/catalog_search_10k",
            "value": 162194.7390134529,
            "unit": "ns"
          },
          {
            "name": "gw8/dynamic_rename_task",
            "value": 4499771.949074075,
            "unit": "ns"
          },
          {
            "name": "gw8/every_tool_rename_task",
            "value": 6506120.53125,
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
          "id": "518b4a4cc6aeef545f4914d137ee6a5eb8d57e16",
          "message": "Write chapters 11 through 20",
          "timestamp": "2026-10-15T04:44:13Z",
          "url": "https://github.com/agnathan/rust-agent-harness/pull/5/commits/518b4a4cc6aeef545f4914d137ee6a5eb8d57e16"
        },
        "date": 1792186973001,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "1.1/serde_50_message_request",
            "value": 8286.91702511044,
            "unit": "ns"
          },
          {
            "name": "1.2/mock_100_concurrent",
            "value": 65369.58024453024,
            "unit": "ns"
          },
          {
            "name": "1.2/mock_turn_overhead",
            "value": 207.424247249232,
            "unit": "ns"
          },
          {
            "name": "1.3/build_request_and_parse_response",
            "value": 2184.3650634186815,
            "unit": "ns"
          },
          {
            "name": "1.4/parse_response_edges",
            "value": 915.9013383082073,
            "unit": "ns"
          },
          {
            "name": "10/scripted_turn",
            "value": 5.383633144941828,
            "unit": "ns"
          },
          {
            "name": "10/turn_transition",
            "value": 0.9129328166704791,
            "unit": "ns"
          },
          {
            "name": "11/skip_decision",
            "value": 212.34040541289596,
            "unit": "ns"
          },
          {
            "name": "12/api_request_hook",
            "value": 1.9096977581566663,
            "unit": "ns"
          },
          {
            "name": "12/empty_pre_post",
            "value": 9.815384919807752,
            "unit": "ns"
          },
          {
            "name": "12/memory_enqueue",
            "value": 46.02263608149865,
            "unit": "ns"
          },
          {
            "name": "13/checkpoint_1000",
            "value": 41470.44178248222,
            "unit": "ns"
          },
          {
            "name": "13/rewind_1000",
            "value": 62575.51927262313,
            "unit": "ns"
          },
          {
            "name": "13/warm_accept",
            "value": 18999.695887445887,
            "unit": "ns"
          },
          {
            "name": "14/empty_budget",
            "value": 0.733535399451678,
            "unit": "ns"
          },
          {
            "name": "14/repetition_guard",
            "value": 1929.2700716988238,
            "unit": "ns"
          },
          {
            "name": "15/open_breaker",
            "value": 20.01740424879815,
            "unit": "ns"
          },
          {
            "name": "15/permit_release",
            "value": 49.150029860326796,
            "unit": "ns"
          },
          {
            "name": "16/choose_route",
            "value": 4.364653252155926,
            "unit": "ns"
          },
          {
            "name": "16/rotate_key",
            "value": 3.038711620056022,
            "unit": "ns"
          },
          {
            "name": "2/chat_turn_overhead",
            "value": 619.00137639155,
            "unit": "ns"
          },
          {
            "name": "2/inbox_submit",
            "value": 33.48802389092229,
            "unit": "ns"
          },
          {
            "name": "3.1/registry_lookup",
            "value": 17.715110050722295,
            "unit": "ns"
          },
          {
            "name": "3.2/available_tools_50",
            "value": 24224.317735956658,
            "unit": "ns"
          },
          {
            "name": "3.3/dispatch_8_reads_concurrent",
            "value": 2143906.7680672267,
            "unit": "ns"
          },
          {
            "name": "3.3/dispatch_8_reads_sequential",
            "value": 16836987.133333333,
            "unit": "ns"
          },
          {
            "name": "4.1/stdio_echo_round_trip",
            "value": 130941.38086642599,
            "unit": "ns"
          },
          {
            "name": "4.2/http_echo_round_trip",
            "value": 315497.1615497076,
            "unit": "ns"
          },
          {
            "name": "4.3/mcp_50_concurrent",
            "value": 3260173.232638889,
            "unit": "ns"
          },
          {
            "name": "4.3/mcp_50_sequential",
            "value": 6650812.132142857,
            "unit": "ns"
          },
          {
            "name": "4.4/mcp_list_cold",
            "value": 138167.0016357688,
            "unit": "ns"
          },
          {
            "name": "4.4/mcp_list_hit",
            "value": 180.64313755378137,
            "unit": "ns"
          },
          {
            "name": "5.1/schema_tokens_1000",
            "value": 991859.096996997,
            "unit": "ns"
          },
          {
            "name": "5.1/schema_tokens_10000",
            "value": 37473912.5,
            "unit": "ns"
          },
          {
            "name": "5.2/index_build_10000",
            "value": 17960361.6875,
            "unit": "ns"
          },
          {
            "name": "5.2/index_search_10000",
            "value": 191718.39716312056,
            "unit": "ns"
          },
          {
            "name": "5.3/full_schema_tokens_10000",
            "value": 19305299.655555554,
            "unit": "ns"
          },
          {
            "name": "5.3/lazy_visible_prompt_10000",
            "value": 12506.986834540647,
            "unit": "ns"
          },
          {
            "name": "5.4/routed_search_10000",
            "value": 18625.763524590162,
            "unit": "ns"
          },
          {
            "name": "5.4/shrink_schemas_10000",
            "value": 62318678.61111111,
            "unit": "ns"
          },
          {
            "name": "5.5/agent_filter_10000",
            "value": 86256.7066774804,
            "unit": "ns"
          },
          {
            "name": "5.5/cache_cold_10000",
            "value": 22251916,
            "unit": "ns"
          },
          {
            "name": "5.5/cache_hit_10000",
            "value": 12.405240040892885,
            "unit": "ns"
          },
          {
            "name": "6.1/gate_check",
            "value": 2.460509821024611,
            "unit": "ns"
          },
          {
            "name": "6.1/live_tool_body",
            "value": 1505.888915502488,
            "unit": "ns"
          },
          {
            "name": "6.1/loop_8_reads",
            "value": 2162320.136134454,
            "unit": "ns"
          },
          {
            "name": "6.1/loop_iteration",
            "value": 285.59403886763096,
            "unit": "ns"
          },
          {
            "name": "6.10/budget_notice",
            "value": 64.83508930917819,
            "unit": "ns"
          },
          {
            "name": "6.10/cancel_select",
            "value": 90.74954592868272,
            "unit": "ns"
          },
          {
            "name": "6.10/canonical_args",
            "value": 853.2250519168185,
            "unit": "ns"
          },
          {
            "name": "6.10/gate_check",
            "value": 5.659216918040142,
            "unit": "ns"
          },
          {
            "name": "6.10/live_tool_body",
            "value": 1526.0714816018224,
            "unit": "ns"
          },
          {
            "name": "6.10/loop_8_reads",
            "value": 2197347.92867647,
            "unit": "ns"
          },
          {
            "name": "6.10/loop_iteration",
            "value": 576.6973552704327,
            "unit": "ns"
          },
          {
            "name": "6.10/outcome_label",
            "value": 7.656398032354092,
            "unit": "ns"
          },
          {
            "name": "6.10/stall_observe",
            "value": 117.69924920817738,
            "unit": "ns"
          },
          {
            "name": "6.10/steer_append",
            "value": 45.58167701902276,
            "unit": "ns"
          },
          {
            "name": "6.10/stress_repair",
            "value": 31111.449074074073,
            "unit": "ns"
          },
          {
            "name": "6.10/truncate_output",
            "value": 3374.9431234132144,
            "unit": "ns"
          },
          {
            "name": "6.10/validate_repair",
            "value": 209.2401215292536,
            "unit": "ns"
          },
          {
            "name": "6.2/cancel_select",
            "value": 91.01157363552582,
            "unit": "ns"
          },
          {
            "name": "6.2/gate_check",
            "value": 5.662268781769332,
            "unit": "ns"
          },
          {
            "name": "6.2/live_tool_body",
            "value": 1453.6576355169943,
            "unit": "ns"
          },
          {
            "name": "6.2/loop_8_reads",
            "value": 2162507.100980392,
            "unit": "ns"
          },
          {
            "name": "6.2/loop_iteration",
            "value": 416.40979545725014,
            "unit": "ns"
          },
          {
            "name": "6.3/cancel_select",
            "value": 92.34225617142971,
            "unit": "ns"
          },
          {
            "name": "6.3/gate_check",
            "value": 5.671251576225673,
            "unit": "ns"
          },
          {
            "name": "6.3/live_tool_body",
            "value": 1506.5397549724278,
            "unit": "ns"
          },
          {
            "name": "6.3/loop_8_reads",
            "value": 2178759.0660130717,
            "unit": "ns"
          },
          {
            "name": "6.3/loop_iteration",
            "value": 421.68300629257055,
            "unit": "ns"
          },
          {
            "name": "6.3/validate_repair",
            "value": 205.19497043423308,
            "unit": "ns"
          },
          {
            "name": "6.4/cancel_select",
            "value": 90.35439457426376,
            "unit": "ns"
          },
          {
            "name": "6.4/gate_check",
            "value": 5.666559011736136,
            "unit": "ns"
          },
          {
            "name": "6.4/live_tool_body",
            "value": 1534.5600311896894,
            "unit": "ns"
          },
          {
            "name": "6.4/loop_8_reads",
            "value": 2187956.5764705883,
            "unit": "ns"
          },
          {
            "name": "6.4/loop_iteration",
            "value": 463.1737680609852,
            "unit": "ns"
          },
          {
            "name": "6.4/stall_observe",
            "value": 117.50333459172077,
            "unit": "ns"
          },
          {
            "name": "6.4/validate_repair",
            "value": 204.67211426589068,
            "unit": "ns"
          },
          {
            "name": "6.5/budget_notice",
            "value": 63.21445960655369,
            "unit": "ns"
          },
          {
            "name": "6.5/cancel_select",
            "value": 89.2329952952461,
            "unit": "ns"
          },
          {
            "name": "6.5/gate_check",
            "value": 5.680548414260079,
            "unit": "ns"
          },
          {
            "name": "6.5/live_tool_body",
            "value": 1486.3155091524154,
            "unit": "ns"
          },
          {
            "name": "6.5/loop_8_reads",
            "value": 2193093.782037815,
            "unit": "ns"
          },
          {
            "name": "6.5/loop_iteration",
            "value": 459.6311335772433,
            "unit": "ns"
          },
          {
            "name": "6.5/stall_observe",
            "value": 115.07350282964619,
            "unit": "ns"
          },
          {
            "name": "6.5/validate_repair",
            "value": 193.4148609222071,
            "unit": "ns"
          },
          {
            "name": "6.6/budget_notice",
            "value": 65.1071978339111,
            "unit": "ns"
          },
          {
            "name": "6.6/cancel_select",
            "value": 93.11726570963006,
            "unit": "ns"
          },
          {
            "name": "6.6/gate_check",
            "value": 5.674516473934021,
            "unit": "ns"
          },
          {
            "name": "6.6/live_tool_body",
            "value": 1511.0396989069911,
            "unit": "ns"
          },
          {
            "name": "6.6/loop_8_reads",
            "value": 2186743.7970588235,
            "unit": "ns"
          },
          {
            "name": "6.6/loop_iteration",
            "value": 458.82567832560204,
            "unit": "ns"
          },
          {
            "name": "6.6/stall_observe",
            "value": 116.45994196633711,
            "unit": "ns"
          },
          {
            "name": "6.6/truncate_output",
            "value": 3496.107144004369,
            "unit": "ns"
          },
          {
            "name": "6.6/validate_repair",
            "value": 204.80000458652478,
            "unit": "ns"
          },
          {
            "name": "6.7/budget_notice",
            "value": 63.961580743984086,
            "unit": "ns"
          },
          {
            "name": "6.7/cancel_select",
            "value": 90.13962604064187,
            "unit": "ns"
          },
          {
            "name": "6.7/gate_check",
            "value": 5.734791458528264,
            "unit": "ns"
          },
          {
            "name": "6.7/live_tool_body",
            "value": 1385.652147437138,
            "unit": "ns"
          },
          {
            "name": "6.7/loop_8_reads",
            "value": 2182843.1220588237,
            "unit": "ns"
          },
          {
            "name": "6.7/loop_iteration",
            "value": 463.2201998923592,
            "unit": "ns"
          },
          {
            "name": "6.7/stall_observe",
            "value": 112.75228486893815,
            "unit": "ns"
          },
          {
            "name": "6.7/steer_append",
            "value": 45.620235919132384,
            "unit": "ns"
          },
          {
            "name": "6.7/truncate_output",
            "value": 3475.1197368001785,
            "unit": "ns"
          },
          {
            "name": "6.7/validate_repair",
            "value": 205.18250850989733,
            "unit": "ns"
          },
          {
            "name": "6.8/budget_notice",
            "value": 65.13880914665982,
            "unit": "ns"
          },
          {
            "name": "6.8/cancel_select",
            "value": 91.62523990328343,
            "unit": "ns"
          },
          {
            "name": "6.8/canonical_args",
            "value": 851.436663964726,
            "unit": "ns"
          },
          {
            "name": "6.8/gate_check",
            "value": 5.840861704599348,
            "unit": "ns"
          },
          {
            "name": "6.8/live_tool_body",
            "value": 1515.8671987297344,
            "unit": "ns"
          },
          {
            "name": "6.8/loop_8_reads",
            "value": 2187845.163398693,
            "unit": "ns"
          },
          {
            "name": "6.8/loop_iteration",
            "value": 532.3409542289066,
            "unit": "ns"
          },
          {
            "name": "6.8/stall_observe",
            "value": 117.37415776834706,
            "unit": "ns"
          },
          {
            "name": "6.8/steer_append",
            "value": 45.61594854552894,
            "unit": "ns"
          },
          {
            "name": "6.8/truncate_output",
            "value": 3401.2741535089094,
            "unit": "ns"
          },
          {
            "name": "6.8/validate_repair",
            "value": 206.4488855235501,
            "unit": "ns"
          },
          {
            "name": "6.9/budget_notice",
            "value": 64.55193958840084,
            "unit": "ns"
          },
          {
            "name": "6.9/cancel_select",
            "value": 91.30795460008189,
            "unit": "ns"
          },
          {
            "name": "6.9/canonical_args",
            "value": 871.7816941199004,
            "unit": "ns"
          },
          {
            "name": "6.9/gate_check",
            "value": 5.670140355128029,
            "unit": "ns"
          },
          {
            "name": "6.9/live_tool_body",
            "value": 1557.1402427142425,
            "unit": "ns"
          },
          {
            "name": "6.9/loop_8_reads",
            "value": 2193769.3272058824,
            "unit": "ns"
          },
          {
            "name": "6.9/loop_iteration",
            "value": 576.5018943264529,
            "unit": "ns"
          },
          {
            "name": "6.9/outcome_label",
            "value": 7.661401767715708,
            "unit": "ns"
          },
          {
            "name": "6.9/stall_observe",
            "value": 115.39405763495361,
            "unit": "ns"
          },
          {
            "name": "6.9/steer_append",
            "value": 45.93748297019911,
            "unit": "ns"
          },
          {
            "name": "6.9/truncate_output",
            "value": 3385.0139977298877,
            "unit": "ns"
          },
          {
            "name": "6.9/validate_repair",
            "value": 205.41500762550027,
            "unit": "ns"
          },
          {
            "name": "gw5/echo_through_gate",
            "value": 37.468800107222876,
            "unit": "ns"
          },
          {
            "name": "gw5/eight_concurrent_sleeps",
            "value": 2133424.6990740737,
            "unit": "ns"
          },
          {
            "name": "gw7/catalog_arc_swap",
            "value": 1964.8103555497546,
            "unit": "ns"
          },
          {
            "name": "gw7/catalog_reload_10k",
            "value": 24874136.223214284,
            "unit": "ns"
          },
          {
            "name": "gw7/catalog_search_10k",
            "value": 188118.53287923418,
            "unit": "ns"
          },
          {
            "name": "gw8/dynamic_rename_task",
            "value": 4496605.429166667,
            "unit": "ns"
          },
          {
            "name": "gw8/every_tool_rename_task",
            "value": 6423190.666666667,
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
          "id": "c7cb5e4aae9d5f6aabcda5be955a26a631781ce0",
          "message": "Write chapters 11 through 20",
          "timestamp": "2026-10-15T04:44:13Z",
          "url": "https://github.com/agnathan/rust-agent-harness/pull/5/commits/c7cb5e4aae9d5f6aabcda5be955a26a631781ce0"
        },
        "date": 1792187533520,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "1.1/serde_50_message_request",
            "value": 7520.555785577187,
            "unit": "ns"
          },
          {
            "name": "1.2/mock_100_concurrent",
            "value": 87607.84293363933,
            "unit": "ns"
          },
          {
            "name": "1.2/mock_turn_overhead",
            "value": 215.25445814024607,
            "unit": "ns"
          },
          {
            "name": "1.3/build_request_and_parse_response",
            "value": 1838.8100553257323,
            "unit": "ns"
          },
          {
            "name": "1.4/parse_response_edges",
            "value": 698.5565484757353,
            "unit": "ns"
          },
          {
            "name": "10/scripted_turn",
            "value": 5.980664091958326,
            "unit": "ns"
          },
          {
            "name": "10/turn_transition",
            "value": 0.5702325208316861,
            "unit": "ns"
          },
          {
            "name": "11/skip_decision",
            "value": 172.78615477086885,
            "unit": "ns"
          },
          {
            "name": "12/api_request_hook",
            "value": 1.6760039028686358,
            "unit": "ns"
          },
          {
            "name": "12/empty_pre_post",
            "value": 7.918150643399159,
            "unit": "ns"
          },
          {
            "name": "12/memory_enqueue",
            "value": 35.50710342699169,
            "unit": "ns"
          },
          {
            "name": "13/checkpoint_1000",
            "value": 63092.72710905349,
            "unit": "ns"
          },
          {
            "name": "13/rewind_1000",
            "value": 64352.01049712814,
            "unit": "ns"
          },
          {
            "name": "13/warm_accept",
            "value": 23726.596419975933,
            "unit": "ns"
          },
          {
            "name": "14/empty_budget",
            "value": 0.6453757960597022,
            "unit": "ns"
          },
          {
            "name": "14/repetition_guard",
            "value": 1602.007365125453,
            "unit": "ns"
          },
          {
            "name": "15/open_breaker",
            "value": 18.97356861910371,
            "unit": "ns"
          },
          {
            "name": "15/permit_release",
            "value": 70.36885352171322,
            "unit": "ns"
          },
          {
            "name": "16/choose_route",
            "value": 3.173722223131323,
            "unit": "ns"
          },
          {
            "name": "16/rotate_key",
            "value": 4.085251980018747,
            "unit": "ns"
          },
          {
            "name": "17/buffer_join",
            "value": 444.4882186655561,
            "unit": "ns"
          },
          {
            "name": "17/fold_deltas",
            "value": 1649.9840988385438,
            "unit": "ns"
          },
          {
            "name": "2/chat_turn_overhead",
            "value": 522.1556856831464,
            "unit": "ns"
          },
          {
            "name": "2/inbox_submit",
            "value": 28.946064007771124,
            "unit": "ns"
          },
          {
            "name": "3.1/registry_lookup",
            "value": 13.167610540806635,
            "unit": "ns"
          },
          {
            "name": "3.2/available_tools_50",
            "value": 20259.615563589174,
            "unit": "ns"
          },
          {
            "name": "3.3/dispatch_8_reads_concurrent",
            "value": 2133548.114197531,
            "unit": "ns"
          },
          {
            "name": "3.3/dispatch_8_reads_sequential",
            "value": 16808049.394444443,
            "unit": "ns"
          },
          {
            "name": "4.1/stdio_echo_round_trip",
            "value": 121780.22009803922,
            "unit": "ns"
          },
          {
            "name": "4.2/http_echo_round_trip",
            "value": 301335.7226775956,
            "unit": "ns"
          },
          {
            "name": "4.3/mcp_50_concurrent",
            "value": 3178627.1083333334,
            "unit": "ns"
          },
          {
            "name": "4.3/mcp_50_sequential",
            "value": 6161038.647959184,
            "unit": "ns"
          },
          {
            "name": "4.4/mcp_list_cold",
            "value": 135188.56623931625,
            "unit": "ns"
          },
          {
            "name": "4.4/mcp_list_hit",
            "value": 158.53695240342628,
            "unit": "ns"
          },
          {
            "name": "5.1/schema_tokens_1000",
            "value": 875675.2554131055,
            "unit": "ns"
          },
          {
            "name": "5.1/schema_tokens_10000",
            "value": 18422873.016666666,
            "unit": "ns"
          },
          {
            "name": "5.2/index_build_10000",
            "value": 13279457.08125,
            "unit": "ns"
          },
          {
            "name": "5.2/index_search_10000",
            "value": 138557.3074712644,
            "unit": "ns"
          },
          {
            "name": "5.3/full_schema_tokens_10000",
            "value": 10206331.705357142,
            "unit": "ns"
          },
          {
            "name": "5.3/lazy_visible_prompt_10000",
            "value": 11676.519444444444,
            "unit": "ns"
          },
          {
            "name": "5.4/routed_search_10000",
            "value": 16508.37444831591,
            "unit": "ns"
          },
          {
            "name": "5.4/shrink_schemas_10000",
            "value": 30067523.633333333,
            "unit": "ns"
          },
          {
            "name": "5.5/agent_filter_10000",
            "value": 50901.6667140825,
            "unit": "ns"
          },
          {
            "name": "5.5/cache_cold_10000",
            "value": 11040750.458333332,
            "unit": "ns"
          },
          {
            "name": "5.5/cache_hit_10000",
            "value": 16.593495637241148,
            "unit": "ns"
          },
          {
            "name": "6.1/gate_check",
            "value": 2.161388275785935,
            "unit": "ns"
          },
          {
            "name": "6.1/live_tool_body",
            "value": 1330.5644128136303,
            "unit": "ns"
          },
          {
            "name": "6.1/loop_8_reads",
            "value": 2136486.929232804,
            "unit": "ns"
          },
          {
            "name": "6.1/loop_iteration",
            "value": 217.78909251834585,
            "unit": "ns"
          },
          {
            "name": "6.10/budget_notice",
            "value": 46.60042428085548,
            "unit": "ns"
          },
          {
            "name": "6.10/cancel_select",
            "value": 148.27982341585022,
            "unit": "ns"
          },
          {
            "name": "6.10/canonical_args",
            "value": 747.3011530137106,
            "unit": "ns"
          },
          {
            "name": "6.10/gate_check",
            "value": 10.820306815302182,
            "unit": "ns"
          },
          {
            "name": "6.10/live_tool_body",
            "value": 1349.3537430506544,
            "unit": "ns"
          },
          {
            "name": "6.10/loop_8_reads",
            "value": 2155186.568627451,
            "unit": "ns"
          },
          {
            "name": "6.10/loop_iteration",
            "value": 586.212245291199,
            "unit": "ns"
          },
          {
            "name": "6.10/outcome_label",
            "value": 5.520733597125529,
            "unit": "ns"
          },
          {
            "name": "6.10/stall_observe",
            "value": 87.84394414705206,
            "unit": "ns"
          },
          {
            "name": "6.10/steer_append",
            "value": 47.567961425669886,
            "unit": "ns"
          },
          {
            "name": "6.10/stress_repair",
            "value": 47070.670387779086,
            "unit": "ns"
          },
          {
            "name": "6.10/truncate_output",
            "value": 2799.170317239961,
            "unit": "ns"
          },
          {
            "name": "6.10/validate_repair",
            "value": 166.67797664527887,
            "unit": "ns"
          },
          {
            "name": "6.2/cancel_select",
            "value": 144.02059516394593,
            "unit": "ns"
          },
          {
            "name": "6.2/gate_check",
            "value": 10.803398900890357,
            "unit": "ns"
          },
          {
            "name": "6.2/live_tool_body",
            "value": 1341.7731746090888,
            "unit": "ns"
          },
          {
            "name": "6.2/loop_8_reads",
            "value": 2137318.3279411765,
            "unit": "ns"
          },
          {
            "name": "6.2/loop_iteration",
            "value": 472.96221244516664,
            "unit": "ns"
          },
          {
            "name": "6.3/cancel_select",
            "value": 147.71789331640667,
            "unit": "ns"
          },
          {
            "name": "6.3/gate_check",
            "value": 11.697405833974333,
            "unit": "ns"
          },
          {
            "name": "6.3/live_tool_body",
            "value": 1420.539438741131,
            "unit": "ns"
          },
          {
            "name": "6.3/loop_8_reads",
            "value": 2149261.5970588233,
            "unit": "ns"
          },
          {
            "name": "6.3/loop_iteration",
            "value": 431.9129793016636,
            "unit": "ns"
          },
          {
            "name": "6.3/validate_repair",
            "value": 196.60829814752316,
            "unit": "ns"
          },
          {
            "name": "6.4/cancel_select",
            "value": 141.98117408324407,
            "unit": "ns"
          },
          {
            "name": "6.4/gate_check",
            "value": 10.810132432727912,
            "unit": "ns"
          },
          {
            "name": "6.4/live_tool_body",
            "value": 1460.4608087544198,
            "unit": "ns"
          },
          {
            "name": "6.4/loop_8_reads",
            "value": 2154103.720588235,
            "unit": "ns"
          },
          {
            "name": "6.4/loop_iteration",
            "value": 436.3357137808355,
            "unit": "ns"
          },
          {
            "name": "6.4/stall_observe",
            "value": 86.27633137822737,
            "unit": "ns"
          },
          {
            "name": "6.4/validate_repair",
            "value": 201.30165379235805,
            "unit": "ns"
          },
          {
            "name": "6.5/budget_notice",
            "value": 47.55851623727064,
            "unit": "ns"
          },
          {
            "name": "6.5/cancel_select",
            "value": 143.27706495769235,
            "unit": "ns"
          },
          {
            "name": "6.5/gate_check",
            "value": 10.803709566241135,
            "unit": "ns"
          },
          {
            "name": "6.5/live_tool_body",
            "value": 1340.3517147613761,
            "unit": "ns"
          },
          {
            "name": "6.5/loop_8_reads",
            "value": 2149672.6794117643,
            "unit": "ns"
          },
          {
            "name": "6.5/loop_iteration",
            "value": 459.74621935789753,
            "unit": "ns"
          },
          {
            "name": "6.5/stall_observe",
            "value": 98.07195584386884,
            "unit": "ns"
          },
          {
            "name": "6.5/validate_repair",
            "value": 199.21473938340057,
            "unit": "ns"
          },
          {
            "name": "6.6/budget_notice",
            "value": 50.46026876528243,
            "unit": "ns"
          },
          {
            "name": "6.6/cancel_select",
            "value": 143.2914832443717,
            "unit": "ns"
          },
          {
            "name": "6.6/gate_check",
            "value": 10.809670319890238,
            "unit": "ns"
          },
          {
            "name": "6.6/live_tool_body",
            "value": 1445.6937427176663,
            "unit": "ns"
          },
          {
            "name": "6.6/loop_8_reads",
            "value": 2145977.0036764704,
            "unit": "ns"
          },
          {
            "name": "6.6/loop_iteration",
            "value": 463.955288841473,
            "unit": "ns"
          },
          {
            "name": "6.6/stall_observe",
            "value": 90.55757547206457,
            "unit": "ns"
          },
          {
            "name": "6.6/truncate_output",
            "value": 2884.2640418297087,
            "unit": "ns"
          },
          {
            "name": "6.6/validate_repair",
            "value": 184.96635086175365,
            "unit": "ns"
          },
          {
            "name": "6.7/budget_notice",
            "value": 45.62981649036277,
            "unit": "ns"
          },
          {
            "name": "6.7/cancel_select",
            "value": 143.61226233320284,
            "unit": "ns"
          },
          {
            "name": "6.7/gate_check",
            "value": 12.355648450623434,
            "unit": "ns"
          },
          {
            "name": "6.7/live_tool_body",
            "value": 1388.0523087189754,
            "unit": "ns"
          },
          {
            "name": "6.7/loop_8_reads",
            "value": 2155628.6288515404,
            "unit": "ns"
          },
          {
            "name": "6.7/loop_iteration",
            "value": 470.9853624535316,
            "unit": "ns"
          },
          {
            "name": "6.7/stall_observe",
            "value": 86.44155954290636,
            "unit": "ns"
          },
          {
            "name": "6.7/steer_append",
            "value": 42.29052483868896,
            "unit": "ns"
          },
          {
            "name": "6.7/truncate_output",
            "value": 2957.0901462225834,
            "unit": "ns"
          },
          {
            "name": "6.7/validate_repair",
            "value": 185.01399983330697,
            "unit": "ns"
          },
          {
            "name": "6.8/budget_notice",
            "value": 46.49938858874302,
            "unit": "ns"
          },
          {
            "name": "6.8/cancel_select",
            "value": 143.52682092501956,
            "unit": "ns"
          },
          {
            "name": "6.8/canonical_args",
            "value": 740.9190914182955,
            "unit": "ns"
          },
          {
            "name": "6.8/gate_check",
            "value": 10.810785710019548,
            "unit": "ns"
          },
          {
            "name": "6.8/live_tool_body",
            "value": 1368.6097674506973,
            "unit": "ns"
          },
          {
            "name": "6.8/loop_8_reads",
            "value": 2163485.448529412,
            "unit": "ns"
          },
          {
            "name": "6.8/loop_iteration",
            "value": 558.7785438465371,
            "unit": "ns"
          },
          {
            "name": "6.8/stall_observe",
            "value": 86.31067369927881,
            "unit": "ns"
          },
          {
            "name": "6.8/steer_append",
            "value": 41.83662394663932,
            "unit": "ns"
          },
          {
            "name": "6.8/truncate_output",
            "value": 2958.489838539125,
            "unit": "ns"
          },
          {
            "name": "6.8/validate_repair",
            "value": 199.55757396131085,
            "unit": "ns"
          },
          {
            "name": "6.9/budget_notice",
            "value": 48.23199107627167,
            "unit": "ns"
          },
          {
            "name": "6.9/cancel_select",
            "value": 142.31631961097008,
            "unit": "ns"
          },
          {
            "name": "6.9/canonical_args",
            "value": 730.3925973729145,
            "unit": "ns"
          },
          {
            "name": "6.9/gate_check",
            "value": 10.804572717028364,
            "unit": "ns"
          },
          {
            "name": "6.9/live_tool_body",
            "value": 1367.7916075943226,
            "unit": "ns"
          },
          {
            "name": "6.9/loop_8_reads",
            "value": 2151096.8029411766,
            "unit": "ns"
          },
          {
            "name": "6.9/loop_iteration",
            "value": 585.5146292388181,
            "unit": "ns"
          },
          {
            "name": "6.9/outcome_label",
            "value": 6.4420278710182854,
            "unit": "ns"
          },
          {
            "name": "6.9/stall_observe",
            "value": 96.95429958774102,
            "unit": "ns"
          },
          {
            "name": "6.9/steer_append",
            "value": 43.12601362817229,
            "unit": "ns"
          },
          {
            "name": "6.9/truncate_output",
            "value": 2839.8128564434,
            "unit": "ns"
          },
          {
            "name": "6.9/validate_repair",
            "value": 249.87974949882653,
            "unit": "ns"
          },
          {
            "name": "gw5/echo_through_gate",
            "value": 71.60494906121241,
            "unit": "ns"
          },
          {
            "name": "gw5/eight_concurrent_sleeps",
            "value": 2097825.739197531,
            "unit": "ns"
          },
          {
            "name": "gw7/catalog_arc_swap",
            "value": 1678.2722397862335,
            "unit": "ns"
          },
          {
            "name": "gw7/catalog_reload_10k",
            "value": 20505167.475,
            "unit": "ns"
          },
          {
            "name": "gw7/catalog_search_10k",
            "value": 134637.6175925926,
            "unit": "ns"
          },
          {
            "name": "gw8/dynamic_rename_task",
            "value": 3800295.15,
            "unit": "ns"
          },
          {
            "name": "gw8/every_tool_rename_task",
            "value": 5891508.887755102,
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
          "id": "8574054cf02b4a4cacbcc2f16f17bf01d98a7e09",
          "message": "Write chapters 11 through 20",
          "timestamp": "2026-10-15T04:44:13Z",
          "url": "https://github.com/agnathan/rust-agent-harness/pull/5/commits/8574054cf02b4a4cacbcc2f16f17bf01d98a7e09"
        },
        "date": 1792188963965,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "1.1/serde_50_message_request",
            "value": 5883.375335507879,
            "unit": "ns"
          },
          {
            "name": "1.2/mock_100_concurrent",
            "value": 59557.42277651801,
            "unit": "ns"
          },
          {
            "name": "1.2/mock_turn_overhead",
            "value": 180.4324395400534,
            "unit": "ns"
          },
          {
            "name": "1.3/build_request_and_parse_response",
            "value": 1459.780566661227,
            "unit": "ns"
          },
          {
            "name": "1.4/parse_response_edges",
            "value": 634.5984806364193,
            "unit": "ns"
          },
          {
            "name": "10/scripted_turn",
            "value": 3.8841757430373747,
            "unit": "ns"
          },
          {
            "name": "10/turn_transition",
            "value": 0.6877901330870544,
            "unit": "ns"
          },
          {
            "name": "11/skip_decision",
            "value": 156.35306657798668,
            "unit": "ns"
          },
          {
            "name": "12/api_request_hook",
            "value": 1.39467448076465,
            "unit": "ns"
          },
          {
            "name": "12/empty_pre_post",
            "value": 7.008904242196288,
            "unit": "ns"
          },
          {
            "name": "12/memory_enqueue",
            "value": 34.016076745621405,
            "unit": "ns"
          },
          {
            "name": "13/checkpoint_1000",
            "value": 43666.71165865385,
            "unit": "ns"
          },
          {
            "name": "13/rewind_1000",
            "value": 64760.365008880995,
            "unit": "ns"
          },
          {
            "name": "13/warm_accept",
            "value": 17245.74849593496,
            "unit": "ns"
          },
          {
            "name": "14/empty_budget",
            "value": 0.5758861680760026,
            "unit": "ns"
          },
          {
            "name": "14/repetition_guard",
            "value": 1421.4307281922602,
            "unit": "ns"
          },
          {
            "name": "15/open_breaker",
            "value": 18.8387580292005,
            "unit": "ns"
          },
          {
            "name": "15/permit_release",
            "value": 75.29260057331204,
            "unit": "ns"
          },
          {
            "name": "16/choose_route",
            "value": 2.645138319221053,
            "unit": "ns"
          },
          {
            "name": "16/rotate_key",
            "value": 2.7165513785893847,
            "unit": "ns"
          },
          {
            "name": "17/buffer_join",
            "value": 359.46125822743204,
            "unit": "ns"
          },
          {
            "name": "17/fold_deltas",
            "value": 1390.8134636058012,
            "unit": "ns"
          },
          {
            "name": "18/wal_append",
            "value": 118857.10329198473,
            "unit": "ns"
          },
          {
            "name": "2/chat_turn_overhead",
            "value": 595.3411633299401,
            "unit": "ns"
          },
          {
            "name": "2/inbox_submit",
            "value": 30.70089137959035,
            "unit": "ns"
          },
          {
            "name": "3.1/registry_lookup",
            "value": 11.811041587627917,
            "unit": "ns"
          },
          {
            "name": "3.2/available_tools_50",
            "value": 18099.545183714003,
            "unit": "ns"
          },
          {
            "name": "3.3/dispatch_8_reads_concurrent",
            "value": 2126494.506944444,
            "unit": "ns"
          },
          {
            "name": "3.3/dispatch_8_reads_sequential",
            "value": 16721287.097222222,
            "unit": "ns"
          },
          {
            "name": "4.1/stdio_echo_round_trip",
            "value": 87284.0510391198,
            "unit": "ns"
          },
          {
            "name": "4.2/http_echo_round_trip",
            "value": 237884.92857142858,
            "unit": "ns"
          },
          {
            "name": "4.3/mcp_50_concurrent",
            "value": 2395851.991319444,
            "unit": "ns"
          },
          {
            "name": "4.3/mcp_50_sequential",
            "value": 4556259.419270834,
            "unit": "ns"
          },
          {
            "name": "4.4/mcp_list_cold",
            "value": 98667.80057932265,
            "unit": "ns"
          },
          {
            "name": "4.4/mcp_list_hit",
            "value": 199.469676324824,
            "unit": "ns"
          },
          {
            "name": "5.1/schema_tokens_1000",
            "value": 713476.6769607842,
            "unit": "ns"
          },
          {
            "name": "5.1/schema_tokens_10000",
            "value": 36974384.7,
            "unit": "ns"
          },
          {
            "name": "5.2/index_build_10000",
            "value": 15775881.535714285,
            "unit": "ns"
          },
          {
            "name": "5.2/index_search_10000",
            "value": 129565.9998733536,
            "unit": "ns"
          },
          {
            "name": "5.3/full_schema_tokens_10000",
            "value": 21111599.94047619,
            "unit": "ns"
          },
          {
            "name": "5.3/lazy_visible_prompt_10000",
            "value": 10778.598918838863,
            "unit": "ns"
          },
          {
            "name": "5.4/routed_search_10000",
            "value": 13152.47081100393,
            "unit": "ns"
          },
          {
            "name": "5.4/shrink_schemas_10000",
            "value": 58891534.25,
            "unit": "ns"
          },
          {
            "name": "5.5/agent_filter_10000",
            "value": 53969.29238393323,
            "unit": "ns"
          },
          {
            "name": "5.5/cache_cold_10000",
            "value": 21719194.6875,
            "unit": "ns"
          },
          {
            "name": "5.5/cache_hit_10000",
            "value": 15.612996466873565,
            "unit": "ns"
          },
          {
            "name": "6.1/gate_check",
            "value": 1.9171556539374577,
            "unit": "ns"
          },
          {
            "name": "6.1/live_tool_body",
            "value": 1306.0663102338003,
            "unit": "ns"
          },
          {
            "name": "6.1/loop_8_reads",
            "value": 2135796.8932980597,
            "unit": "ns"
          },
          {
            "name": "6.1/loop_iteration",
            "value": 223.61361517631707,
            "unit": "ns"
          },
          {
            "name": "6.10/budget_notice",
            "value": 40.78282510078798,
            "unit": "ns"
          },
          {
            "name": "6.10/cancel_select",
            "value": 142.83001731387324,
            "unit": "ns"
          },
          {
            "name": "6.10/canonical_args",
            "value": 663.0960462845749,
            "unit": "ns"
          },
          {
            "name": "6.10/gate_check",
            "value": 11.866595346848507,
            "unit": "ns"
          },
          {
            "name": "6.10/live_tool_body",
            "value": 1260.7836698114315,
            "unit": "ns"
          },
          {
            "name": "6.10/loop_8_reads",
            "value": 2148101.1222222224,
            "unit": "ns"
          },
          {
            "name": "6.10/loop_iteration",
            "value": 587.3807529546182,
            "unit": "ns"
          },
          {
            "name": "6.10/outcome_label",
            "value": 5.2720852809373415,
            "unit": "ns"
          },
          {
            "name": "6.10/stall_observe",
            "value": 68.36002941588077,
            "unit": "ns"
          },
          {
            "name": "6.10/steer_append",
            "value": 41.894098007032404,
            "unit": "ns"
          },
          {
            "name": "6.10/stress_repair",
            "value": 26792.512354651164,
            "unit": "ns"
          },
          {
            "name": "6.10/truncate_output",
            "value": 2212.1124507406794,
            "unit": "ns"
          },
          {
            "name": "6.10/validate_repair",
            "value": 182.5217971856456,
            "unit": "ns"
          },
          {
            "name": "6.2/cancel_select",
            "value": 144.7402765072814,
            "unit": "ns"
          },
          {
            "name": "6.2/gate_check",
            "value": 11.846180297857604,
            "unit": "ns"
          },
          {
            "name": "6.2/live_tool_body",
            "value": 1245.108226420177,
            "unit": "ns"
          },
          {
            "name": "6.2/loop_8_reads",
            "value": 2130982.104166667,
            "unit": "ns"
          },
          {
            "name": "6.2/loop_iteration",
            "value": 420.8971195950429,
            "unit": "ns"
          },
          {
            "name": "6.3/cancel_select",
            "value": 152.97285658043612,
            "unit": "ns"
          },
          {
            "name": "6.3/gate_check",
            "value": 11.930195789692092,
            "unit": "ns"
          },
          {
            "name": "6.3/live_tool_body",
            "value": 1340.480905328312,
            "unit": "ns"
          },
          {
            "name": "6.3/loop_8_reads",
            "value": 2147492.7441176474,
            "unit": "ns"
          },
          {
            "name": "6.3/loop_iteration",
            "value": 437.2627666760374,
            "unit": "ns"
          },
          {
            "name": "6.3/validate_repair",
            "value": 175.63429920748786,
            "unit": "ns"
          },
          {
            "name": "6.4/cancel_select",
            "value": 147.4519702633536,
            "unit": "ns"
          },
          {
            "name": "6.4/gate_check",
            "value": 11.872213147725116,
            "unit": "ns"
          },
          {
            "name": "6.4/live_tool_body",
            "value": 1312.3132610643254,
            "unit": "ns"
          },
          {
            "name": "6.4/loop_8_reads",
            "value": 2153201.84375,
            "unit": "ns"
          },
          {
            "name": "6.4/loop_iteration",
            "value": 433.8036867036316,
            "unit": "ns"
          },
          {
            "name": "6.4/stall_observe",
            "value": 69.6536486483306,
            "unit": "ns"
          },
          {
            "name": "6.4/validate_repair",
            "value": 183.54067473042275,
            "unit": "ns"
          },
          {
            "name": "6.5/budget_notice",
            "value": 42.996273106561674,
            "unit": "ns"
          },
          {
            "name": "6.5/cancel_select",
            "value": 174.92913749221884,
            "unit": "ns"
          },
          {
            "name": "6.5/gate_check",
            "value": 11.859205746496631,
            "unit": "ns"
          },
          {
            "name": "6.5/live_tool_body",
            "value": 1306.1200405811805,
            "unit": "ns"
          },
          {
            "name": "6.5/loop_8_reads",
            "value": 2149308.288888889,
            "unit": "ns"
          },
          {
            "name": "6.5/loop_iteration",
            "value": 457.34342108415314,
            "unit": "ns"
          },
          {
            "name": "6.5/stall_observe",
            "value": 72.30403722436026,
            "unit": "ns"
          },
          {
            "name": "6.5/validate_repair",
            "value": 178.97419460306685,
            "unit": "ns"
          },
          {
            "name": "6.6/budget_notice",
            "value": 41.49668050511491,
            "unit": "ns"
          },
          {
            "name": "6.6/cancel_select",
            "value": 141.2951713586695,
            "unit": "ns"
          },
          {
            "name": "6.6/gate_check",
            "value": 11.773155648457081,
            "unit": "ns"
          },
          {
            "name": "6.6/live_tool_body",
            "value": 1227.4474089661012,
            "unit": "ns"
          },
          {
            "name": "6.6/loop_8_reads",
            "value": 2139401.987962963,
            "unit": "ns"
          },
          {
            "name": "6.6/loop_iteration",
            "value": 448.9686862480279,
            "unit": "ns"
          },
          {
            "name": "6.6/stall_observe",
            "value": 67.8794160534,
            "unit": "ns"
          },
          {
            "name": "6.6/truncate_output",
            "value": 2253.0789231786166,
            "unit": "ns"
          },
          {
            "name": "6.6/validate_repair",
            "value": 174.86872286790629,
            "unit": "ns"
          },
          {
            "name": "6.7/budget_notice",
            "value": 43.11547787782131,
            "unit": "ns"
          },
          {
            "name": "6.7/cancel_select",
            "value": 151.80986740177136,
            "unit": "ns"
          },
          {
            "name": "6.7/gate_check",
            "value": 11.894409050671861,
            "unit": "ns"
          },
          {
            "name": "6.7/live_tool_body",
            "value": 1174.6138839662449,
            "unit": "ns"
          },
          {
            "name": "6.7/loop_8_reads",
            "value": 2145743.0300925924,
            "unit": "ns"
          },
          {
            "name": "6.7/loop_iteration",
            "value": 478.6434221557978,
            "unit": "ns"
          },
          {
            "name": "6.7/stall_observe",
            "value": 68.86049307310725,
            "unit": "ns"
          },
          {
            "name": "6.7/steer_append",
            "value": 41.154848486915554,
            "unit": "ns"
          },
          {
            "name": "6.7/truncate_output",
            "value": 2300.128728377219,
            "unit": "ns"
          },
          {
            "name": "6.7/validate_repair",
            "value": 177.9904169206468,
            "unit": "ns"
          },
          {
            "name": "6.8/budget_notice",
            "value": 44.1227024810259,
            "unit": "ns"
          },
          {
            "name": "6.8/cancel_select",
            "value": 144.4791179731666,
            "unit": "ns"
          },
          {
            "name": "6.8/canonical_args",
            "value": 666.5880797097677,
            "unit": "ns"
          },
          {
            "name": "6.8/gate_check",
            "value": 11.730788968860196,
            "unit": "ns"
          },
          {
            "name": "6.8/live_tool_body",
            "value": 1274.7985156928194,
            "unit": "ns"
          },
          {
            "name": "6.8/loop_8_reads",
            "value": 2147836.363271605,
            "unit": "ns"
          },
          {
            "name": "6.8/loop_iteration",
            "value": 591.7547547244154,
            "unit": "ns"
          },
          {
            "name": "6.8/stall_observe",
            "value": 70.73500551614487,
            "unit": "ns"
          },
          {
            "name": "6.8/steer_append",
            "value": 41.19827671330253,
            "unit": "ns"
          },
          {
            "name": "6.8/truncate_output",
            "value": 2881.1619153497977,
            "unit": "ns"
          },
          {
            "name": "6.8/validate_repair",
            "value": 186.26400006165164,
            "unit": "ns"
          },
          {
            "name": "6.9/budget_notice",
            "value": 42.17818483297255,
            "unit": "ns"
          },
          {
            "name": "6.9/cancel_select",
            "value": 148.56557410057314,
            "unit": "ns"
          },
          {
            "name": "6.9/canonical_args",
            "value": 679.9193555316025,
            "unit": "ns"
          },
          {
            "name": "6.9/gate_check",
            "value": 11.91924707734119,
            "unit": "ns"
          },
          {
            "name": "6.9/live_tool_body",
            "value": 1255.078198819775,
            "unit": "ns"
          },
          {
            "name": "6.9/loop_8_reads",
            "value": 2140593.8272058824,
            "unit": "ns"
          },
          {
            "name": "6.9/loop_iteration",
            "value": 619.2997722597707,
            "unit": "ns"
          },
          {
            "name": "6.9/outcome_label",
            "value": 5.335390790177906,
            "unit": "ns"
          },
          {
            "name": "6.9/stall_observe",
            "value": 68.52265975821308,
            "unit": "ns"
          },
          {
            "name": "6.9/steer_append",
            "value": 41.4238048409959,
            "unit": "ns"
          },
          {
            "name": "6.9/truncate_output",
            "value": 2216.0358280254777,
            "unit": "ns"
          },
          {
            "name": "6.9/validate_repair",
            "value": 182.67093209867178,
            "unit": "ns"
          },
          {
            "name": "gw5/echo_through_gate",
            "value": 72.4020840545266,
            "unit": "ns"
          },
          {
            "name": "gw5/eight_concurrent_sleeps",
            "value": 2111711.7555555557,
            "unit": "ns"
          },
          {
            "name": "gw7/catalog_arc_swap",
            "value": 1554.7632906740455,
            "unit": "ns"
          },
          {
            "name": "gw7/catalog_reload_10k",
            "value": 19117790.208333332,
            "unit": "ns"
          },
          {
            "name": "gw7/catalog_search_10k",
            "value": 126085.78426566705,
            "unit": "ns"
          },
          {
            "name": "gw8/dynamic_rename_task",
            "value": 3491886.1409090906,
            "unit": "ns"
          },
          {
            "name": "gw8/every_tool_rename_task",
            "value": 5015654.491071429,
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
          "id": "83d4b0037b4995a54595e4572a3f9bb83a8f7f1c",
          "message": "Write chapters 11 through 20",
          "timestamp": "2026-10-15T04:44:13Z",
          "url": "https://github.com/agnathan/rust-agent-harness/pull/5/commits/83d4b0037b4995a54595e4572a3f9bb83a8f7f1c"
        },
        "date": 1792189827522,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "1.1/serde_50_message_request",
            "value": 8292.093510205648,
            "unit": "ns"
          },
          {
            "name": "1.2/mock_100_concurrent",
            "value": 63244.10405797101,
            "unit": "ns"
          },
          {
            "name": "1.2/mock_turn_overhead",
            "value": 208.47291332415244,
            "unit": "ns"
          },
          {
            "name": "1.3/build_request_and_parse_response",
            "value": 2181.8432123218827,
            "unit": "ns"
          },
          {
            "name": "1.4/parse_response_edges",
            "value": 918.178148534467,
            "unit": "ns"
          },
          {
            "name": "10/scripted_turn",
            "value": 5.41417279902544,
            "unit": "ns"
          },
          {
            "name": "10/turn_transition",
            "value": 0.9237975010846706,
            "unit": "ns"
          },
          {
            "name": "11/skip_decision",
            "value": 211.91890436024545,
            "unit": "ns"
          },
          {
            "name": "12/api_request_hook",
            "value": 1.9208137421721805,
            "unit": "ns"
          },
          {
            "name": "12/empty_pre_post",
            "value": 9.825538649317355,
            "unit": "ns"
          },
          {
            "name": "12/memory_enqueue",
            "value": 45.32669832761685,
            "unit": "ns"
          },
          {
            "name": "13/checkpoint_1000",
            "value": 41723.95510262258,
            "unit": "ns"
          },
          {
            "name": "13/rewind_1000",
            "value": 63118.39547826087,
            "unit": "ns"
          },
          {
            "name": "13/warm_accept",
            "value": 18140.99217945417,
            "unit": "ns"
          },
          {
            "name": "14/empty_budget",
            "value": 0.7329876295925015,
            "unit": "ns"
          },
          {
            "name": "14/repetition_guard",
            "value": 1967.0945527398744,
            "unit": "ns"
          },
          {
            "name": "15/open_breaker",
            "value": 19.844368464807708,
            "unit": "ns"
          },
          {
            "name": "15/permit_release",
            "value": 49.02814750765117,
            "unit": "ns"
          },
          {
            "name": "16/choose_route",
            "value": 4.365582893949023,
            "unit": "ns"
          },
          {
            "name": "16/rotate_key",
            "value": 3.0409250938238777,
            "unit": "ns"
          },
          {
            "name": "17/buffer_join",
            "value": 636.8955229043014,
            "unit": "ns"
          },
          {
            "name": "17/fold_deltas",
            "value": 2350.4681393366445,
            "unit": "ns"
          },
          {
            "name": "18/wal_append",
            "value": 103506.34119217082,
            "unit": "ns"
          },
          {
            "name": "19/memory_enqueue",
            "value": 45.37068878052756,
            "unit": "ns"
          },
          {
            "name": "2/chat_turn_overhead",
            "value": 664.5208296206721,
            "unit": "ns"
          },
          {
            "name": "2/inbox_submit",
            "value": 33.4749966436919,
            "unit": "ns"
          },
          {
            "name": "3.1/registry_lookup",
            "value": 17.67627138001223,
            "unit": "ns"
          },
          {
            "name": "3.2/available_tools_50",
            "value": 23962.62211893168,
            "unit": "ns"
          },
          {
            "name": "3.3/dispatch_8_reads_concurrent",
            "value": 2146635.18627451,
            "unit": "ns"
          },
          {
            "name": "3.3/dispatch_8_reads_sequential",
            "value": 16809544.89814815,
            "unit": "ns"
          },
          {
            "name": "4.1/stdio_echo_round_trip",
            "value": 130769.2422939068,
            "unit": "ns"
          },
          {
            "name": "4.2/http_echo_round_trip",
            "value": 309461.6756120527,
            "unit": "ns"
          },
          {
            "name": "4.3/mcp_50_concurrent",
            "value": 3258322.9583333335,
            "unit": "ns"
          },
          {
            "name": "4.3/mcp_50_sequential",
            "value": 6687558.06547619,
            "unit": "ns"
          },
          {
            "name": "4.4/mcp_list_cold",
            "value": 136645.71952830188,
            "unit": "ns"
          },
          {
            "name": "4.4/mcp_list_hit",
            "value": 181.7008633851657,
            "unit": "ns"
          },
          {
            "name": "5.1/schema_tokens_1000",
            "value": 1000961.3555555556,
            "unit": "ns"
          },
          {
            "name": "5.1/schema_tokens_10000",
            "value": 38200496.39583333,
            "unit": "ns"
          },
          {
            "name": "5.2/index_build_10000",
            "value": 17578424.020833336,
            "unit": "ns"
          },
          {
            "name": "5.2/index_search_10000",
            "value": 193596.8005319149,
            "unit": "ns"
          },
          {
            "name": "5.3/full_schema_tokens_10000",
            "value": 24077156.25,
            "unit": "ns"
          },
          {
            "name": "5.3/lazy_visible_prompt_10000",
            "value": 12623.38571346836,
            "unit": "ns"
          },
          {
            "name": "5.4/routed_search_10000",
            "value": 18882.180064680066,
            "unit": "ns"
          },
          {
            "name": "5.4/shrink_schemas_10000",
            "value": 60628030.31944445,
            "unit": "ns"
          },
          {
            "name": "5.5/agent_filter_10000",
            "value": 87968.09574468085,
            "unit": "ns"
          },
          {
            "name": "5.5/cache_cold_10000",
            "value": 24033730.475,
            "unit": "ns"
          },
          {
            "name": "5.5/cache_hit_10000",
            "value": 12.326792012665145,
            "unit": "ns"
          },
          {
            "name": "6.1/gate_check",
            "value": 2.455600717839694,
            "unit": "ns"
          },
          {
            "name": "6.1/live_tool_body",
            "value": 1499.7920217607148,
            "unit": "ns"
          },
          {
            "name": "6.1/loop_8_reads",
            "value": 2158008.660294118,
            "unit": "ns"
          },
          {
            "name": "6.1/loop_iteration",
            "value": 277.60409962803965,
            "unit": "ns"
          },
          {
            "name": "6.10/budget_notice",
            "value": 64.6406215980532,
            "unit": "ns"
          },
          {
            "name": "6.10/cancel_select",
            "value": 93.08002575924979,
            "unit": "ns"
          },
          {
            "name": "6.10/canonical_args",
            "value": 860.1232361920622,
            "unit": "ns"
          },
          {
            "name": "6.10/gate_check",
            "value": 5.674364931345782,
            "unit": "ns"
          },
          {
            "name": "6.10/live_tool_body",
            "value": 1512.3011781079067,
            "unit": "ns"
          },
          {
            "name": "6.10/loop_8_reads",
            "value": 2197132.666666667,
            "unit": "ns"
          },
          {
            "name": "6.10/loop_iteration",
            "value": 574.5259415881634,
            "unit": "ns"
          },
          {
            "name": "6.10/outcome_label",
            "value": 7.657655449586677,
            "unit": "ns"
          },
          {
            "name": "6.10/stall_observe",
            "value": 116.05863192397581,
            "unit": "ns"
          },
          {
            "name": "6.10/steer_append",
            "value": 45.58931779653466,
            "unit": "ns"
          },
          {
            "name": "6.10/stress_repair",
            "value": 30493.86153092639,
            "unit": "ns"
          },
          {
            "name": "6.10/truncate_output",
            "value": 3435.022769345519,
            "unit": "ns"
          },
          {
            "name": "6.10/validate_repair",
            "value": 207.485343106809,
            "unit": "ns"
          },
          {
            "name": "6.2/cancel_select",
            "value": 94.00449290973685,
            "unit": "ns"
          },
          {
            "name": "6.2/gate_check",
            "value": 5.66997474297696,
            "unit": "ns"
          },
          {
            "name": "6.2/live_tool_body",
            "value": 1509.675730265935,
            "unit": "ns"
          },
          {
            "name": "6.2/loop_8_reads",
            "value": 2161582.583006536,
            "unit": "ns"
          },
          {
            "name": "6.2/loop_iteration",
            "value": 410.114860999778,
            "unit": "ns"
          },
          {
            "name": "6.3/cancel_select",
            "value": 91.75337558031698,
            "unit": "ns"
          },
          {
            "name": "6.3/gate_check",
            "value": 5.666112011965968,
            "unit": "ns"
          },
          {
            "name": "6.3/live_tool_body",
            "value": 1541.9361093096986,
            "unit": "ns"
          },
          {
            "name": "6.3/loop_8_reads",
            "value": 2184480.341503268,
            "unit": "ns"
          },
          {
            "name": "6.3/loop_iteration",
            "value": 413.9230130335504,
            "unit": "ns"
          },
          {
            "name": "6.3/validate_repair",
            "value": 204.38627610945105,
            "unit": "ns"
          },
          {
            "name": "6.4/cancel_select",
            "value": 91.2888321104123,
            "unit": "ns"
          },
          {
            "name": "6.4/gate_check",
            "value": 5.666910875371602,
            "unit": "ns"
          },
          {
            "name": "6.4/live_tool_body",
            "value": 1523.7646982346657,
            "unit": "ns"
          },
          {
            "name": "6.4/loop_8_reads",
            "value": 2185836.1588235293,
            "unit": "ns"
          },
          {
            "name": "6.4/loop_iteration",
            "value": 429.4070572065027,
            "unit": "ns"
          },
          {
            "name": "6.4/stall_observe",
            "value": 118.13482202206282,
            "unit": "ns"
          },
          {
            "name": "6.4/validate_repair",
            "value": 205.57253763227885,
            "unit": "ns"
          },
          {
            "name": "6.5/budget_notice",
            "value": 65.81147716052934,
            "unit": "ns"
          },
          {
            "name": "6.5/cancel_select",
            "value": 91.35632703048057,
            "unit": "ns"
          },
          {
            "name": "6.5/gate_check",
            "value": 5.679370924261143,
            "unit": "ns"
          },
          {
            "name": "6.5/live_tool_body",
            "value": 1576.003658630412,
            "unit": "ns"
          },
          {
            "name": "6.5/loop_8_reads",
            "value": 2191909.3643790847,
            "unit": "ns"
          },
          {
            "name": "6.5/loop_iteration",
            "value": 484.54762747933614,
            "unit": "ns"
          },
          {
            "name": "6.5/stall_observe",
            "value": 115.75609269098061,
            "unit": "ns"
          },
          {
            "name": "6.5/validate_repair",
            "value": 207.55108199449455,
            "unit": "ns"
          },
          {
            "name": "6.6/budget_notice",
            "value": 63.078291760993366,
            "unit": "ns"
          },
          {
            "name": "6.6/cancel_select",
            "value": 91.74418677994385,
            "unit": "ns"
          },
          {
            "name": "6.6/gate_check",
            "value": 5.667672566299772,
            "unit": "ns"
          },
          {
            "name": "6.6/live_tool_body",
            "value": 1497.0774275755994,
            "unit": "ns"
          },
          {
            "name": "6.6/loop_8_reads",
            "value": 2173875.3774509802,
            "unit": "ns"
          },
          {
            "name": "6.6/loop_iteration",
            "value": 459.1089350157649,
            "unit": "ns"
          },
          {
            "name": "6.6/stall_observe",
            "value": 115.16038712685088,
            "unit": "ns"
          },
          {
            "name": "6.6/truncate_output",
            "value": 3490.9093100425744,
            "unit": "ns"
          },
          {
            "name": "6.6/validate_repair",
            "value": 192.70124400962757,
            "unit": "ns"
          },
          {
            "name": "6.7/budget_notice",
            "value": 60.1703717826105,
            "unit": "ns"
          },
          {
            "name": "6.7/cancel_select",
            "value": 91.02786601661782,
            "unit": "ns"
          },
          {
            "name": "6.7/gate_check",
            "value": 5.734574450904047,
            "unit": "ns"
          },
          {
            "name": "6.7/live_tool_body",
            "value": 1451.23508564637,
            "unit": "ns"
          },
          {
            "name": "6.7/loop_8_reads",
            "value": 2197027.544526144,
            "unit": "ns"
          },
          {
            "name": "6.7/loop_iteration",
            "value": 454.850395187051,
            "unit": "ns"
          },
          {
            "name": "6.7/stall_observe",
            "value": 114.92800186536203,
            "unit": "ns"
          },
          {
            "name": "6.7/steer_append",
            "value": 45.568332951302054,
            "unit": "ns"
          },
          {
            "name": "6.7/truncate_output",
            "value": 3455.7242270805727,
            "unit": "ns"
          },
          {
            "name": "6.7/validate_repair",
            "value": 204.32939545140042,
            "unit": "ns"
          },
          {
            "name": "6.8/budget_notice",
            "value": 65.316055114386,
            "unit": "ns"
          },
          {
            "name": "6.8/cancel_select",
            "value": 98.2219525398442,
            "unit": "ns"
          },
          {
            "name": "6.8/canonical_args",
            "value": 924.2778178287057,
            "unit": "ns"
          },
          {
            "name": "6.8/gate_check",
            "value": 7.6049282728373235,
            "unit": "ns"
          },
          {
            "name": "6.8/live_tool_body",
            "value": 1706.9129690633056,
            "unit": "ns"
          },
          {
            "name": "6.8/loop_8_reads",
            "value": 2180003.9747899156,
            "unit": "ns"
          },
          {
            "name": "6.8/loop_iteration",
            "value": 583.4503755426617,
            "unit": "ns"
          },
          {
            "name": "6.8/stall_observe",
            "value": 122.39534769786408,
            "unit": "ns"
          },
          {
            "name": "6.8/steer_append",
            "value": 46.0748194189986,
            "unit": "ns"
          },
          {
            "name": "6.8/truncate_output",
            "value": 3491.473054432573,
            "unit": "ns"
          },
          {
            "name": "6.8/validate_repair",
            "value": 225.2670216251978,
            "unit": "ns"
          },
          {
            "name": "6.9/budget_notice",
            "value": 68.71754475439377,
            "unit": "ns"
          },
          {
            "name": "6.9/cancel_select",
            "value": 93.89860391551503,
            "unit": "ns"
          },
          {
            "name": "6.9/canonical_args",
            "value": 887.1707239502905,
            "unit": "ns"
          },
          {
            "name": "6.9/gate_check",
            "value": 5.65959756127292,
            "unit": "ns"
          },
          {
            "name": "6.9/live_tool_body",
            "value": 1542.040429135271,
            "unit": "ns"
          },
          {
            "name": "6.9/loop_8_reads",
            "value": 2190411.5326797385,
            "unit": "ns"
          },
          {
            "name": "6.9/loop_iteration",
            "value": 575.7855800277653,
            "unit": "ns"
          },
          {
            "name": "6.9/outcome_label",
            "value": 7.657004845607151,
            "unit": "ns"
          },
          {
            "name": "6.9/stall_observe",
            "value": 116.17821330639342,
            "unit": "ns"
          },
          {
            "name": "6.9/steer_append",
            "value": 45.8947032068847,
            "unit": "ns"
          },
          {
            "name": "6.9/truncate_output",
            "value": 3414.5329587814103,
            "unit": "ns"
          },
          {
            "name": "6.9/validate_repair",
            "value": 206.2043173994658,
            "unit": "ns"
          },
          {
            "name": "gw5/echo_through_gate",
            "value": 38.23223090021253,
            "unit": "ns"
          },
          {
            "name": "gw5/eight_concurrent_sleeps",
            "value": 2131592.654761905,
            "unit": "ns"
          },
          {
            "name": "gw7/catalog_arc_swap",
            "value": 2002.2303535435096,
            "unit": "ns"
          },
          {
            "name": "gw7/catalog_reload_10k",
            "value": 22853640.6875,
            "unit": "ns"
          },
          {
            "name": "gw7/catalog_search_10k",
            "value": 187741.7143777614,
            "unit": "ns"
          },
          {
            "name": "gw8/dynamic_rename_task",
            "value": 4511792.105555556,
            "unit": "ns"
          },
          {
            "name": "gw8/every_tool_rename_task",
            "value": 6407770.083333334,
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
          "id": "23cd4a25b62b102e0f7a290c672d9173ab8fab2d",
          "message": "Write chapters 11 through 20",
          "timestamp": "2026-10-15T04:44:13Z",
          "url": "https://github.com/agnathan/rust-agent-harness/pull/5/commits/23cd4a25b62b102e0f7a290c672d9173ab8fab2d"
        },
        "date": 1792190670052,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "1.1/serde_50_message_request",
            "value": 10182.760312411649,
            "unit": "ns"
          },
          {
            "name": "1.2/mock_100_concurrent",
            "value": 79018.8582791034,
            "unit": "ns"
          },
          {
            "name": "1.2/mock_turn_overhead",
            "value": 256.5336198383988,
            "unit": "ns"
          },
          {
            "name": "1.3/build_request_and_parse_response",
            "value": 2955.6499798565787,
            "unit": "ns"
          },
          {
            "name": "1.4/parse_response_edges",
            "value": 1101.7606294919835,
            "unit": "ns"
          },
          {
            "name": "10/scripted_turn",
            "value": 8.109362096935364,
            "unit": "ns"
          },
          {
            "name": "10/turn_transition",
            "value": 1.0983640758483562,
            "unit": "ns"
          },
          {
            "name": "11/skip_decision",
            "value": 251.3276851397744,
            "unit": "ns"
          },
          {
            "name": "12/api_request_hook",
            "value": 2.179631666673859,
            "unit": "ns"
          },
          {
            "name": "12/empty_pre_post",
            "value": 10.903267579618138,
            "unit": "ns"
          },
          {
            "name": "12/memory_enqueue",
            "value": 54.12068407868563,
            "unit": "ns"
          },
          {
            "name": "13/checkpoint_1000",
            "value": 50911.801010101015,
            "unit": "ns"
          },
          {
            "name": "13/rewind_1000",
            "value": 81062.64598214286,
            "unit": "ns"
          },
          {
            "name": "13/warm_accept",
            "value": 29543.722933813442,
            "unit": "ns"
          },
          {
            "name": "14/empty_budget",
            "value": 0.9043714829806135,
            "unit": "ns"
          },
          {
            "name": "14/repetition_guard",
            "value": 2573.5596249115356,
            "unit": "ns"
          },
          {
            "name": "15/open_breaker",
            "value": 29.998620629256553,
            "unit": "ns"
          },
          {
            "name": "15/permit_release",
            "value": 60.151801541454894,
            "unit": "ns"
          },
          {
            "name": "16/choose_route",
            "value": 6.228087121312669,
            "unit": "ns"
          },
          {
            "name": "16/rotate_key",
            "value": 3.453464016328299,
            "unit": "ns"
          },
          {
            "name": "17/buffer_join",
            "value": 657.9067406122349,
            "unit": "ns"
          },
          {
            "name": "17/fold_deltas",
            "value": 2806.262363838585,
            "unit": "ns"
          },
          {
            "name": "18/wal_append",
            "value": 145556.04715909093,
            "unit": "ns"
          },
          {
            "name": "19/memory_enqueue",
            "value": 53.04156182131632,
            "unit": "ns"
          },
          {
            "name": "2/chat_turn_overhead",
            "value": 770.8436456931487,
            "unit": "ns"
          },
          {
            "name": "2/inbox_submit",
            "value": 40.52996646934636,
            "unit": "ns"
          },
          {
            "name": "20/discover_10k",
            "value": 60202.43818371944,
            "unit": "ns"
          },
          {
            "name": "3.1/registry_lookup",
            "value": 26.904304443225946,
            "unit": "ns"
          },
          {
            "name": "3.2/available_tools_50",
            "value": 30682.68996627319,
            "unit": "ns"
          },
          {
            "name": "3.3/dispatch_8_reads_concurrent",
            "value": 2139645.7016806724,
            "unit": "ns"
          },
          {
            "name": "3.3/dispatch_8_reads_sequential",
            "value": 16725433.296296297,
            "unit": "ns"
          },
          {
            "name": "4.1/stdio_echo_round_trip",
            "value": 204954.1875780275,
            "unit": "ns"
          },
          {
            "name": "4.2/http_echo_round_trip",
            "value": 416849.77150974027,
            "unit": "ns"
          },
          {
            "name": "4.3/mcp_50_concurrent",
            "value": 5013500.833333334,
            "unit": "ns"
          },
          {
            "name": "4.3/mcp_50_sequential",
            "value": 10219154.32142857,
            "unit": "ns"
          },
          {
            "name": "4.4/mcp_list_cold",
            "value": 215089.48464052286,
            "unit": "ns"
          },
          {
            "name": "4.4/mcp_list_hit",
            "value": 255.4635222056751,
            "unit": "ns"
          },
          {
            "name": "5.1/schema_tokens_1000",
            "value": 1263236.130952381,
            "unit": "ns"
          },
          {
            "name": "5.1/schema_tokens_10000",
            "value": 44598225.442857146,
            "unit": "ns"
          },
          {
            "name": "5.2/index_build_10000",
            "value": 21982773.910714284,
            "unit": "ns"
          },
          {
            "name": "5.2/index_search_10000",
            "value": 339092.35032765404,
            "unit": "ns"
          },
          {
            "name": "5.3/full_schema_tokens_10000",
            "value": 26723873.222222224,
            "unit": "ns"
          },
          {
            "name": "5.3/lazy_visible_prompt_10000",
            "value": 15749.508566637505,
            "unit": "ns"
          },
          {
            "name": "5.4/routed_search_10000",
            "value": 26793.364608978998,
            "unit": "ns"
          },
          {
            "name": "5.4/shrink_schemas_10000",
            "value": 64503376.16666667,
            "unit": "ns"
          },
          {
            "name": "5.5/agent_filter_10000",
            "value": 124674.32275862069,
            "unit": "ns"
          },
          {
            "name": "5.5/cache_cold_10000",
            "value": 24090426.75,
            "unit": "ns"
          },
          {
            "name": "5.5/cache_hit_10000",
            "value": 13.790356404282461,
            "unit": "ns"
          },
          {
            "name": "6.1/gate_check",
            "value": 3.4308666506424355,
            "unit": "ns"
          },
          {
            "name": "6.1/live_tool_body",
            "value": 1836.8139960779354,
            "unit": "ns"
          },
          {
            "name": "6.1/loop_8_reads",
            "value": 2146758.2609243696,
            "unit": "ns"
          },
          {
            "name": "6.1/loop_iteration",
            "value": 338.0887424651923,
            "unit": "ns"
          },
          {
            "name": "6.10/budget_notice",
            "value": 76.29258078629914,
            "unit": "ns"
          },
          {
            "name": "6.10/cancel_select",
            "value": 94.9965373667836,
            "unit": "ns"
          },
          {
            "name": "6.10/canonical_args",
            "value": 1059.9868301290758,
            "unit": "ns"
          },
          {
            "name": "6.10/gate_check",
            "value": 7.473432347451327,
            "unit": "ns"
          },
          {
            "name": "6.10/live_tool_body",
            "value": 1729.9792866679381,
            "unit": "ns"
          },
          {
            "name": "6.10/loop_8_reads",
            "value": 2185829.9522058824,
            "unit": "ns"
          },
          {
            "name": "6.10/loop_iteration",
            "value": 749.3312943610927,
            "unit": "ns"
          },
          {
            "name": "6.10/outcome_label",
            "value": 8.397833491036334,
            "unit": "ns"
          },
          {
            "name": "6.10/stall_observe",
            "value": 138.20254796264004,
            "unit": "ns"
          },
          {
            "name": "6.10/steer_append",
            "value": 59.89668124303013,
            "unit": "ns"
          },
          {
            "name": "6.10/stress_repair",
            "value": 53729.95484913793,
            "unit": "ns"
          },
          {
            "name": "6.10/truncate_output",
            "value": 4674.375073803107,
            "unit": "ns"
          },
          {
            "name": "6.10/validate_repair",
            "value": 280.69517074148604,
            "unit": "ns"
          },
          {
            "name": "6.2/cancel_select",
            "value": 97.05792306618645,
            "unit": "ns"
          },
          {
            "name": "6.2/gate_check",
            "value": 7.476030532554093,
            "unit": "ns"
          },
          {
            "name": "6.2/live_tool_body",
            "value": 1785.6635000979288,
            "unit": "ns"
          },
          {
            "name": "6.2/loop_8_reads",
            "value": 2151808.9826680673,
            "unit": "ns"
          },
          {
            "name": "6.2/loop_iteration",
            "value": 531.3881364176493,
            "unit": "ns"
          },
          {
            "name": "6.3/cancel_select",
            "value": 97.93063858658998,
            "unit": "ns"
          },
          {
            "name": "6.3/gate_check",
            "value": 7.474358151509234,
            "unit": "ns"
          },
          {
            "name": "6.3/live_tool_body",
            "value": 1784.0428942135131,
            "unit": "ns"
          },
          {
            "name": "6.3/loop_8_reads",
            "value": 2172185.617647059,
            "unit": "ns"
          },
          {
            "name": "6.3/loop_iteration",
            "value": 496.90597368792294,
            "unit": "ns"
          },
          {
            "name": "6.3/validate_repair",
            "value": 261.4924140313246,
            "unit": "ns"
          },
          {
            "name": "6.4/cancel_select",
            "value": 96.34840544410106,
            "unit": "ns"
          },
          {
            "name": "6.4/gate_check",
            "value": 7.479167430372052,
            "unit": "ns"
          },
          {
            "name": "6.4/live_tool_body",
            "value": 1821.126918804282,
            "unit": "ns"
          },
          {
            "name": "6.4/loop_8_reads",
            "value": 2177963.354411765,
            "unit": "ns"
          },
          {
            "name": "6.4/loop_iteration",
            "value": 540.3347028545038,
            "unit": "ns"
          },
          {
            "name": "6.4/stall_observe",
            "value": 139.10291515587926,
            "unit": "ns"
          },
          {
            "name": "6.4/validate_repair",
            "value": 265.3584626719057,
            "unit": "ns"
          },
          {
            "name": "6.5/budget_notice",
            "value": 76.12716196963905,
            "unit": "ns"
          },
          {
            "name": "6.5/cancel_select",
            "value": 95.04477471072212,
            "unit": "ns"
          },
          {
            "name": "6.5/gate_check",
            "value": 7.483481745743173,
            "unit": "ns"
          },
          {
            "name": "6.5/live_tool_body",
            "value": 1682.229454761978,
            "unit": "ns"
          },
          {
            "name": "6.5/loop_8_reads",
            "value": 2173673.3986928104,
            "unit": "ns"
          },
          {
            "name": "6.5/loop_iteration",
            "value": 613.7883371352369,
            "unit": "ns"
          },
          {
            "name": "6.5/stall_observe",
            "value": 136.72358795061209,
            "unit": "ns"
          },
          {
            "name": "6.5/validate_repair",
            "value": 270.4467178396839,
            "unit": "ns"
          },
          {
            "name": "6.6/budget_notice",
            "value": 79.93648665048545,
            "unit": "ns"
          },
          {
            "name": "6.6/cancel_select",
            "value": 96.72891564841605,
            "unit": "ns"
          },
          {
            "name": "6.6/gate_check",
            "value": 7.474917026651395,
            "unit": "ns"
          },
          {
            "name": "6.6/live_tool_body",
            "value": 1705.0010647409422,
            "unit": "ns"
          },
          {
            "name": "6.6/loop_8_reads",
            "value": 2180359.707983193,
            "unit": "ns"
          },
          {
            "name": "6.6/loop_iteration",
            "value": 622.5358635515536,
            "unit": "ns"
          },
          {
            "name": "6.6/stall_observe",
            "value": 141.9557573763342,
            "unit": "ns"
          },
          {
            "name": "6.6/truncate_output",
            "value": 4645.114154161969,
            "unit": "ns"
          },
          {
            "name": "6.6/validate_repair",
            "value": 265.99780705000126,
            "unit": "ns"
          },
          {
            "name": "6.7/budget_notice",
            "value": 77.43160774101929,
            "unit": "ns"
          },
          {
            "name": "6.7/cancel_select",
            "value": 95.38316603436257,
            "unit": "ns"
          },
          {
            "name": "6.7/gate_check",
            "value": 7.481568413473935,
            "unit": "ns"
          },
          {
            "name": "6.7/live_tool_body",
            "value": 1692.6356349651658,
            "unit": "ns"
          },
          {
            "name": "6.7/loop_8_reads",
            "value": 2180270.0588235296,
            "unit": "ns"
          },
          {
            "name": "6.7/loop_iteration",
            "value": 615.7592284835158,
            "unit": "ns"
          },
          {
            "name": "6.7/stall_observe",
            "value": 141.09226935946617,
            "unit": "ns"
          },
          {
            "name": "6.7/steer_append",
            "value": 58.34345204758412,
            "unit": "ns"
          },
          {
            "name": "6.7/truncate_output",
            "value": 4542.929149890513,
            "unit": "ns"
          },
          {
            "name": "6.7/validate_repair",
            "value": 259.2980268805453,
            "unit": "ns"
          },
          {
            "name": "6.8/budget_notice",
            "value": 76.58662350714764,
            "unit": "ns"
          },
          {
            "name": "6.8/cancel_select",
            "value": 96.03344370565597,
            "unit": "ns"
          },
          {
            "name": "6.8/canonical_args",
            "value": 1051.9261968213507,
            "unit": "ns"
          },
          {
            "name": "6.8/gate_check",
            "value": 7.472698932486691,
            "unit": "ns"
          },
          {
            "name": "6.8/live_tool_body",
            "value": 1772.6188112714913,
            "unit": "ns"
          },
          {
            "name": "6.8/loop_8_reads",
            "value": 2181663.431372549,
            "unit": "ns"
          },
          {
            "name": "6.8/loop_iteration",
            "value": 707.7460279382486,
            "unit": "ns"
          },
          {
            "name": "6.8/stall_observe",
            "value": 140.3519313892826,
            "unit": "ns"
          },
          {
            "name": "6.8/steer_append",
            "value": 57.955204771656014,
            "unit": "ns"
          },
          {
            "name": "6.8/truncate_output",
            "value": 4602.64319560273,
            "unit": "ns"
          },
          {
            "name": "6.8/validate_repair",
            "value": 260.2317312901828,
            "unit": "ns"
          },
          {
            "name": "6.9/budget_notice",
            "value": 83.69308814264642,
            "unit": "ns"
          },
          {
            "name": "6.9/cancel_select",
            "value": 96.13230186681733,
            "unit": "ns"
          },
          {
            "name": "6.9/canonical_args",
            "value": 1074.3193925549153,
            "unit": "ns"
          },
          {
            "name": "6.9/gate_check",
            "value": 7.472847532344744,
            "unit": "ns"
          },
          {
            "name": "6.9/live_tool_body",
            "value": 1759.7185199211044,
            "unit": "ns"
          },
          {
            "name": "6.9/loop_8_reads",
            "value": 2181085.731617647,
            "unit": "ns"
          },
          {
            "name": "6.9/loop_iteration",
            "value": 768.6804474907349,
            "unit": "ns"
          },
          {
            "name": "6.9/outcome_label",
            "value": 8.398881732287037,
            "unit": "ns"
          },
          {
            "name": "6.9/stall_observe",
            "value": 144.65097362320222,
            "unit": "ns"
          },
          {
            "name": "6.9/steer_append",
            "value": 62.81140871373112,
            "unit": "ns"
          },
          {
            "name": "6.9/truncate_output",
            "value": 4630.787667084378,
            "unit": "ns"
          },
          {
            "name": "6.9/validate_repair",
            "value": 271.0564192764619,
            "unit": "ns"
          },
          {
            "name": "gw5/echo_through_gate",
            "value": 45.50103431522389,
            "unit": "ns"
          },
          {
            "name": "gw5/eight_concurrent_sleeps",
            "value": 2122319.4290123456,
            "unit": "ns"
          },
          {
            "name": "gw7/catalog_arc_swap",
            "value": 2397.1196527366137,
            "unit": "ns"
          },
          {
            "name": "gw7/catalog_reload_10k",
            "value": 26756599.652777776,
            "unit": "ns"
          },
          {
            "name": "gw7/catalog_search_10k",
            "value": 295454.0911730546,
            "unit": "ns"
          },
          {
            "name": "gw8/dynamic_rename_task",
            "value": 5745377.717261905,
            "unit": "ns"
          },
          {
            "name": "gw8/every_tool_rename_task",
            "value": 8407245.2625,
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
          "id": "ded698fb4fb59d6c38ecf4e384e603f075413b74",
          "message": "Write chapters 11 through 20",
          "timestamp": "2026-10-15T04:44:13Z",
          "url": "https://github.com/agnathan/rust-agent-harness/pull/5/commits/ded698fb4fb59d6c38ecf4e384e603f075413b74"
        },
        "date": 1792199342638,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "1.1/serde_50_message_request",
            "value": 6060.308044920024,
            "unit": "ns"
          },
          {
            "name": "1.2/mock_100_concurrent",
            "value": 59666.94041604755,
            "unit": "ns"
          },
          {
            "name": "1.2/mock_turn_overhead",
            "value": 185.12835814038954,
            "unit": "ns"
          },
          {
            "name": "1.3/build_request_and_parse_response",
            "value": 1526.6801503033007,
            "unit": "ns"
          },
          {
            "name": "1.4/parse_response_edges",
            "value": 644.3719288872513,
            "unit": "ns"
          },
          {
            "name": "10/scripted_turn",
            "value": 3.82403555721721,
            "unit": "ns"
          },
          {
            "name": "10/turn_transition",
            "value": 0.6547057471774558,
            "unit": "ns"
          },
          {
            "name": "11/skip_decision",
            "value": 152.11199582950928,
            "unit": "ns"
          },
          {
            "name": "12/api_request_hook",
            "value": 1.3837507560829287,
            "unit": "ns"
          },
          {
            "name": "12/empty_pre_post",
            "value": 7.012385290034775,
            "unit": "ns"
          },
          {
            "name": "12/memory_enqueue",
            "value": 33.00612845732913,
            "unit": "ns"
          },
          {
            "name": "13/checkpoint_1000",
            "value": 41887.86928739694,
            "unit": "ns"
          },
          {
            "name": "13/rewind_1000",
            "value": 61626.669780927834,
            "unit": "ns"
          },
          {
            "name": "13/warm_accept",
            "value": 16257.739875389409,
            "unit": "ns"
          },
          {
            "name": "14/empty_budget",
            "value": 0.5583731216177452,
            "unit": "ns"
          },
          {
            "name": "14/repetition_guard",
            "value": 1407.844936328031,
            "unit": "ns"
          },
          {
            "name": "15/open_breaker",
            "value": 18.294317521591882,
            "unit": "ns"
          },
          {
            "name": "15/permit_release",
            "value": 71.35103149861592,
            "unit": "ns"
          },
          {
            "name": "16/choose_route",
            "value": 2.4958815139243202,
            "unit": "ns"
          },
          {
            "name": "16/rotate_key",
            "value": 2.666081306387376,
            "unit": "ns"
          },
          {
            "name": "17/buffer_join",
            "value": 365.0948668547503,
            "unit": "ns"
          },
          {
            "name": "17/fold_deltas",
            "value": 1385.951203364748,
            "unit": "ns"
          },
          {
            "name": "18/wal_append",
            "value": 116051.30306799337,
            "unit": "ns"
          },
          {
            "name": "19/memory_enqueue",
            "value": 34.69386336256933,
            "unit": "ns"
          },
          {
            "name": "2/chat_turn_overhead",
            "value": 596.6066784814263,
            "unit": "ns"
          },
          {
            "name": "2/inbox_submit",
            "value": 30.837817899390892,
            "unit": "ns"
          },
          {
            "name": "20/discover_10k",
            "value": 22819.53812297735,
            "unit": "ns"
          },
          {
            "name": "3.1/registry_lookup",
            "value": 11.732367424852995,
            "unit": "ns"
          },
          {
            "name": "3.2/available_tools_50",
            "value": 17280.602031730614,
            "unit": "ns"
          },
          {
            "name": "3.3/dispatch_8_reads_concurrent",
            "value": 2120149.650925926,
            "unit": "ns"
          },
          {
            "name": "3.3/dispatch_8_reads_sequential",
            "value": 16748584.688888889,
            "unit": "ns"
          },
          {
            "name": "4.1/stdio_echo_round_trip",
            "value": 87257.48493740219,
            "unit": "ns"
          },
          {
            "name": "4.2/http_echo_round_trip",
            "value": 230665.433694997,
            "unit": "ns"
          },
          {
            "name": "4.3/mcp_50_concurrent",
            "value": 2358504.615017361,
            "unit": "ns"
          },
          {
            "name": "4.3/mcp_50_sequential",
            "value": 4420507.287301587,
            "unit": "ns"
          },
          {
            "name": "4.4/mcp_list_cold",
            "value": 98535.13768115942,
            "unit": "ns"
          },
          {
            "name": "4.4/mcp_list_hit",
            "value": 204.1787484938431,
            "unit": "ns"
          },
          {
            "name": "5.1/schema_tokens_1000",
            "value": 725571.745,
            "unit": "ns"
          },
          {
            "name": "5.1/schema_tokens_10000",
            "value": 28554250.366666667,
            "unit": "ns"
          },
          {
            "name": "5.2/index_build_10000",
            "value": 13570279.125,
            "unit": "ns"
          },
          {
            "name": "5.2/index_search_10000",
            "value": 128351.00758928571,
            "unit": "ns"
          },
          {
            "name": "5.3/full_schema_tokens_10000",
            "value": 13680961.16111111,
            "unit": "ns"
          },
          {
            "name": "5.3/lazy_visible_prompt_10000",
            "value": 10637.165067364545,
            "unit": "ns"
          },
          {
            "name": "5.4/routed_search_10000",
            "value": 13160.510004684256,
            "unit": "ns"
          },
          {
            "name": "5.4/shrink_schemas_10000",
            "value": 50011887.65178572,
            "unit": "ns"
          },
          {
            "name": "5.5/agent_filter_10000",
            "value": 53265.3996366279,
            "unit": "ns"
          },
          {
            "name": "5.5/cache_cold_10000",
            "value": 16604401.6875,
            "unit": "ns"
          },
          {
            "name": "5.5/cache_hit_10000",
            "value": 14.951354767161883,
            "unit": "ns"
          },
          {
            "name": "6.1/gate_check",
            "value": 1.8204882160038258,
            "unit": "ns"
          },
          {
            "name": "6.1/live_tool_body",
            "value": 1183.7130264945652,
            "unit": "ns"
          },
          {
            "name": "6.1/loop_8_reads",
            "value": 2124577.861111111,
            "unit": "ns"
          },
          {
            "name": "6.1/loop_iteration",
            "value": 215.18018980168787,
            "unit": "ns"
          },
          {
            "name": "6.10/budget_notice",
            "value": 43.19355104162803,
            "unit": "ns"
          },
          {
            "name": "6.10/cancel_select",
            "value": 140.53754862905018,
            "unit": "ns"
          },
          {
            "name": "6.10/canonical_args",
            "value": 696.3606418508787,
            "unit": "ns"
          },
          {
            "name": "6.10/gate_check",
            "value": 11.421011898339797,
            "unit": "ns"
          },
          {
            "name": "6.10/live_tool_body",
            "value": 1226.1454193947907,
            "unit": "ns"
          },
          {
            "name": "6.10/loop_8_reads",
            "value": 2132891.421296296,
            "unit": "ns"
          },
          {
            "name": "6.10/loop_iteration",
            "value": 583.577332129024,
            "unit": "ns"
          },
          {
            "name": "6.10/outcome_label",
            "value": 5.148402272989417,
            "unit": "ns"
          },
          {
            "name": "6.10/stall_observe",
            "value": 71.81858060320145,
            "unit": "ns"
          },
          {
            "name": "6.10/steer_append",
            "value": 40.59869234825966,
            "unit": "ns"
          },
          {
            "name": "6.10/stress_repair",
            "value": 26052.526644816702,
            "unit": "ns"
          },
          {
            "name": "6.10/truncate_output",
            "value": 2185.437910016978,
            "unit": "ns"
          },
          {
            "name": "6.10/validate_repair",
            "value": 173.3805153002317,
            "unit": "ns"
          },
          {
            "name": "6.2/cancel_select",
            "value": 154.7568733997398,
            "unit": "ns"
          },
          {
            "name": "6.2/gate_check",
            "value": 12.189562442017245,
            "unit": "ns"
          },
          {
            "name": "6.2/live_tool_body",
            "value": 1248.3882247780753,
            "unit": "ns"
          },
          {
            "name": "6.2/loop_8_reads",
            "value": 2121429.2268518517,
            "unit": "ns"
          },
          {
            "name": "6.2/loop_iteration",
            "value": 405.63597770603053,
            "unit": "ns"
          },
          {
            "name": "6.3/cancel_select",
            "value": 153.77613112103668,
            "unit": "ns"
          },
          {
            "name": "6.3/gate_check",
            "value": 12.06629085137714,
            "unit": "ns"
          },
          {
            "name": "6.3/live_tool_body",
            "value": 1284.7977915509011,
            "unit": "ns"
          },
          {
            "name": "6.3/loop_8_reads",
            "value": 2128236.569444444,
            "unit": "ns"
          },
          {
            "name": "6.3/loop_iteration",
            "value": 432.92628795870576,
            "unit": "ns"
          },
          {
            "name": "6.3/validate_repair",
            "value": 185.43587500184066,
            "unit": "ns"
          },
          {
            "name": "6.4/cancel_select",
            "value": 152.3315745328107,
            "unit": "ns"
          },
          {
            "name": "6.4/gate_check",
            "value": 12.01238439147922,
            "unit": "ns"
          },
          {
            "name": "6.4/live_tool_body",
            "value": 1288.0649689817456,
            "unit": "ns"
          },
          {
            "name": "6.4/loop_8_reads",
            "value": 2137328.125,
            "unit": "ns"
          },
          {
            "name": "6.4/loop_iteration",
            "value": 442.45515970964095,
            "unit": "ns"
          },
          {
            "name": "6.4/stall_observe",
            "value": 67.3426041869995,
            "unit": "ns"
          },
          {
            "name": "6.4/validate_repair",
            "value": 181.33046378708042,
            "unit": "ns"
          },
          {
            "name": "6.5/budget_notice",
            "value": 42.36621522391327,
            "unit": "ns"
          },
          {
            "name": "6.5/cancel_select",
            "value": 153.75057554544156,
            "unit": "ns"
          },
          {
            "name": "6.5/gate_check",
            "value": 11.915702716053307,
            "unit": "ns"
          },
          {
            "name": "6.5/live_tool_body",
            "value": 1337.922793086203,
            "unit": "ns"
          },
          {
            "name": "6.5/loop_8_reads",
            "value": 2144381.6666666665,
            "unit": "ns"
          },
          {
            "name": "6.5/loop_iteration",
            "value": 472.84258660412524,
            "unit": "ns"
          },
          {
            "name": "6.5/stall_observe",
            "value": 69.19178327764513,
            "unit": "ns"
          },
          {
            "name": "6.5/validate_repair",
            "value": 186.91792325816223,
            "unit": "ns"
          },
          {
            "name": "6.6/budget_notice",
            "value": 43.02049078584191,
            "unit": "ns"
          },
          {
            "name": "6.6/cancel_select",
            "value": 153.04055020528722,
            "unit": "ns"
          },
          {
            "name": "6.6/gate_check",
            "value": 11.969679455741623,
            "unit": "ns"
          },
          {
            "name": "6.6/live_tool_body",
            "value": 1313.2317818857018,
            "unit": "ns"
          },
          {
            "name": "6.6/loop_8_reads",
            "value": 2146213.948039216,
            "unit": "ns"
          },
          {
            "name": "6.6/loop_iteration",
            "value": 480.69114000453226,
            "unit": "ns"
          },
          {
            "name": "6.6/stall_observe",
            "value": 69.01027939662671,
            "unit": "ns"
          },
          {
            "name": "6.6/truncate_output",
            "value": 2311.907350324139,
            "unit": "ns"
          },
          {
            "name": "6.6/validate_repair",
            "value": 150.03239520252185,
            "unit": "ns"
          },
          {
            "name": "6.7/budget_notice",
            "value": 43.27499256766221,
            "unit": "ns"
          },
          {
            "name": "6.7/cancel_select",
            "value": 150.9111638134466,
            "unit": "ns"
          },
          {
            "name": "6.7/gate_check",
            "value": 12.07871414792588,
            "unit": "ns"
          },
          {
            "name": "6.7/live_tool_body",
            "value": 1214.576967142605,
            "unit": "ns"
          },
          {
            "name": "6.7/loop_8_reads",
            "value": 2147678.8078703703,
            "unit": "ns"
          },
          {
            "name": "6.7/loop_iteration",
            "value": 469.3125184375494,
            "unit": "ns"
          },
          {
            "name": "6.7/stall_observe",
            "value": 80.31273545783449,
            "unit": "ns"
          },
          {
            "name": "6.7/steer_append",
            "value": 40.54451866572937,
            "unit": "ns"
          },
          {
            "name": "6.7/truncate_output",
            "value": 2257.3572290590964,
            "unit": "ns"
          },
          {
            "name": "6.7/validate_repair",
            "value": 171.97423831335306,
            "unit": "ns"
          },
          {
            "name": "6.8/budget_notice",
            "value": 43.05709414596128,
            "unit": "ns"
          },
          {
            "name": "6.8/cancel_select",
            "value": 147.01527806202483,
            "unit": "ns"
          },
          {
            "name": "6.8/canonical_args",
            "value": 657.6751503325851,
            "unit": "ns"
          },
          {
            "name": "6.8/gate_check",
            "value": 11.365427052961646,
            "unit": "ns"
          },
          {
            "name": "6.8/live_tool_body",
            "value": 1223.8356158619963,
            "unit": "ns"
          },
          {
            "name": "6.8/loop_8_reads",
            "value": 2137758.9205882354,
            "unit": "ns"
          },
          {
            "name": "6.8/loop_iteration",
            "value": 521.4003762921527,
            "unit": "ns"
          },
          {
            "name": "6.8/stall_observe",
            "value": 69.66081933212843,
            "unit": "ns"
          },
          {
            "name": "6.8/steer_append",
            "value": 40.093019839087184,
            "unit": "ns"
          },
          {
            "name": "6.8/truncate_output",
            "value": 2797.408153090965,
            "unit": "ns"
          },
          {
            "name": "6.8/validate_repair",
            "value": 177.52638316236877,
            "unit": "ns"
          },
          {
            "name": "6.9/budget_notice",
            "value": 42.43039244742111,
            "unit": "ns"
          },
          {
            "name": "6.9/cancel_select",
            "value": 146.58626419684117,
            "unit": "ns"
          },
          {
            "name": "6.9/canonical_args",
            "value": 661.2268908349238,
            "unit": "ns"
          },
          {
            "name": "6.9/gate_check",
            "value": 11.515037874826806,
            "unit": "ns"
          },
          {
            "name": "6.9/live_tool_body",
            "value": 1299.5848712408347,
            "unit": "ns"
          },
          {
            "name": "6.9/loop_8_reads",
            "value": 2137941.8086419753,
            "unit": "ns"
          },
          {
            "name": "6.9/loop_iteration",
            "value": 568.4505307524196,
            "unit": "ns"
          },
          {
            "name": "6.9/outcome_label",
            "value": 5.158900280002836,
            "unit": "ns"
          },
          {
            "name": "6.9/stall_observe",
            "value": 70.81649652930547,
            "unit": "ns"
          },
          {
            "name": "6.9/steer_append",
            "value": 41.32268861945587,
            "unit": "ns"
          },
          {
            "name": "6.9/truncate_output",
            "value": 2148.4091737667936,
            "unit": "ns"
          },
          {
            "name": "6.9/validate_repair",
            "value": 174.5489072380717,
            "unit": "ns"
          },
          {
            "name": "gw5/echo_through_gate",
            "value": 72.44798594688893,
            "unit": "ns"
          },
          {
            "name": "gw5/eight_concurrent_sleeps",
            "value": 2098612.244047619,
            "unit": "ns"
          },
          {
            "name": "gw7/catalog_arc_swap",
            "value": 1554.4042302900625,
            "unit": "ns"
          },
          {
            "name": "gw7/catalog_reload_10k",
            "value": 16806812.111111112,
            "unit": "ns"
          },
          {
            "name": "gw7/catalog_search_10k",
            "value": 120882.67434210525,
            "unit": "ns"
          },
          {
            "name": "gw8/dynamic_rename_task",
            "value": 3541777.75,
            "unit": "ns"
          },
          {
            "name": "gw8/every_tool_rename_task",
            "value": 4873635.916666666,
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
          "id": "2315ac37985b3e360aa72dcd25c72a7a2b0ca32f",
          "message": "Write chapters 11 through 20",
          "timestamp": "2026-10-15T04:44:13Z",
          "url": "https://github.com/agnathan/rust-agent-harness/pull/5/commits/2315ac37985b3e360aa72dcd25c72a7a2b0ca32f"
        },
        "date": 1792199580085,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "1.1/serde_50_message_request",
            "value": 10267.614586285068,
            "unit": "ns"
          },
          {
            "name": "1.2/mock_100_concurrent",
            "value": 77773.25880902042,
            "unit": "ns"
          },
          {
            "name": "1.2/mock_turn_overhead",
            "value": 251.559425772876,
            "unit": "ns"
          },
          {
            "name": "1.3/build_request_and_parse_response",
            "value": 2945.9716248229092,
            "unit": "ns"
          },
          {
            "name": "1.4/parse_response_edges",
            "value": 1125.4304163837796,
            "unit": "ns"
          },
          {
            "name": "10/scripted_turn",
            "value": 8.125109543303314,
            "unit": "ns"
          },
          {
            "name": "10/turn_transition",
            "value": 1.0987795308744568,
            "unit": "ns"
          },
          {
            "name": "11/skip_decision",
            "value": 251.19909410889719,
            "unit": "ns"
          },
          {
            "name": "12/api_request_hook",
            "value": 2.1807822673288593,
            "unit": "ns"
          },
          {
            "name": "12/empty_pre_post",
            "value": 10.902331458686987,
            "unit": "ns"
          },
          {
            "name": "12/memory_enqueue",
            "value": 54.94284976538362,
            "unit": "ns"
          },
          {
            "name": "13/checkpoint_1000",
            "value": 50588.00350694444,
            "unit": "ns"
          },
          {
            "name": "13/rewind_1000",
            "value": 79324.42560572686,
            "unit": "ns"
          },
          {
            "name": "13/warm_accept",
            "value": 29605.169030351866,
            "unit": "ns"
          },
          {
            "name": "14/empty_budget",
            "value": 0.9036262111094508,
            "unit": "ns"
          },
          {
            "name": "14/repetition_guard",
            "value": 2539.603118680706,
            "unit": "ns"
          },
          {
            "name": "15/open_breaker",
            "value": 30.028447218559947,
            "unit": "ns"
          },
          {
            "name": "15/permit_release",
            "value": 59.04939655939732,
            "unit": "ns"
          },
          {
            "name": "16/choose_route",
            "value": 5.920743370349252,
            "unit": "ns"
          },
          {
            "name": "16/rotate_key",
            "value": 3.4459288748271555,
            "unit": "ns"
          },
          {
            "name": "17/buffer_join",
            "value": 674.92506807715,
            "unit": "ns"
          },
          {
            "name": "17/fold_deltas",
            "value": 2832.0777059815755,
            "unit": "ns"
          },
          {
            "name": "18/wal_append",
            "value": 134601.77093023257,
            "unit": "ns"
          },
          {
            "name": "19/memory_enqueue",
            "value": 56.19661345685286,
            "unit": "ns"
          },
          {
            "name": "2/chat_turn_overhead",
            "value": 698.509442421778,
            "unit": "ns"
          },
          {
            "name": "2/inbox_submit",
            "value": 40.43598729698647,
            "unit": "ns"
          },
          {
            "name": "20/discover_10k",
            "value": 61645.10644067796,
            "unit": "ns"
          },
          {
            "name": "3.1/registry_lookup",
            "value": 25.73468961333186,
            "unit": "ns"
          },
          {
            "name": "3.2/available_tools_50",
            "value": 30444.15973632346,
            "unit": "ns"
          },
          {
            "name": "3.3/dispatch_8_reads_concurrent",
            "value": 2139011.3,
            "unit": "ns"
          },
          {
            "name": "3.3/dispatch_8_reads_sequential",
            "value": 16716109.708333334,
            "unit": "ns"
          },
          {
            "name": "4.1/stdio_echo_round_trip",
            "value": 199111.24246031744,
            "unit": "ns"
          },
          {
            "name": "4.2/http_echo_round_trip",
            "value": 408816.59735449735,
            "unit": "ns"
          },
          {
            "name": "4.3/mcp_50_concurrent",
            "value": 4876437.983333333,
            "unit": "ns"
          },
          {
            "name": "4.3/mcp_50_sequential",
            "value": 9869070.239285715,
            "unit": "ns"
          },
          {
            "name": "4.4/mcp_list_cold",
            "value": 215139.82419590643,
            "unit": "ns"
          },
          {
            "name": "4.4/mcp_list_hit",
            "value": 257.8669716112146,
            "unit": "ns"
          },
          {
            "name": "5.1/schema_tokens_1000",
            "value": 1280290.935632184,
            "unit": "ns"
          },
          {
            "name": "5.1/schema_tokens_10000",
            "value": 42636141.25,
            "unit": "ns"
          },
          {
            "name": "5.2/index_build_10000",
            "value": 21612418.291666664,
            "unit": "ns"
          },
          {
            "name": "5.2/index_search_10000",
            "value": 340623.39058641973,
            "unit": "ns"
          },
          {
            "name": "5.3/full_schema_tokens_10000",
            "value": 23070372.05,
            "unit": "ns"
          },
          {
            "name": "5.3/lazy_visible_prompt_10000",
            "value": 15193.290527754416,
            "unit": "ns"
          },
          {
            "name": "5.4/routed_search_10000",
            "value": 25875.170581257415,
            "unit": "ns"
          },
          {
            "name": "5.4/shrink_schemas_10000",
            "value": 63455501.75,
            "unit": "ns"
          },
          {
            "name": "5.5/agent_filter_10000",
            "value": 125657.84689655172,
            "unit": "ns"
          },
          {
            "name": "5.5/cache_cold_10000",
            "value": 25881661.3,
            "unit": "ns"
          },
          {
            "name": "5.5/cache_hit_10000",
            "value": 13.80087022849804,
            "unit": "ns"
          },
          {
            "name": "6.1/gate_check",
            "value": 3.425245726459498,
            "unit": "ns"
          },
          {
            "name": "6.1/live_tool_body",
            "value": 1715.3755873558307,
            "unit": "ns"
          },
          {
            "name": "6.1/loop_8_reads",
            "value": 2150890.7647058824,
            "unit": "ns"
          },
          {
            "name": "6.1/loop_iteration",
            "value": 362.5690218885677,
            "unit": "ns"
          },
          {
            "name": "6.10/budget_notice",
            "value": 77.10097999056232,
            "unit": "ns"
          },
          {
            "name": "6.10/cancel_select",
            "value": 94.84233215389439,
            "unit": "ns"
          },
          {
            "name": "6.10/canonical_args",
            "value": 1070.0448296534582,
            "unit": "ns"
          },
          {
            "name": "6.10/gate_check",
            "value": 7.474114491513372,
            "unit": "ns"
          },
          {
            "name": "6.10/live_tool_body",
            "value": 1779.4904154713863,
            "unit": "ns"
          },
          {
            "name": "6.10/loop_8_reads",
            "value": 2180757.0280112047,
            "unit": "ns"
          },
          {
            "name": "6.10/loop_iteration",
            "value": 774.4382543676138,
            "unit": "ns"
          },
          {
            "name": "6.10/outcome_label",
            "value": 8.397537091946996,
            "unit": "ns"
          },
          {
            "name": "6.10/stall_observe",
            "value": 138.8603154232261,
            "unit": "ns"
          },
          {
            "name": "6.10/steer_append",
            "value": 59.8929918926447,
            "unit": "ns"
          },
          {
            "name": "6.10/stress_repair",
            "value": 50617.600016650016,
            "unit": "ns"
          },
          {
            "name": "6.10/truncate_output",
            "value": 4668.684933645589,
            "unit": "ns"
          },
          {
            "name": "6.10/validate_repair",
            "value": 284.05903150543486,
            "unit": "ns"
          },
          {
            "name": "6.2/cancel_select",
            "value": 98.84257236257628,
            "unit": "ns"
          },
          {
            "name": "6.2/gate_check",
            "value": 7.476494994588689,
            "unit": "ns"
          },
          {
            "name": "6.2/live_tool_body",
            "value": 1783.7836795240182,
            "unit": "ns"
          },
          {
            "name": "6.2/loop_8_reads",
            "value": 2160936.4437908498,
            "unit": "ns"
          },
          {
            "name": "6.2/loop_iteration",
            "value": 523.0144234025057,
            "unit": "ns"
          },
          {
            "name": "6.3/cancel_select",
            "value": 97.97189024909122,
            "unit": "ns"
          },
          {
            "name": "6.3/gate_check",
            "value": 7.473172315682126,
            "unit": "ns"
          },
          {
            "name": "6.3/live_tool_body",
            "value": 1716.72544341417,
            "unit": "ns"
          },
          {
            "name": "6.3/loop_8_reads",
            "value": 2170315.762254902,
            "unit": "ns"
          },
          {
            "name": "6.3/loop_iteration",
            "value": 534.0427815796908,
            "unit": "ns"
          },
          {
            "name": "6.3/validate_repair",
            "value": 260.88278186846736,
            "unit": "ns"
          },
          {
            "name": "6.4/cancel_select",
            "value": 95.96220754978725,
            "unit": "ns"
          },
          {
            "name": "6.4/gate_check",
            "value": 7.476319402791939,
            "unit": "ns"
          },
          {
            "name": "6.4/live_tool_body",
            "value": 1787.440104600767,
            "unit": "ns"
          },
          {
            "name": "6.4/loop_8_reads",
            "value": 2177017.6018907563,
            "unit": "ns"
          },
          {
            "name": "6.4/loop_iteration",
            "value": 565.5026826714764,
            "unit": "ns"
          },
          {
            "name": "6.4/stall_observe",
            "value": 139.11708303944545,
            "unit": "ns"
          },
          {
            "name": "6.4/validate_repair",
            "value": 262.68644591417876,
            "unit": "ns"
          },
          {
            "name": "6.5/budget_notice",
            "value": 77.44668984330816,
            "unit": "ns"
          },
          {
            "name": "6.5/cancel_select",
            "value": 96.56981462551111,
            "unit": "ns"
          },
          {
            "name": "6.5/gate_check",
            "value": 7.475109241127203,
            "unit": "ns"
          },
          {
            "name": "6.5/live_tool_body",
            "value": 1746.3606040678285,
            "unit": "ns"
          },
          {
            "name": "6.5/loop_8_reads",
            "value": 2174198.176470588,
            "unit": "ns"
          },
          {
            "name": "6.5/loop_iteration",
            "value": 591.3203244679117,
            "unit": "ns"
          },
          {
            "name": "6.5/stall_observe",
            "value": 137.5431253396478,
            "unit": "ns"
          },
          {
            "name": "6.5/validate_repair",
            "value": 291.9861897195917,
            "unit": "ns"
          },
          {
            "name": "6.6/budget_notice",
            "value": 83.2569485600439,
            "unit": "ns"
          },
          {
            "name": "6.6/cancel_select",
            "value": 104.03970855404839,
            "unit": "ns"
          },
          {
            "name": "6.6/gate_check",
            "value": 7.471725590037124,
            "unit": "ns"
          },
          {
            "name": "6.6/live_tool_body",
            "value": 1726.792245784974,
            "unit": "ns"
          },
          {
            "name": "6.6/loop_8_reads",
            "value": 2167418.0455882354,
            "unit": "ns"
          },
          {
            "name": "6.6/loop_iteration",
            "value": 615.3895551921662,
            "unit": "ns"
          },
          {
            "name": "6.6/stall_observe",
            "value": 140.9595954643006,
            "unit": "ns"
          },
          {
            "name": "6.6/truncate_output",
            "value": 4532.208818649411,
            "unit": "ns"
          },
          {
            "name": "6.6/validate_repair",
            "value": 266.8840718796174,
            "unit": "ns"
          },
          {
            "name": "6.7/budget_notice",
            "value": 77.58587003359585,
            "unit": "ns"
          },
          {
            "name": "6.7/cancel_select",
            "value": 96.36484208552318,
            "unit": "ns"
          },
          {
            "name": "6.7/gate_check",
            "value": 7.475354586816045,
            "unit": "ns"
          },
          {
            "name": "6.7/live_tool_body",
            "value": 1673.732396476571,
            "unit": "ns"
          },
          {
            "name": "6.7/loop_8_reads",
            "value": 2173329.2787114847,
            "unit": "ns"
          },
          {
            "name": "6.7/loop_iteration",
            "value": 608.2243323564389,
            "unit": "ns"
          },
          {
            "name": "6.7/stall_observe",
            "value": 140.30752375861633,
            "unit": "ns"
          },
          {
            "name": "6.7/steer_append",
            "value": 58.41255048490463,
            "unit": "ns"
          },
          {
            "name": "6.7/truncate_output",
            "value": 4579.6028494101565,
            "unit": "ns"
          },
          {
            "name": "6.7/validate_repair",
            "value": 270.4077147788229,
            "unit": "ns"
          },
          {
            "name": "6.8/budget_notice",
            "value": 77.37519122444505,
            "unit": "ns"
          },
          {
            "name": "6.8/cancel_select",
            "value": 96.68213515161362,
            "unit": "ns"
          },
          {
            "name": "6.8/canonical_args",
            "value": 1086.4478906466834,
            "unit": "ns"
          },
          {
            "name": "6.8/gate_check",
            "value": 7.471070679023063,
            "unit": "ns"
          },
          {
            "name": "6.8/live_tool_body",
            "value": 1764.389049210732,
            "unit": "ns"
          },
          {
            "name": "6.8/loop_8_reads",
            "value": 2168392.674136321,
            "unit": "ns"
          },
          {
            "name": "6.8/loop_iteration",
            "value": 709.2184720745109,
            "unit": "ns"
          },
          {
            "name": "6.8/stall_observe",
            "value": 140.30314116918507,
            "unit": "ns"
          },
          {
            "name": "6.8/steer_append",
            "value": 58.01339081211441,
            "unit": "ns"
          },
          {
            "name": "6.8/truncate_output",
            "value": 4713.736507832039,
            "unit": "ns"
          },
          {
            "name": "6.8/validate_repair",
            "value": 258.15660032537187,
            "unit": "ns"
          },
          {
            "name": "6.9/budget_notice",
            "value": 80.90826703823396,
            "unit": "ns"
          },
          {
            "name": "6.9/cancel_select",
            "value": 96.41408208642568,
            "unit": "ns"
          },
          {
            "name": "6.9/canonical_args",
            "value": 1048.7712158076592,
            "unit": "ns"
          },
          {
            "name": "6.9/gate_check",
            "value": 7.474743379851333,
            "unit": "ns"
          },
          {
            "name": "6.9/live_tool_body",
            "value": 1768.9319664775824,
            "unit": "ns"
          },
          {
            "name": "6.9/loop_8_reads",
            "value": 2174516.37377451,
            "unit": "ns"
          },
          {
            "name": "6.9/loop_iteration",
            "value": 750.2127372276433,
            "unit": "ns"
          },
          {
            "name": "6.9/outcome_label",
            "value": 8.398376952827217,
            "unit": "ns"
          },
          {
            "name": "6.9/stall_observe",
            "value": 136.322182118176,
            "unit": "ns"
          },
          {
            "name": "6.9/steer_append",
            "value": 59.298528661600315,
            "unit": "ns"
          },
          {
            "name": "6.9/truncate_output",
            "value": 4553.93283227848,
            "unit": "ns"
          },
          {
            "name": "6.9/validate_repair",
            "value": 266.9369371026607,
            "unit": "ns"
          },
          {
            "name": "gw5/echo_through_gate",
            "value": 45.61648248155019,
            "unit": "ns"
          },
          {
            "name": "gw5/eight_concurrent_sleeps",
            "value": 2115966.2157407408,
            "unit": "ns"
          },
          {
            "name": "gw7/catalog_arc_swap",
            "value": 2429.6437839485643,
            "unit": "ns"
          },
          {
            "name": "gw7/catalog_reload_10k",
            "value": 28667334.913194444,
            "unit": "ns"
          },
          {
            "name": "gw7/catalog_search_10k",
            "value": 305731.19404761906,
            "unit": "ns"
          },
          {
            "name": "gw8/dynamic_rename_task",
            "value": 5711543.464285715,
            "unit": "ns"
          },
          {
            "name": "gw8/every_tool_rename_task",
            "value": 8445387.65,
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
          "id": "5d044a6b159cce9775739c6e59a677ca73e31615",
          "message": "Write chapters 11 through 20",
          "timestamp": "2026-10-15T04:44:13Z",
          "url": "https://github.com/agnathan/rust-agent-harness/pull/5/commits/5d044a6b159cce9775739c6e59a677ca73e31615"
        },
        "date": 1792199653790,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "1.1/serde_50_message_request",
            "value": 10048.655811292354,
            "unit": "ns"
          },
          {
            "name": "1.2/mock_100_concurrent",
            "value": 77886.22744890768,
            "unit": "ns"
          },
          {
            "name": "1.2/mock_turn_overhead",
            "value": 250.94700683458228,
            "unit": "ns"
          },
          {
            "name": "1.3/build_request_and_parse_response",
            "value": 2931.8884814890152,
            "unit": "ns"
          },
          {
            "name": "1.4/parse_response_edges",
            "value": 1135.3680408420862,
            "unit": "ns"
          },
          {
            "name": "10/scripted_turn",
            "value": 8.083293119309223,
            "unit": "ns"
          },
          {
            "name": "10/turn_transition",
            "value": 1.0927217332333945,
            "unit": "ns"
          },
          {
            "name": "11/skip_decision",
            "value": 251.906578526084,
            "unit": "ns"
          },
          {
            "name": "12/api_request_hook",
            "value": 2.1812172739120257,
            "unit": "ns"
          },
          {
            "name": "12/empty_pre_post",
            "value": 10.909820753209317,
            "unit": "ns"
          },
          {
            "name": "12/memory_enqueue",
            "value": 56.63104188270326,
            "unit": "ns"
          },
          {
            "name": "13/checkpoint_1000",
            "value": 50589.83793849482,
            "unit": "ns"
          },
          {
            "name": "13/rewind_1000",
            "value": 80907.19419753086,
            "unit": "ns"
          },
          {
            "name": "13/warm_accept",
            "value": 29344.837080009667,
            "unit": "ns"
          },
          {
            "name": "14/empty_budget",
            "value": 0.9039842911805209,
            "unit": "ns"
          },
          {
            "name": "14/repetition_guard",
            "value": 2554.3573962923183,
            "unit": "ns"
          },
          {
            "name": "15/open_breaker",
            "value": 30.176925526690475,
            "unit": "ns"
          },
          {
            "name": "15/permit_release",
            "value": 58.78199039731008,
            "unit": "ns"
          },
          {
            "name": "16/choose_route",
            "value": 6.226762119803944,
            "unit": "ns"
          },
          {
            "name": "16/rotate_key",
            "value": 3.4456910082228838,
            "unit": "ns"
          },
          {
            "name": "17/buffer_join",
            "value": 728.0989888493665,
            "unit": "ns"
          },
          {
            "name": "17/fold_deltas",
            "value": 2832.564423476814,
            "unit": "ns"
          },
          {
            "name": "18/wal_append",
            "value": 137660.1643622322,
            "unit": "ns"
          },
          {
            "name": "19/memory_enqueue",
            "value": 53.19497854051776,
            "unit": "ns"
          },
          {
            "name": "2/chat_turn_overhead",
            "value": 665.8946913810233,
            "unit": "ns"
          },
          {
            "name": "2/inbox_submit",
            "value": 40.418758330669164,
            "unit": "ns"
          },
          {
            "name": "20/discover_10k",
            "value": 59241.23381514658,
            "unit": "ns"
          },
          {
            "name": "3.1/registry_lookup",
            "value": 25.755566586642008,
            "unit": "ns"
          },
          {
            "name": "3.2/available_tools_50",
            "value": 30590.79742329818,
            "unit": "ns"
          },
          {
            "name": "3.3/dispatch_8_reads_concurrent",
            "value": 2137156.856617647,
            "unit": "ns"
          },
          {
            "name": "3.3/dispatch_8_reads_sequential",
            "value": 16701075.833333332,
            "unit": "ns"
          },
          {
            "name": "4.1/stdio_echo_round_trip",
            "value": 201371.45425925925,
            "unit": "ns"
          },
          {
            "name": "4.2/http_echo_round_trip",
            "value": 417335.70716089464,
            "unit": "ns"
          },
          {
            "name": "4.3/mcp_50_concurrent",
            "value": 4977226.8,
            "unit": "ns"
          },
          {
            "name": "4.3/mcp_50_sequential",
            "value": 10099125.6,
            "unit": "ns"
          },
          {
            "name": "4.4/mcp_list_cold",
            "value": 212631.99883040937,
            "unit": "ns"
          },
          {
            "name": "4.4/mcp_list_hit",
            "value": 253.65049916059232,
            "unit": "ns"
          },
          {
            "name": "5.1/schema_tokens_1000",
            "value": 1285181.987244898,
            "unit": "ns"
          },
          {
            "name": "5.1/schema_tokens_10000",
            "value": 41341095,
            "unit": "ns"
          },
          {
            "name": "5.2/index_build_10000",
            "value": 21446836.59166667,
            "unit": "ns"
          },
          {
            "name": "5.2/index_search_10000",
            "value": 332835.20642201835,
            "unit": "ns"
          },
          {
            "name": "5.3/full_schema_tokens_10000",
            "value": 21664237.875,
            "unit": "ns"
          },
          {
            "name": "5.3/lazy_visible_prompt_10000",
            "value": 15514.190084968519,
            "unit": "ns"
          },
          {
            "name": "5.4/routed_search_10000",
            "value": 25846.567379933425,
            "unit": "ns"
          },
          {
            "name": "5.4/shrink_schemas_10000",
            "value": 60617970.08333333,
            "unit": "ns"
          },
          {
            "name": "5.5/agent_filter_10000",
            "value": 126949.34659090909,
            "unit": "ns"
          },
          {
            "name": "5.5/cache_cold_10000",
            "value": 24146263.444444444,
            "unit": "ns"
          },
          {
            "name": "5.5/cache_hit_10000",
            "value": 13.797650550637846,
            "unit": "ns"
          },
          {
            "name": "6.1/gate_check",
            "value": 3.4269348910720696,
            "unit": "ns"
          },
          {
            "name": "6.1/live_tool_body",
            "value": 1740.5462413511436,
            "unit": "ns"
          },
          {
            "name": "6.1/loop_8_reads",
            "value": 2155679.2225490194,
            "unit": "ns"
          },
          {
            "name": "6.1/loop_iteration",
            "value": 367.96386695290585,
            "unit": "ns"
          },
          {
            "name": "6.10/budget_notice",
            "value": 80.0634734013765,
            "unit": "ns"
          },
          {
            "name": "6.10/cancel_select",
            "value": 96.49954287179173,
            "unit": "ns"
          },
          {
            "name": "6.10/canonical_args",
            "value": 1057.8005092284068,
            "unit": "ns"
          },
          {
            "name": "6.10/gate_check",
            "value": 7.474374352707663,
            "unit": "ns"
          },
          {
            "name": "6.10/live_tool_body",
            "value": 1784.722751785454,
            "unit": "ns"
          },
          {
            "name": "6.10/loop_8_reads",
            "value": 2185022.5343137253,
            "unit": "ns"
          },
          {
            "name": "6.10/loop_iteration",
            "value": 731.4080088753237,
            "unit": "ns"
          },
          {
            "name": "6.10/outcome_label",
            "value": 8.399471846409323,
            "unit": "ns"
          },
          {
            "name": "6.10/stall_observe",
            "value": 138.123777762781,
            "unit": "ns"
          },
          {
            "name": "6.10/steer_append",
            "value": 59.80421070982746,
            "unit": "ns"
          },
          {
            "name": "6.10/stress_repair",
            "value": 51308.031782670456,
            "unit": "ns"
          },
          {
            "name": "6.10/truncate_output",
            "value": 4590.952515642957,
            "unit": "ns"
          },
          {
            "name": "6.10/validate_repair",
            "value": 271.3415968483798,
            "unit": "ns"
          },
          {
            "name": "6.2/cancel_select",
            "value": 95.1444079793909,
            "unit": "ns"
          },
          {
            "name": "6.2/gate_check",
            "value": 7.473475160433487,
            "unit": "ns"
          },
          {
            "name": "6.2/live_tool_body",
            "value": 1719.160673779932,
            "unit": "ns"
          },
          {
            "name": "6.2/loop_8_reads",
            "value": 2164632.62605042,
            "unit": "ns"
          },
          {
            "name": "6.2/loop_iteration",
            "value": 531.2737667545595,
            "unit": "ns"
          },
          {
            "name": "6.3/cancel_select",
            "value": 96.34119611140741,
            "unit": "ns"
          },
          {
            "name": "6.3/gate_check",
            "value": 7.469223721580404,
            "unit": "ns"
          },
          {
            "name": "6.3/live_tool_body",
            "value": 1806.333619351341,
            "unit": "ns"
          },
          {
            "name": "6.3/loop_8_reads",
            "value": 2170810.609477124,
            "unit": "ns"
          },
          {
            "name": "6.3/loop_iteration",
            "value": 550.9958427272314,
            "unit": "ns"
          },
          {
            "name": "6.3/validate_repair",
            "value": 254.36657485890535,
            "unit": "ns"
          },
          {
            "name": "6.4/cancel_select",
            "value": 94.69156955230034,
            "unit": "ns"
          },
          {
            "name": "6.4/gate_check",
            "value": 7.469053439463583,
            "unit": "ns"
          },
          {
            "name": "6.4/live_tool_body",
            "value": 1767.174047642211,
            "unit": "ns"
          },
          {
            "name": "6.4/loop_8_reads",
            "value": 2174812.8607843136,
            "unit": "ns"
          },
          {
            "name": "6.4/loop_iteration",
            "value": 540.8064507973928,
            "unit": "ns"
          },
          {
            "name": "6.4/stall_observe",
            "value": 139.02047843068317,
            "unit": "ns"
          },
          {
            "name": "6.4/validate_repair",
            "value": 262.00728386888255,
            "unit": "ns"
          },
          {
            "name": "6.5/budget_notice",
            "value": 75.59748734227702,
            "unit": "ns"
          },
          {
            "name": "6.5/cancel_select",
            "value": 96.53476212948605,
            "unit": "ns"
          },
          {
            "name": "6.5/gate_check",
            "value": 7.4763230637490645,
            "unit": "ns"
          },
          {
            "name": "6.5/live_tool_body",
            "value": 1735.0527005514473,
            "unit": "ns"
          },
          {
            "name": "6.5/loop_8_reads",
            "value": 2173582.770588235,
            "unit": "ns"
          },
          {
            "name": "6.5/loop_iteration",
            "value": 594.1191421752509,
            "unit": "ns"
          },
          {
            "name": "6.5/stall_observe",
            "value": 136.66500591012095,
            "unit": "ns"
          },
          {
            "name": "6.5/validate_repair",
            "value": 290.36184332924034,
            "unit": "ns"
          },
          {
            "name": "6.6/budget_notice",
            "value": 78.79492611919508,
            "unit": "ns"
          },
          {
            "name": "6.6/cancel_select",
            "value": 96.16305775390379,
            "unit": "ns"
          },
          {
            "name": "6.6/gate_check",
            "value": 7.475325169942351,
            "unit": "ns"
          },
          {
            "name": "6.6/live_tool_body",
            "value": 1769.8792149746882,
            "unit": "ns"
          },
          {
            "name": "6.6/loop_8_reads",
            "value": 2179183.6470588236,
            "unit": "ns"
          },
          {
            "name": "6.6/loop_iteration",
            "value": 613.3377044366769,
            "unit": "ns"
          },
          {
            "name": "6.6/stall_observe",
            "value": 140.62987576853595,
            "unit": "ns"
          },
          {
            "name": "6.6/truncate_output",
            "value": 4614.016793860216,
            "unit": "ns"
          },
          {
            "name": "6.6/validate_repair",
            "value": 267.67192941078196,
            "unit": "ns"
          },
          {
            "name": "6.7/budget_notice",
            "value": 76.73631923841867,
            "unit": "ns"
          },
          {
            "name": "6.7/cancel_select",
            "value": 95.3528598751796,
            "unit": "ns"
          },
          {
            "name": "6.7/gate_check",
            "value": 7.472036255460795,
            "unit": "ns"
          },
          {
            "name": "6.7/live_tool_body",
            "value": 1658.3993041975532,
            "unit": "ns"
          },
          {
            "name": "6.7/loop_8_reads",
            "value": 2179895.4588235295,
            "unit": "ns"
          },
          {
            "name": "6.7/loop_iteration",
            "value": 607.7163048143041,
            "unit": "ns"
          },
          {
            "name": "6.7/stall_observe",
            "value": 140.76599392486472,
            "unit": "ns"
          },
          {
            "name": "6.7/steer_append",
            "value": 58.4936631286125,
            "unit": "ns"
          },
          {
            "name": "6.7/truncate_output",
            "value": 4574.741835387324,
            "unit": "ns"
          },
          {
            "name": "6.7/validate_repair",
            "value": 269.3268122839304,
            "unit": "ns"
          },
          {
            "name": "6.8/budget_notice",
            "value": 79.42166785108623,
            "unit": "ns"
          },
          {
            "name": "6.8/cancel_select",
            "value": 97.3366093074421,
            "unit": "ns"
          },
          {
            "name": "6.8/canonical_args",
            "value": 1056.6782186986952,
            "unit": "ns"
          },
          {
            "name": "6.8/gate_check",
            "value": 7.473318814944105,
            "unit": "ns"
          },
          {
            "name": "6.8/live_tool_body",
            "value": 1781.1282884960108,
            "unit": "ns"
          },
          {
            "name": "6.8/loop_8_reads",
            "value": 2185184.4264705884,
            "unit": "ns"
          },
          {
            "name": "6.8/loop_iteration",
            "value": 671.8221958947388,
            "unit": "ns"
          },
          {
            "name": "6.8/stall_observe",
            "value": 138.90658556261158,
            "unit": "ns"
          },
          {
            "name": "6.8/steer_append",
            "value": 58.02746588441592,
            "unit": "ns"
          },
          {
            "name": "6.8/truncate_output",
            "value": 4763.364568033116,
            "unit": "ns"
          },
          {
            "name": "6.8/validate_repair",
            "value": 267.9594978496872,
            "unit": "ns"
          },
          {
            "name": "6.9/budget_notice",
            "value": 80.42863428201073,
            "unit": "ns"
          },
          {
            "name": "6.9/cancel_select",
            "value": 96.74403592547618,
            "unit": "ns"
          },
          {
            "name": "6.9/canonical_args",
            "value": 1062.2282582240523,
            "unit": "ns"
          },
          {
            "name": "6.9/gate_check",
            "value": 7.4730083703109855,
            "unit": "ns"
          },
          {
            "name": "6.9/live_tool_body",
            "value": 1832.8543555451724,
            "unit": "ns"
          },
          {
            "name": "6.9/loop_8_reads",
            "value": 2184934.738970588,
            "unit": "ns"
          },
          {
            "name": "6.9/loop_iteration",
            "value": 741.5198895871079,
            "unit": "ns"
          },
          {
            "name": "6.9/outcome_label",
            "value": 8.397704440988646,
            "unit": "ns"
          },
          {
            "name": "6.9/stall_observe",
            "value": 136.12358801280732,
            "unit": "ns"
          },
          {
            "name": "6.9/steer_append",
            "value": 59.24676435242975,
            "unit": "ns"
          },
          {
            "name": "6.9/truncate_output",
            "value": 4717.0278422199435,
            "unit": "ns"
          },
          {
            "name": "6.9/validate_repair",
            "value": 267.8221559066237,
            "unit": "ns"
          },
          {
            "name": "gw5/echo_through_gate",
            "value": 45.733696944309216,
            "unit": "ns"
          },
          {
            "name": "gw5/eight_concurrent_sleeps",
            "value": 2111658.6220238097,
            "unit": "ns"
          },
          {
            "name": "gw7/catalog_arc_swap",
            "value": 2416.5439545015606,
            "unit": "ns"
          },
          {
            "name": "gw7/catalog_reload_10k",
            "value": 28197736.566666666,
            "unit": "ns"
          },
          {
            "name": "gw7/catalog_search_10k",
            "value": 301619.4036885246,
            "unit": "ns"
          },
          {
            "name": "gw8/dynamic_rename_task",
            "value": 5702156.196428571,
            "unit": "ns"
          },
          {
            "name": "gw8/every_tool_rename_task",
            "value": 8455390.066666666,
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
          "id": "f4c71f784378cb39a03c00cbedca0f1019637a34",
          "message": "Chapters 21–30: skills, reflection, and team patterns",
          "timestamp": "2026-10-15T04:44:13Z",
          "url": "https://github.com/agnathan/rust-agent-harness/pull/6/commits/f4c71f784378cb39a03c00cbedca0f1019637a34"
        },
        "date": 1792199820735,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "1.1/serde_50_message_request",
            "value": 8199.207175141242,
            "unit": "ns"
          },
          {
            "name": "1.2/mock_100_concurrent",
            "value": 66193.77777777778,
            "unit": "ns"
          },
          {
            "name": "1.2/mock_turn_overhead",
            "value": 208.18420663581665,
            "unit": "ns"
          },
          {
            "name": "1.3/build_request_and_parse_response",
            "value": 2141.4320068287516,
            "unit": "ns"
          },
          {
            "name": "1.4/parse_response_edges",
            "value": 914.5882443472717,
            "unit": "ns"
          },
          {
            "name": "10/scripted_turn",
            "value": 5.357501901010558,
            "unit": "ns"
          },
          {
            "name": "10/turn_transition",
            "value": 0.9122989599889869,
            "unit": "ns"
          },
          {
            "name": "11/skip_decision",
            "value": 212.17364961918915,
            "unit": "ns"
          },
          {
            "name": "12/api_request_hook",
            "value": 1.6376042844427086,
            "unit": "ns"
          },
          {
            "name": "12/empty_pre_post",
            "value": 9.819698839747637,
            "unit": "ns"
          },
          {
            "name": "12/memory_enqueue",
            "value": 46.53159382228385,
            "unit": "ns"
          },
          {
            "name": "13/checkpoint_1000",
            "value": 41472.543825075074,
            "unit": "ns"
          },
          {
            "name": "13/rewind_1000",
            "value": 62574.25335591133,
            "unit": "ns"
          },
          {
            "name": "13/warm_accept",
            "value": 20045.663426635474,
            "unit": "ns"
          },
          {
            "name": "14/empty_budget",
            "value": 0.7336311619891542,
            "unit": "ns"
          },
          {
            "name": "14/repetition_guard",
            "value": 2005.5086003136987,
            "unit": "ns"
          },
          {
            "name": "15/open_breaker",
            "value": 19.920811731543058,
            "unit": "ns"
          },
          {
            "name": "15/permit_release",
            "value": 49.19808455138053,
            "unit": "ns"
          },
          {
            "name": "16/choose_route",
            "value": 4.36369780059316,
            "unit": "ns"
          },
          {
            "name": "16/rotate_key",
            "value": 3.040911367572804,
            "unit": "ns"
          },
          {
            "name": "17/buffer_join",
            "value": 599.0655845244179,
            "unit": "ns"
          },
          {
            "name": "17/fold_deltas",
            "value": 2353.906392767865,
            "unit": "ns"
          },
          {
            "name": "18/wal_append",
            "value": 144711.48723723722,
            "unit": "ns"
          },
          {
            "name": "19/memory_enqueue",
            "value": 46.11502608273993,
            "unit": "ns"
          },
          {
            "name": "2/chat_turn_overhead",
            "value": 608.7426322574605,
            "unit": "ns"
          },
          {
            "name": "2/inbox_submit",
            "value": 33.20256284567854,
            "unit": "ns"
          },
          {
            "name": "20/discover_10k",
            "value": 35503.71382113821,
            "unit": "ns"
          },
          {
            "name": "21/apply_offered",
            "value": 99.37776792381098,
            "unit": "ns"
          },
          {
            "name": "21/predict_offline",
            "value": 218504.70386904763,
            "unit": "ns"
          },
          {
            "name": "22/load_render_500",
            "value": 5291421.142857143,
            "unit": "ns"
          },
          {
            "name": "3.1/registry_lookup",
            "value": 17.683669915446036,
            "unit": "ns"
          },
          {
            "name": "3.2/available_tools_50",
            "value": 23987.64800664452,
            "unit": "ns"
          },
          {
            "name": "3.3/dispatch_8_reads_concurrent",
            "value": 2141825.307189543,
            "unit": "ns"
          },
          {
            "name": "3.3/dispatch_8_reads_sequential",
            "value": 16923115.019047618,
            "unit": "ns"
          },
          {
            "name": "4.1/stdio_echo_round_trip",
            "value": 136652.86216730037,
            "unit": "ns"
          },
          {
            "name": "4.2/http_echo_round_trip",
            "value": 351120.2836283186,
            "unit": "ns"
          },
          {
            "name": "4.3/mcp_50_concurrent",
            "value": 3569251.4272727272,
            "unit": "ns"
          },
          {
            "name": "4.3/mcp_50_sequential",
            "value": 7147065.986111111,
            "unit": "ns"
          },
          {
            "name": "4.4/mcp_list_cold",
            "value": 142708.58399209485,
            "unit": "ns"
          },
          {
            "name": "4.4/mcp_list_hit",
            "value": 181.14658321480334,
            "unit": "ns"
          },
          {
            "name": "5.1/schema_tokens_1000",
            "value": 1000724.2010135136,
            "unit": "ns"
          },
          {
            "name": "5.1/schema_tokens_10000",
            "value": 44450538.233333334,
            "unit": "ns"
          },
          {
            "name": "5.2/index_build_10000",
            "value": 18960455.35,
            "unit": "ns"
          },
          {
            "name": "5.2/index_search_10000",
            "value": 190434.24921465968,
            "unit": "ns"
          },
          {
            "name": "5.3/full_schema_tokens_10000",
            "value": 28702144.833333336,
            "unit": "ns"
          },
          {
            "name": "5.3/lazy_visible_prompt_10000",
            "value": 12423.883233089262,
            "unit": "ns"
          },
          {
            "name": "5.4/routed_search_10000",
            "value": 19082.706975794696,
            "unit": "ns"
          },
          {
            "name": "5.4/shrink_schemas_10000",
            "value": 67438545.75,
            "unit": "ns"
          },
          {
            "name": "5.5/agent_filter_10000",
            "value": 88451.39452913278,
            "unit": "ns"
          },
          {
            "name": "5.5/cache_cold_10000",
            "value": 26543534.666666664,
            "unit": "ns"
          },
          {
            "name": "5.5/cache_hit_10000",
            "value": 12.411894169869651,
            "unit": "ns"
          },
          {
            "name": "6.1/gate_check",
            "value": 2.4574424025294364,
            "unit": "ns"
          },
          {
            "name": "6.1/live_tool_body",
            "value": 1486.8573447135213,
            "unit": "ns"
          },
          {
            "name": "6.1/loop_8_reads",
            "value": 2147770.8300653594,
            "unit": "ns"
          },
          {
            "name": "6.1/loop_iteration",
            "value": 286.3150811510325,
            "unit": "ns"
          },
          {
            "name": "6.10/budget_notice",
            "value": 63.898711011247904,
            "unit": "ns"
          },
          {
            "name": "6.10/cancel_select",
            "value": 91.57467661942931,
            "unit": "ns"
          },
          {
            "name": "6.10/canonical_args",
            "value": 930.3527372693976,
            "unit": "ns"
          },
          {
            "name": "6.10/gate_check",
            "value": 5.66536071750826,
            "unit": "ns"
          },
          {
            "name": "6.10/live_tool_body",
            "value": 1604.4143510258696,
            "unit": "ns"
          },
          {
            "name": "6.10/loop_8_reads",
            "value": 2189028.021008403,
            "unit": "ns"
          },
          {
            "name": "6.10/loop_iteration",
            "value": 592.6776232735288,
            "unit": "ns"
          },
          {
            "name": "6.10/outcome_label",
            "value": 7.656814647442085,
            "unit": "ns"
          },
          {
            "name": "6.10/stall_observe",
            "value": 112.66012237493558,
            "unit": "ns"
          },
          {
            "name": "6.10/steer_append",
            "value": 45.57605793163038,
            "unit": "ns"
          },
          {
            "name": "6.10/stress_repair",
            "value": 32409.294669335708,
            "unit": "ns"
          },
          {
            "name": "6.10/truncate_output",
            "value": 3505.6280580249468,
            "unit": "ns"
          },
          {
            "name": "6.10/validate_repair",
            "value": 199.0577018903415,
            "unit": "ns"
          },
          {
            "name": "6.2/cancel_select",
            "value": 90.82109836220641,
            "unit": "ns"
          },
          {
            "name": "6.2/gate_check",
            "value": 5.67190596583686,
            "unit": "ns"
          },
          {
            "name": "6.2/live_tool_body",
            "value": 1490.673130517922,
            "unit": "ns"
          },
          {
            "name": "6.2/loop_8_reads",
            "value": 2153635.3194444445,
            "unit": "ns"
          },
          {
            "name": "6.2/loop_iteration",
            "value": 401.9684171424756,
            "unit": "ns"
          },
          {
            "name": "6.3/cancel_select",
            "value": 91.02946416641245,
            "unit": "ns"
          },
          {
            "name": "6.3/gate_check",
            "value": 5.718847479513727,
            "unit": "ns"
          },
          {
            "name": "6.3/live_tool_body",
            "value": 1533.0715486790182,
            "unit": "ns"
          },
          {
            "name": "6.3/loop_8_reads",
            "value": 2180816.8406862747,
            "unit": "ns"
          },
          {
            "name": "6.3/loop_iteration",
            "value": 428.7644747112806,
            "unit": "ns"
          },
          {
            "name": "6.3/validate_repair",
            "value": 200.73141678303224,
            "unit": "ns"
          },
          {
            "name": "6.4/cancel_select",
            "value": 92.96229940255982,
            "unit": "ns"
          },
          {
            "name": "6.4/gate_check",
            "value": 5.6734269056529385,
            "unit": "ns"
          },
          {
            "name": "6.4/live_tool_body",
            "value": 1536.1566346522425,
            "unit": "ns"
          },
          {
            "name": "6.4/loop_8_reads",
            "value": 2190631.1531862747,
            "unit": "ns"
          },
          {
            "name": "6.4/loop_iteration",
            "value": 426.89994652406415,
            "unit": "ns"
          },
          {
            "name": "6.4/stall_observe",
            "value": 116.64583222697865,
            "unit": "ns"
          },
          {
            "name": "6.4/validate_repair",
            "value": 200.44109345447208,
            "unit": "ns"
          },
          {
            "name": "6.5/budget_notice",
            "value": 61.507915215986415,
            "unit": "ns"
          },
          {
            "name": "6.5/cancel_select",
            "value": 90.51496490792657,
            "unit": "ns"
          },
          {
            "name": "6.5/gate_check",
            "value": 5.674603044340907,
            "unit": "ns"
          },
          {
            "name": "6.5/live_tool_body",
            "value": 1506.7835531257824,
            "unit": "ns"
          },
          {
            "name": "6.5/loop_8_reads",
            "value": 2185702.930147059,
            "unit": "ns"
          },
          {
            "name": "6.5/loop_iteration",
            "value": 463.23073658374454,
            "unit": "ns"
          },
          {
            "name": "6.5/stall_observe",
            "value": 114.24005992108951,
            "unit": "ns"
          },
          {
            "name": "6.5/validate_repair",
            "value": 201.7054819063256,
            "unit": "ns"
          },
          {
            "name": "6.6/budget_notice",
            "value": 62.9338031200072,
            "unit": "ns"
          },
          {
            "name": "6.6/cancel_select",
            "value": 90.47509089168321,
            "unit": "ns"
          },
          {
            "name": "6.6/gate_check",
            "value": 5.674910385477989,
            "unit": "ns"
          },
          {
            "name": "6.6/live_tool_body",
            "value": 1539.8599292920583,
            "unit": "ns"
          },
          {
            "name": "6.6/loop_8_reads",
            "value": 2189699.2084967317,
            "unit": "ns"
          },
          {
            "name": "6.6/loop_iteration",
            "value": 454.5757189601371,
            "unit": "ns"
          },
          {
            "name": "6.6/stall_observe",
            "value": 118.61888668313048,
            "unit": "ns"
          },
          {
            "name": "6.6/truncate_output",
            "value": 3580.205584518676,
            "unit": "ns"
          },
          {
            "name": "6.6/validate_repair",
            "value": 199.90434466574146,
            "unit": "ns"
          },
          {
            "name": "6.7/budget_notice",
            "value": 61.8806142182962,
            "unit": "ns"
          },
          {
            "name": "6.7/cancel_select",
            "value": 90.85666276822741,
            "unit": "ns"
          },
          {
            "name": "6.7/gate_check",
            "value": 6.134490674996063,
            "unit": "ns"
          },
          {
            "name": "6.7/live_tool_body",
            "value": 1408.7427791024938,
            "unit": "ns"
          },
          {
            "name": "6.7/loop_8_reads",
            "value": 2160841.7009803923,
            "unit": "ns"
          },
          {
            "name": "6.7/loop_iteration",
            "value": 455.80720013852056,
            "unit": "ns"
          },
          {
            "name": "6.7/stall_observe",
            "value": 114.36302867850384,
            "unit": "ns"
          },
          {
            "name": "6.7/steer_append",
            "value": 45.553356293962935,
            "unit": "ns"
          },
          {
            "name": "6.7/truncate_output",
            "value": 3445.126661302537,
            "unit": "ns"
          },
          {
            "name": "6.7/validate_repair",
            "value": 201.3825607125076,
            "unit": "ns"
          },
          {
            "name": "6.8/budget_notice",
            "value": 61.556943507711956,
            "unit": "ns"
          },
          {
            "name": "6.8/cancel_select",
            "value": 91.33986691916266,
            "unit": "ns"
          },
          {
            "name": "6.8/canonical_args",
            "value": 862.4205059467324,
            "unit": "ns"
          },
          {
            "name": "6.8/gate_check",
            "value": 5.684775182998109,
            "unit": "ns"
          },
          {
            "name": "6.8/live_tool_body",
            "value": 1527.6381889680765,
            "unit": "ns"
          },
          {
            "name": "6.8/loop_8_reads",
            "value": 2203730.8039215687,
            "unit": "ns"
          },
          {
            "name": "6.8/loop_iteration",
            "value": 531.0635453551913,
            "unit": "ns"
          },
          {
            "name": "6.8/stall_observe",
            "value": 115.21343056030125,
            "unit": "ns"
          },
          {
            "name": "6.8/steer_append",
            "value": 45.59076277683188,
            "unit": "ns"
          },
          {
            "name": "6.8/truncate_output",
            "value": 3477.078956180635,
            "unit": "ns"
          },
          {
            "name": "6.8/validate_repair",
            "value": 205.64266525414126,
            "unit": "ns"
          },
          {
            "name": "6.9/budget_notice",
            "value": 67.12745086558542,
            "unit": "ns"
          },
          {
            "name": "6.9/cancel_select",
            "value": 92.24817961833179,
            "unit": "ns"
          },
          {
            "name": "6.9/canonical_args",
            "value": 862.8937314184329,
            "unit": "ns"
          },
          {
            "name": "6.9/gate_check",
            "value": 5.664277689380247,
            "unit": "ns"
          },
          {
            "name": "6.9/live_tool_body",
            "value": 1586.400775160353,
            "unit": "ns"
          },
          {
            "name": "6.9/loop_8_reads",
            "value": 2175010.863970588,
            "unit": "ns"
          },
          {
            "name": "6.9/loop_iteration",
            "value": 578.1033248151975,
            "unit": "ns"
          },
          {
            "name": "6.9/outcome_label",
            "value": 7.658072716755174,
            "unit": "ns"
          },
          {
            "name": "6.9/stall_observe",
            "value": 116.01912189827178,
            "unit": "ns"
          },
          {
            "name": "6.9/steer_append",
            "value": 46.23144409297164,
            "unit": "ns"
          },
          {
            "name": "6.9/truncate_output",
            "value": 3413.8208710395475,
            "unit": "ns"
          },
          {
            "name": "6.9/validate_repair",
            "value": 201.0974355917133,
            "unit": "ns"
          },
          {
            "name": "gw5/echo_through_gate",
            "value": 37.48547545689156,
            "unit": "ns"
          },
          {
            "name": "gw5/eight_concurrent_sleeps",
            "value": 2133809.8953703702,
            "unit": "ns"
          },
          {
            "name": "gw7/catalog_arc_swap",
            "value": 1978.3591772720736,
            "unit": "ns"
          },
          {
            "name": "gw7/catalog_reload_10k",
            "value": 22973029.979166664,
            "unit": "ns"
          },
          {
            "name": "gw7/catalog_search_10k",
            "value": 196353.9466845878,
            "unit": "ns"
          },
          {
            "name": "gw8/dynamic_rename_task",
            "value": 4502624.111111111,
            "unit": "ns"
          },
          {
            "name": "gw8/every_tool_rename_task",
            "value": 6397394.416666666,
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
          "id": "d03aec6501583b37a37d959a266fe6238926585a",
          "message": "Chapters 21–30: skills, reflection, and team patterns",
          "timestamp": "2026-10-15T04:44:13Z",
          "url": "https://github.com/agnathan/rust-agent-harness/pull/6/commits/d03aec6501583b37a37d959a266fe6238926585a"
        },
        "date": 1792199913457,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "1.1/serde_50_message_request",
            "value": 10413.598184912082,
            "unit": "ns"
          },
          {
            "name": "1.2/mock_100_concurrent",
            "value": 77459.39270613107,
            "unit": "ns"
          },
          {
            "name": "1.2/mock_turn_overhead",
            "value": 249.03757311151577,
            "unit": "ns"
          },
          {
            "name": "1.3/build_request_and_parse_response",
            "value": 2988.5804428094007,
            "unit": "ns"
          },
          {
            "name": "1.4/parse_response_edges",
            "value": 1143.0019657631826,
            "unit": "ns"
          },
          {
            "name": "10/scripted_turn",
            "value": 8.106263444041254,
            "unit": "ns"
          },
          {
            "name": "10/turn_transition",
            "value": 1.0902892632739127,
            "unit": "ns"
          },
          {
            "name": "11/skip_decision",
            "value": 251.07019559648273,
            "unit": "ns"
          },
          {
            "name": "12/api_request_hook",
            "value": 2.1793492654506226,
            "unit": "ns"
          },
          {
            "name": "12/empty_pre_post",
            "value": 10.897101525285867,
            "unit": "ns"
          },
          {
            "name": "12/memory_enqueue",
            "value": 54.3194263114127,
            "unit": "ns"
          },
          {
            "name": "13/checkpoint_1000",
            "value": 50679.372156453115,
            "unit": "ns"
          },
          {
            "name": "13/rewind_1000",
            "value": 81281.0117768095,
            "unit": "ns"
          },
          {
            "name": "13/warm_accept",
            "value": 28334.195720040283,
            "unit": "ns"
          },
          {
            "name": "14/empty_budget",
            "value": 0.9053248819561073,
            "unit": "ns"
          },
          {
            "name": "14/repetition_guard",
            "value": 2582.8134779185457,
            "unit": "ns"
          },
          {
            "name": "15/open_breaker",
            "value": 30.13282200846423,
            "unit": "ns"
          },
          {
            "name": "15/permit_release",
            "value": 59.12912975571662,
            "unit": "ns"
          },
          {
            "name": "16/choose_route",
            "value": 5.917905735337251,
            "unit": "ns"
          },
          {
            "name": "16/rotate_key",
            "value": 3.44528751597894,
            "unit": "ns"
          },
          {
            "name": "17/buffer_join",
            "value": 786.1523596391946,
            "unit": "ns"
          },
          {
            "name": "17/fold_deltas",
            "value": 2835.2237089247647,
            "unit": "ns"
          },
          {
            "name": "18/wal_append",
            "value": 156810.6327724359,
            "unit": "ns"
          },
          {
            "name": "19/memory_enqueue",
            "value": 55.18729705974822,
            "unit": "ns"
          },
          {
            "name": "2/chat_turn_overhead",
            "value": 677.103220528935,
            "unit": "ns"
          },
          {
            "name": "2/inbox_submit",
            "value": 40.29555806102316,
            "unit": "ns"
          },
          {
            "name": "20/discover_10k",
            "value": 61806.7083535109,
            "unit": "ns"
          },
          {
            "name": "21/apply_offered",
            "value": 112.82428612949452,
            "unit": "ns"
          },
          {
            "name": "21/predict_offline",
            "value": 290650.89894179895,
            "unit": "ns"
          },
          {
            "name": "3.1/registry_lookup",
            "value": 26.94924711356463,
            "unit": "ns"
          },
          {
            "name": "3.2/available_tools_50",
            "value": 30732.374116161616,
            "unit": "ns"
          },
          {
            "name": "3.3/dispatch_8_reads_concurrent",
            "value": 2134485.269444444,
            "unit": "ns"
          },
          {
            "name": "3.3/dispatch_8_reads_sequential",
            "value": 16723041.388888888,
            "unit": "ns"
          },
          {
            "name": "4.1/stdio_echo_round_trip",
            "value": 202426.08314285713,
            "unit": "ns"
          },
          {
            "name": "4.2/http_echo_round_trip",
            "value": 408042.0276388889,
            "unit": "ns"
          },
          {
            "name": "4.3/mcp_50_concurrent",
            "value": 4966018.027083334,
            "unit": "ns"
          },
          {
            "name": "4.3/mcp_50_sequential",
            "value": 10189738.458333332,
            "unit": "ns"
          },
          {
            "name": "4.4/mcp_list_cold",
            "value": 212576.04247076024,
            "unit": "ns"
          },
          {
            "name": "4.4/mcp_list_hit",
            "value": 253.92697136890754,
            "unit": "ns"
          },
          {
            "name": "5.1/schema_tokens_1000",
            "value": 1251781.36875,
            "unit": "ns"
          },
          {
            "name": "5.1/schema_tokens_10000",
            "value": 39873092.91666667,
            "unit": "ns"
          },
          {
            "name": "5.2/index_build_10000",
            "value": 20488527.4375,
            "unit": "ns"
          },
          {
            "name": "5.2/index_search_10000",
            "value": 331076.921969697,
            "unit": "ns"
          },
          {
            "name": "5.3/full_schema_tokens_10000",
            "value": 18727255.221428573,
            "unit": "ns"
          },
          {
            "name": "5.3/lazy_visible_prompt_10000",
            "value": 15429.12227481354,
            "unit": "ns"
          },
          {
            "name": "5.4/routed_search_10000",
            "value": 27243.29880680376,
            "unit": "ns"
          },
          {
            "name": "5.4/shrink_schemas_10000",
            "value": 57463752.1,
            "unit": "ns"
          },
          {
            "name": "5.5/agent_filter_10000",
            "value": 122735.62372881356,
            "unit": "ns"
          },
          {
            "name": "5.5/cache_cold_10000",
            "value": 22720638.111111112,
            "unit": "ns"
          },
          {
            "name": "5.5/cache_hit_10000",
            "value": 13.805313506568847,
            "unit": "ns"
          },
          {
            "name": "6.1/gate_check",
            "value": 3.4233015777573943,
            "unit": "ns"
          },
          {
            "name": "6.1/live_tool_body",
            "value": 1815.9099056368266,
            "unit": "ns"
          },
          {
            "name": "6.1/loop_8_reads",
            "value": 2143700,
            "unit": "ns"
          },
          {
            "name": "6.1/loop_iteration",
            "value": 363.0508111205693,
            "unit": "ns"
          },
          {
            "name": "6.10/budget_notice",
            "value": 76.32651720271936,
            "unit": "ns"
          },
          {
            "name": "6.10/cancel_select",
            "value": 94.64711069381039,
            "unit": "ns"
          },
          {
            "name": "6.10/canonical_args",
            "value": 1067.5989301430955,
            "unit": "ns"
          },
          {
            "name": "6.10/gate_check",
            "value": 7.473266708300889,
            "unit": "ns"
          },
          {
            "name": "6.10/live_tool_body",
            "value": 1721.9953614689484,
            "unit": "ns"
          },
          {
            "name": "6.10/loop_8_reads",
            "value": 2180331.711764706,
            "unit": "ns"
          },
          {
            "name": "6.10/loop_iteration",
            "value": 782.4878958065528,
            "unit": "ns"
          },
          {
            "name": "6.10/outcome_label",
            "value": 8.398605943182002,
            "unit": "ns"
          },
          {
            "name": "6.10/stall_observe",
            "value": 138.07632233394793,
            "unit": "ns"
          },
          {
            "name": "6.10/steer_append",
            "value": 60.75838638116491,
            "unit": "ns"
          },
          {
            "name": "6.10/stress_repair",
            "value": 51546.31968553459,
            "unit": "ns"
          },
          {
            "name": "6.10/truncate_output",
            "value": 4563.959590710343,
            "unit": "ns"
          },
          {
            "name": "6.10/validate_repair",
            "value": 276.366053239685,
            "unit": "ns"
          },
          {
            "name": "6.2/cancel_select",
            "value": 95.94853485414347,
            "unit": "ns"
          },
          {
            "name": "6.2/gate_check",
            "value": 7.471291896062276,
            "unit": "ns"
          },
          {
            "name": "6.2/live_tool_body",
            "value": 1780.6519942408045,
            "unit": "ns"
          },
          {
            "name": "6.2/loop_8_reads",
            "value": 2160954.649509804,
            "unit": "ns"
          },
          {
            "name": "6.2/loop_iteration",
            "value": 480.6567863221363,
            "unit": "ns"
          },
          {
            "name": "6.3/cancel_select",
            "value": 98.09833732430374,
            "unit": "ns"
          },
          {
            "name": "6.3/gate_check",
            "value": 7.478344838441533,
            "unit": "ns"
          },
          {
            "name": "6.3/live_tool_body",
            "value": 1702.6327877825968,
            "unit": "ns"
          },
          {
            "name": "6.3/loop_8_reads",
            "value": 2167906.382352941,
            "unit": "ns"
          },
          {
            "name": "6.3/loop_iteration",
            "value": 564.6319700533082,
            "unit": "ns"
          },
          {
            "name": "6.3/validate_repair",
            "value": 261.53205847598315,
            "unit": "ns"
          },
          {
            "name": "6.4/cancel_select",
            "value": 96.31054778189119,
            "unit": "ns"
          },
          {
            "name": "6.4/gate_check",
            "value": 7.477423536416936,
            "unit": "ns"
          },
          {
            "name": "6.4/live_tool_body",
            "value": 1865.3867825877578,
            "unit": "ns"
          },
          {
            "name": "6.4/loop_8_reads",
            "value": 2175346.5130718956,
            "unit": "ns"
          },
          {
            "name": "6.4/loop_iteration",
            "value": 536.1696157877761,
            "unit": "ns"
          },
          {
            "name": "6.4/stall_observe",
            "value": 140.9386125399868,
            "unit": "ns"
          },
          {
            "name": "6.4/validate_repair",
            "value": 266.87430552495476,
            "unit": "ns"
          },
          {
            "name": "6.5/budget_notice",
            "value": 77.55038805594322,
            "unit": "ns"
          },
          {
            "name": "6.5/cancel_select",
            "value": 96.56730043661646,
            "unit": "ns"
          },
          {
            "name": "6.5/gate_check",
            "value": 7.476134540033222,
            "unit": "ns"
          },
          {
            "name": "6.5/live_tool_body",
            "value": 1765.4850395228166,
            "unit": "ns"
          },
          {
            "name": "6.5/loop_8_reads",
            "value": 2168054.875,
            "unit": "ns"
          },
          {
            "name": "6.5/loop_iteration",
            "value": 606.9914117015567,
            "unit": "ns"
          },
          {
            "name": "6.5/stall_observe",
            "value": 137.13841056959245,
            "unit": "ns"
          },
          {
            "name": "6.5/validate_repair",
            "value": 269.5523084042993,
            "unit": "ns"
          },
          {
            "name": "6.6/budget_notice",
            "value": 83.71067111795139,
            "unit": "ns"
          },
          {
            "name": "6.6/cancel_select",
            "value": 94.95178840601248,
            "unit": "ns"
          },
          {
            "name": "6.6/gate_check",
            "value": 7.4703116184572504,
            "unit": "ns"
          },
          {
            "name": "6.6/live_tool_body",
            "value": 1703.173166428072,
            "unit": "ns"
          },
          {
            "name": "6.6/loop_8_reads",
            "value": 2178482.519607843,
            "unit": "ns"
          },
          {
            "name": "6.6/loop_iteration",
            "value": 625.0439001530551,
            "unit": "ns"
          },
          {
            "name": "6.6/stall_observe",
            "value": 142.17844660479324,
            "unit": "ns"
          },
          {
            "name": "6.6/truncate_output",
            "value": 4591.791576845042,
            "unit": "ns"
          },
          {
            "name": "6.6/validate_repair",
            "value": 266.61610006259355,
            "unit": "ns"
          },
          {
            "name": "6.7/budget_notice",
            "value": 77.57749735024936,
            "unit": "ns"
          },
          {
            "name": "6.7/cancel_select",
            "value": 100.97035159844671,
            "unit": "ns"
          },
          {
            "name": "6.7/gate_check",
            "value": 7.480980630054901,
            "unit": "ns"
          },
          {
            "name": "6.7/live_tool_body",
            "value": 1609.0576053397444,
            "unit": "ns"
          },
          {
            "name": "6.7/loop_8_reads",
            "value": 2176197.4025210086,
            "unit": "ns"
          },
          {
            "name": "6.7/loop_iteration",
            "value": 617.2821538216347,
            "unit": "ns"
          },
          {
            "name": "6.7/stall_observe",
            "value": 139.71864194104245,
            "unit": "ns"
          },
          {
            "name": "6.7/steer_append",
            "value": 58.350742075848586,
            "unit": "ns"
          },
          {
            "name": "6.7/truncate_output",
            "value": 4586.0084069122095,
            "unit": "ns"
          },
          {
            "name": "6.7/validate_repair",
            "value": 267.78324394807794,
            "unit": "ns"
          },
          {
            "name": "6.8/budget_notice",
            "value": 84.04871928558643,
            "unit": "ns"
          },
          {
            "name": "6.8/cancel_select",
            "value": 95.08703865541457,
            "unit": "ns"
          },
          {
            "name": "6.8/canonical_args",
            "value": 1056.2904561131734,
            "unit": "ns"
          },
          {
            "name": "6.8/gate_check",
            "value": 7.468762520380098,
            "unit": "ns"
          },
          {
            "name": "6.8/live_tool_body",
            "value": 1779.4252952623629,
            "unit": "ns"
          },
          {
            "name": "6.8/loop_8_reads",
            "value": 2182684.7565359473,
            "unit": "ns"
          },
          {
            "name": "6.8/loop_iteration",
            "value": 698.7558882286871,
            "unit": "ns"
          },
          {
            "name": "6.8/stall_observe",
            "value": 137.84411081153058,
            "unit": "ns"
          },
          {
            "name": "6.8/steer_append",
            "value": 57.93989181607377,
            "unit": "ns"
          },
          {
            "name": "6.8/truncate_output",
            "value": 4543.594095786881,
            "unit": "ns"
          },
          {
            "name": "6.8/validate_repair",
            "value": 258.12334624499704,
            "unit": "ns"
          },
          {
            "name": "6.9/budget_notice",
            "value": 77.67978724443148,
            "unit": "ns"
          },
          {
            "name": "6.9/cancel_select",
            "value": 95.14263505726404,
            "unit": "ns"
          },
          {
            "name": "6.9/canonical_args",
            "value": 1066.4072475329433,
            "unit": "ns"
          },
          {
            "name": "6.9/gate_check",
            "value": 7.469521069236297,
            "unit": "ns"
          },
          {
            "name": "6.9/live_tool_body",
            "value": 1767.8805445556582,
            "unit": "ns"
          },
          {
            "name": "6.9/loop_8_reads",
            "value": 2175965.249579832,
            "unit": "ns"
          },
          {
            "name": "6.9/loop_iteration",
            "value": 740.513942560862,
            "unit": "ns"
          },
          {
            "name": "6.9/outcome_label",
            "value": 8.402210682636547,
            "unit": "ns"
          },
          {
            "name": "6.9/stall_observe",
            "value": 135.5616865342775,
            "unit": "ns"
          },
          {
            "name": "6.9/steer_append",
            "value": 59.19210241054897,
            "unit": "ns"
          },
          {
            "name": "6.9/truncate_output",
            "value": 4661.466967871485,
            "unit": "ns"
          },
          {
            "name": "6.9/validate_repair",
            "value": 260.160295153967,
            "unit": "ns"
          },
          {
            "name": "gw5/echo_through_gate",
            "value": 45.91177678098113,
            "unit": "ns"
          },
          {
            "name": "gw5/eight_concurrent_sleeps",
            "value": 2117642.0059523806,
            "unit": "ns"
          },
          {
            "name": "gw7/catalog_arc_swap",
            "value": 2406.3253335091536,
            "unit": "ns"
          },
          {
            "name": "gw7/catalog_reload_10k",
            "value": 26444600.777777776,
            "unit": "ns"
          },
          {
            "name": "gw7/catalog_search_10k",
            "value": 297180.98238482384,
            "unit": "ns"
          },
          {
            "name": "gw8/dynamic_rename_task",
            "value": 5731260.451020408,
            "unit": "ns"
          },
          {
            "name": "gw8/every_tool_rename_task",
            "value": 8392621.83,
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
          "id": "22760551961386f6511e9d2d56bfbe0d6d23f2c5",
          "message": "Chapters 21–30: skills, reflection, and team patterns",
          "timestamp": "2026-10-15T04:44:13Z",
          "url": "https://github.com/agnathan/rust-agent-harness/pull/6/commits/22760551961386f6511e9d2d56bfbe0d6d23f2c5"
        },
        "date": 1792200107289,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "1.1/serde_50_message_request",
            "value": 10492.704158387738,
            "unit": "ns"
          },
          {
            "name": "1.2/mock_100_concurrent",
            "value": 78079.57291666666,
            "unit": "ns"
          },
          {
            "name": "1.2/mock_turn_overhead",
            "value": 250.10997857274845,
            "unit": "ns"
          },
          {
            "name": "1.3/build_request_and_parse_response",
            "value": 2942.647792137857,
            "unit": "ns"
          },
          {
            "name": "1.4/parse_response_edges",
            "value": 1135.73009175795,
            "unit": "ns"
          },
          {
            "name": "10/scripted_turn",
            "value": 7.8498095908261325,
            "unit": "ns"
          },
          {
            "name": "10/turn_transition",
            "value": 1.096564775604601,
            "unit": "ns"
          },
          {
            "name": "11/skip_decision",
            "value": 251.1342220440808,
            "unit": "ns"
          },
          {
            "name": "12/api_request_hook",
            "value": 2.1780130444337344,
            "unit": "ns"
          },
          {
            "name": "12/empty_pre_post",
            "value": 10.903209022124418,
            "unit": "ns"
          },
          {
            "name": "12/memory_enqueue",
            "value": 53.701825073201675,
            "unit": "ns"
          },
          {
            "name": "13/checkpoint_1000",
            "value": 51693.75808709176,
            "unit": "ns"
          },
          {
            "name": "13/rewind_1000",
            "value": 83859.12725779967,
            "unit": "ns"
          },
          {
            "name": "13/warm_accept",
            "value": 29094.04393747845,
            "unit": "ns"
          },
          {
            "name": "14/empty_budget",
            "value": 0.9035937993641465,
            "unit": "ns"
          },
          {
            "name": "14/repetition_guard",
            "value": 2571.4413968613035,
            "unit": "ns"
          },
          {
            "name": "15/open_breaker",
            "value": 30.004006690312373,
            "unit": "ns"
          },
          {
            "name": "15/permit_release",
            "value": 58.795714929464225,
            "unit": "ns"
          },
          {
            "name": "16/choose_route",
            "value": 6.09591236287967,
            "unit": "ns"
          },
          {
            "name": "16/rotate_key",
            "value": 3.4460456990982635,
            "unit": "ns"
          },
          {
            "name": "17/buffer_join",
            "value": 679.3578856019415,
            "unit": "ns"
          },
          {
            "name": "17/fold_deltas",
            "value": 2835.2851947292174,
            "unit": "ns"
          },
          {
            "name": "18/wal_append",
            "value": 143619.62291666667,
            "unit": "ns"
          },
          {
            "name": "19/memory_enqueue",
            "value": 57.7261252800122,
            "unit": "ns"
          },
          {
            "name": "2/chat_turn_overhead",
            "value": 664.2255294906412,
            "unit": "ns"
          },
          {
            "name": "2/inbox_submit",
            "value": 40.4659627314482,
            "unit": "ns"
          },
          {
            "name": "20/discover_10k",
            "value": 60171.020289256194,
            "unit": "ns"
          },
          {
            "name": "21/apply_offered",
            "value": 112.59500688669752,
            "unit": "ns"
          },
          {
            "name": "21/predict_offline",
            "value": 301026.58978494623,
            "unit": "ns"
          },
          {
            "name": "22/load_render_500",
            "value": 5855240.416099774,
            "unit": "ns"
          },
          {
            "name": "23/enqueue_unsolicited",
            "value": 19.013454828361713,
            "unit": "ns"
          },
          {
            "name": "23/scan_one_skill",
            "value": 285.15766437932353,
            "unit": "ns"
          },
          {
            "name": "3.1/registry_lookup",
            "value": 25.708406826012776,
            "unit": "ns"
          },
          {
            "name": "3.2/available_tools_50",
            "value": 30733.152320675104,
            "unit": "ns"
          },
          {
            "name": "3.3/dispatch_8_reads_concurrent",
            "value": 2134906.669117647,
            "unit": "ns"
          },
          {
            "name": "3.3/dispatch_8_reads_sequential",
            "value": 16771677.233333332,
            "unit": "ns"
          },
          {
            "name": "4.1/stdio_echo_round_trip",
            "value": 202209.09565903712,
            "unit": "ns"
          },
          {
            "name": "4.2/http_echo_round_trip",
            "value": 410030.1053370787,
            "unit": "ns"
          },
          {
            "name": "4.3/mcp_50_concurrent",
            "value": 4918057.170634921,
            "unit": "ns"
          },
          {
            "name": "4.3/mcp_50_sequential",
            "value": 10299262.8625,
            "unit": "ns"
          },
          {
            "name": "4.4/mcp_list_cold",
            "value": 210471.7130398671,
            "unit": "ns"
          },
          {
            "name": "4.4/mcp_list_hit",
            "value": 255.0273405828375,
            "unit": "ns"
          },
          {
            "name": "5.1/schema_tokens_1000",
            "value": 1254245.1330049261,
            "unit": "ns"
          },
          {
            "name": "5.1/schema_tokens_10000",
            "value": 40667745.54166667,
            "unit": "ns"
          },
          {
            "name": "5.2/index_build_10000",
            "value": 20826932.635416664,
            "unit": "ns"
          },
          {
            "name": "5.2/index_search_10000",
            "value": 334256.61926605506,
            "unit": "ns"
          },
          {
            "name": "5.3/full_schema_tokens_10000",
            "value": 22761523.875,
            "unit": "ns"
          },
          {
            "name": "5.3/lazy_visible_prompt_10000",
            "value": 15547.065179934034,
            "unit": "ns"
          },
          {
            "name": "5.4/routed_search_10000",
            "value": 25781.734079071968,
            "unit": "ns"
          },
          {
            "name": "5.4/shrink_schemas_10000",
            "value": 61068100.37222222,
            "unit": "ns"
          },
          {
            "name": "5.5/agent_filter_10000",
            "value": 124592.76700796359,
            "unit": "ns"
          },
          {
            "name": "5.5/cache_cold_10000",
            "value": 25526703.611111112,
            "unit": "ns"
          },
          {
            "name": "5.5/cache_hit_10000",
            "value": 13.805087192686418,
            "unit": "ns"
          },
          {
            "name": "6.1/gate_check",
            "value": 3.4291879765669684,
            "unit": "ns"
          },
          {
            "name": "6.1/live_tool_body",
            "value": 1726.9984782814645,
            "unit": "ns"
          },
          {
            "name": "6.1/loop_8_reads",
            "value": 2154345.573529412,
            "unit": "ns"
          },
          {
            "name": "6.1/loop_iteration",
            "value": 365.29132439228835,
            "unit": "ns"
          },
          {
            "name": "6.10/budget_notice",
            "value": 78.0821514572694,
            "unit": "ns"
          },
          {
            "name": "6.10/cancel_select",
            "value": 95.04193044546362,
            "unit": "ns"
          },
          {
            "name": "6.10/canonical_args",
            "value": 1056.7506456535775,
            "unit": "ns"
          },
          {
            "name": "6.10/gate_check",
            "value": 7.478658095198533,
            "unit": "ns"
          },
          {
            "name": "6.10/live_tool_body",
            "value": 1716.588908257169,
            "unit": "ns"
          },
          {
            "name": "6.10/loop_8_reads",
            "value": 2183389.2872549016,
            "unit": "ns"
          },
          {
            "name": "6.10/loop_iteration",
            "value": 756.1188966555321,
            "unit": "ns"
          },
          {
            "name": "6.10/outcome_label",
            "value": 8.403065507437585,
            "unit": "ns"
          },
          {
            "name": "6.10/stall_observe",
            "value": 138.46877417190456,
            "unit": "ns"
          },
          {
            "name": "6.10/steer_append",
            "value": 60.13007027963294,
            "unit": "ns"
          },
          {
            "name": "6.10/stress_repair",
            "value": 50627.526617034906,
            "unit": "ns"
          },
          {
            "name": "6.10/truncate_output",
            "value": 4757.506392434194,
            "unit": "ns"
          },
          {
            "name": "6.10/validate_repair",
            "value": 276.798065680978,
            "unit": "ns"
          },
          {
            "name": "6.2/cancel_select",
            "value": 94.71605644798476,
            "unit": "ns"
          },
          {
            "name": "6.2/gate_check",
            "value": 7.47310051197098,
            "unit": "ns"
          },
          {
            "name": "6.2/live_tool_body",
            "value": 1786.74660270992,
            "unit": "ns"
          },
          {
            "name": "6.2/loop_8_reads",
            "value": 2158701.5625,
            "unit": "ns"
          },
          {
            "name": "6.2/loop_iteration",
            "value": 498.5584466180858,
            "unit": "ns"
          },
          {
            "name": "6.3/cancel_select",
            "value": 97.90548744015182,
            "unit": "ns"
          },
          {
            "name": "6.3/gate_check",
            "value": 7.476631037952901,
            "unit": "ns"
          },
          {
            "name": "6.3/live_tool_body",
            "value": 1728.7206395256667,
            "unit": "ns"
          },
          {
            "name": "6.3/loop_8_reads",
            "value": 2168815.7977941176,
            "unit": "ns"
          },
          {
            "name": "6.3/loop_iteration",
            "value": 547.0580596962275,
            "unit": "ns"
          },
          {
            "name": "6.3/validate_repair",
            "value": 261.3098495512958,
            "unit": "ns"
          },
          {
            "name": "6.4/cancel_select",
            "value": 95.32523190190275,
            "unit": "ns"
          },
          {
            "name": "6.4/gate_check",
            "value": 7.480875235287197,
            "unit": "ns"
          },
          {
            "name": "6.4/live_tool_body",
            "value": 1778.0131163643864,
            "unit": "ns"
          },
          {
            "name": "6.4/loop_8_reads",
            "value": 2170824.3664799253,
            "unit": "ns"
          },
          {
            "name": "6.4/loop_iteration",
            "value": 498.0880519651687,
            "unit": "ns"
          },
          {
            "name": "6.4/stall_observe",
            "value": 138.45217451821483,
            "unit": "ns"
          },
          {
            "name": "6.4/validate_repair",
            "value": 263.2020153250054,
            "unit": "ns"
          },
          {
            "name": "6.5/budget_notice",
            "value": 76.45836936598448,
            "unit": "ns"
          },
          {
            "name": "6.5/cancel_select",
            "value": 96.02880163917752,
            "unit": "ns"
          },
          {
            "name": "6.5/gate_check",
            "value": 7.468201264957595,
            "unit": "ns"
          },
          {
            "name": "6.5/live_tool_body",
            "value": 1726.5367076631978,
            "unit": "ns"
          },
          {
            "name": "6.5/loop_8_reads",
            "value": 2174661.980392157,
            "unit": "ns"
          },
          {
            "name": "6.5/loop_iteration",
            "value": 597.9752735052535,
            "unit": "ns"
          },
          {
            "name": "6.5/stall_observe",
            "value": 136.49885202233543,
            "unit": "ns"
          },
          {
            "name": "6.5/validate_repair",
            "value": 291.0378186666667,
            "unit": "ns"
          },
          {
            "name": "6.6/budget_notice",
            "value": 78.63579311078394,
            "unit": "ns"
          },
          {
            "name": "6.6/cancel_select",
            "value": 99.8016537311555,
            "unit": "ns"
          },
          {
            "name": "6.6/gate_check",
            "value": 7.471343197479236,
            "unit": "ns"
          },
          {
            "name": "6.6/live_tool_body",
            "value": 1739.4683542912762,
            "unit": "ns"
          },
          {
            "name": "6.6/loop_8_reads",
            "value": 2177472.2147058826,
            "unit": "ns"
          },
          {
            "name": "6.6/loop_iteration",
            "value": 625.0384804389175,
            "unit": "ns"
          },
          {
            "name": "6.6/stall_observe",
            "value": 142.8677517077283,
            "unit": "ns"
          },
          {
            "name": "6.6/truncate_output",
            "value": 4580.325264329438,
            "unit": "ns"
          },
          {
            "name": "6.6/validate_repair",
            "value": 266.14073276913916,
            "unit": "ns"
          },
          {
            "name": "6.7/budget_notice",
            "value": 82.4706517701851,
            "unit": "ns"
          },
          {
            "name": "6.7/cancel_select",
            "value": 96.82357051392763,
            "unit": "ns"
          },
          {
            "name": "6.7/gate_check",
            "value": 7.474719994657178,
            "unit": "ns"
          },
          {
            "name": "6.7/live_tool_body",
            "value": 1684.5018663174142,
            "unit": "ns"
          },
          {
            "name": "6.7/loop_8_reads",
            "value": 2173178.949019608,
            "unit": "ns"
          },
          {
            "name": "6.7/loop_iteration",
            "value": 612.6073605233922,
            "unit": "ns"
          },
          {
            "name": "6.7/stall_observe",
            "value": 140.3015307712665,
            "unit": "ns"
          },
          {
            "name": "6.7/steer_append",
            "value": 58.404934894993225,
            "unit": "ns"
          },
          {
            "name": "6.7/truncate_output",
            "value": 4653.905843651821,
            "unit": "ns"
          },
          {
            "name": "6.7/validate_repair",
            "value": 259.78595680195446,
            "unit": "ns"
          },
          {
            "name": "6.8/budget_notice",
            "value": 80.4198013097083,
            "unit": "ns"
          },
          {
            "name": "6.8/cancel_select",
            "value": 95.77845105973796,
            "unit": "ns"
          },
          {
            "name": "6.8/canonical_args",
            "value": 1058.4180911100766,
            "unit": "ns"
          },
          {
            "name": "6.8/gate_check",
            "value": 7.477039090745873,
            "unit": "ns"
          },
          {
            "name": "6.8/live_tool_body",
            "value": 1731.0392475590868,
            "unit": "ns"
          },
          {
            "name": "6.8/loop_8_reads",
            "value": 2178900.159477124,
            "unit": "ns"
          },
          {
            "name": "6.8/loop_iteration",
            "value": 694.6054239449893,
            "unit": "ns"
          },
          {
            "name": "6.8/stall_observe",
            "value": 140.20344178912816,
            "unit": "ns"
          },
          {
            "name": "6.8/steer_append",
            "value": 58.00160276230881,
            "unit": "ns"
          },
          {
            "name": "6.8/truncate_output",
            "value": 4681.024847094801,
            "unit": "ns"
          },
          {
            "name": "6.8/validate_repair",
            "value": 265.54611061988936,
            "unit": "ns"
          },
          {
            "name": "6.9/budget_notice",
            "value": 76.81304343683905,
            "unit": "ns"
          },
          {
            "name": "6.9/cancel_select",
            "value": 95.35907180430486,
            "unit": "ns"
          },
          {
            "name": "6.9/canonical_args",
            "value": 1063.2914533306302,
            "unit": "ns"
          },
          {
            "name": "6.9/gate_check",
            "value": 7.477239592839063,
            "unit": "ns"
          },
          {
            "name": "6.9/live_tool_body",
            "value": 1788.784877095597,
            "unit": "ns"
          },
          {
            "name": "6.9/loop_8_reads",
            "value": 2178852.3598039215,
            "unit": "ns"
          },
          {
            "name": "6.9/loop_iteration",
            "value": 738.2818042623868,
            "unit": "ns"
          },
          {
            "name": "6.9/outcome_label",
            "value": 8.395222913463755,
            "unit": "ns"
          },
          {
            "name": "6.9/stall_observe",
            "value": 137.38083438872553,
            "unit": "ns"
          },
          {
            "name": "6.9/steer_append",
            "value": 59.26099081958063,
            "unit": "ns"
          },
          {
            "name": "6.9/truncate_output",
            "value": 4556.108959718911,
            "unit": "ns"
          },
          {
            "name": "6.9/validate_repair",
            "value": 267.79030533061643,
            "unit": "ns"
          },
          {
            "name": "gw5/echo_through_gate",
            "value": 45.595588511715974,
            "unit": "ns"
          },
          {
            "name": "gw5/eight_concurrent_sleeps",
            "value": 2114294.45,
            "unit": "ns"
          },
          {
            "name": "gw7/catalog_arc_swap",
            "value": 2426.668762478371,
            "unit": "ns"
          },
          {
            "name": "gw7/catalog_reload_10k",
            "value": 27436400.6,
            "unit": "ns"
          },
          {
            "name": "gw7/catalog_search_10k",
            "value": 298938.34611631534,
            "unit": "ns"
          },
          {
            "name": "gw8/dynamic_rename_task",
            "value": 5701699.1875,
            "unit": "ns"
          },
          {
            "name": "gw8/every_tool_rename_task",
            "value": 8394650.33888889,
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
          "id": "e3badb639e315b78374413f153fed096c5344065",
          "message": "Chapters 21–30: skills, reflection, and team patterns",
          "timestamp": "2026-10-15T04:44:13Z",
          "url": "https://github.com/agnathan/rust-agent-harness/pull/6/commits/e3badb639e315b78374413f153fed096c5344065"
        },
        "date": 1792200192394,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "1.1/serde_50_message_request",
            "value": 8888.753405734678,
            "unit": "ns"
          },
          {
            "name": "1.2/mock_100_concurrent",
            "value": 111846.83739430351,
            "unit": "ns"
          },
          {
            "name": "1.2/mock_turn_overhead",
            "value": 260.5550444812078,
            "unit": "ns"
          },
          {
            "name": "1.3/build_request_and_parse_response",
            "value": 2746.0759241531664,
            "unit": "ns"
          },
          {
            "name": "1.4/parse_response_edges",
            "value": 994.8992329914248,
            "unit": "ns"
          },
          {
            "name": "10/scripted_turn",
            "value": 9.22607512885201,
            "unit": "ns"
          },
          {
            "name": "10/turn_transition",
            "value": 1.302149318164972,
            "unit": "ns"
          },
          {
            "name": "11/skip_decision",
            "value": 234.26648659012648,
            "unit": "ns"
          },
          {
            "name": "12/api_request_hook",
            "value": 2.275429207740796,
            "unit": "ns"
          },
          {
            "name": "12/empty_pre_post",
            "value": 10.728556635914753,
            "unit": "ns"
          },
          {
            "name": "12/memory_enqueue",
            "value": 51.493324047748004,
            "unit": "ns"
          },
          {
            "name": "13/checkpoint_1000",
            "value": 78823.14402958154,
            "unit": "ns"
          },
          {
            "name": "13/rewind_1000",
            "value": 92327.49589002268,
            "unit": "ns"
          },
          {
            "name": "13/warm_accept",
            "value": 23804.20193877551,
            "unit": "ns"
          },
          {
            "name": "14/empty_budget",
            "value": 0.8814625064630733,
            "unit": "ns"
          },
          {
            "name": "14/repetition_guard",
            "value": 2153.1879639251783,
            "unit": "ns"
          },
          {
            "name": "15/open_breaker",
            "value": 28.302658359992872,
            "unit": "ns"
          },
          {
            "name": "15/permit_release",
            "value": 116.57825740165364,
            "unit": "ns"
          },
          {
            "name": "16/choose_route",
            "value": 5.2875496846069,
            "unit": "ns"
          },
          {
            "name": "16/rotate_key",
            "value": 5.524888269124169,
            "unit": "ns"
          },
          {
            "name": "17/buffer_join",
            "value": 602.0714915572232,
            "unit": "ns"
          },
          {
            "name": "17/fold_deltas",
            "value": 2527.9457003607604,
            "unit": "ns"
          },
          {
            "name": "18/wal_append",
            "value": 177234.4511261261,
            "unit": "ns"
          },
          {
            "name": "19/memory_enqueue",
            "value": 48.943428594453906,
            "unit": "ns"
          },
          {
            "name": "2/chat_turn_overhead",
            "value": 802.6731740831233,
            "unit": "ns"
          },
          {
            "name": "2/inbox_submit",
            "value": 40.63826392701005,
            "unit": "ns"
          },
          {
            "name": "20/discover_10k",
            "value": 36287.29334447923,
            "unit": "ns"
          },
          {
            "name": "21/apply_offered",
            "value": 117.79592488093593,
            "unit": "ns"
          },
          {
            "name": "21/predict_offline",
            "value": 219116.9356287425,
            "unit": "ns"
          },
          {
            "name": "22/load_render_500",
            "value": 2954841.176282051,
            "unit": "ns"
          },
          {
            "name": "23/enqueue_unsolicited",
            "value": 25.60568568772497,
            "unit": "ns"
          },
          {
            "name": "23/scan_one_skill",
            "value": 303.0278207632838,
            "unit": "ns"
          },
          {
            "name": "24/cancel_review",
            "value": 751787.1206597222,
            "unit": "ns"
          },
          {
            "name": "24/spawn_reviewer",
            "value": 74.1546778138927,
            "unit": "ns"
          },
          {
            "name": "3.1/registry_lookup",
            "value": 17.788347555641487,
            "unit": "ns"
          },
          {
            "name": "3.2/available_tools_50",
            "value": 28201.218379701186,
            "unit": "ns"
          },
          {
            "name": "3.3/dispatch_8_reads_concurrent",
            "value": 2131069.855882353,
            "unit": "ns"
          },
          {
            "name": "3.3/dispatch_8_reads_sequential",
            "value": 16644036.892592594,
            "unit": "ns"
          },
          {
            "name": "4.1/stdio_echo_round_trip",
            "value": 170273.126271518,
            "unit": "ns"
          },
          {
            "name": "4.2/http_echo_round_trip",
            "value": 378439.2303019146,
            "unit": "ns"
          },
          {
            "name": "4.3/mcp_50_concurrent",
            "value": 3462212.971590909,
            "unit": "ns"
          },
          {
            "name": "4.3/mcp_50_sequential",
            "value": 7191559.15,
            "unit": "ns"
          },
          {
            "name": "4.4/mcp_list_cold",
            "value": 187384.5287132988,
            "unit": "ns"
          },
          {
            "name": "4.4/mcp_list_hit",
            "value": 226.11349801015507,
            "unit": "ns"
          },
          {
            "name": "5.1/schema_tokens_1000",
            "value": 1128709.9595959596,
            "unit": "ns"
          },
          {
            "name": "5.1/schema_tokens_10000",
            "value": 34194302.27083333,
            "unit": "ns"
          },
          {
            "name": "5.2/index_build_10000",
            "value": 19335554.48809524,
            "unit": "ns"
          },
          {
            "name": "5.2/index_search_10000",
            "value": 188495.05442708335,
            "unit": "ns"
          },
          {
            "name": "5.3/full_schema_tokens_10000",
            "value": 17017616.833333336,
            "unit": "ns"
          },
          {
            "name": "5.3/lazy_visible_prompt_10000",
            "value": 16713.038835689316,
            "unit": "ns"
          },
          {
            "name": "5.4/routed_search_10000",
            "value": 20381.1545616003,
            "unit": "ns"
          },
          {
            "name": "5.4/shrink_schemas_10000",
            "value": 57750298.25,
            "unit": "ns"
          },
          {
            "name": "5.5/agent_filter_10000",
            "value": 63834.500175438596,
            "unit": "ns"
          },
          {
            "name": "5.5/cache_cold_10000",
            "value": 21191877.10714286,
            "unit": "ns"
          },
          {
            "name": "5.5/cache_hit_10000",
            "value": 22.81834270091054,
            "unit": "ns"
          },
          {
            "name": "6.1/gate_check",
            "value": 3.3446312827511573,
            "unit": "ns"
          },
          {
            "name": "6.1/live_tool_body",
            "value": 1867.8858132375576,
            "unit": "ns"
          },
          {
            "name": "6.1/loop_8_reads",
            "value": 2140652.3793209875,
            "unit": "ns"
          },
          {
            "name": "6.1/loop_iteration",
            "value": 326.3563196025326,
            "unit": "ns"
          },
          {
            "name": "6.10/budget_notice",
            "value": 72.05473197834758,
            "unit": "ns"
          },
          {
            "name": "6.10/cancel_select",
            "value": 206.30155305987375,
            "unit": "ns"
          },
          {
            "name": "6.10/canonical_args",
            "value": 1020.0119723854784,
            "unit": "ns"
          },
          {
            "name": "6.10/gate_check",
            "value": 18.85818871962625,
            "unit": "ns"
          },
          {
            "name": "6.10/live_tool_body",
            "value": 1861.131329951666,
            "unit": "ns"
          },
          {
            "name": "6.10/loop_8_reads",
            "value": 2167097.0910364147,
            "unit": "ns"
          },
          {
            "name": "6.10/loop_iteration",
            "value": 827.6565982271039,
            "unit": "ns"
          },
          {
            "name": "6.10/outcome_label",
            "value": 7.152179839520748,
            "unit": "ns"
          },
          {
            "name": "6.10/stall_observe",
            "value": 128.81115043703795,
            "unit": "ns"
          },
          {
            "name": "6.10/steer_append",
            "value": 62.04922364863238,
            "unit": "ns"
          },
          {
            "name": "6.10/stress_repair",
            "value": 46661.82916138572,
            "unit": "ns"
          },
          {
            "name": "6.10/truncate_output",
            "value": 3962.9690036766046,
            "unit": "ns"
          },
          {
            "name": "6.10/validate_repair",
            "value": 273.53389164282567,
            "unit": "ns"
          },
          {
            "name": "6.2/cancel_select",
            "value": 206.62914744740044,
            "unit": "ns"
          },
          {
            "name": "6.2/gate_check",
            "value": 18.880239093977167,
            "unit": "ns"
          },
          {
            "name": "6.2/live_tool_body",
            "value": 1896.255039967701,
            "unit": "ns"
          },
          {
            "name": "6.2/loop_8_reads",
            "value": 2144751.969012605,
            "unit": "ns"
          },
          {
            "name": "6.2/loop_iteration",
            "value": 614.0021639520445,
            "unit": "ns"
          },
          {
            "name": "6.3/cancel_select",
            "value": 207.68181110352032,
            "unit": "ns"
          },
          {
            "name": "6.3/gate_check",
            "value": 18.954150528042735,
            "unit": "ns"
          },
          {
            "name": "6.3/live_tool_body",
            "value": 2057.9752219144,
            "unit": "ns"
          },
          {
            "name": "6.3/loop_8_reads",
            "value": 2160828.5279411767,
            "unit": "ns"
          },
          {
            "name": "6.3/loop_iteration",
            "value": 610.9878662221918,
            "unit": "ns"
          },
          {
            "name": "6.3/validate_repair",
            "value": 277.9083045797137,
            "unit": "ns"
          },
          {
            "name": "6.4/cancel_select",
            "value": 249.54519256942234,
            "unit": "ns"
          },
          {
            "name": "6.4/gate_check",
            "value": 18.88618184147634,
            "unit": "ns"
          },
          {
            "name": "6.4/live_tool_body",
            "value": 1970.8035368762394,
            "unit": "ns"
          },
          {
            "name": "6.4/loop_8_reads",
            "value": 2163605.9509803923,
            "unit": "ns"
          },
          {
            "name": "6.4/loop_iteration",
            "value": 640.4708591838519,
            "unit": "ns"
          },
          {
            "name": "6.4/stall_observe",
            "value": 128.07327817160825,
            "unit": "ns"
          },
          {
            "name": "6.4/validate_repair",
            "value": 271.6796414391139,
            "unit": "ns"
          },
          {
            "name": "6.5/budget_notice",
            "value": 71.48743281582473,
            "unit": "ns"
          },
          {
            "name": "6.5/cancel_select",
            "value": 206.53940589768303,
            "unit": "ns"
          },
          {
            "name": "6.5/gate_check",
            "value": 18.867364728166834,
            "unit": "ns"
          },
          {
            "name": "6.5/live_tool_body",
            "value": 1963.774179340315,
            "unit": "ns"
          },
          {
            "name": "6.5/loop_8_reads",
            "value": 2165266.8284313725,
            "unit": "ns"
          },
          {
            "name": "6.5/loop_iteration",
            "value": 669.1264145073592,
            "unit": "ns"
          },
          {
            "name": "6.5/stall_observe",
            "value": 126.6287916489225,
            "unit": "ns"
          },
          {
            "name": "6.5/validate_repair",
            "value": 245.23518965012295,
            "unit": "ns"
          },
          {
            "name": "6.6/budget_notice",
            "value": 74.7604171354142,
            "unit": "ns"
          },
          {
            "name": "6.6/cancel_select",
            "value": 205.88983269443992,
            "unit": "ns"
          },
          {
            "name": "6.6/gate_check",
            "value": 18.947232086163602,
            "unit": "ns"
          },
          {
            "name": "6.6/live_tool_body",
            "value": 1928.6399538458022,
            "unit": "ns"
          },
          {
            "name": "6.6/loop_8_reads",
            "value": 2168443.268907563,
            "unit": "ns"
          },
          {
            "name": "6.6/loop_iteration",
            "value": 673.2437004598819,
            "unit": "ns"
          },
          {
            "name": "6.6/stall_observe",
            "value": 133.00200461961924,
            "unit": "ns"
          },
          {
            "name": "6.6/truncate_output",
            "value": 4164.971079161416,
            "unit": "ns"
          },
          {
            "name": "6.6/validate_repair",
            "value": 292.3930820163406,
            "unit": "ns"
          },
          {
            "name": "6.7/budget_notice",
            "value": 68.57205320388644,
            "unit": "ns"
          },
          {
            "name": "6.7/cancel_select",
            "value": 207.5895148603082,
            "unit": "ns"
          },
          {
            "name": "6.7/gate_check",
            "value": 18.85991843290676,
            "unit": "ns"
          },
          {
            "name": "6.7/live_tool_body",
            "value": 1773.2424136229024,
            "unit": "ns"
          },
          {
            "name": "6.7/loop_8_reads",
            "value": 2162799.1076680673,
            "unit": "ns"
          },
          {
            "name": "6.7/loop_iteration",
            "value": 673.2494264009349,
            "unit": "ns"
          },
          {
            "name": "6.7/stall_observe",
            "value": 126.45123188616515,
            "unit": "ns"
          },
          {
            "name": "6.7/steer_append",
            "value": 62.01209232547824,
            "unit": "ns"
          },
          {
            "name": "6.7/truncate_output",
            "value": 4037.1230995124115,
            "unit": "ns"
          },
          {
            "name": "6.7/validate_repair",
            "value": 288.3746200397375,
            "unit": "ns"
          },
          {
            "name": "6.8/budget_notice",
            "value": 69.69209872834352,
            "unit": "ns"
          },
          {
            "name": "6.8/cancel_select",
            "value": 207.49345512503015,
            "unit": "ns"
          },
          {
            "name": "6.8/canonical_args",
            "value": 1019.3904890595857,
            "unit": "ns"
          },
          {
            "name": "6.8/gate_check",
            "value": 18.860349840878833,
            "unit": "ns"
          },
          {
            "name": "6.8/live_tool_body",
            "value": 1962.0046173806209,
            "unit": "ns"
          },
          {
            "name": "6.8/loop_8_reads",
            "value": 2168841.426470588,
            "unit": "ns"
          },
          {
            "name": "6.8/loop_iteration",
            "value": 854.1436153955831,
            "unit": "ns"
          },
          {
            "name": "6.8/stall_observe",
            "value": 131.02548145072183,
            "unit": "ns"
          },
          {
            "name": "6.8/steer_append",
            "value": 62.007105480206846,
            "unit": "ns"
          },
          {
            "name": "6.8/truncate_output",
            "value": 4028.6232381908585,
            "unit": "ns"
          },
          {
            "name": "6.8/validate_repair",
            "value": 287.70372188316435,
            "unit": "ns"
          },
          {
            "name": "6.9/budget_notice",
            "value": 70.58359074412442,
            "unit": "ns"
          },
          {
            "name": "6.9/cancel_select",
            "value": 205.84885142942125,
            "unit": "ns"
          },
          {
            "name": "6.9/canonical_args",
            "value": 1006.7515294134503,
            "unit": "ns"
          },
          {
            "name": "6.9/gate_check",
            "value": 18.865235115957077,
            "unit": "ns"
          },
          {
            "name": "6.9/live_tool_body",
            "value": 1913.9628402384567,
            "unit": "ns"
          },
          {
            "name": "6.9/loop_8_reads",
            "value": 2172201.882352941,
            "unit": "ns"
          },
          {
            "name": "6.9/loop_iteration",
            "value": 929.4675334449202,
            "unit": "ns"
          },
          {
            "name": "6.9/outcome_label",
            "value": 7.1537837544357235,
            "unit": "ns"
          },
          {
            "name": "6.9/stall_observe",
            "value": 125.45593537938743,
            "unit": "ns"
          },
          {
            "name": "6.9/steer_append",
            "value": 62.0175434806859,
            "unit": "ns"
          },
          {
            "name": "6.9/truncate_output",
            "value": 4007.5846612630494,
            "unit": "ns"
          },
          {
            "name": "6.9/validate_repair",
            "value": 272.33980437730975,
            "unit": "ns"
          },
          {
            "name": "gw5/echo_through_gate",
            "value": 108.74419123792754,
            "unit": "ns"
          },
          {
            "name": "gw5/eight_concurrent_sleeps",
            "value": 2094305.371031746,
            "unit": "ns"
          },
          {
            "name": "gw7/catalog_arc_swap",
            "value": 2374.170849146967,
            "unit": "ns"
          },
          {
            "name": "gw7/catalog_reload_10k",
            "value": 28503478.840277776,
            "unit": "ns"
          },
          {
            "name": "gw7/catalog_search_10k",
            "value": 179793.6038587849,
            "unit": "ns"
          },
          {
            "name": "gw8/dynamic_rename_task",
            "value": 5201049.678571429,
            "unit": "ns"
          },
          {
            "name": "gw8/every_tool_rename_task",
            "value": 7550750.0625,
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
          "id": "84c30cbd6de7c685c7357a7d455eac9432c8fc5f",
          "message": "Chapters 21–30: skills, reflection, and team patterns",
          "timestamp": "2026-10-15T04:44:13Z",
          "url": "https://github.com/agnathan/rust-agent-harness/pull/6/commits/84c30cbd6de7c685c7357a7d455eac9432c8fc5f"
        },
        "date": 1792200209995,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "1.1/serde_50_message_request",
            "value": 10536.456094676927,
            "unit": "ns"
          },
          {
            "name": "1.2/mock_100_concurrent",
            "value": 78508.32033542977,
            "unit": "ns"
          },
          {
            "name": "1.2/mock_turn_overhead",
            "value": 275.3426328229111,
            "unit": "ns"
          },
          {
            "name": "1.3/build_request_and_parse_response",
            "value": 2724.73206788745,
            "unit": "ns"
          },
          {
            "name": "1.4/parse_response_edges",
            "value": 1183.239121849783,
            "unit": "ns"
          },
          {
            "name": "10/scripted_turn",
            "value": 6.929517486797424,
            "unit": "ns"
          },
          {
            "name": "10/turn_transition",
            "value": 1.1910087618764473,
            "unit": "ns"
          },
          {
            "name": "11/skip_decision",
            "value": 273.667965306061,
            "unit": "ns"
          },
          {
            "name": "12/api_request_hook",
            "value": 2.4852694253608503,
            "unit": "ns"
          },
          {
            "name": "12/empty_pre_post",
            "value": 12.669141486642488,
            "unit": "ns"
          },
          {
            "name": "12/memory_enqueue",
            "value": 54.10148293955538,
            "unit": "ns"
          },
          {
            "name": "13/checkpoint_1000",
            "value": 52440.74829244829,
            "unit": "ns"
          },
          {
            "name": "13/rewind_1000",
            "value": 79504.73110151188,
            "unit": "ns"
          },
          {
            "name": "13/warm_accept",
            "value": 23958.71415352261,
            "unit": "ns"
          },
          {
            "name": "14/empty_budget",
            "value": 0.9452986406228352,
            "unit": "ns"
          },
          {
            "name": "14/repetition_guard",
            "value": 2506.449245829355,
            "unit": "ns"
          },
          {
            "name": "15/open_breaker",
            "value": 26.260005726236358,
            "unit": "ns"
          },
          {
            "name": "15/permit_release",
            "value": 61.8250837177676,
            "unit": "ns"
          },
          {
            "name": "16/choose_route",
            "value": 6.334589572688069,
            "unit": "ns"
          },
          {
            "name": "16/rotate_key",
            "value": 3.919036823220992,
            "unit": "ns"
          },
          {
            "name": "17/buffer_join",
            "value": 688.016689815065,
            "unit": "ns"
          },
          {
            "name": "17/fold_deltas",
            "value": 2889.998097636404,
            "unit": "ns"
          },
          {
            "name": "18/wal_append",
            "value": 91664.75809256287,
            "unit": "ns"
          },
          {
            "name": "19/memory_enqueue",
            "value": 53.94618976739882,
            "unit": "ns"
          },
          {
            "name": "2/chat_turn_overhead",
            "value": 647.2732016803084,
            "unit": "ns"
          },
          {
            "name": "2/inbox_submit",
            "value": 42.683823245536345,
            "unit": "ns"
          },
          {
            "name": "20/discover_10k",
            "value": 46000.311118188256,
            "unit": "ns"
          },
          {
            "name": "21/apply_offered",
            "value": 129.5133699861537,
            "unit": "ns"
          },
          {
            "name": "21/predict_offline",
            "value": 300088.08470191224,
            "unit": "ns"
          },
          {
            "name": "22/load_render_500",
            "value": 6888051.444047619,
            "unit": "ns"
          },
          {
            "name": "23/enqueue_unsolicited",
            "value": 20.417684800664453,
            "unit": "ns"
          },
          {
            "name": "23/scan_one_skill",
            "value": 307.5157625053277,
            "unit": "ns"
          },
          {
            "name": "24/cancel_review",
            "value": 757837.0066326531,
            "unit": "ns"
          },
          {
            "name": "24/spawn_reviewer",
            "value": 86.15256909736235,
            "unit": "ns"
          },
          {
            "name": "25/enqueue_curation",
            "value": 20.72580087776049,
            "unit": "ns"
          },
          {
            "name": "26/arc_pass",
            "value": 5.1576162336025675,
            "unit": "ns"
          },
          {
            "name": "26/copy_transcript",
            "value": 8297.846770017477,
            "unit": "ns"
          },
          {
            "name": "3.1/registry_lookup",
            "value": 22.69081717669208,
            "unit": "ns"
          },
          {
            "name": "3.2/available_tools_50",
            "value": 30720.426284751476,
            "unit": "ns"
          },
          {
            "name": "3.3/dispatch_8_reads_concurrent",
            "value": 2153216.4632352944,
            "unit": "ns"
          },
          {
            "name": "3.3/dispatch_8_reads_sequential",
            "value": 16904868.766666666,
            "unit": "ns"
          },
          {
            "name": "4.1/stdio_echo_round_trip",
            "value": 166104.46640574036,
            "unit": "ns"
          },
          {
            "name": "4.2/http_echo_round_trip",
            "value": 396570.0201539855,
            "unit": "ns"
          },
          {
            "name": "4.3/mcp_50_concurrent",
            "value": 4095712.4953703703,
            "unit": "ns"
          },
          {
            "name": "4.3/mcp_50_sequential",
            "value": 8579503.138888888,
            "unit": "ns"
          },
          {
            "name": "4.4/mcp_list_cold",
            "value": 171952.1248034591,
            "unit": "ns"
          },
          {
            "name": "4.4/mcp_list_hit",
            "value": 240.361928556423,
            "unit": "ns"
          },
          {
            "name": "5.1/schema_tokens_1000",
            "value": 1265624.6213054187,
            "unit": "ns"
          },
          {
            "name": "5.1/schema_tokens_10000",
            "value": 40089377.1,
            "unit": "ns"
          },
          {
            "name": "5.2/index_build_10000",
            "value": 21824858.5,
            "unit": "ns"
          },
          {
            "name": "5.2/index_search_10000",
            "value": 242691.49,
            "unit": "ns"
          },
          {
            "name": "5.3/full_schema_tokens_10000",
            "value": 20255580.523809522,
            "unit": "ns"
          },
          {
            "name": "5.3/lazy_visible_prompt_10000",
            "value": 16148.189893617022,
            "unit": "ns"
          },
          {
            "name": "5.4/routed_search_10000",
            "value": 24165.13482990943,
            "unit": "ns"
          },
          {
            "name": "5.4/shrink_schemas_10000",
            "value": 63044157.96666667,
            "unit": "ns"
          },
          {
            "name": "5.5/agent_filter_10000",
            "value": 113290.40310559006,
            "unit": "ns"
          },
          {
            "name": "5.5/cache_cold_10000",
            "value": 24397513.933333334,
            "unit": "ns"
          },
          {
            "name": "5.5/cache_hit_10000",
            "value": 15.988028994431568,
            "unit": "ns"
          },
          {
            "name": "6.1/gate_check",
            "value": 3.170818968125148,
            "unit": "ns"
          },
          {
            "name": "6.1/live_tool_body",
            "value": 1901.4385091707163,
            "unit": "ns"
          },
          {
            "name": "6.1/loop_8_reads",
            "value": 2186555.144607843,
            "unit": "ns"
          },
          {
            "name": "6.1/loop_iteration",
            "value": 368.02551754592355,
            "unit": "ns"
          },
          {
            "name": "6.10/budget_notice",
            "value": 79.30501025902967,
            "unit": "ns"
          },
          {
            "name": "6.10/cancel_select",
            "value": 117.52322080870705,
            "unit": "ns"
          },
          {
            "name": "6.10/canonical_args",
            "value": 1078.690865779892,
            "unit": "ns"
          },
          {
            "name": "6.10/gate_check",
            "value": 7.304196217600217,
            "unit": "ns"
          },
          {
            "name": "6.10/live_tool_body",
            "value": 1958.7124934416065,
            "unit": "ns"
          },
          {
            "name": "6.10/loop_8_reads",
            "value": 2228676.669117647,
            "unit": "ns"
          },
          {
            "name": "6.10/loop_iteration",
            "value": 729.3922826091773,
            "unit": "ns"
          },
          {
            "name": "6.10/outcome_label",
            "value": 9.87289138562745,
            "unit": "ns"
          },
          {
            "name": "6.10/stall_observe",
            "value": 145.30915334039116,
            "unit": "ns"
          },
          {
            "name": "6.10/steer_append",
            "value": 58.81482946369478,
            "unit": "ns"
          },
          {
            "name": "6.10/stress_repair",
            "value": 39462.98251607717,
            "unit": "ns"
          },
          {
            "name": "6.10/truncate_output",
            "value": 4440.87996579526,
            "unit": "ns"
          },
          {
            "name": "6.10/validate_repair",
            "value": 271.2796525001013,
            "unit": "ns"
          },
          {
            "name": "6.2/cancel_select",
            "value": 119.45609040394581,
            "unit": "ns"
          },
          {
            "name": "6.2/gate_check",
            "value": 7.335182897253712,
            "unit": "ns"
          },
          {
            "name": "6.2/live_tool_body",
            "value": 1908.981255418245,
            "unit": "ns"
          },
          {
            "name": "6.2/loop_8_reads",
            "value": 2191189.8882352943,
            "unit": "ns"
          },
          {
            "name": "6.2/loop_iteration",
            "value": 541.1333239201764,
            "unit": "ns"
          },
          {
            "name": "6.3/cancel_select",
            "value": 119.4924128893197,
            "unit": "ns"
          },
          {
            "name": "6.3/gate_check",
            "value": 7.316378538900851,
            "unit": "ns"
          },
          {
            "name": "6.3/live_tool_body",
            "value": 1975.4843809928225,
            "unit": "ns"
          },
          {
            "name": "6.3/loop_8_reads",
            "value": 2214732.6392156864,
            "unit": "ns"
          },
          {
            "name": "6.3/loop_iteration",
            "value": 522.5623569002671,
            "unit": "ns"
          },
          {
            "name": "6.3/validate_repair",
            "value": 258.4554748854712,
            "unit": "ns"
          },
          {
            "name": "6.4/cancel_select",
            "value": 116.26395227685515,
            "unit": "ns"
          },
          {
            "name": "6.4/gate_check",
            "value": 7.304313786873413,
            "unit": "ns"
          },
          {
            "name": "6.4/live_tool_body",
            "value": 1955.8988574015875,
            "unit": "ns"
          },
          {
            "name": "6.4/loop_8_reads",
            "value": 2217999.9784663864,
            "unit": "ns"
          },
          {
            "name": "6.4/loop_iteration",
            "value": 544.4792794419286,
            "unit": "ns"
          },
          {
            "name": "6.4/stall_observe",
            "value": 145.47283572133728,
            "unit": "ns"
          },
          {
            "name": "6.4/validate_repair",
            "value": 263.89545296691875,
            "unit": "ns"
          },
          {
            "name": "6.5/budget_notice",
            "value": 83.9321158753648,
            "unit": "ns"
          },
          {
            "name": "6.5/cancel_select",
            "value": 117.34370460582008,
            "unit": "ns"
          },
          {
            "name": "6.5/gate_check",
            "value": 7.319945989051357,
            "unit": "ns"
          },
          {
            "name": "6.5/live_tool_body",
            "value": 1983.9684223480954,
            "unit": "ns"
          },
          {
            "name": "6.5/loop_8_reads",
            "value": 2211675.637815126,
            "unit": "ns"
          },
          {
            "name": "6.5/loop_iteration",
            "value": 596.2963656477915,
            "unit": "ns"
          },
          {
            "name": "6.5/stall_observe",
            "value": 143.3050304986744,
            "unit": "ns"
          },
          {
            "name": "6.5/validate_repair",
            "value": 265.7741178078247,
            "unit": "ns"
          },
          {
            "name": "6.6/budget_notice",
            "value": 77.89853693702972,
            "unit": "ns"
          },
          {
            "name": "6.6/cancel_select",
            "value": 119.08948411298479,
            "unit": "ns"
          },
          {
            "name": "6.6/gate_check",
            "value": 7.336343592888447,
            "unit": "ns"
          },
          {
            "name": "6.6/live_tool_body",
            "value": 1903.421418591163,
            "unit": "ns"
          },
          {
            "name": "6.6/loop_8_reads",
            "value": 2230632.4411764704,
            "unit": "ns"
          },
          {
            "name": "6.6/loop_iteration",
            "value": 620.2234765006448,
            "unit": "ns"
          },
          {
            "name": "6.6/stall_observe",
            "value": 146.22360894772908,
            "unit": "ns"
          },
          {
            "name": "6.6/truncate_output",
            "value": 4334.545228072962,
            "unit": "ns"
          },
          {
            "name": "6.6/validate_repair",
            "value": 259.680172863876,
            "unit": "ns"
          },
          {
            "name": "6.7/budget_notice",
            "value": 77.69754068396593,
            "unit": "ns"
          },
          {
            "name": "6.7/cancel_select",
            "value": 118.76314658390393,
            "unit": "ns"
          },
          {
            "name": "6.7/gate_check",
            "value": 7.376234918116756,
            "unit": "ns"
          },
          {
            "name": "6.7/live_tool_body",
            "value": 1784.7937683987966,
            "unit": "ns"
          },
          {
            "name": "6.7/loop_8_reads",
            "value": 2215145.777777778,
            "unit": "ns"
          },
          {
            "name": "6.7/loop_iteration",
            "value": 607.7077568376562,
            "unit": "ns"
          },
          {
            "name": "6.7/stall_observe",
            "value": 142.24008535726858,
            "unit": "ns"
          },
          {
            "name": "6.7/steer_append",
            "value": 58.772644946582666,
            "unit": "ns"
          },
          {
            "name": "6.7/truncate_output",
            "value": 4323.9238832638475,
            "unit": "ns"
          },
          {
            "name": "6.7/validate_repair",
            "value": 259.1693804637175,
            "unit": "ns"
          },
          {
            "name": "6.8/budget_notice",
            "value": 81.9176084763338,
            "unit": "ns"
          },
          {
            "name": "6.8/cancel_select",
            "value": 107.06404581852986,
            "unit": "ns"
          },
          {
            "name": "6.8/canonical_args",
            "value": 1082.7224960051135,
            "unit": "ns"
          },
          {
            "name": "6.8/gate_check",
            "value": 7.3265605607794395,
            "unit": "ns"
          },
          {
            "name": "6.8/live_tool_body",
            "value": 1982.23434681607,
            "unit": "ns"
          },
          {
            "name": "6.8/loop_8_reads",
            "value": 2235532.386764706,
            "unit": "ns"
          },
          {
            "name": "6.8/loop_iteration",
            "value": 675.2671828155235,
            "unit": "ns"
          },
          {
            "name": "6.8/stall_observe",
            "value": 146.36347880802447,
            "unit": "ns"
          },
          {
            "name": "6.8/steer_append",
            "value": 58.80630354129686,
            "unit": "ns"
          },
          {
            "name": "6.8/truncate_output",
            "value": 4392.186789033331,
            "unit": "ns"
          },
          {
            "name": "6.8/validate_repair",
            "value": 265.53997600577156,
            "unit": "ns"
          },
          {
            "name": "6.9/budget_notice",
            "value": 79.14159964787927,
            "unit": "ns"
          },
          {
            "name": "6.9/cancel_select",
            "value": 109.69122267925617,
            "unit": "ns"
          },
          {
            "name": "6.9/canonical_args",
            "value": 1076.5124572358013,
            "unit": "ns"
          },
          {
            "name": "6.9/gate_check",
            "value": 7.331714711089967,
            "unit": "ns"
          },
          {
            "name": "6.9/live_tool_body",
            "value": 1929.4332634428579,
            "unit": "ns"
          },
          {
            "name": "6.9/loop_8_reads",
            "value": 2233961.238235294,
            "unit": "ns"
          },
          {
            "name": "6.9/loop_iteration",
            "value": 770.1824452694391,
            "unit": "ns"
          },
          {
            "name": "6.9/outcome_label",
            "value": 9.871626268430916,
            "unit": "ns"
          },
          {
            "name": "6.9/stall_observe",
            "value": 143.38519699491206,
            "unit": "ns"
          },
          {
            "name": "6.9/steer_append",
            "value": 59.126049617308055,
            "unit": "ns"
          },
          {
            "name": "6.9/truncate_output",
            "value": 4439.955705659917,
            "unit": "ns"
          },
          {
            "name": "6.9/validate_repair",
            "value": 264.90522150782476,
            "unit": "ns"
          },
          {
            "name": "gw5/echo_through_gate",
            "value": 49.40223299390896,
            "unit": "ns"
          },
          {
            "name": "gw5/eight_concurrent_sleeps",
            "value": 2140766.261904762,
            "unit": "ns"
          },
          {
            "name": "gw7/catalog_arc_swap",
            "value": 2512.380378255234,
            "unit": "ns"
          },
          {
            "name": "gw7/catalog_reload_10k",
            "value": 28890841.71875,
            "unit": "ns"
          },
          {
            "name": "gw7/catalog_search_10k",
            "value": 242312.4197368421,
            "unit": "ns"
          },
          {
            "name": "gw8/dynamic_rename_task",
            "value": 5910537.470663265,
            "unit": "ns"
          },
          {
            "name": "gw8/every_tool_rename_task",
            "value": 8770531.84,
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
          "id": "b34a1d1e419f73f8620c0a3257842d818505c945",
          "message": "Chapters 21–30: skills, reflection, and team patterns",
          "timestamp": "2026-10-15T04:44:13Z",
          "url": "https://github.com/agnathan/rust-agent-harness/pull/6/commits/b34a1d1e419f73f8620c0a3257842d818505c945"
        },
        "date": 1792200240709,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "1.1/serde_50_message_request",
            "value": 10095.789667128987,
            "unit": "ns"
          },
          {
            "name": "1.2/mock_100_concurrent",
            "value": 80216.41766004416,
            "unit": "ns"
          },
          {
            "name": "1.2/mock_turn_overhead",
            "value": 253.19811936251256,
            "unit": "ns"
          },
          {
            "name": "1.3/build_request_and_parse_response",
            "value": 2961.557701599805,
            "unit": "ns"
          },
          {
            "name": "1.4/parse_response_edges",
            "value": 1087.93886883991,
            "unit": "ns"
          },
          {
            "name": "10/scripted_turn",
            "value": 8.11549181918754,
            "unit": "ns"
          },
          {
            "name": "10/turn_transition",
            "value": 1.1002778373974875,
            "unit": "ns"
          },
          {
            "name": "11/skip_decision",
            "value": 251.04290585007388,
            "unit": "ns"
          },
          {
            "name": "12/api_request_hook",
            "value": 2.1808701243680897,
            "unit": "ns"
          },
          {
            "name": "12/empty_pre_post",
            "value": 10.909260720997061,
            "unit": "ns"
          },
          {
            "name": "12/memory_enqueue",
            "value": 54.57896575466839,
            "unit": "ns"
          },
          {
            "name": "13/checkpoint_1000",
            "value": 50478.751139290674,
            "unit": "ns"
          },
          {
            "name": "13/rewind_1000",
            "value": 79892.48981277533,
            "unit": "ns"
          },
          {
            "name": "13/warm_accept",
            "value": 30187.13984565394,
            "unit": "ns"
          },
          {
            "name": "14/empty_budget",
            "value": 0.9041707325797825,
            "unit": "ns"
          },
          {
            "name": "14/repetition_guard",
            "value": 2533.9657128830913,
            "unit": "ns"
          },
          {
            "name": "15/open_breaker",
            "value": 30.099803785992762,
            "unit": "ns"
          },
          {
            "name": "15/permit_release",
            "value": 58.7295202420345,
            "unit": "ns"
          },
          {
            "name": "16/choose_route",
            "value": 5.917980057829793,
            "unit": "ns"
          },
          {
            "name": "16/rotate_key",
            "value": 3.4467982080055144,
            "unit": "ns"
          },
          {
            "name": "17/buffer_join",
            "value": 697.8607108951176,
            "unit": "ns"
          },
          {
            "name": "17/fold_deltas",
            "value": 2815.7780239368476,
            "unit": "ns"
          },
          {
            "name": "18/wal_append",
            "value": 149126.94715447153,
            "unit": "ns"
          },
          {
            "name": "19/memory_enqueue",
            "value": 54.6501524564714,
            "unit": "ns"
          },
          {
            "name": "2/chat_turn_overhead",
            "value": 715.5936730969047,
            "unit": "ns"
          },
          {
            "name": "2/inbox_submit",
            "value": 40.78501523603062,
            "unit": "ns"
          },
          {
            "name": "20/discover_10k",
            "value": 61052.51235318792,
            "unit": "ns"
          },
          {
            "name": "21/apply_offered",
            "value": 112.89860717183039,
            "unit": "ns"
          },
          {
            "name": "21/predict_offline",
            "value": 293660.6716,
            "unit": "ns"
          },
          {
            "name": "22/load_render_500",
            "value": 5842990.020408163,
            "unit": "ns"
          },
          {
            "name": "23/enqueue_unsolicited",
            "value": 19.152009849129435,
            "unit": "ns"
          },
          {
            "name": "23/scan_one_skill",
            "value": 283.9772636281428,
            "unit": "ns"
          },
          {
            "name": "24/cancel_review",
            "value": 704224.9692307692,
            "unit": "ns"
          },
          {
            "name": "24/spawn_reviewer",
            "value": 81.27814616811693,
            "unit": "ns"
          },
          {
            "name": "25/enqueue_curation",
            "value": 19.93621267268331,
            "unit": "ns"
          },
          {
            "name": "3.1/registry_lookup",
            "value": 25.635435260516143,
            "unit": "ns"
          },
          {
            "name": "3.2/available_tools_50",
            "value": 30574.208703481392,
            "unit": "ns"
          },
          {
            "name": "3.3/dispatch_8_reads_concurrent",
            "value": 2134879.251633987,
            "unit": "ns"
          },
          {
            "name": "3.3/dispatch_8_reads_sequential",
            "value": 16666285.430555556,
            "unit": "ns"
          },
          {
            "name": "4.1/stdio_echo_round_trip",
            "value": 204060.33130411257,
            "unit": "ns"
          },
          {
            "name": "4.2/http_echo_round_trip",
            "value": 423825.5327586207,
            "unit": "ns"
          },
          {
            "name": "4.3/mcp_50_concurrent",
            "value": 5005158.928571429,
            "unit": "ns"
          },
          {
            "name": "4.3/mcp_50_sequential",
            "value": 10357312.128472222,
            "unit": "ns"
          },
          {
            "name": "4.4/mcp_list_cold",
            "value": 218649.24852071004,
            "unit": "ns"
          },
          {
            "name": "4.4/mcp_list_hit",
            "value": 255.37500855154312,
            "unit": "ns"
          },
          {
            "name": "5.1/schema_tokens_1000",
            "value": 1271729.8987068967,
            "unit": "ns"
          },
          {
            "name": "5.1/schema_tokens_10000",
            "value": 42534939,
            "unit": "ns"
          },
          {
            "name": "5.2/index_build_10000",
            "value": 21766945.25,
            "unit": "ns"
          },
          {
            "name": "5.2/index_search_10000",
            "value": 332138.5496527778,
            "unit": "ns"
          },
          {
            "name": "5.3/full_schema_tokens_10000",
            "value": 23690834.35,
            "unit": "ns"
          },
          {
            "name": "5.3/lazy_visible_prompt_10000",
            "value": 15544.278145616783,
            "unit": "ns"
          },
          {
            "name": "5.4/routed_search_10000",
            "value": 25906.368252840908,
            "unit": "ns"
          },
          {
            "name": "5.4/shrink_schemas_10000",
            "value": 64285601.125,
            "unit": "ns"
          },
          {
            "name": "5.5/agent_filter_10000",
            "value": 133578.36592458887,
            "unit": "ns"
          },
          {
            "name": "5.5/cache_cold_10000",
            "value": 26419717.222222224,
            "unit": "ns"
          },
          {
            "name": "5.5/cache_hit_10000",
            "value": 13.790907118890825,
            "unit": "ns"
          },
          {
            "name": "6.1/gate_check",
            "value": 3.4262110642711994,
            "unit": "ns"
          },
          {
            "name": "6.1/live_tool_body",
            "value": 1723.7959971389482,
            "unit": "ns"
          },
          {
            "name": "6.1/loop_8_reads",
            "value": 2145725.761764706,
            "unit": "ns"
          },
          {
            "name": "6.1/loop_iteration",
            "value": 362.1890038599131,
            "unit": "ns"
          },
          {
            "name": "6.10/budget_notice",
            "value": 76.89420020378421,
            "unit": "ns"
          },
          {
            "name": "6.10/cancel_select",
            "value": 95.03515950070371,
            "unit": "ns"
          },
          {
            "name": "6.10/canonical_args",
            "value": 1064.2394608538084,
            "unit": "ns"
          },
          {
            "name": "6.10/gate_check",
            "value": 7.473702587385027,
            "unit": "ns"
          },
          {
            "name": "6.10/live_tool_body",
            "value": 1727.317175303499,
            "unit": "ns"
          },
          {
            "name": "6.10/loop_8_reads",
            "value": 2180563.387394958,
            "unit": "ns"
          },
          {
            "name": "6.10/loop_iteration",
            "value": 815.4591573604062,
            "unit": "ns"
          },
          {
            "name": "6.10/outcome_label",
            "value": 8.39946751185971,
            "unit": "ns"
          },
          {
            "name": "6.10/stall_observe",
            "value": 138.16375300977606,
            "unit": "ns"
          },
          {
            "name": "6.10/steer_append",
            "value": 60.08909733118822,
            "unit": "ns"
          },
          {
            "name": "6.10/stress_repair",
            "value": 51666.8533243486,
            "unit": "ns"
          },
          {
            "name": "6.10/truncate_output",
            "value": 4959.577033492823,
            "unit": "ns"
          },
          {
            "name": "6.10/validate_repair",
            "value": 279.33731110812903,
            "unit": "ns"
          },
          {
            "name": "6.2/cancel_select",
            "value": 96.40926062353616,
            "unit": "ns"
          },
          {
            "name": "6.2/gate_check",
            "value": 7.47414954005345,
            "unit": "ns"
          },
          {
            "name": "6.2/live_tool_body",
            "value": 1724.3073111889541,
            "unit": "ns"
          },
          {
            "name": "6.2/loop_8_reads",
            "value": 2157766.196078432,
            "unit": "ns"
          },
          {
            "name": "6.2/loop_iteration",
            "value": 547.1314250850164,
            "unit": "ns"
          },
          {
            "name": "6.3/cancel_select",
            "value": 96.1691601867094,
            "unit": "ns"
          },
          {
            "name": "6.3/gate_check",
            "value": 7.469632855004537,
            "unit": "ns"
          },
          {
            "name": "6.3/live_tool_body",
            "value": 1786.2333809587099,
            "unit": "ns"
          },
          {
            "name": "6.3/loop_8_reads",
            "value": 2173678.522549019,
            "unit": "ns"
          },
          {
            "name": "6.3/loop_iteration",
            "value": 544.804779873285,
            "unit": "ns"
          },
          {
            "name": "6.3/validate_repair",
            "value": 256.00265930479605,
            "unit": "ns"
          },
          {
            "name": "6.4/cancel_select",
            "value": 96.46061987742607,
            "unit": "ns"
          },
          {
            "name": "6.4/gate_check",
            "value": 7.472674091278348,
            "unit": "ns"
          },
          {
            "name": "6.4/live_tool_body",
            "value": 1786.919220363994,
            "unit": "ns"
          },
          {
            "name": "6.4/loop_8_reads",
            "value": 2177976.4117647056,
            "unit": "ns"
          },
          {
            "name": "6.4/loop_iteration",
            "value": 556.9841337080773,
            "unit": "ns"
          },
          {
            "name": "6.4/stall_observe",
            "value": 140.37367187049392,
            "unit": "ns"
          },
          {
            "name": "6.4/validate_repair",
            "value": 255.79373019725932,
            "unit": "ns"
          },
          {
            "name": "6.5/budget_notice",
            "value": 78.18297433951452,
            "unit": "ns"
          },
          {
            "name": "6.5/cancel_select",
            "value": 96.45050738591459,
            "unit": "ns"
          },
          {
            "name": "6.5/gate_check",
            "value": 7.468373192264517,
            "unit": "ns"
          },
          {
            "name": "6.5/live_tool_body",
            "value": 1733.9483147131225,
            "unit": "ns"
          },
          {
            "name": "6.5/loop_8_reads",
            "value": 2173987.651260504,
            "unit": "ns"
          },
          {
            "name": "6.5/loop_iteration",
            "value": 621.5994058027604,
            "unit": "ns"
          },
          {
            "name": "6.5/stall_observe",
            "value": 138.5472438202235,
            "unit": "ns"
          },
          {
            "name": "6.5/validate_repair",
            "value": 291.62575289688675,
            "unit": "ns"
          },
          {
            "name": "6.6/budget_notice",
            "value": 79.07952168648755,
            "unit": "ns"
          },
          {
            "name": "6.6/cancel_select",
            "value": 156.5954104441661,
            "unit": "ns"
          },
          {
            "name": "6.6/gate_check",
            "value": 7.469413836979797,
            "unit": "ns"
          },
          {
            "name": "6.6/live_tool_body",
            "value": 1750.7628970256114,
            "unit": "ns"
          },
          {
            "name": "6.6/loop_8_reads",
            "value": 2180126.124369748,
            "unit": "ns"
          },
          {
            "name": "6.6/loop_iteration",
            "value": 623.8402445229601,
            "unit": "ns"
          },
          {
            "name": "6.6/stall_observe",
            "value": 139.07946978071368,
            "unit": "ns"
          },
          {
            "name": "6.6/truncate_output",
            "value": 4599.015385042474,
            "unit": "ns"
          },
          {
            "name": "6.6/validate_repair",
            "value": 257.52050092567197,
            "unit": "ns"
          },
          {
            "name": "6.7/budget_notice",
            "value": 78.08049795100897,
            "unit": "ns"
          },
          {
            "name": "6.7/cancel_select",
            "value": 96.98202575650433,
            "unit": "ns"
          },
          {
            "name": "6.7/gate_check",
            "value": 7.475052404157597,
            "unit": "ns"
          },
          {
            "name": "6.7/live_tool_body",
            "value": 1667.5326227977182,
            "unit": "ns"
          },
          {
            "name": "6.7/loop_8_reads",
            "value": 2181114.9295051354,
            "unit": "ns"
          },
          {
            "name": "6.7/loop_iteration",
            "value": 557.366082355612,
            "unit": "ns"
          },
          {
            "name": "6.7/stall_observe",
            "value": 140.35711584548494,
            "unit": "ns"
          },
          {
            "name": "6.7/steer_append",
            "value": 58.4069636021439,
            "unit": "ns"
          },
          {
            "name": "6.7/truncate_output",
            "value": 4573.173676508017,
            "unit": "ns"
          },
          {
            "name": "6.7/validate_repair",
            "value": 261.971760354205,
            "unit": "ns"
          },
          {
            "name": "6.8/budget_notice",
            "value": 77.99966915937961,
            "unit": "ns"
          },
          {
            "name": "6.8/cancel_select",
            "value": 97.06517352144363,
            "unit": "ns"
          },
          {
            "name": "6.8/canonical_args",
            "value": 1072.7548257480476,
            "unit": "ns"
          },
          {
            "name": "6.8/gate_check",
            "value": 7.474835822944933,
            "unit": "ns"
          },
          {
            "name": "6.8/live_tool_body",
            "value": 1860.1361039445019,
            "unit": "ns"
          },
          {
            "name": "6.8/loop_8_reads",
            "value": 2185496.088235294,
            "unit": "ns"
          },
          {
            "name": "6.8/loop_iteration",
            "value": 721.66106035702,
            "unit": "ns"
          },
          {
            "name": "6.8/stall_observe",
            "value": 137.8513106311599,
            "unit": "ns"
          },
          {
            "name": "6.8/steer_append",
            "value": 58.04845116033384,
            "unit": "ns"
          },
          {
            "name": "6.8/truncate_output",
            "value": 4678.612826733106,
            "unit": "ns"
          },
          {
            "name": "6.8/validate_repair",
            "value": 258.8710470838764,
            "unit": "ns"
          },
          {
            "name": "6.9/budget_notice",
            "value": 83.15001186267953,
            "unit": "ns"
          },
          {
            "name": "6.9/cancel_select",
            "value": 95.2942563844569,
            "unit": "ns"
          },
          {
            "name": "6.9/canonical_args",
            "value": 1083.2672959099189,
            "unit": "ns"
          },
          {
            "name": "6.9/gate_check",
            "value": 7.473039276166341,
            "unit": "ns"
          },
          {
            "name": "6.9/live_tool_body",
            "value": 1735.723491505565,
            "unit": "ns"
          },
          {
            "name": "6.9/loop_8_reads",
            "value": 2186301.125,
            "unit": "ns"
          },
          {
            "name": "6.9/loop_iteration",
            "value": 752.8276055593248,
            "unit": "ns"
          },
          {
            "name": "6.9/outcome_label",
            "value": 8.397011305411073,
            "unit": "ns"
          },
          {
            "name": "6.9/stall_observe",
            "value": 136.4943587620192,
            "unit": "ns"
          },
          {
            "name": "6.9/steer_append",
            "value": 59.19310327423932,
            "unit": "ns"
          },
          {
            "name": "6.9/truncate_output",
            "value": 4569.392708988058,
            "unit": "ns"
          },
          {
            "name": "6.9/validate_repair",
            "value": 259.95981357489825,
            "unit": "ns"
          },
          {
            "name": "gw5/echo_through_gate",
            "value": 45.60854245615443,
            "unit": "ns"
          },
          {
            "name": "gw5/eight_concurrent_sleeps",
            "value": 2116057.359722222,
            "unit": "ns"
          },
          {
            "name": "gw7/catalog_arc_swap",
            "value": 2380.0872779234014,
            "unit": "ns"
          },
          {
            "name": "gw7/catalog_reload_10k",
            "value": 27855445.941666666,
            "unit": "ns"
          },
          {
            "name": "gw7/catalog_search_10k",
            "value": 297413.2667349727,
            "unit": "ns"
          },
          {
            "name": "gw8/dynamic_rename_task",
            "value": 5728934.535714285,
            "unit": "ns"
          },
          {
            "name": "gw8/every_tool_rename_task",
            "value": 8415091.235714287,
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
          "id": "69d4ba8d7b8e6904672c9810ff68a5fa4eef594e",
          "message": "Chapters 21–30: skills, reflection, and team patterns",
          "timestamp": "2026-10-15T04:44:13Z",
          "url": "https://github.com/agnathan/rust-agent-harness/pull/6/commits/69d4ba8d7b8e6904672c9810ff68a5fa4eef594e"
        },
        "date": 1792200248971,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "1.1/serde_50_message_request",
            "value": 8395.373488947542,
            "unit": "ns"
          },
          {
            "name": "1.2/mock_100_concurrent",
            "value": 63465.94599303136,
            "unit": "ns"
          },
          {
            "name": "1.2/mock_turn_overhead",
            "value": 210.27135636283953,
            "unit": "ns"
          },
          {
            "name": "1.3/build_request_and_parse_response",
            "value": 2160.290133719898,
            "unit": "ns"
          },
          {
            "name": "1.4/parse_response_edges",
            "value": 928.3233525440263,
            "unit": "ns"
          },
          {
            "name": "10/scripted_turn",
            "value": 5.405003478084735,
            "unit": "ns"
          },
          {
            "name": "10/turn_transition",
            "value": 0.9119000787466649,
            "unit": "ns"
          },
          {
            "name": "11/skip_decision",
            "value": 212.2800425758727,
            "unit": "ns"
          },
          {
            "name": "12/api_request_hook",
            "value": 1.909064096917031,
            "unit": "ns"
          },
          {
            "name": "12/empty_pre_post",
            "value": 9.817591442570096,
            "unit": "ns"
          },
          {
            "name": "12/memory_enqueue",
            "value": 42.91398962918488,
            "unit": "ns"
          },
          {
            "name": "13/checkpoint_1000",
            "value": 40998.48579970105,
            "unit": "ns"
          },
          {
            "name": "13/rewind_1000",
            "value": 62675.346267806264,
            "unit": "ns"
          },
          {
            "name": "13/warm_accept",
            "value": 18731.60211223984,
            "unit": "ns"
          },
          {
            "name": "14/empty_budget",
            "value": 0.7332696639930708,
            "unit": "ns"
          },
          {
            "name": "14/repetition_guard",
            "value": 1952.0673758486128,
            "unit": "ns"
          },
          {
            "name": "15/open_breaker",
            "value": 19.847666201530302,
            "unit": "ns"
          },
          {
            "name": "15/permit_release",
            "value": 49.040387803158026,
            "unit": "ns"
          },
          {
            "name": "16/choose_route",
            "value": 4.365593045787607,
            "unit": "ns"
          },
          {
            "name": "16/rotate_key",
            "value": 3.0382723114630865,
            "unit": "ns"
          },
          {
            "name": "17/buffer_join",
            "value": 549.792150985417,
            "unit": "ns"
          },
          {
            "name": "17/fold_deltas",
            "value": 2349.341293359602,
            "unit": "ns"
          },
          {
            "name": "18/wal_append",
            "value": 95830.60638757014,
            "unit": "ns"
          },
          {
            "name": "19/memory_enqueue",
            "value": 43.660938011948545,
            "unit": "ns"
          },
          {
            "name": "2/chat_turn_overhead",
            "value": 582.2033701244169,
            "unit": "ns"
          },
          {
            "name": "2/inbox_submit",
            "value": 33.26519310456683,
            "unit": "ns"
          },
          {
            "name": "20/discover_10k",
            "value": 35576.47437363834,
            "unit": "ns"
          },
          {
            "name": "21/apply_offered",
            "value": 111.22625493132105,
            "unit": "ns"
          },
          {
            "name": "21/predict_offline",
            "value": 218876.6350299401,
            "unit": "ns"
          },
          {
            "name": "22/load_render_500",
            "value": 5206966.347619047,
            "unit": "ns"
          },
          {
            "name": "23/enqueue_unsolicited",
            "value": 15.726942078428799,
            "unit": "ns"
          },
          {
            "name": "23/scan_one_skill",
            "value": 231.65723378449565,
            "unit": "ns"
          },
          {
            "name": "24/cancel_review",
            "value": 812075.5447722567,
            "unit": "ns"
          },
          {
            "name": "24/spawn_reviewer",
            "value": 72.48866683399858,
            "unit": "ns"
          },
          {
            "name": "25/enqueue_curation",
            "value": 16.080570967569862,
            "unit": "ns"
          },
          {
            "name": "26/arc_pass",
            "value": 4.015786663090459,
            "unit": "ns"
          },
          {
            "name": "26/copy_transcript",
            "value": 6560.576800828381,
            "unit": "ns"
          },
          {
            "name": "27/enqueue_reflection",
            "value": 17.783600274734134,
            "unit": "ns"
          },
          {
            "name": "27/schedule_eight",
            "value": 175917.44083867522,
            "unit": "ns"
          },
          {
            "name": "28/filter_100",
            "value": 3367.0416519667006,
            "unit": "ns"
          },
          {
            "name": "29/schedule_100",
            "value": 5188.208741457858,
            "unit": "ns"
          },
          {
            "name": "3.1/registry_lookup",
            "value": 17.862800341033655,
            "unit": "ns"
          },
          {
            "name": "3.2/available_tools_50",
            "value": 24156.00122435473,
            "unit": "ns"
          },
          {
            "name": "3.3/dispatch_8_reads_concurrent",
            "value": 2150709.9814814813,
            "unit": "ns"
          },
          {
            "name": "3.3/dispatch_8_reads_sequential",
            "value": 16760173.666666666,
            "unit": "ns"
          },
          {
            "name": "4.1/stdio_echo_round_trip",
            "value": 130492.11371841155,
            "unit": "ns"
          },
          {
            "name": "4.2/http_echo_round_trip",
            "value": 310288.1803874092,
            "unit": "ns"
          },
          {
            "name": "4.3/mcp_50_concurrent",
            "value": 3279496.3722222224,
            "unit": "ns"
          },
          {
            "name": "4.3/mcp_50_sequential",
            "value": 6659383.861111111,
            "unit": "ns"
          },
          {
            "name": "4.4/mcp_list_cold",
            "value": 134541.90101851852,
            "unit": "ns"
          },
          {
            "name": "4.4/mcp_list_hit",
            "value": 182.66142112882642,
            "unit": "ns"
          },
          {
            "name": "5.1/schema_tokens_1000",
            "value": 1001004.1582207207,
            "unit": "ns"
          },
          {
            "name": "5.1/schema_tokens_10000",
            "value": 36425983.1875,
            "unit": "ns"
          },
          {
            "name": "5.2/index_build_10000",
            "value": 16646256.96875,
            "unit": "ns"
          },
          {
            "name": "5.2/index_search_10000",
            "value": 189970.17745535716,
            "unit": "ns"
          },
          {
            "name": "5.3/full_schema_tokens_10000",
            "value": 18225024.5,
            "unit": "ns"
          },
          {
            "name": "5.3/lazy_visible_prompt_10000",
            "value": 12520.09781146227,
            "unit": "ns"
          },
          {
            "name": "5.4/routed_search_10000",
            "value": 18937.08662613982,
            "unit": "ns"
          },
          {
            "name": "5.4/shrink_schemas_10000",
            "value": 55707039.60416667,
            "unit": "ns"
          },
          {
            "name": "5.5/agent_filter_10000",
            "value": 87542.66436298078,
            "unit": "ns"
          },
          {
            "name": "5.5/cache_cold_10000",
            "value": 21896140.3,
            "unit": "ns"
          },
          {
            "name": "5.5/cache_hit_10000",
            "value": 12.415582193131065,
            "unit": "ns"
          },
          {
            "name": "6.1/gate_check",
            "value": 2.4558385702013616,
            "unit": "ns"
          },
          {
            "name": "6.1/live_tool_body",
            "value": 1474.9942168186421,
            "unit": "ns"
          },
          {
            "name": "6.1/loop_8_reads",
            "value": 2154582.658088235,
            "unit": "ns"
          },
          {
            "name": "6.1/loop_iteration",
            "value": 287.5471223595523,
            "unit": "ns"
          },
          {
            "name": "6.10/budget_notice",
            "value": 63.06444308758469,
            "unit": "ns"
          },
          {
            "name": "6.10/cancel_select",
            "value": 91.07111866846284,
            "unit": "ns"
          },
          {
            "name": "6.10/canonical_args",
            "value": 856.6407590933673,
            "unit": "ns"
          },
          {
            "name": "6.10/gate_check",
            "value": 5.661479960818518,
            "unit": "ns"
          },
          {
            "name": "6.10/live_tool_body",
            "value": 1509.8021940292258,
            "unit": "ns"
          },
          {
            "name": "6.10/loop_8_reads",
            "value": 2197945.780252101,
            "unit": "ns"
          },
          {
            "name": "6.10/loop_iteration",
            "value": 581.9344741292701,
            "unit": "ns"
          },
          {
            "name": "6.10/outcome_label",
            "value": 7.657620046511882,
            "unit": "ns"
          },
          {
            "name": "6.10/stall_observe",
            "value": 113.46332914719324,
            "unit": "ns"
          },
          {
            "name": "6.10/steer_append",
            "value": 45.57258975619216,
            "unit": "ns"
          },
          {
            "name": "6.10/stress_repair",
            "value": 30197.14143537987,
            "unit": "ns"
          },
          {
            "name": "6.10/truncate_output",
            "value": 3376.562161315765,
            "unit": "ns"
          },
          {
            "name": "6.10/validate_repair",
            "value": 205.24649049059747,
            "unit": "ns"
          },
          {
            "name": "6.2/cancel_select",
            "value": 91.38549853987456,
            "unit": "ns"
          },
          {
            "name": "6.2/gate_check",
            "value": 5.660112472129454,
            "unit": "ns"
          },
          {
            "name": "6.2/live_tool_body",
            "value": 1472.907957397288,
            "unit": "ns"
          },
          {
            "name": "6.2/loop_8_reads",
            "value": 2171374.1970588234,
            "unit": "ns"
          },
          {
            "name": "6.2/loop_iteration",
            "value": 410.58652945636175,
            "unit": "ns"
          },
          {
            "name": "6.3/cancel_select",
            "value": 89.68844303880871,
            "unit": "ns"
          },
          {
            "name": "6.3/gate_check",
            "value": 5.667585388451915,
            "unit": "ns"
          },
          {
            "name": "6.3/live_tool_body",
            "value": 1507.3265456830313,
            "unit": "ns"
          },
          {
            "name": "6.3/loop_8_reads",
            "value": 2187587.0274509806,
            "unit": "ns"
          },
          {
            "name": "6.3/loop_iteration",
            "value": 406.20795484894415,
            "unit": "ns"
          },
          {
            "name": "6.3/validate_repair",
            "value": 204.1512017747905,
            "unit": "ns"
          },
          {
            "name": "6.4/cancel_select",
            "value": 92.74101447265224,
            "unit": "ns"
          },
          {
            "name": "6.4/gate_check",
            "value": 5.6748022446407065,
            "unit": "ns"
          },
          {
            "name": "6.4/live_tool_body",
            "value": 1519.3135262882565,
            "unit": "ns"
          },
          {
            "name": "6.4/loop_8_reads",
            "value": 2184280.1529411767,
            "unit": "ns"
          },
          {
            "name": "6.4/loop_iteration",
            "value": 448.58750369785014,
            "unit": "ns"
          },
          {
            "name": "6.4/stall_observe",
            "value": 114.96433415376143,
            "unit": "ns"
          },
          {
            "name": "6.4/validate_repair",
            "value": 201.00964873461487,
            "unit": "ns"
          },
          {
            "name": "6.5/budget_notice",
            "value": 67.8810943987169,
            "unit": "ns"
          },
          {
            "name": "6.5/cancel_select",
            "value": 93.51442634982413,
            "unit": "ns"
          },
          {
            "name": "6.5/gate_check",
            "value": 5.66827057561348,
            "unit": "ns"
          },
          {
            "name": "6.5/live_tool_body",
            "value": 1510.137962616693,
            "unit": "ns"
          },
          {
            "name": "6.5/loop_8_reads",
            "value": 2189388.3513071896,
            "unit": "ns"
          },
          {
            "name": "6.5/loop_iteration",
            "value": 463.1104217227254,
            "unit": "ns"
          },
          {
            "name": "6.5/stall_observe",
            "value": 117.52927487951882,
            "unit": "ns"
          },
          {
            "name": "6.5/validate_repair",
            "value": 204.91413636931708,
            "unit": "ns"
          },
          {
            "name": "6.6/budget_notice",
            "value": 63.436306705389,
            "unit": "ns"
          },
          {
            "name": "6.6/cancel_select",
            "value": 90.32541068893325,
            "unit": "ns"
          },
          {
            "name": "6.6/gate_check",
            "value": 5.682225361712239,
            "unit": "ns"
          },
          {
            "name": "6.6/live_tool_body",
            "value": 1487.454226591277,
            "unit": "ns"
          },
          {
            "name": "6.6/loop_8_reads",
            "value": 2199122.2781862747,
            "unit": "ns"
          },
          {
            "name": "6.6/loop_iteration",
            "value": 460.9847983225735,
            "unit": "ns"
          },
          {
            "name": "6.6/stall_observe",
            "value": 116.92167467230402,
            "unit": "ns"
          },
          {
            "name": "6.6/truncate_output",
            "value": 3456.207014890647,
            "unit": "ns"
          },
          {
            "name": "6.6/validate_repair",
            "value": 204.47033826781222,
            "unit": "ns"
          },
          {
            "name": "6.7/budget_notice",
            "value": 63.783417939177774,
            "unit": "ns"
          },
          {
            "name": "6.7/cancel_select",
            "value": 90.42055209517659,
            "unit": "ns"
          },
          {
            "name": "6.7/gate_check",
            "value": 5.731642000231297,
            "unit": "ns"
          },
          {
            "name": "6.7/live_tool_body",
            "value": 1477.4163628738595,
            "unit": "ns"
          },
          {
            "name": "6.7/loop_8_reads",
            "value": 2180512.2411764706,
            "unit": "ns"
          },
          {
            "name": "6.7/loop_iteration",
            "value": 456.71386288526423,
            "unit": "ns"
          },
          {
            "name": "6.7/stall_observe",
            "value": 109.13124720668783,
            "unit": "ns"
          },
          {
            "name": "6.7/steer_append",
            "value": 45.577944904763996,
            "unit": "ns"
          },
          {
            "name": "6.7/truncate_output",
            "value": 3381.9417724609375,
            "unit": "ns"
          },
          {
            "name": "6.7/validate_repair",
            "value": 192.9687588578501,
            "unit": "ns"
          },
          {
            "name": "6.8/budget_notice",
            "value": 64.42988366608628,
            "unit": "ns"
          },
          {
            "name": "6.8/cancel_select",
            "value": 92.61331041842689,
            "unit": "ns"
          },
          {
            "name": "6.8/canonical_args",
            "value": 871.6702843647349,
            "unit": "ns"
          },
          {
            "name": "6.8/gate_check",
            "value": 5.673403697176422,
            "unit": "ns"
          },
          {
            "name": "6.8/live_tool_body",
            "value": 1512.7138628425298,
            "unit": "ns"
          },
          {
            "name": "6.8/loop_8_reads",
            "value": 2178561.7268907563,
            "unit": "ns"
          },
          {
            "name": "6.8/loop_iteration",
            "value": 526.3122347696954,
            "unit": "ns"
          },
          {
            "name": "6.8/stall_observe",
            "value": 115.43940147881695,
            "unit": "ns"
          },
          {
            "name": "6.8/steer_append",
            "value": 45.55745508548429,
            "unit": "ns"
          },
          {
            "name": "6.8/truncate_output",
            "value": 3515.821050516587,
            "unit": "ns"
          },
          {
            "name": "6.8/validate_repair",
            "value": 206.20096030647807,
            "unit": "ns"
          },
          {
            "name": "6.9/budget_notice",
            "value": 69.74643730328238,
            "unit": "ns"
          },
          {
            "name": "6.9/cancel_select",
            "value": 91.45541599485645,
            "unit": "ns"
          },
          {
            "name": "6.9/canonical_args",
            "value": 879.1229336122996,
            "unit": "ns"
          },
          {
            "name": "6.9/gate_check",
            "value": 5.671707838053852,
            "unit": "ns"
          },
          {
            "name": "6.9/live_tool_body",
            "value": 1517.2287680429674,
            "unit": "ns"
          },
          {
            "name": "6.9/loop_8_reads",
            "value": 2193477.6503267973,
            "unit": "ns"
          },
          {
            "name": "6.9/loop_iteration",
            "value": 594.5887897081611,
            "unit": "ns"
          },
          {
            "name": "6.9/outcome_label",
            "value": 7.658651385557013,
            "unit": "ns"
          },
          {
            "name": "6.9/stall_observe",
            "value": 115.25192097412213,
            "unit": "ns"
          },
          {
            "name": "6.9/steer_append",
            "value": 45.97150833965159,
            "unit": "ns"
          },
          {
            "name": "6.9/truncate_output",
            "value": 3507.106179389208,
            "unit": "ns"
          },
          {
            "name": "6.9/validate_repair",
            "value": 206.4517651726929,
            "unit": "ns"
          },
          {
            "name": "gw5/echo_through_gate",
            "value": 38.30699256289247,
            "unit": "ns"
          },
          {
            "name": "gw5/eight_concurrent_sleeps",
            "value": 2126958.109722222,
            "unit": "ns"
          },
          {
            "name": "gw7/catalog_arc_swap",
            "value": 1953.7059194292458,
            "unit": "ns"
          },
          {
            "name": "gw7/catalog_reload_10k",
            "value": 22074690.472222224,
            "unit": "ns"
          },
          {
            "name": "gw7/catalog_search_10k",
            "value": 190622.8282722513,
            "unit": "ns"
          },
          {
            "name": "gw8/dynamic_rename_task",
            "value": 4505645.416666666,
            "unit": "ns"
          },
          {
            "name": "gw8/every_tool_rename_task",
            "value": 6437009.5,
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
          "id": "754d7d7838eb0602120d118e050f32274db00391",
          "message": "Chapters 21–30: skills, reflection, and team patterns",
          "timestamp": "2026-10-15T04:44:13Z",
          "url": "https://github.com/agnathan/rust-agent-harness/pull/6/commits/754d7d7838eb0602120d118e050f32274db00391"
        },
        "date": 1792200311204,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "1.1/serde_50_message_request",
            "value": 10290.532429775281,
            "unit": "ns"
          },
          {
            "name": "1.2/mock_100_concurrent",
            "value": 78897.42401433692,
            "unit": "ns"
          },
          {
            "name": "1.2/mock_turn_overhead",
            "value": 249.8836018144693,
            "unit": "ns"
          },
          {
            "name": "1.3/build_request_and_parse_response",
            "value": 2934.499034007444,
            "unit": "ns"
          },
          {
            "name": "1.4/parse_response_edges",
            "value": 1105.5936758773391,
            "unit": "ns"
          },
          {
            "name": "10/scripted_turn",
            "value": 7.790049001806941,
            "unit": "ns"
          },
          {
            "name": "10/turn_transition",
            "value": 1.1005900993027793,
            "unit": "ns"
          },
          {
            "name": "11/skip_decision",
            "value": 251.10222932385489,
            "unit": "ns"
          },
          {
            "name": "12/api_request_hook",
            "value": 2.1789682723059967,
            "unit": "ns"
          },
          {
            "name": "12/empty_pre_post",
            "value": 10.904248249004967,
            "unit": "ns"
          },
          {
            "name": "12/memory_enqueue",
            "value": 54.01967578227858,
            "unit": "ns"
          },
          {
            "name": "13/checkpoint_1000",
            "value": 50368.91141272043,
            "unit": "ns"
          },
          {
            "name": "13/rewind_1000",
            "value": 79808.8283235203,
            "unit": "ns"
          },
          {
            "name": "13/warm_accept",
            "value": 27884.133265832104,
            "unit": "ns"
          },
          {
            "name": "14/empty_budget",
            "value": 0.9036268135255298,
            "unit": "ns"
          },
          {
            "name": "14/repetition_guard",
            "value": 2544.4760637499626,
            "unit": "ns"
          },
          {
            "name": "15/open_breaker",
            "value": 30.46332101312342,
            "unit": "ns"
          },
          {
            "name": "15/permit_release",
            "value": 61.803381474035476,
            "unit": "ns"
          },
          {
            "name": "16/choose_route",
            "value": 5.914942645073674,
            "unit": "ns"
          },
          {
            "name": "16/rotate_key",
            "value": 3.445499980954316,
            "unit": "ns"
          },
          {
            "name": "17/buffer_join",
            "value": 696.0381216267843,
            "unit": "ns"
          },
          {
            "name": "17/fold_deltas",
            "value": 2834.548103866591,
            "unit": "ns"
          },
          {
            "name": "18/wal_append",
            "value": 139776.30143805308,
            "unit": "ns"
          },
          {
            "name": "19/memory_enqueue",
            "value": 53.942784043304755,
            "unit": "ns"
          },
          {
            "name": "2/chat_turn_overhead",
            "value": 691.0525415124222,
            "unit": "ns"
          },
          {
            "name": "2/inbox_submit",
            "value": 40.43119209899599,
            "unit": "ns"
          },
          {
            "name": "20/discover_10k",
            "value": 60504.01640365449,
            "unit": "ns"
          },
          {
            "name": "21/apply_offered",
            "value": 112.43309291524722,
            "unit": "ns"
          },
          {
            "name": "21/predict_offline",
            "value": 287650.0794666667,
            "unit": "ns"
          },
          {
            "name": "22/load_render_500",
            "value": 5843927.094444444,
            "unit": "ns"
          },
          {
            "name": "23/enqueue_unsolicited",
            "value": 18.993429753929362,
            "unit": "ns"
          },
          {
            "name": "23/scan_one_skill",
            "value": 280.813656513067,
            "unit": "ns"
          },
          {
            "name": "24/cancel_review",
            "value": 697496.5952380953,
            "unit": "ns"
          },
          {
            "name": "24/spawn_reviewer",
            "value": 85.623329139913,
            "unit": "ns"
          },
          {
            "name": "25/enqueue_curation",
            "value": 20.344553678690456,
            "unit": "ns"
          },
          {
            "name": "26/arc_pass",
            "value": 4.508303878958282,
            "unit": "ns"
          },
          {
            "name": "26/copy_transcript",
            "value": 7543.023248725013,
            "unit": "ns"
          },
          {
            "name": "27/enqueue_reflection",
            "value": 19.344418510698617,
            "unit": "ns"
          },
          {
            "name": "27/schedule_eight",
            "value": 277621.0807463953,
            "unit": "ns"
          },
          {
            "name": "3.1/registry_lookup",
            "value": 25.959214871790486,
            "unit": "ns"
          },
          {
            "name": "3.2/available_tools_50",
            "value": 31243.231653965566,
            "unit": "ns"
          },
          {
            "name": "3.3/dispatch_8_reads_concurrent",
            "value": 2129481.6225490198,
            "unit": "ns"
          },
          {
            "name": "3.3/dispatch_8_reads_sequential",
            "value": 16724224.441666666,
            "unit": "ns"
          },
          {
            "name": "4.1/stdio_echo_round_trip",
            "value": 202749.65408020752,
            "unit": "ns"
          },
          {
            "name": "4.2/http_echo_round_trip",
            "value": 421334.64504652435,
            "unit": "ns"
          },
          {
            "name": "4.3/mcp_50_concurrent",
            "value": 4949848.09375,
            "unit": "ns"
          },
          {
            "name": "4.3/mcp_50_sequential",
            "value": 10168045.875,
            "unit": "ns"
          },
          {
            "name": "4.4/mcp_list_cold",
            "value": 213560.51498074573,
            "unit": "ns"
          },
          {
            "name": "4.4/mcp_list_hit",
            "value": 254.91657014782857,
            "unit": "ns"
          },
          {
            "name": "5.1/schema_tokens_1000",
            "value": 1257887.0745073892,
            "unit": "ns"
          },
          {
            "name": "5.1/schema_tokens_10000",
            "value": 40226388.08333333,
            "unit": "ns"
          },
          {
            "name": "5.2/index_build_10000",
            "value": 20515287.641666666,
            "unit": "ns"
          },
          {
            "name": "5.2/index_search_10000",
            "value": 334029.77981651376,
            "unit": "ns"
          },
          {
            "name": "5.3/full_schema_tokens_10000",
            "value": 19084839.875,
            "unit": "ns"
          },
          {
            "name": "5.3/lazy_visible_prompt_10000",
            "value": 15536.828566341066,
            "unit": "ns"
          },
          {
            "name": "5.4/routed_search_10000",
            "value": 26039.40869487909,
            "unit": "ns"
          },
          {
            "name": "5.4/shrink_schemas_10000",
            "value": 59105145.05357143,
            "unit": "ns"
          },
          {
            "name": "5.5/agent_filter_10000",
            "value": 124606.78865979382,
            "unit": "ns"
          },
          {
            "name": "5.5/cache_cold_10000",
            "value": 22740493.82142857,
            "unit": "ns"
          },
          {
            "name": "5.5/cache_hit_10000",
            "value": 13.797246872494702,
            "unit": "ns"
          },
          {
            "name": "6.1/gate_check",
            "value": 3.423948065268922,
            "unit": "ns"
          },
          {
            "name": "6.1/live_tool_body",
            "value": 1850.2671470925306,
            "unit": "ns"
          },
          {
            "name": "6.1/loop_8_reads",
            "value": 2156877.5274509806,
            "unit": "ns"
          },
          {
            "name": "6.1/loop_iteration",
            "value": 360.29858228152546,
            "unit": "ns"
          },
          {
            "name": "6.10/budget_notice",
            "value": 79.26313363187805,
            "unit": "ns"
          },
          {
            "name": "6.10/cancel_select",
            "value": 96.22844447634043,
            "unit": "ns"
          },
          {
            "name": "6.10/canonical_args",
            "value": 1062.0286612209675,
            "unit": "ns"
          },
          {
            "name": "6.10/gate_check",
            "value": 7.467763224473872,
            "unit": "ns"
          },
          {
            "name": "6.10/live_tool_body",
            "value": 1774.2270138167933,
            "unit": "ns"
          },
          {
            "name": "6.10/loop_8_reads",
            "value": 2177219.0798319327,
            "unit": "ns"
          },
          {
            "name": "6.10/loop_iteration",
            "value": 739.5566398043426,
            "unit": "ns"
          },
          {
            "name": "6.10/outcome_label",
            "value": 8.398426844700833,
            "unit": "ns"
          },
          {
            "name": "6.10/stall_observe",
            "value": 138.0911078581895,
            "unit": "ns"
          },
          {
            "name": "6.10/steer_append",
            "value": 59.85914843148355,
            "unit": "ns"
          },
          {
            "name": "6.10/stress_repair",
            "value": 51843.20356321839,
            "unit": "ns"
          },
          {
            "name": "6.10/truncate_output",
            "value": 4683.224084892614,
            "unit": "ns"
          },
          {
            "name": "6.10/validate_repair",
            "value": 270.57804515463613,
            "unit": "ns"
          },
          {
            "name": "6.2/cancel_select",
            "value": 94.95415688423155,
            "unit": "ns"
          },
          {
            "name": "6.2/gate_check",
            "value": 7.476170717988774,
            "unit": "ns"
          },
          {
            "name": "6.2/live_tool_body",
            "value": 1765.3406675289243,
            "unit": "ns"
          },
          {
            "name": "6.2/loop_8_reads",
            "value": 2159922.2852941174,
            "unit": "ns"
          },
          {
            "name": "6.2/loop_iteration",
            "value": 539.1354557625837,
            "unit": "ns"
          },
          {
            "name": "6.3/cancel_select",
            "value": 97.00821910851155,
            "unit": "ns"
          },
          {
            "name": "6.3/gate_check",
            "value": 7.47516962801784,
            "unit": "ns"
          },
          {
            "name": "6.3/live_tool_body",
            "value": 1747.3208553262957,
            "unit": "ns"
          },
          {
            "name": "6.3/loop_8_reads",
            "value": 2169864.5794117646,
            "unit": "ns"
          },
          {
            "name": "6.3/loop_iteration",
            "value": 535.8059898014666,
            "unit": "ns"
          },
          {
            "name": "6.3/validate_repair",
            "value": 260.1774434009602,
            "unit": "ns"
          },
          {
            "name": "6.4/cancel_select",
            "value": 96.78957879185818,
            "unit": "ns"
          },
          {
            "name": "6.4/gate_check",
            "value": 7.472160653204558,
            "unit": "ns"
          },
          {
            "name": "6.4/live_tool_body",
            "value": 1752.6623258885688,
            "unit": "ns"
          },
          {
            "name": "6.4/loop_8_reads",
            "value": 2170979.7901960784,
            "unit": "ns"
          },
          {
            "name": "6.4/loop_iteration",
            "value": 555.1344828294657,
            "unit": "ns"
          },
          {
            "name": "6.4/stall_observe",
            "value": 139.25684958132584,
            "unit": "ns"
          },
          {
            "name": "6.4/validate_repair",
            "value": 263.1465303070896,
            "unit": "ns"
          },
          {
            "name": "6.5/budget_notice",
            "value": 76.75243878765735,
            "unit": "ns"
          },
          {
            "name": "6.5/cancel_select",
            "value": 96.93644326449981,
            "unit": "ns"
          },
          {
            "name": "6.5/gate_check",
            "value": 7.473022610044799,
            "unit": "ns"
          },
          {
            "name": "6.5/live_tool_body",
            "value": 1760.0750367394926,
            "unit": "ns"
          },
          {
            "name": "6.5/loop_8_reads",
            "value": 2176522.543137255,
            "unit": "ns"
          },
          {
            "name": "6.5/loop_iteration",
            "value": 597.6696847324952,
            "unit": "ns"
          },
          {
            "name": "6.5/stall_observe",
            "value": 136.55222551873064,
            "unit": "ns"
          },
          {
            "name": "6.5/validate_repair",
            "value": 270.95955050328456,
            "unit": "ns"
          },
          {
            "name": "6.6/budget_notice",
            "value": 76.55741002274881,
            "unit": "ns"
          },
          {
            "name": "6.6/cancel_select",
            "value": 96.13982127960263,
            "unit": "ns"
          },
          {
            "name": "6.6/gate_check",
            "value": 7.47089413811863,
            "unit": "ns"
          },
          {
            "name": "6.6/live_tool_body",
            "value": 1724.6240215287285,
            "unit": "ns"
          },
          {
            "name": "6.6/loop_8_reads",
            "value": 2178554.1029411764,
            "unit": "ns"
          },
          {
            "name": "6.6/loop_iteration",
            "value": 622.2487429371209,
            "unit": "ns"
          },
          {
            "name": "6.6/stall_observe",
            "value": 139.03325613625432,
            "unit": "ns"
          },
          {
            "name": "6.6/truncate_output",
            "value": 4652.04880832705,
            "unit": "ns"
          },
          {
            "name": "6.6/validate_repair",
            "value": 268.15266333840873,
            "unit": "ns"
          },
          {
            "name": "6.7/budget_notice",
            "value": 76.84689611913367,
            "unit": "ns"
          },
          {
            "name": "6.7/cancel_select",
            "value": 99.2253661025832,
            "unit": "ns"
          },
          {
            "name": "6.7/gate_check",
            "value": 7.477664709033728,
            "unit": "ns"
          },
          {
            "name": "6.7/live_tool_body",
            "value": 1623.9322414808357,
            "unit": "ns"
          },
          {
            "name": "6.7/loop_8_reads",
            "value": 2177561.4529411765,
            "unit": "ns"
          },
          {
            "name": "6.7/loop_iteration",
            "value": 620.6193405529718,
            "unit": "ns"
          },
          {
            "name": "6.7/stall_observe",
            "value": 137.92294877276356,
            "unit": "ns"
          },
          {
            "name": "6.7/steer_append",
            "value": 58.35961971790813,
            "unit": "ns"
          },
          {
            "name": "6.7/truncate_output",
            "value": 4665.8735726192,
            "unit": "ns"
          },
          {
            "name": "6.7/validate_repair",
            "value": 271.78962881909547,
            "unit": "ns"
          },
          {
            "name": "6.8/budget_notice",
            "value": 79.11155336412168,
            "unit": "ns"
          },
          {
            "name": "6.8/cancel_select",
            "value": 97.88155709541986,
            "unit": "ns"
          },
          {
            "name": "6.8/canonical_args",
            "value": 1074.444003924114,
            "unit": "ns"
          },
          {
            "name": "6.8/gate_check",
            "value": 7.4674073150759455,
            "unit": "ns"
          },
          {
            "name": "6.8/live_tool_body",
            "value": 1758.2556740826167,
            "unit": "ns"
          },
          {
            "name": "6.8/loop_8_reads",
            "value": 2176241.6382352943,
            "unit": "ns"
          },
          {
            "name": "6.8/loop_iteration",
            "value": 696.7652655752702,
            "unit": "ns"
          },
          {
            "name": "6.8/stall_observe",
            "value": 138.10071729357537,
            "unit": "ns"
          },
          {
            "name": "6.8/steer_append",
            "value": 58.01387563702042,
            "unit": "ns"
          },
          {
            "name": "6.8/truncate_output",
            "value": 4569.625521576809,
            "unit": "ns"
          },
          {
            "name": "6.8/validate_repair",
            "value": 258.0384532154994,
            "unit": "ns"
          },
          {
            "name": "6.9/budget_notice",
            "value": 77.1016129126134,
            "unit": "ns"
          },
          {
            "name": "6.9/cancel_select",
            "value": 97.80591225154225,
            "unit": "ns"
          },
          {
            "name": "6.9/canonical_args",
            "value": 1060.1971853441023,
            "unit": "ns"
          },
          {
            "name": "6.9/gate_check",
            "value": 7.468607990645118,
            "unit": "ns"
          },
          {
            "name": "6.9/live_tool_body",
            "value": 1800.8903734993332,
            "unit": "ns"
          },
          {
            "name": "6.9/loop_8_reads",
            "value": 2181695.312184874,
            "unit": "ns"
          },
          {
            "name": "6.9/loop_iteration",
            "value": 748.8649353721637,
            "unit": "ns"
          },
          {
            "name": "6.9/outcome_label",
            "value": 8.399120488506147,
            "unit": "ns"
          },
          {
            "name": "6.9/stall_observe",
            "value": 135.42711716872833,
            "unit": "ns"
          },
          {
            "name": "6.9/steer_append",
            "value": 59.17274767280073,
            "unit": "ns"
          },
          {
            "name": "6.9/truncate_output",
            "value": 4693.216903951638,
            "unit": "ns"
          },
          {
            "name": "6.9/validate_repair",
            "value": 248.87449876830436,
            "unit": "ns"
          },
          {
            "name": "gw5/echo_through_gate",
            "value": 45.52694168469097,
            "unit": "ns"
          },
          {
            "name": "gw5/eight_concurrent_sleeps",
            "value": 2127374.459876543,
            "unit": "ns"
          },
          {
            "name": "gw7/catalog_arc_swap",
            "value": 2373.631890449744,
            "unit": "ns"
          },
          {
            "name": "gw7/catalog_reload_10k",
            "value": 26657354.875,
            "unit": "ns"
          },
          {
            "name": "gw7/catalog_search_10k",
            "value": 309865.78908898303,
            "unit": "ns"
          },
          {
            "name": "gw8/dynamic_rename_task",
            "value": 5694728.003571428,
            "unit": "ns"
          },
          {
            "name": "gw8/every_tool_rename_task",
            "value": 8375730.771428572,
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
          "id": "12a9b2a6ff588d55f42d25c8ae5ed06e5aa3d1a2",
          "message": "Chapters 21–30: skills, reflection, and team patterns",
          "timestamp": "2026-10-15T04:44:13Z",
          "url": "https://github.com/agnathan/rust-agent-harness/pull/6/commits/12a9b2a6ff588d55f42d25c8ae5ed06e5aa3d1a2"
        },
        "date": 1792200328784,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "1.1/serde_50_message_request",
            "value": 6466.889134796978,
            "unit": "ns"
          },
          {
            "name": "1.2/mock_100_concurrent",
            "value": 89847.7776119403,
            "unit": "ns"
          },
          {
            "name": "1.2/mock_turn_overhead",
            "value": 196.9927638321983,
            "unit": "ns"
          },
          {
            "name": "1.3/build_request_and_parse_response",
            "value": 1870.440453029461,
            "unit": "ns"
          },
          {
            "name": "1.4/parse_response_edges",
            "value": 789.0008289567508,
            "unit": "ns"
          },
          {
            "name": "10/scripted_turn",
            "value": 6.242307962670135,
            "unit": "ns"
          },
          {
            "name": "10/turn_transition",
            "value": 0.6000844259090488,
            "unit": "ns"
          },
          {
            "name": "11/skip_decision",
            "value": 177.82247902764772,
            "unit": "ns"
          },
          {
            "name": "12/api_request_hook",
            "value": 1.6836361074285984,
            "unit": "ns"
          },
          {
            "name": "12/empty_pre_post",
            "value": 8.984565305790618,
            "unit": "ns"
          },
          {
            "name": "12/memory_enqueue",
            "value": 36.14691899286442,
            "unit": "ns"
          },
          {
            "name": "13/checkpoint_1000",
            "value": 60111.13308515589,
            "unit": "ns"
          },
          {
            "name": "13/rewind_1000",
            "value": 67491.64036697248,
            "unit": "ns"
          },
          {
            "name": "13/warm_accept",
            "value": 23923.79222046659,
            "unit": "ns"
          },
          {
            "name": "14/empty_budget",
            "value": 0.6605430665216364,
            "unit": "ns"
          },
          {
            "name": "14/repetition_guard",
            "value": 1620.9686731276488,
            "unit": "ns"
          },
          {
            "name": "15/open_breaker",
            "value": 19.27737242777507,
            "unit": "ns"
          },
          {
            "name": "15/permit_release",
            "value": 71.48963824842417,
            "unit": "ns"
          },
          {
            "name": "16/choose_route",
            "value": 3.164656437443007,
            "unit": "ns"
          },
          {
            "name": "16/rotate_key",
            "value": 4.1872061279053305,
            "unit": "ns"
          },
          {
            "name": "17/buffer_join",
            "value": 437.8965121852238,
            "unit": "ns"
          },
          {
            "name": "17/fold_deltas",
            "value": 1669.9010172143976,
            "unit": "ns"
          },
          {
            "name": "18/wal_append",
            "value": 270497.75787815126,
            "unit": "ns"
          },
          {
            "name": "19/memory_enqueue",
            "value": 35.40038266787505,
            "unit": "ns"
          },
          {
            "name": "2/chat_turn_overhead",
            "value": 536.2933514412182,
            "unit": "ns"
          },
          {
            "name": "2/inbox_submit",
            "value": 32.568047480966186,
            "unit": "ns"
          },
          {
            "name": "20/discover_10k",
            "value": 26306.698692551505,
            "unit": "ns"
          },
          {
            "name": "21/apply_offered",
            "value": 78.9034567869262,
            "unit": "ns"
          },
          {
            "name": "21/predict_offline",
            "value": 150086.2995951417,
            "unit": "ns"
          },
          {
            "name": "22/load_render_500",
            "value": 2181835.5857843137,
            "unit": "ns"
          },
          {
            "name": "23/enqueue_unsolicited",
            "value": 17.95848977506884,
            "unit": "ns"
          },
          {
            "name": "23/scan_one_skill",
            "value": 242.42567150879316,
            "unit": "ns"
          },
          {
            "name": "24/cancel_review",
            "value": 800132.6024305555,
            "unit": "ns"
          },
          {
            "name": "24/spawn_reviewer",
            "value": 62.637924394901006,
            "unit": "ns"
          },
          {
            "name": "25/enqueue_curation",
            "value": 27.631107032934853,
            "unit": "ns"
          },
          {
            "name": "26/arc_pass",
            "value": 10.344680951859837,
            "unit": "ns"
          },
          {
            "name": "26/copy_transcript",
            "value": 6506.4865504017525,
            "unit": "ns"
          },
          {
            "name": "27/enqueue_reflection",
            "value": 20.796138327526528,
            "unit": "ns"
          },
          {
            "name": "27/schedule_eight",
            "value": 148298.36081081082,
            "unit": "ns"
          },
          {
            "name": "28/filter_100",
            "value": 2602.039929095703,
            "unit": "ns"
          },
          {
            "name": "29/schedule_100",
            "value": 4270.1059505455405,
            "unit": "ns"
          },
          {
            "name": "3.1/registry_lookup",
            "value": 13.548921281813978,
            "unit": "ns"
          },
          {
            "name": "3.2/available_tools_50",
            "value": 21134.207554866607,
            "unit": "ns"
          },
          {
            "name": "3.3/dispatch_8_reads_concurrent",
            "value": 2137462.151234568,
            "unit": "ns"
          },
          {
            "name": "3.3/dispatch_8_reads_sequential",
            "value": 16730321.154761905,
            "unit": "ns"
          },
          {
            "name": "30/judge_one_permit",
            "value": 195.46892629565554,
            "unit": "ns"
          },
          {
            "name": "4.1/stdio_echo_round_trip",
            "value": 135973.20211267605,
            "unit": "ns"
          },
          {
            "name": "4.2/http_echo_round_trip",
            "value": 313633.06895280234,
            "unit": "ns"
          },
          {
            "name": "4.3/mcp_50_concurrent",
            "value": 3123990.6027777777,
            "unit": "ns"
          },
          {
            "name": "4.3/mcp_50_sequential",
            "value": 6833821.41005291,
            "unit": "ns"
          },
          {
            "name": "4.4/mcp_list_cold",
            "value": 148255.860866373,
            "unit": "ns"
          },
          {
            "name": "4.4/mcp_list_hit",
            "value": 168.33763572559434,
            "unit": "ns"
          },
          {
            "name": "5.1/schema_tokens_1000",
            "value": 932420.2181944444,
            "unit": "ns"
          },
          {
            "name": "5.1/schema_tokens_10000",
            "value": 31088649.708333336,
            "unit": "ns"
          },
          {
            "name": "5.2/index_build_10000",
            "value": 14646455.222222222,
            "unit": "ns"
          },
          {
            "name": "5.2/index_search_10000",
            "value": 145008.8572,
            "unit": "ns"
          },
          {
            "name": "5.3/full_schema_tokens_10000",
            "value": 15653443.227777777,
            "unit": "ns"
          },
          {
            "name": "5.3/lazy_visible_prompt_10000",
            "value": 12192.49291482745,
            "unit": "ns"
          },
          {
            "name": "5.4/routed_search_10000",
            "value": 15476.787080825452,
            "unit": "ns"
          },
          {
            "name": "5.4/shrink_schemas_10000",
            "value": 51037211.7,
            "unit": "ns"
          },
          {
            "name": "5.5/agent_filter_10000",
            "value": 54403.14799824984,
            "unit": "ns"
          },
          {
            "name": "5.5/cache_cold_10000",
            "value": 18124273.014285713,
            "unit": "ns"
          },
          {
            "name": "5.5/cache_hit_10000",
            "value": 17.024441388211315,
            "unit": "ns"
          },
          {
            "name": "6.1/gate_check",
            "value": 2.404575152316864,
            "unit": "ns"
          },
          {
            "name": "6.1/live_tool_body",
            "value": 1536.9068461943837,
            "unit": "ns"
          },
          {
            "name": "6.1/loop_8_reads",
            "value": 2159650.044117647,
            "unit": "ns"
          },
          {
            "name": "6.1/loop_iteration",
            "value": 238.60715204135306,
            "unit": "ns"
          },
          {
            "name": "6.10/budget_notice",
            "value": 48.59897004877388,
            "unit": "ns"
          },
          {
            "name": "6.10/cancel_select",
            "value": 157.60531467048867,
            "unit": "ns"
          },
          {
            "name": "6.10/canonical_args",
            "value": 774.1522068249127,
            "unit": "ns"
          },
          {
            "name": "6.10/gate_check",
            "value": 11.827268046577984,
            "unit": "ns"
          },
          {
            "name": "6.10/live_tool_body",
            "value": 1424.6179556937427,
            "unit": "ns"
          },
          {
            "name": "6.10/loop_8_reads",
            "value": 2161322.7264705882,
            "unit": "ns"
          },
          {
            "name": "6.10/loop_iteration",
            "value": 617.2021004169262,
            "unit": "ns"
          },
          {
            "name": "6.10/outcome_label",
            "value": 5.619245385551471,
            "unit": "ns"
          },
          {
            "name": "6.10/stall_observe",
            "value": 91.07849147132956,
            "unit": "ns"
          },
          {
            "name": "6.10/steer_append",
            "value": 42.99982765092833,
            "unit": "ns"
          },
          {
            "name": "6.10/stress_repair",
            "value": 46017.93193069307,
            "unit": "ns"
          },
          {
            "name": "6.10/truncate_output",
            "value": 2950.0890461172435,
            "unit": "ns"
          },
          {
            "name": "6.10/validate_repair",
            "value": 217.04144326651638,
            "unit": "ns"
          },
          {
            "name": "6.2/cancel_select",
            "value": 147.6577380588073,
            "unit": "ns"
          },
          {
            "name": "6.2/gate_check",
            "value": 11.502108727951454,
            "unit": "ns"
          },
          {
            "name": "6.2/live_tool_body",
            "value": 1380.0622249724972,
            "unit": "ns"
          },
          {
            "name": "6.2/loop_8_reads",
            "value": 2168283.345798319,
            "unit": "ns"
          },
          {
            "name": "6.2/loop_iteration",
            "value": 446.68903954973473,
            "unit": "ns"
          },
          {
            "name": "6.3/cancel_select",
            "value": 156.7713075532291,
            "unit": "ns"
          },
          {
            "name": "6.3/gate_check",
            "value": 11.428517019622447,
            "unit": "ns"
          },
          {
            "name": "6.3/live_tool_body",
            "value": 1550.4204351290336,
            "unit": "ns"
          },
          {
            "name": "6.3/loop_8_reads",
            "value": 2168298.344771242,
            "unit": "ns"
          },
          {
            "name": "6.3/loop_iteration",
            "value": 462.3467707285379,
            "unit": "ns"
          },
          {
            "name": "6.3/validate_repair",
            "value": 188.93089142328174,
            "unit": "ns"
          },
          {
            "name": "6.4/cancel_select",
            "value": 153.1401093419023,
            "unit": "ns"
          },
          {
            "name": "6.4/gate_check",
            "value": 11.269885744306679,
            "unit": "ns"
          },
          {
            "name": "6.4/live_tool_body",
            "value": 1602.14213675753,
            "unit": "ns"
          },
          {
            "name": "6.4/loop_8_reads",
            "value": 2155728.1285714284,
            "unit": "ns"
          },
          {
            "name": "6.4/loop_iteration",
            "value": 441.9736626942687,
            "unit": "ns"
          },
          {
            "name": "6.4/stall_observe",
            "value": 93.95277934696645,
            "unit": "ns"
          },
          {
            "name": "6.4/validate_repair",
            "value": 207.4031743233443,
            "unit": "ns"
          },
          {
            "name": "6.5/budget_notice",
            "value": 50.009042592259206,
            "unit": "ns"
          },
          {
            "name": "6.5/cancel_select",
            "value": 148.15237205503678,
            "unit": "ns"
          },
          {
            "name": "6.5/gate_check",
            "value": 10.955545407476723,
            "unit": "ns"
          },
          {
            "name": "6.5/live_tool_body",
            "value": 1420.3620115409938,
            "unit": "ns"
          },
          {
            "name": "6.5/loop_8_reads",
            "value": 2168806.480392157,
            "unit": "ns"
          },
          {
            "name": "6.5/loop_iteration",
            "value": 458.8568348245365,
            "unit": "ns"
          },
          {
            "name": "6.5/stall_observe",
            "value": 87.95387997439636,
            "unit": "ns"
          },
          {
            "name": "6.5/validate_repair",
            "value": 204.23781749636981,
            "unit": "ns"
          },
          {
            "name": "6.6/budget_notice",
            "value": 54.784986839987695,
            "unit": "ns"
          },
          {
            "name": "6.6/cancel_select",
            "value": 160.1104593244612,
            "unit": "ns"
          },
          {
            "name": "6.6/gate_check",
            "value": 11.94843374150076,
            "unit": "ns"
          },
          {
            "name": "6.6/live_tool_body",
            "value": 1485.8518784578264,
            "unit": "ns"
          },
          {
            "name": "6.6/loop_8_reads",
            "value": 2168685.380252101,
            "unit": "ns"
          },
          {
            "name": "6.6/loop_iteration",
            "value": 506.76824415055955,
            "unit": "ns"
          },
          {
            "name": "6.6/stall_observe",
            "value": 99.4899147594582,
            "unit": "ns"
          },
          {
            "name": "6.6/truncate_output",
            "value": 3066.3224441708417,
            "unit": "ns"
          },
          {
            "name": "6.6/validate_repair",
            "value": 201.6212494576524,
            "unit": "ns"
          },
          {
            "name": "6.7/budget_notice",
            "value": 50.31629951971152,
            "unit": "ns"
          },
          {
            "name": "6.7/cancel_select",
            "value": 147.10785159648862,
            "unit": "ns"
          },
          {
            "name": "6.7/gate_check",
            "value": 11.097116429218826,
            "unit": "ns"
          },
          {
            "name": "6.7/live_tool_body",
            "value": 1425.7376553515119,
            "unit": "ns"
          },
          {
            "name": "6.7/loop_8_reads",
            "value": 2161902.634453782,
            "unit": "ns"
          },
          {
            "name": "6.7/loop_iteration",
            "value": 492.8246063831658,
            "unit": "ns"
          },
          {
            "name": "6.7/stall_observe",
            "value": 94.31333246110779,
            "unit": "ns"
          },
          {
            "name": "6.7/steer_append",
            "value": 43.452712239415135,
            "unit": "ns"
          },
          {
            "name": "6.7/truncate_output",
            "value": 3001.3544518215867,
            "unit": "ns"
          },
          {
            "name": "6.7/validate_repair",
            "value": 204.74491340100434,
            "unit": "ns"
          },
          {
            "name": "6.8/budget_notice",
            "value": 50.571836567124215,
            "unit": "ns"
          },
          {
            "name": "6.8/cancel_select",
            "value": 147.15284643780444,
            "unit": "ns"
          },
          {
            "name": "6.8/canonical_args",
            "value": 814.0828678109674,
            "unit": "ns"
          },
          {
            "name": "6.8/gate_check",
            "value": 11.11866155505334,
            "unit": "ns"
          },
          {
            "name": "6.8/live_tool_body",
            "value": 1406.4482077335592,
            "unit": "ns"
          },
          {
            "name": "6.8/loop_8_reads",
            "value": 2164692.938235294,
            "unit": "ns"
          },
          {
            "name": "6.8/loop_iteration",
            "value": 569.3235159940856,
            "unit": "ns"
          },
          {
            "name": "6.8/stall_observe",
            "value": 97.40333560552008,
            "unit": "ns"
          },
          {
            "name": "6.8/steer_append",
            "value": 48.84388347844725,
            "unit": "ns"
          },
          {
            "name": "6.8/truncate_output",
            "value": 2972.5819100942563,
            "unit": "ns"
          },
          {
            "name": "6.8/validate_repair",
            "value": 204.0352935718097,
            "unit": "ns"
          },
          {
            "name": "6.9/budget_notice",
            "value": 47.36753564733595,
            "unit": "ns"
          },
          {
            "name": "6.9/cancel_select",
            "value": 151.24324127906976,
            "unit": "ns"
          },
          {
            "name": "6.9/canonical_args",
            "value": 784.3938878126435,
            "unit": "ns"
          },
          {
            "name": "6.9/gate_check",
            "value": 11.39125924240324,
            "unit": "ns"
          },
          {
            "name": "6.9/live_tool_body",
            "value": 1416.8313686273145,
            "unit": "ns"
          },
          {
            "name": "6.9/loop_8_reads",
            "value": 2172244.094771242,
            "unit": "ns"
          },
          {
            "name": "6.9/loop_iteration",
            "value": 609.0935200448652,
            "unit": "ns"
          },
          {
            "name": "6.9/outcome_label",
            "value": 5.664880986228102,
            "unit": "ns"
          },
          {
            "name": "6.9/stall_observe",
            "value": 97.61379021459719,
            "unit": "ns"
          },
          {
            "name": "6.9/steer_append",
            "value": 43.01560045657631,
            "unit": "ns"
          },
          {
            "name": "6.9/truncate_output",
            "value": 2968.843535636607,
            "unit": "ns"
          },
          {
            "name": "6.9/validate_repair",
            "value": 228.85279912151208,
            "unit": "ns"
          },
          {
            "name": "gw5/echo_through_gate",
            "value": 73.33540815397225,
            "unit": "ns"
          },
          {
            "name": "gw5/eight_concurrent_sleeps",
            "value": 2116337.0574074076,
            "unit": "ns"
          },
          {
            "name": "gw7/catalog_arc_swap",
            "value": 1796.936067785946,
            "unit": "ns"
          },
          {
            "name": "gw7/catalog_reload_10k",
            "value": 19912900.625,
            "unit": "ns"
          },
          {
            "name": "gw7/catalog_search_10k",
            "value": 134833.16349809885,
            "unit": "ns"
          },
          {
            "name": "gw8/dynamic_rename_task",
            "value": 3929429.45,
            "unit": "ns"
          },
          {
            "name": "gw8/every_tool_rename_task",
            "value": 5593503.633928572,
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
          "id": "f998c179ec8bfc9dfccd183c2a4974a743d5c6d4",
          "message": "Chapters 21–30: skills, reflection, and team patterns",
          "timestamp": "2026-10-15T04:44:13Z",
          "url": "https://github.com/agnathan/rust-agent-harness/pull/6/commits/f998c179ec8bfc9dfccd183c2a4974a743d5c6d4"
        },
        "date": 1792200417496,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "1.1/serde_50_message_request",
            "value": 10232.959934807255,
            "unit": "ns"
          },
          {
            "name": "1.2/mock_100_concurrent",
            "value": 79794.17297979798,
            "unit": "ns"
          },
          {
            "name": "1.2/mock_turn_overhead",
            "value": 252.4611743705673,
            "unit": "ns"
          },
          {
            "name": "1.3/build_request_and_parse_response",
            "value": 2942.361461115842,
            "unit": "ns"
          },
          {
            "name": "1.4/parse_response_edges",
            "value": 1146.5858980370094,
            "unit": "ns"
          },
          {
            "name": "10/scripted_turn",
            "value": 7.792728196844948,
            "unit": "ns"
          },
          {
            "name": "10/turn_transition",
            "value": 1.0916451806194871,
            "unit": "ns"
          },
          {
            "name": "11/skip_decision",
            "value": 251.3014760944037,
            "unit": "ns"
          },
          {
            "name": "12/api_request_hook",
            "value": 2.1812846877135295,
            "unit": "ns"
          },
          {
            "name": "12/empty_pre_post",
            "value": 10.904791362240783,
            "unit": "ns"
          },
          {
            "name": "12/memory_enqueue",
            "value": 54.33520846868572,
            "unit": "ns"
          },
          {
            "name": "13/checkpoint_1000",
            "value": 50745.065792540794,
            "unit": "ns"
          },
          {
            "name": "13/rewind_1000",
            "value": 80526.47189542484,
            "unit": "ns"
          },
          {
            "name": "13/warm_accept",
            "value": 30968.778442938907,
            "unit": "ns"
          },
          {
            "name": "14/empty_budget",
            "value": 0.9037800927202895,
            "unit": "ns"
          },
          {
            "name": "14/repetition_guard",
            "value": 2552.0848092834103,
            "unit": "ns"
          },
          {
            "name": "15/open_breaker",
            "value": 29.98668053419774,
            "unit": "ns"
          },
          {
            "name": "15/permit_release",
            "value": 59.068774261415825,
            "unit": "ns"
          },
          {
            "name": "16/choose_route",
            "value": 5.914717536675381,
            "unit": "ns"
          },
          {
            "name": "16/rotate_key",
            "value": 3.4464379436602304,
            "unit": "ns"
          },
          {
            "name": "17/buffer_join",
            "value": 699.6628322261834,
            "unit": "ns"
          },
          {
            "name": "17/fold_deltas",
            "value": 2835.3113220747105,
            "unit": "ns"
          },
          {
            "name": "18/wal_append",
            "value": 146962.86320754717,
            "unit": "ns"
          },
          {
            "name": "19/memory_enqueue",
            "value": 53.07425580016525,
            "unit": "ns"
          },
          {
            "name": "2/chat_turn_overhead",
            "value": 691.1616294323364,
            "unit": "ns"
          },
          {
            "name": "2/inbox_submit",
            "value": 40.5709411503974,
            "unit": "ns"
          },
          {
            "name": "20/discover_10k",
            "value": 58565.42089861751,
            "unit": "ns"
          },
          {
            "name": "21/apply_offered",
            "value": 112.55009746786996,
            "unit": "ns"
          },
          {
            "name": "21/predict_offline",
            "value": 287294.47322834644,
            "unit": "ns"
          },
          {
            "name": "22/load_render_500",
            "value": 5886045.04489796,
            "unit": "ns"
          },
          {
            "name": "23/enqueue_unsolicited",
            "value": 19.010047524594192,
            "unit": "ns"
          },
          {
            "name": "23/scan_one_skill",
            "value": 281.98272649797565,
            "unit": "ns"
          },
          {
            "name": "24/cancel_review",
            "value": 772047.3362745098,
            "unit": "ns"
          },
          {
            "name": "24/spawn_reviewer",
            "value": 86.00571383571805,
            "unit": "ns"
          },
          {
            "name": "25/enqueue_curation",
            "value": 19.96327757571047,
            "unit": "ns"
          },
          {
            "name": "26/arc_pass",
            "value": 4.470166081936254,
            "unit": "ns"
          },
          {
            "name": "26/copy_transcript",
            "value": 7749.969663811758,
            "unit": "ns"
          },
          {
            "name": "27/enqueue_reflection",
            "value": 19.321661740680845,
            "unit": "ns"
          },
          {
            "name": "27/schedule_eight",
            "value": 275436.20148430875,
            "unit": "ns"
          },
          {
            "name": "28/filter_100",
            "value": 4469.26533105865,
            "unit": "ns"
          },
          {
            "name": "3.1/registry_lookup",
            "value": 27.13755046387532,
            "unit": "ns"
          },
          {
            "name": "3.2/available_tools_50",
            "value": 30654.68090452261,
            "unit": "ns"
          },
          {
            "name": "3.3/dispatch_8_reads_concurrent",
            "value": 2142273.5069444445,
            "unit": "ns"
          },
          {
            "name": "3.3/dispatch_8_reads_sequential",
            "value": 16735567.941666666,
            "unit": "ns"
          },
          {
            "name": "4.1/stdio_echo_round_trip",
            "value": 210853.22631578948,
            "unit": "ns"
          },
          {
            "name": "4.2/http_echo_round_trip",
            "value": 418556.40056559024,
            "unit": "ns"
          },
          {
            "name": "4.3/mcp_50_concurrent",
            "value": 5166126.333333334,
            "unit": "ns"
          },
          {
            "name": "4.3/mcp_50_sequential",
            "value": 10490409.638888888,
            "unit": "ns"
          },
          {
            "name": "4.4/mcp_list_cold",
            "value": 219519.69633838383,
            "unit": "ns"
          },
          {
            "name": "4.4/mcp_list_hit",
            "value": 252.12367904006737,
            "unit": "ns"
          },
          {
            "name": "5.1/schema_tokens_1000",
            "value": 1276892.459770115,
            "unit": "ns"
          },
          {
            "name": "5.1/schema_tokens_10000",
            "value": 40905759.15,
            "unit": "ns"
          },
          {
            "name": "5.2/index_build_10000",
            "value": 21678230.791666664,
            "unit": "ns"
          },
          {
            "name": "5.2/index_search_10000",
            "value": 332886.5988636364,
            "unit": "ns"
          },
          {
            "name": "5.3/full_schema_tokens_10000",
            "value": 24425297.05,
            "unit": "ns"
          },
          {
            "name": "5.3/lazy_visible_prompt_10000",
            "value": 15223.199482766728,
            "unit": "ns"
          },
          {
            "name": "5.4/routed_search_10000",
            "value": 26037.68314162473,
            "unit": "ns"
          },
          {
            "name": "5.4/shrink_schemas_10000",
            "value": 61753519.41666667,
            "unit": "ns"
          },
          {
            "name": "5.5/agent_filter_10000",
            "value": 124924.91395547945,
            "unit": "ns"
          },
          {
            "name": "5.5/cache_cold_10000",
            "value": 25835270.125,
            "unit": "ns"
          },
          {
            "name": "5.5/cache_hit_10000",
            "value": 13.789455208008343,
            "unit": "ns"
          },
          {
            "name": "6.1/gate_check",
            "value": 3.425284416646492,
            "unit": "ns"
          },
          {
            "name": "6.1/live_tool_body",
            "value": 1888.5347839650601,
            "unit": "ns"
          },
          {
            "name": "6.1/loop_8_reads",
            "value": 2153660.1637254898,
            "unit": "ns"
          },
          {
            "name": "6.1/loop_iteration",
            "value": 361.6201255450308,
            "unit": "ns"
          },
          {
            "name": "6.10/budget_notice",
            "value": 77.36351257059837,
            "unit": "ns"
          },
          {
            "name": "6.10/cancel_select",
            "value": 96.25660518470647,
            "unit": "ns"
          },
          {
            "name": "6.10/canonical_args",
            "value": 1057.6626237333803,
            "unit": "ns"
          },
          {
            "name": "6.10/gate_check",
            "value": 7.473993474216087,
            "unit": "ns"
          },
          {
            "name": "6.10/live_tool_body",
            "value": 1722.488976973993,
            "unit": "ns"
          },
          {
            "name": "6.10/loop_8_reads",
            "value": 2185437.3151260503,
            "unit": "ns"
          },
          {
            "name": "6.10/loop_iteration",
            "value": 753.3127060800168,
            "unit": "ns"
          },
          {
            "name": "6.10/outcome_label",
            "value": 8.402210753544786,
            "unit": "ns"
          },
          {
            "name": "6.10/stall_observe",
            "value": 138.14629780340138,
            "unit": "ns"
          },
          {
            "name": "6.10/steer_append",
            "value": 59.83097275086434,
            "unit": "ns"
          },
          {
            "name": "6.10/stress_repair",
            "value": 52514.92214774473,
            "unit": "ns"
          },
          {
            "name": "6.10/truncate_output",
            "value": 4642.327578061246,
            "unit": "ns"
          },
          {
            "name": "6.10/validate_repair",
            "value": 277.7024790675547,
            "unit": "ns"
          },
          {
            "name": "6.2/cancel_select",
            "value": 124.0357349397159,
            "unit": "ns"
          },
          {
            "name": "6.2/gate_check",
            "value": 7.476790810681234,
            "unit": "ns"
          },
          {
            "name": "6.2/live_tool_body",
            "value": 1754.2899699112957,
            "unit": "ns"
          },
          {
            "name": "6.2/loop_8_reads",
            "value": 2159454.7352941176,
            "unit": "ns"
          },
          {
            "name": "6.2/loop_iteration",
            "value": 533.982159079063,
            "unit": "ns"
          },
          {
            "name": "6.3/cancel_select",
            "value": 101.83286602620294,
            "unit": "ns"
          },
          {
            "name": "6.3/gate_check",
            "value": 7.4743742432519245,
            "unit": "ns"
          },
          {
            "name": "6.3/live_tool_body",
            "value": 1747.4545311145362,
            "unit": "ns"
          },
          {
            "name": "6.3/loop_8_reads",
            "value": 2174969.490196078,
            "unit": "ns"
          },
          {
            "name": "6.3/loop_iteration",
            "value": 553.7480453691019,
            "unit": "ns"
          },
          {
            "name": "6.3/validate_repair",
            "value": 261.7579769562168,
            "unit": "ns"
          },
          {
            "name": "6.4/cancel_select",
            "value": 95.86274998852717,
            "unit": "ns"
          },
          {
            "name": "6.4/gate_check",
            "value": 7.472473387913082,
            "unit": "ns"
          },
          {
            "name": "6.4/live_tool_body",
            "value": 1768.354694854887,
            "unit": "ns"
          },
          {
            "name": "6.4/loop_8_reads",
            "value": 2175946.3039215687,
            "unit": "ns"
          },
          {
            "name": "6.4/loop_iteration",
            "value": 549.3117240815355,
            "unit": "ns"
          },
          {
            "name": "6.4/stall_observe",
            "value": 139.0835069148028,
            "unit": "ns"
          },
          {
            "name": "6.4/validate_repair",
            "value": 263.2853465497025,
            "unit": "ns"
          },
          {
            "name": "6.5/budget_notice",
            "value": 78.4943451545323,
            "unit": "ns"
          },
          {
            "name": "6.5/cancel_select",
            "value": 95.13591573624996,
            "unit": "ns"
          },
          {
            "name": "6.5/gate_check",
            "value": 7.477722664535666,
            "unit": "ns"
          },
          {
            "name": "6.5/live_tool_body",
            "value": 1751.3435737179489,
            "unit": "ns"
          },
          {
            "name": "6.5/loop_8_reads",
            "value": 2170851.6647058823,
            "unit": "ns"
          },
          {
            "name": "6.5/loop_iteration",
            "value": 562.1237752461062,
            "unit": "ns"
          },
          {
            "name": "6.5/stall_observe",
            "value": 135.93877748850605,
            "unit": "ns"
          },
          {
            "name": "6.5/validate_repair",
            "value": 287.9791328020722,
            "unit": "ns"
          },
          {
            "name": "6.6/budget_notice",
            "value": 76.75245608995834,
            "unit": "ns"
          },
          {
            "name": "6.6/cancel_select",
            "value": 95.97175103178013,
            "unit": "ns"
          },
          {
            "name": "6.6/gate_check",
            "value": 7.472156888014965,
            "unit": "ns"
          },
          {
            "name": "6.6/live_tool_body",
            "value": 1766.5929228298949,
            "unit": "ns"
          },
          {
            "name": "6.6/loop_8_reads",
            "value": 2182081.705882353,
            "unit": "ns"
          },
          {
            "name": "6.6/loop_iteration",
            "value": 620.9117742971073,
            "unit": "ns"
          },
          {
            "name": "6.6/stall_observe",
            "value": 140.35576625962761,
            "unit": "ns"
          },
          {
            "name": "6.6/truncate_output",
            "value": 4648.130766223612,
            "unit": "ns"
          },
          {
            "name": "6.6/validate_repair",
            "value": 259.9854182384447,
            "unit": "ns"
          },
          {
            "name": "6.7/budget_notice",
            "value": 76.43229960701953,
            "unit": "ns"
          },
          {
            "name": "6.7/cancel_select",
            "value": 95.29312604256964,
            "unit": "ns"
          },
          {
            "name": "6.7/gate_check",
            "value": 7.4772198145502315,
            "unit": "ns"
          },
          {
            "name": "6.7/live_tool_body",
            "value": 1674.5483490049817,
            "unit": "ns"
          },
          {
            "name": "6.7/loop_8_reads",
            "value": 2176900.4967320263,
            "unit": "ns"
          },
          {
            "name": "6.7/loop_iteration",
            "value": 629.3350104768087,
            "unit": "ns"
          },
          {
            "name": "6.7/stall_observe",
            "value": 139.61597948674734,
            "unit": "ns"
          },
          {
            "name": "6.7/steer_append",
            "value": 58.431469174713214,
            "unit": "ns"
          },
          {
            "name": "6.7/truncate_output",
            "value": 4583.3512951682005,
            "unit": "ns"
          },
          {
            "name": "6.7/validate_repair",
            "value": 268.37448394071555,
            "unit": "ns"
          },
          {
            "name": "6.8/budget_notice",
            "value": 78.66150183654243,
            "unit": "ns"
          },
          {
            "name": "6.8/cancel_select",
            "value": 95.76908205561,
            "unit": "ns"
          },
          {
            "name": "6.8/canonical_args",
            "value": 1056.0003039601668,
            "unit": "ns"
          },
          {
            "name": "6.8/gate_check",
            "value": 7.47866562281107,
            "unit": "ns"
          },
          {
            "name": "6.8/live_tool_body",
            "value": 1780.1088999609224,
            "unit": "ns"
          },
          {
            "name": "6.8/loop_8_reads",
            "value": 2179270.094537815,
            "unit": "ns"
          },
          {
            "name": "6.8/loop_iteration",
            "value": 698.2889977974171,
            "unit": "ns"
          },
          {
            "name": "6.8/stall_observe",
            "value": 136.2708292639784,
            "unit": "ns"
          },
          {
            "name": "6.8/steer_append",
            "value": 57.95683980257151,
            "unit": "ns"
          },
          {
            "name": "6.8/truncate_output",
            "value": 4612.317758589512,
            "unit": "ns"
          },
          {
            "name": "6.8/validate_repair",
            "value": 258.75308079789966,
            "unit": "ns"
          },
          {
            "name": "6.9/budget_notice",
            "value": 77.64073769831337,
            "unit": "ns"
          },
          {
            "name": "6.9/cancel_select",
            "value": 108.29604111360561,
            "unit": "ns"
          },
          {
            "name": "6.9/canonical_args",
            "value": 1075.727913818722,
            "unit": "ns"
          },
          {
            "name": "6.9/gate_check",
            "value": 7.475889606522632,
            "unit": "ns"
          },
          {
            "name": "6.9/live_tool_body",
            "value": 1883.6847141828437,
            "unit": "ns"
          },
          {
            "name": "6.9/loop_8_reads",
            "value": 2185775.1294117644,
            "unit": "ns"
          },
          {
            "name": "6.9/loop_iteration",
            "value": 770.105559263613,
            "unit": "ns"
          },
          {
            "name": "6.9/outcome_label",
            "value": 8.402472318091329,
            "unit": "ns"
          },
          {
            "name": "6.9/stall_observe",
            "value": 136.03403728757633,
            "unit": "ns"
          },
          {
            "name": "6.9/steer_append",
            "value": 59.274884288318134,
            "unit": "ns"
          },
          {
            "name": "6.9/truncate_output",
            "value": 4606.662736554317,
            "unit": "ns"
          },
          {
            "name": "6.9/validate_repair",
            "value": 267.6551551882607,
            "unit": "ns"
          },
          {
            "name": "gw5/echo_through_gate",
            "value": 45.58911426266347,
            "unit": "ns"
          },
          {
            "name": "gw5/eight_concurrent_sleeps",
            "value": 2120157.625,
            "unit": "ns"
          },
          {
            "name": "gw7/catalog_arc_swap",
            "value": 2472.5013300628757,
            "unit": "ns"
          },
          {
            "name": "gw7/catalog_reload_10k",
            "value": 30593901.0625,
            "unit": "ns"
          },
          {
            "name": "gw7/catalog_search_10k",
            "value": 298633.97520661156,
            "unit": "ns"
          },
          {
            "name": "gw8/dynamic_rename_task",
            "value": 5679203.953968255,
            "unit": "ns"
          },
          {
            "name": "gw8/every_tool_rename_task",
            "value": 8473299.68,
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
          "id": "60689b361bfb242474368b516a3cbe6b563c08b3",
          "message": "Add chapters 31–40: composition through editor integration",
          "timestamp": "2026-10-15T04:44:13Z",
          "url": "https://github.com/agnathan/rust-agent-harness/pull/7/commits/60689b361bfb242474368b516a3cbe6b563c08b3"
        },
        "date": 1792205258352,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "1.1/serde_50_message_request",
            "value": 10393.23222986529,
            "unit": "ns"
          },
          {
            "name": "1.2/mock_100_concurrent",
            "value": 78815.03450106157,
            "unit": "ns"
          },
          {
            "name": "1.2/mock_turn_overhead",
            "value": 255.42168446754434,
            "unit": "ns"
          },
          {
            "name": "1.3/build_request_and_parse_response",
            "value": 2897.6645438340624,
            "unit": "ns"
          },
          {
            "name": "1.4/parse_response_edges",
            "value": 1095.6193732322322,
            "unit": "ns"
          },
          {
            "name": "10/scripted_turn",
            "value": 8.09766280815018,
            "unit": "ns"
          },
          {
            "name": "10/turn_transition",
            "value": 1.0931128994660955,
            "unit": "ns"
          },
          {
            "name": "11/skip_decision",
            "value": 251.11103220416226,
            "unit": "ns"
          },
          {
            "name": "12/api_request_hook",
            "value": 2.1812222856496932,
            "unit": "ns"
          },
          {
            "name": "12/empty_pre_post",
            "value": 10.902185382252215,
            "unit": "ns"
          },
          {
            "name": "12/memory_enqueue",
            "value": 57.27013801883048,
            "unit": "ns"
          },
          {
            "name": "13/checkpoint_1000",
            "value": 50667.48988842399,
            "unit": "ns"
          },
          {
            "name": "13/rewind_1000",
            "value": 78776.49963924964,
            "unit": "ns"
          },
          {
            "name": "13/warm_accept",
            "value": 29718.84641253208,
            "unit": "ns"
          },
          {
            "name": "14/empty_budget",
            "value": 0.9042729254051223,
            "unit": "ns"
          },
          {
            "name": "14/repetition_guard",
            "value": 2547.306268971732,
            "unit": "ns"
          },
          {
            "name": "15/open_breaker",
            "value": 30.078336099157898,
            "unit": "ns"
          },
          {
            "name": "15/permit_release",
            "value": 58.90266646173034,
            "unit": "ns"
          },
          {
            "name": "16/choose_route",
            "value": 6.227306866974233,
            "unit": "ns"
          },
          {
            "name": "16/rotate_key",
            "value": 3.445576947567902,
            "unit": "ns"
          },
          {
            "name": "17/buffer_join",
            "value": 736.8737294023217,
            "unit": "ns"
          },
          {
            "name": "17/fold_deltas",
            "value": 2832.9317637220374,
            "unit": "ns"
          },
          {
            "name": "18/wal_append",
            "value": 140309.8794498382,
            "unit": "ns"
          },
          {
            "name": "19/memory_enqueue",
            "value": 54.55070180414934,
            "unit": "ns"
          },
          {
            "name": "2/chat_turn_overhead",
            "value": 658.0967676794575,
            "unit": "ns"
          },
          {
            "name": "2/inbox_submit",
            "value": 40.41787258953734,
            "unit": "ns"
          },
          {
            "name": "20/discover_10k",
            "value": 59187.06788419913,
            "unit": "ns"
          },
          {
            "name": "21/apply_offered",
            "value": 112.17273166654724,
            "unit": "ns"
          },
          {
            "name": "21/predict_offline",
            "value": 288550.0652557319,
            "unit": "ns"
          },
          {
            "name": "22/load_render_500",
            "value": 5850637.973214285,
            "unit": "ns"
          },
          {
            "name": "23/enqueue_unsolicited",
            "value": 19.83632063645017,
            "unit": "ns"
          },
          {
            "name": "23/scan_one_skill",
            "value": 284.44194479616567,
            "unit": "ns"
          },
          {
            "name": "24/cancel_review",
            "value": 749166.9775641025,
            "unit": "ns"
          },
          {
            "name": "24/spawn_reviewer",
            "value": 85.52391894226628,
            "unit": "ns"
          },
          {
            "name": "25/enqueue_curation",
            "value": 19.93943181425967,
            "unit": "ns"
          },
          {
            "name": "26/arc_pass",
            "value": 4.527123439880967,
            "unit": "ns"
          },
          {
            "name": "26/copy_transcript",
            "value": 7463.468258835286,
            "unit": "ns"
          },
          {
            "name": "27/enqueue_reflection",
            "value": 19.630259157481746,
            "unit": "ns"
          },
          {
            "name": "27/schedule_eight",
            "value": 267891.83802572126,
            "unit": "ns"
          },
          {
            "name": "28/filter_100",
            "value": 4016.038117318025,
            "unit": "ns"
          },
          {
            "name": "29/schedule_100",
            "value": 6790.448545469144,
            "unit": "ns"
          },
          {
            "name": "3.1/registry_lookup",
            "value": 25.599215483139698,
            "unit": "ns"
          },
          {
            "name": "3.2/available_tools_50",
            "value": 30492.36423378076,
            "unit": "ns"
          },
          {
            "name": "3.3/dispatch_8_reads_concurrent",
            "value": 2137723.3904411765,
            "unit": "ns"
          },
          {
            "name": "3.3/dispatch_8_reads_sequential",
            "value": 16710417.495833334,
            "unit": "ns"
          },
          {
            "name": "30/judge_one_permit",
            "value": 386.3905289867356,
            "unit": "ns"
          },
          {
            "name": "31/nested_eight",
            "value": 270443.6813432836,
            "unit": "ns"
          },
          {
            "name": "32/share_snapshot",
            "value": 5.257429928560155,
            "unit": "ns"
          },
          {
            "name": "33/inbox_submit",
            "value": 309.3165575151759,
            "unit": "ns"
          },
          {
            "name": "34/restart_bookkeeping",
            "value": 27.245128762157407,
            "unit": "ns"
          },
          {
            "name": "35/pick_least",
            "value": 3.7372052460884526,
            "unit": "ns"
          },
          {
            "name": "36/resolve_profile",
            "value": 95.89625384020545,
            "unit": "ns"
          },
          {
            "name": "37/enqueue_platform",
            "value": 177.69227287689267,
            "unit": "ns"
          },
          {
            "name": "38/route_session",
            "value": 19.42129891899563,
            "unit": "ns"
          },
          {
            "name": "39/scan_thousand",
            "value": 1601.2764280995375,
            "unit": "ns"
          },
          {
            "name": "4.1/stdio_echo_round_trip",
            "value": 200706.91081349208,
            "unit": "ns"
          },
          {
            "name": "4.2/http_echo_round_trip",
            "value": 411286.60174781526,
            "unit": "ns"
          },
          {
            "name": "4.3/mcp_50_concurrent",
            "value": 5027022.416666666,
            "unit": "ns"
          },
          {
            "name": "4.3/mcp_50_sequential",
            "value": 10245073.857142858,
            "unit": "ns"
          },
          {
            "name": "4.4/mcp_list_cold",
            "value": 215822.1875748503,
            "unit": "ns"
          },
          {
            "name": "4.4/mcp_list_hit",
            "value": 255.89066552423586,
            "unit": "ns"
          },
          {
            "name": "40/translate_update",
            "value": 128.87337746361663,
            "unit": "ns"
          },
          {
            "name": "5.1/schema_tokens_1000",
            "value": 1384510.076923077,
            "unit": "ns"
          },
          {
            "name": "5.1/schema_tokens_10000",
            "value": 47029270.08333333,
            "unit": "ns"
          },
          {
            "name": "5.2/index_build_10000",
            "value": 23015055.55,
            "unit": "ns"
          },
          {
            "name": "5.2/index_search_10000",
            "value": 333999.74508519005,
            "unit": "ns"
          },
          {
            "name": "5.3/full_schema_tokens_10000",
            "value": 28540292.885714285,
            "unit": "ns"
          },
          {
            "name": "5.3/lazy_visible_prompt_10000",
            "value": 15473.169639054515,
            "unit": "ns"
          },
          {
            "name": "5.4/routed_search_10000",
            "value": 26113.912679083092,
            "unit": "ns"
          },
          {
            "name": "5.4/shrink_schemas_10000",
            "value": 67741455.14285713,
            "unit": "ns"
          },
          {
            "name": "5.5/agent_filter_10000",
            "value": 127168.96377233879,
            "unit": "ns"
          },
          {
            "name": "5.5/cache_cold_10000",
            "value": 27804491.5,
            "unit": "ns"
          },
          {
            "name": "5.5/cache_hit_10000",
            "value": 13.805226522750825,
            "unit": "ns"
          },
          {
            "name": "6.1/gate_check",
            "value": 3.4235342383789,
            "unit": "ns"
          },
          {
            "name": "6.1/live_tool_body",
            "value": 1744.1628812285223,
            "unit": "ns"
          },
          {
            "name": "6.1/loop_8_reads",
            "value": 2157591.6102941176,
            "unit": "ns"
          },
          {
            "name": "6.1/loop_iteration",
            "value": 376.8653816815473,
            "unit": "ns"
          },
          {
            "name": "6.10/budget_notice",
            "value": 76.10208010969795,
            "unit": "ns"
          },
          {
            "name": "6.10/cancel_select",
            "value": 95.91358254118674,
            "unit": "ns"
          },
          {
            "name": "6.10/canonical_args",
            "value": 1102.9540316474863,
            "unit": "ns"
          },
          {
            "name": "6.10/gate_check",
            "value": 7.474321024137833,
            "unit": "ns"
          },
          {
            "name": "6.10/live_tool_body",
            "value": 1731.2940510677367,
            "unit": "ns"
          },
          {
            "name": "6.10/loop_8_reads",
            "value": 2188555.8835784313,
            "unit": "ns"
          },
          {
            "name": "6.10/loop_iteration",
            "value": 728.9656126222617,
            "unit": "ns"
          },
          {
            "name": "6.10/outcome_label",
            "value": 8.399445533363078,
            "unit": "ns"
          },
          {
            "name": "6.10/stall_observe",
            "value": 136.4434220765757,
            "unit": "ns"
          },
          {
            "name": "6.10/steer_append",
            "value": 59.83849321223033,
            "unit": "ns"
          },
          {
            "name": "6.10/stress_repair",
            "value": 51525.02731481481,
            "unit": "ns"
          },
          {
            "name": "6.10/truncate_output",
            "value": 4584.09346623533,
            "unit": "ns"
          },
          {
            "name": "6.10/validate_repair",
            "value": 279.61840315589575,
            "unit": "ns"
          },
          {
            "name": "6.2/cancel_select",
            "value": 96.02333500131444,
            "unit": "ns"
          },
          {
            "name": "6.2/gate_check",
            "value": 7.46882650403904,
            "unit": "ns"
          },
          {
            "name": "6.2/live_tool_body",
            "value": 1762.5412326347162,
            "unit": "ns"
          },
          {
            "name": "6.2/loop_8_reads",
            "value": 2161076.0095588234,
            "unit": "ns"
          },
          {
            "name": "6.2/loop_iteration",
            "value": 533.2714500965158,
            "unit": "ns"
          },
          {
            "name": "6.3/cancel_select",
            "value": 98.13834098561372,
            "unit": "ns"
          },
          {
            "name": "6.3/gate_check",
            "value": 7.485442728345232,
            "unit": "ns"
          },
          {
            "name": "6.3/live_tool_body",
            "value": 1758.4936415057539,
            "unit": "ns"
          },
          {
            "name": "6.3/loop_8_reads",
            "value": 2169973.3970588236,
            "unit": "ns"
          },
          {
            "name": "6.3/loop_iteration",
            "value": 536.2777559977317,
            "unit": "ns"
          },
          {
            "name": "6.3/validate_repair",
            "value": 261.133716072028,
            "unit": "ns"
          },
          {
            "name": "6.4/cancel_select",
            "value": 94.842899806957,
            "unit": "ns"
          },
          {
            "name": "6.4/gate_check",
            "value": 7.47804668499497,
            "unit": "ns"
          },
          {
            "name": "6.4/live_tool_body",
            "value": 1778.8614515682732,
            "unit": "ns"
          },
          {
            "name": "6.4/loop_8_reads",
            "value": 2171285.254084967,
            "unit": "ns"
          },
          {
            "name": "6.4/loop_iteration",
            "value": 553.9669284836381,
            "unit": "ns"
          },
          {
            "name": "6.4/stall_observe",
            "value": 139.70823335287528,
            "unit": "ns"
          },
          {
            "name": "6.4/validate_repair",
            "value": 245.26269846094272,
            "unit": "ns"
          },
          {
            "name": "6.5/budget_notice",
            "value": 78.62042721979306,
            "unit": "ns"
          },
          {
            "name": "6.5/cancel_select",
            "value": 95.75527024879042,
            "unit": "ns"
          },
          {
            "name": "6.5/gate_check",
            "value": 7.4819031420806805,
            "unit": "ns"
          },
          {
            "name": "6.5/live_tool_body",
            "value": 1760.701141028737,
            "unit": "ns"
          },
          {
            "name": "6.5/loop_8_reads",
            "value": 2179598.981617647,
            "unit": "ns"
          },
          {
            "name": "6.5/loop_iteration",
            "value": 593.2033290367494,
            "unit": "ns"
          },
          {
            "name": "6.5/stall_observe",
            "value": 142.98852079083161,
            "unit": "ns"
          },
          {
            "name": "6.5/validate_repair",
            "value": 274.52455699873315,
            "unit": "ns"
          },
          {
            "name": "6.6/budget_notice",
            "value": 77.04566406970699,
            "unit": "ns"
          },
          {
            "name": "6.6/cancel_select",
            "value": 96.83938111886275,
            "unit": "ns"
          },
          {
            "name": "6.6/gate_check",
            "value": 7.481578919306429,
            "unit": "ns"
          },
          {
            "name": "6.6/live_tool_body",
            "value": 1783.4697950045188,
            "unit": "ns"
          },
          {
            "name": "6.6/loop_8_reads",
            "value": 2184728.5966386553,
            "unit": "ns"
          },
          {
            "name": "6.6/loop_iteration",
            "value": 634.6176857745471,
            "unit": "ns"
          },
          {
            "name": "6.6/stall_observe",
            "value": 139.84643281401017,
            "unit": "ns"
          },
          {
            "name": "6.6/truncate_output",
            "value": 4538.643124933053,
            "unit": "ns"
          },
          {
            "name": "6.6/validate_repair",
            "value": 260.6912386808615,
            "unit": "ns"
          },
          {
            "name": "6.7/budget_notice",
            "value": 76.7754287841747,
            "unit": "ns"
          },
          {
            "name": "6.7/cancel_select",
            "value": 95.46713360776947,
            "unit": "ns"
          },
          {
            "name": "6.7/gate_check",
            "value": 7.475251714373693,
            "unit": "ns"
          },
          {
            "name": "6.7/live_tool_body",
            "value": 1609.582419889503,
            "unit": "ns"
          },
          {
            "name": "6.7/loop_8_reads",
            "value": 2183996.7784313727,
            "unit": "ns"
          },
          {
            "name": "6.7/loop_iteration",
            "value": 616.8896575420636,
            "unit": "ns"
          },
          {
            "name": "6.7/stall_observe",
            "value": 140.15425788690362,
            "unit": "ns"
          },
          {
            "name": "6.7/steer_append",
            "value": 58.393526409864975,
            "unit": "ns"
          },
          {
            "name": "6.7/truncate_output",
            "value": 4985.726869544704,
            "unit": "ns"
          },
          {
            "name": "6.7/validate_repair",
            "value": 259.89492500124936,
            "unit": "ns"
          },
          {
            "name": "6.8/budget_notice",
            "value": 79.34421377163875,
            "unit": "ns"
          },
          {
            "name": "6.8/cancel_select",
            "value": 97.00912083992515,
            "unit": "ns"
          },
          {
            "name": "6.8/canonical_args",
            "value": 1058.2836865299246,
            "unit": "ns"
          },
          {
            "name": "6.8/gate_check",
            "value": 7.48455683677741,
            "unit": "ns"
          },
          {
            "name": "6.8/live_tool_body",
            "value": 1774.3607408128182,
            "unit": "ns"
          },
          {
            "name": "6.8/loop_8_reads",
            "value": 2191970.488235294,
            "unit": "ns"
          },
          {
            "name": "6.8/loop_iteration",
            "value": 690.1920334981345,
            "unit": "ns"
          },
          {
            "name": "6.8/stall_observe",
            "value": 138.23658371668103,
            "unit": "ns"
          },
          {
            "name": "6.8/steer_append",
            "value": 57.978940316027334,
            "unit": "ns"
          },
          {
            "name": "6.8/truncate_output",
            "value": 4739.943687497271,
            "unit": "ns"
          },
          {
            "name": "6.8/validate_repair",
            "value": 266.66705966066183,
            "unit": "ns"
          },
          {
            "name": "6.9/budget_notice",
            "value": 79.99565604479605,
            "unit": "ns"
          },
          {
            "name": "6.9/cancel_select",
            "value": 96.23004255784068,
            "unit": "ns"
          },
          {
            "name": "6.9/canonical_args",
            "value": 1056.2636249879681,
            "unit": "ns"
          },
          {
            "name": "6.9/gate_check",
            "value": 7.471137816517131,
            "unit": "ns"
          },
          {
            "name": "6.9/live_tool_body",
            "value": 1712.7615601149532,
            "unit": "ns"
          },
          {
            "name": "6.9/loop_8_reads",
            "value": 2186205.777777778,
            "unit": "ns"
          },
          {
            "name": "6.9/loop_iteration",
            "value": 758.2603697473305,
            "unit": "ns"
          },
          {
            "name": "6.9/outcome_label",
            "value": 8.400170569219934,
            "unit": "ns"
          },
          {
            "name": "6.9/stall_observe",
            "value": 136.31639460128264,
            "unit": "ns"
          },
          {
            "name": "6.9/steer_append",
            "value": 59.246027663148965,
            "unit": "ns"
          },
          {
            "name": "6.9/truncate_output",
            "value": 4785.796738462095,
            "unit": "ns"
          },
          {
            "name": "6.9/validate_repair",
            "value": 250.01260906469145,
            "unit": "ns"
          },
          {
            "name": "gw5/echo_through_gate",
            "value": 45.75605225197734,
            "unit": "ns"
          },
          {
            "name": "gw5/eight_concurrent_sleeps",
            "value": 2120719.4746031747,
            "unit": "ns"
          },
          {
            "name": "gw7/catalog_arc_swap",
            "value": 2386.0786574682434,
            "unit": "ns"
          },
          {
            "name": "gw7/catalog_reload_10k",
            "value": 31683060.625,
            "unit": "ns"
          },
          {
            "name": "gw7/catalog_search_10k",
            "value": 298702.7786885246,
            "unit": "ns"
          },
          {
            "name": "gw8/dynamic_rename_task",
            "value": 5804338.280952381,
            "unit": "ns"
          },
          {
            "name": "gw8/every_tool_rename_task",
            "value": 8558828.565277778,
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
          "id": "8f7303e8b56aaf821c86fe924a477629bc03a19a",
          "message": "Write chapters 11 through 20",
          "timestamp": "2026-10-15T04:44:13Z",
          "url": "https://github.com/agnathan/rust-agent-harness/pull/5/commits/8f7303e8b56aaf821c86fe924a477629bc03a19a"
        },
        "date": 1792208979922,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "1.1/serde_50_message_request",
            "value": 10398.54685264664,
            "unit": "ns"
          },
          {
            "name": "1.2/mock_100_concurrent",
            "value": 78264.15870488322,
            "unit": "ns"
          },
          {
            "name": "1.2/mock_turn_overhead",
            "value": 250.01392326304875,
            "unit": "ns"
          },
          {
            "name": "1.3/build_request_and_parse_response",
            "value": 2956.914519386107,
            "unit": "ns"
          },
          {
            "name": "1.4/parse_response_edges",
            "value": 1147.1033787992028,
            "unit": "ns"
          },
          {
            "name": "10/scripted_turn",
            "value": 8.11769490063665,
            "unit": "ns"
          },
          {
            "name": "10/turn_transition",
            "value": 1.1050410709812692,
            "unit": "ns"
          },
          {
            "name": "11/skip_decision",
            "value": 250.93507988581163,
            "unit": "ns"
          },
          {
            "name": "12/api_request_hook",
            "value": 3.1111328285173236,
            "unit": "ns"
          },
          {
            "name": "12/empty_pre_post",
            "value": 10.908682164707745,
            "unit": "ns"
          },
          {
            "name": "12/memory_enqueue",
            "value": 56.84007489648262,
            "unit": "ns"
          },
          {
            "name": "13/checkpoint_1000",
            "value": 50364.9334796764,
            "unit": "ns"
          },
          {
            "name": "13/rewind_1000",
            "value": 78939.96176789587,
            "unit": "ns"
          },
          {
            "name": "13/warm_accept",
            "value": 28447.10047953608,
            "unit": "ns"
          },
          {
            "name": "14/empty_budget",
            "value": 0.9034219650156341,
            "unit": "ns"
          },
          {
            "name": "14/repetition_guard",
            "value": 2550.629698786731,
            "unit": "ns"
          },
          {
            "name": "15/open_breaker",
            "value": 30.089418674379054,
            "unit": "ns"
          },
          {
            "name": "15/permit_release",
            "value": 58.85706371265742,
            "unit": "ns"
          },
          {
            "name": "16/choose_route",
            "value": 6.228547600477517,
            "unit": "ns"
          },
          {
            "name": "16/rotate_key",
            "value": 3.4457650702451503,
            "unit": "ns"
          },
          {
            "name": "17/buffer_join",
            "value": 729.9971896605557,
            "unit": "ns"
          },
          {
            "name": "17/fold_deltas",
            "value": 2831.172513367596,
            "unit": "ns"
          },
          {
            "name": "18/wal_append",
            "value": 136240.66233183857,
            "unit": "ns"
          },
          {
            "name": "19/memory_enqueue",
            "value": 54.756542901421454,
            "unit": "ns"
          },
          {
            "name": "2/chat_turn_overhead",
            "value": 683.4282431537789,
            "unit": "ns"
          },
          {
            "name": "2/inbox_submit",
            "value": 40.48804903899696,
            "unit": "ns"
          },
          {
            "name": "20/discover_10k",
            "value": 59065.63106332484,
            "unit": "ns"
          },
          {
            "name": "3.1/registry_lookup",
            "value": 25.684781551407738,
            "unit": "ns"
          },
          {
            "name": "3.2/available_tools_50",
            "value": 30659.617706002984,
            "unit": "ns"
          },
          {
            "name": "3.3/dispatch_8_reads_concurrent",
            "value": 2128408.2438271604,
            "unit": "ns"
          },
          {
            "name": "3.3/dispatch_8_reads_sequential",
            "value": 16698657.475000001,
            "unit": "ns"
          },
          {
            "name": "4.1/stdio_echo_round_trip",
            "value": 198666.3653314917,
            "unit": "ns"
          },
          {
            "name": "4.2/http_echo_round_trip",
            "value": 406365.92397003743,
            "unit": "ns"
          },
          {
            "name": "4.3/mcp_50_concurrent",
            "value": 5037059.458333334,
            "unit": "ns"
          },
          {
            "name": "4.3/mcp_50_sequential",
            "value": 10173989.64732143,
            "unit": "ns"
          },
          {
            "name": "4.4/mcp_list_cold",
            "value": 211813.82746647508,
            "unit": "ns"
          },
          {
            "name": "4.4/mcp_list_hit",
            "value": 255.3317516183772,
            "unit": "ns"
          },
          {
            "name": "5.1/schema_tokens_1000",
            "value": 1256371.3754789273,
            "unit": "ns"
          },
          {
            "name": "5.1/schema_tokens_10000",
            "value": 39179194.2625,
            "unit": "ns"
          },
          {
            "name": "5.2/index_build_10000",
            "value": 20607031.585714288,
            "unit": "ns"
          },
          {
            "name": "5.2/index_search_10000",
            "value": 334743.22477064223,
            "unit": "ns"
          },
          {
            "name": "5.3/full_schema_tokens_10000",
            "value": 19708331.75,
            "unit": "ns"
          },
          {
            "name": "5.3/lazy_visible_prompt_10000",
            "value": 15536.43619616933,
            "unit": "ns"
          },
          {
            "name": "5.4/routed_search_10000",
            "value": 25915.518609156723,
            "unit": "ns"
          },
          {
            "name": "5.4/shrink_schemas_10000",
            "value": 57358540.4375,
            "unit": "ns"
          },
          {
            "name": "5.5/agent_filter_10000",
            "value": 125031.57620689654,
            "unit": "ns"
          },
          {
            "name": "5.5/cache_cold_10000",
            "value": 22559004.93888889,
            "unit": "ns"
          },
          {
            "name": "5.5/cache_hit_10000",
            "value": 13.79626788308591,
            "unit": "ns"
          },
          {
            "name": "6.1/gate_check",
            "value": 3.42504883571327,
            "unit": "ns"
          },
          {
            "name": "6.1/live_tool_body",
            "value": 1787.5899294285432,
            "unit": "ns"
          },
          {
            "name": "6.1/loop_8_reads",
            "value": 2149338.1705882354,
            "unit": "ns"
          },
          {
            "name": "6.1/loop_iteration",
            "value": 359.7439827709693,
            "unit": "ns"
          },
          {
            "name": "6.10/budget_notice",
            "value": 76.0343150044882,
            "unit": "ns"
          },
          {
            "name": "6.10/cancel_select",
            "value": 96.42513698869037,
            "unit": "ns"
          },
          {
            "name": "6.10/canonical_args",
            "value": 1057.3897756419597,
            "unit": "ns"
          },
          {
            "name": "6.10/gate_check",
            "value": 7.46593533051969,
            "unit": "ns"
          },
          {
            "name": "6.10/live_tool_body",
            "value": 1738.4053962773428,
            "unit": "ns"
          },
          {
            "name": "6.10/loop_8_reads",
            "value": 2181349.1176470593,
            "unit": "ns"
          },
          {
            "name": "6.10/loop_iteration",
            "value": 759.7384182146366,
            "unit": "ns"
          },
          {
            "name": "6.10/outcome_label",
            "value": 8.399098179748776,
            "unit": "ns"
          },
          {
            "name": "6.10/stall_observe",
            "value": 138.08097589201083,
            "unit": "ns"
          },
          {
            "name": "6.10/steer_append",
            "value": 59.823083983912554,
            "unit": "ns"
          },
          {
            "name": "6.10/stress_repair",
            "value": 51409.06040992448,
            "unit": "ns"
          },
          {
            "name": "6.10/truncate_output",
            "value": 4655.107017163113,
            "unit": "ns"
          },
          {
            "name": "6.10/validate_repair",
            "value": 280.8773419452301,
            "unit": "ns"
          },
          {
            "name": "6.2/cancel_select",
            "value": 95.93831611361283,
            "unit": "ns"
          },
          {
            "name": "6.2/gate_check",
            "value": 7.477102738492506,
            "unit": "ns"
          },
          {
            "name": "6.2/live_tool_body",
            "value": 1719.623460893236,
            "unit": "ns"
          },
          {
            "name": "6.2/loop_8_reads",
            "value": 2159307.2107843137,
            "unit": "ns"
          },
          {
            "name": "6.2/loop_iteration",
            "value": 563.2010406247082,
            "unit": "ns"
          },
          {
            "name": "6.3/cancel_select",
            "value": 96.13372828676094,
            "unit": "ns"
          },
          {
            "name": "6.3/gate_check",
            "value": 7.470255844338745,
            "unit": "ns"
          },
          {
            "name": "6.3/live_tool_body",
            "value": 1717.54466069991,
            "unit": "ns"
          },
          {
            "name": "6.3/loop_8_reads",
            "value": 2168177.967647059,
            "unit": "ns"
          },
          {
            "name": "6.3/loop_iteration",
            "value": 532.6873788523332,
            "unit": "ns"
          },
          {
            "name": "6.3/validate_repair",
            "value": 260.9393760293385,
            "unit": "ns"
          },
          {
            "name": "6.4/cancel_select",
            "value": 122.18579331708068,
            "unit": "ns"
          },
          {
            "name": "6.4/gate_check",
            "value": 7.469705352258588,
            "unit": "ns"
          },
          {
            "name": "6.4/live_tool_body",
            "value": 1938.5189782429566,
            "unit": "ns"
          },
          {
            "name": "6.4/loop_8_reads",
            "value": 2169822.2470588237,
            "unit": "ns"
          },
          {
            "name": "6.4/loop_iteration",
            "value": 618.993785324867,
            "unit": "ns"
          },
          {
            "name": "6.4/stall_observe",
            "value": 138.5263363434519,
            "unit": "ns"
          },
          {
            "name": "6.4/validate_repair",
            "value": 262.87949003718006,
            "unit": "ns"
          },
          {
            "name": "6.5/budget_notice",
            "value": 77.89932950551719,
            "unit": "ns"
          },
          {
            "name": "6.5/cancel_select",
            "value": 122.67033260250994,
            "unit": "ns"
          },
          {
            "name": "6.5/gate_check",
            "value": 7.476696345169269,
            "unit": "ns"
          },
          {
            "name": "6.5/live_tool_body",
            "value": 1706.5361343937516,
            "unit": "ns"
          },
          {
            "name": "6.5/loop_8_reads",
            "value": 2172258.617647059,
            "unit": "ns"
          },
          {
            "name": "6.5/loop_iteration",
            "value": 620.8689056599959,
            "unit": "ns"
          },
          {
            "name": "6.5/stall_observe",
            "value": 136.52802453332203,
            "unit": "ns"
          },
          {
            "name": "6.5/validate_repair",
            "value": 288.1255585168532,
            "unit": "ns"
          },
          {
            "name": "6.6/budget_notice",
            "value": 77.57860897961473,
            "unit": "ns"
          },
          {
            "name": "6.6/cancel_select",
            "value": 95.1688198447437,
            "unit": "ns"
          },
          {
            "name": "6.6/gate_check",
            "value": 7.469123182922264,
            "unit": "ns"
          },
          {
            "name": "6.6/live_tool_body",
            "value": 1737.6686739649704,
            "unit": "ns"
          },
          {
            "name": "6.6/loop_8_reads",
            "value": 2176941.911764706,
            "unit": "ns"
          },
          {
            "name": "6.6/loop_iteration",
            "value": 613.6606812635342,
            "unit": "ns"
          },
          {
            "name": "6.6/stall_observe",
            "value": 141.22044825815672,
            "unit": "ns"
          },
          {
            "name": "6.6/truncate_output",
            "value": 4582.003954730952,
            "unit": "ns"
          },
          {
            "name": "6.6/validate_repair",
            "value": 257.0507988980159,
            "unit": "ns"
          },
          {
            "name": "6.7/budget_notice",
            "value": 76.3290397988442,
            "unit": "ns"
          },
          {
            "name": "6.7/cancel_select",
            "value": 97.71636497362783,
            "unit": "ns"
          },
          {
            "name": "6.7/gate_check",
            "value": 7.476300577113225,
            "unit": "ns"
          },
          {
            "name": "6.7/live_tool_body",
            "value": 1689.408050063175,
            "unit": "ns"
          },
          {
            "name": "6.7/loop_8_reads",
            "value": 2178053.2816993464,
            "unit": "ns"
          },
          {
            "name": "6.7/loop_iteration",
            "value": 622.3816098842346,
            "unit": "ns"
          },
          {
            "name": "6.7/stall_observe",
            "value": 140.13910567062442,
            "unit": "ns"
          },
          {
            "name": "6.7/steer_append",
            "value": 58.392890836644,
            "unit": "ns"
          },
          {
            "name": "6.7/truncate_output",
            "value": 4714.570022768072,
            "unit": "ns"
          },
          {
            "name": "6.7/validate_repair",
            "value": 260.18944017399775,
            "unit": "ns"
          },
          {
            "name": "6.8/budget_notice",
            "value": 76.6447257858923,
            "unit": "ns"
          },
          {
            "name": "6.8/cancel_select",
            "value": 95.782945868436,
            "unit": "ns"
          },
          {
            "name": "6.8/canonical_args",
            "value": 1053.8286398284656,
            "unit": "ns"
          },
          {
            "name": "6.8/gate_check",
            "value": 7.474365720769225,
            "unit": "ns"
          },
          {
            "name": "6.8/live_tool_body",
            "value": 1733.9337163148812,
            "unit": "ns"
          },
          {
            "name": "6.8/loop_8_reads",
            "value": 2182165.1588235293,
            "unit": "ns"
          },
          {
            "name": "6.8/loop_iteration",
            "value": 763.8272036240786,
            "unit": "ns"
          },
          {
            "name": "6.8/stall_observe",
            "value": 137.1877737016284,
            "unit": "ns"
          },
          {
            "name": "6.8/steer_append",
            "value": 57.99378949304995,
            "unit": "ns"
          },
          {
            "name": "6.8/truncate_output",
            "value": 4756.818594326533,
            "unit": "ns"
          },
          {
            "name": "6.8/validate_repair",
            "value": 258.0159532133822,
            "unit": "ns"
          },
          {
            "name": "6.9/budget_notice",
            "value": 77.9957489715627,
            "unit": "ns"
          },
          {
            "name": "6.9/cancel_select",
            "value": 95.195810819435,
            "unit": "ns"
          },
          {
            "name": "6.9/canonical_args",
            "value": 1042.266038597365,
            "unit": "ns"
          },
          {
            "name": "6.9/gate_check",
            "value": 7.467108279130622,
            "unit": "ns"
          },
          {
            "name": "6.9/live_tool_body",
            "value": 1706.2579398386947,
            "unit": "ns"
          },
          {
            "name": "6.9/loop_8_reads",
            "value": 2172939.0274509806,
            "unit": "ns"
          },
          {
            "name": "6.9/loop_iteration",
            "value": 760.444374930486,
            "unit": "ns"
          },
          {
            "name": "6.9/outcome_label",
            "value": 8.397998878504337,
            "unit": "ns"
          },
          {
            "name": "6.9/stall_observe",
            "value": 136.74637058600774,
            "unit": "ns"
          },
          {
            "name": "6.9/steer_append",
            "value": 59.1990962175574,
            "unit": "ns"
          },
          {
            "name": "6.9/truncate_output",
            "value": 4583.135561314473,
            "unit": "ns"
          },
          {
            "name": "6.9/validate_repair",
            "value": 268.5302720216739,
            "unit": "ns"
          },
          {
            "name": "gw5/echo_through_gate",
            "value": 45.687657722714455,
            "unit": "ns"
          },
          {
            "name": "gw5/eight_concurrent_sleeps",
            "value": 2113466.248070988,
            "unit": "ns"
          },
          {
            "name": "gw7/catalog_arc_swap",
            "value": 2425.6242425251385,
            "unit": "ns"
          },
          {
            "name": "gw7/catalog_reload_10k",
            "value": 27399142.75,
            "unit": "ns"
          },
          {
            "name": "gw7/catalog_search_10k",
            "value": 294784.6737903226,
            "unit": "ns"
          },
          {
            "name": "gw8/dynamic_rename_task",
            "value": 5701650.224489796,
            "unit": "ns"
          },
          {
            "name": "gw8/every_tool_rename_task",
            "value": 8415190.757777778,
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
          "id": "1974a196edb8b1733410cb0eccc3b2d6780ad0d6",
          "message": "Add chapters 41 through 48",
          "timestamp": "2026-10-15T04:44:13Z",
          "url": "https://github.com/agnathan/rust-agent-harness/pull/8/commits/1974a196edb8b1733410cb0eccc3b2d6780ad0d6"
        },
        "date": 1792209253382,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "1.1/serde_50_message_request",
            "value": 10297.897830491895,
            "unit": "ns"
          },
          {
            "name": "1.2/mock_100_concurrent",
            "value": 79562.17462039046,
            "unit": "ns"
          },
          {
            "name": "1.2/mock_turn_overhead",
            "value": 248.5302860836331,
            "unit": "ns"
          },
          {
            "name": "1.3/build_request_and_parse_response",
            "value": 2926.583470923474,
            "unit": "ns"
          },
          {
            "name": "1.4/parse_response_edges",
            "value": 1168.973873030248,
            "unit": "ns"
          },
          {
            "name": "10/scripted_turn",
            "value": 7.853295691731914,
            "unit": "ns"
          },
          {
            "name": "10/turn_transition",
            "value": 1.0910074316537164,
            "unit": "ns"
          },
          {
            "name": "11/skip_decision",
            "value": 251.1681287964121,
            "unit": "ns"
          },
          {
            "name": "12/api_request_hook",
            "value": 2.1801126118956797,
            "unit": "ns"
          },
          {
            "name": "12/empty_pre_post",
            "value": 10.91643227058513,
            "unit": "ns"
          },
          {
            "name": "12/memory_enqueue",
            "value": 60.1030201177724,
            "unit": "ns"
          },
          {
            "name": "13/checkpoint_1000",
            "value": 50623.1956420955,
            "unit": "ns"
          },
          {
            "name": "13/rewind_1000",
            "value": 80601.23106725146,
            "unit": "ns"
          },
          {
            "name": "13/warm_accept",
            "value": 29277.633170212765,
            "unit": "ns"
          },
          {
            "name": "14/empty_budget",
            "value": 0.9043165237457838,
            "unit": "ns"
          },
          {
            "name": "14/repetition_guard",
            "value": 2549.5754944694413,
            "unit": "ns"
          },
          {
            "name": "15/open_breaker",
            "value": 29.943877519781726,
            "unit": "ns"
          },
          {
            "name": "15/permit_release",
            "value": 59.13320971679344,
            "unit": "ns"
          },
          {
            "name": "16/choose_route",
            "value": 6.231321837604115,
            "unit": "ns"
          },
          {
            "name": "16/rotate_key",
            "value": 3.4470082924427263,
            "unit": "ns"
          },
          {
            "name": "17/buffer_join",
            "value": 730.1662615895303,
            "unit": "ns"
          },
          {
            "name": "17/fold_deltas",
            "value": 2835.8197520274484,
            "unit": "ns"
          },
          {
            "name": "18/wal_append",
            "value": 142245.81139937107,
            "unit": "ns"
          },
          {
            "name": "19/memory_enqueue",
            "value": 53.09398629603629,
            "unit": "ns"
          },
          {
            "name": "2/chat_turn_overhead",
            "value": 697.2793075768681,
            "unit": "ns"
          },
          {
            "name": "2/inbox_submit",
            "value": 40.724135131851284,
            "unit": "ns"
          },
          {
            "name": "20/discover_10k",
            "value": 59012.36166936791,
            "unit": "ns"
          },
          {
            "name": "21/apply_offered",
            "value": 112.39108929296184,
            "unit": "ns"
          },
          {
            "name": "21/predict_offline",
            "value": 302255.4693548387,
            "unit": "ns"
          },
          {
            "name": "22/load_render_500",
            "value": 5957978.678571429,
            "unit": "ns"
          },
          {
            "name": "23/enqueue_unsolicited",
            "value": 19.405282248264072,
            "unit": "ns"
          },
          {
            "name": "23/scan_one_skill",
            "value": 281.4301104423115,
            "unit": "ns"
          },
          {
            "name": "24/cancel_review",
            "value": 732511.8602941176,
            "unit": "ns"
          },
          {
            "name": "24/spawn_reviewer",
            "value": 86.08136863035213,
            "unit": "ns"
          },
          {
            "name": "25/enqueue_curation",
            "value": 20.874453277022006,
            "unit": "ns"
          },
          {
            "name": "26/arc_pass",
            "value": 4.564759417850992,
            "unit": "ns"
          },
          {
            "name": "26/copy_transcript",
            "value": 7489.066172813924,
            "unit": "ns"
          },
          {
            "name": "27/enqueue_reflection",
            "value": 19.211144307023133,
            "unit": "ns"
          },
          {
            "name": "27/schedule_eight",
            "value": 267082.7289624183,
            "unit": "ns"
          },
          {
            "name": "28/filter_100",
            "value": 4060.901490818901,
            "unit": "ns"
          },
          {
            "name": "29/schedule_100",
            "value": 6812.688316763584,
            "unit": "ns"
          },
          {
            "name": "3.1/registry_lookup",
            "value": 27.142492218212176,
            "unit": "ns"
          },
          {
            "name": "3.2/available_tools_50",
            "value": 30873.05085905787,
            "unit": "ns"
          },
          {
            "name": "3.3/dispatch_8_reads_concurrent",
            "value": 2136111.435185185,
            "unit": "ns"
          },
          {
            "name": "3.3/dispatch_8_reads_sequential",
            "value": 16708231.5,
            "unit": "ns"
          },
          {
            "name": "30/judge_one_permit",
            "value": 381.32526111874563,
            "unit": "ns"
          },
          {
            "name": "31/nested_eight",
            "value": 281304.34684065933,
            "unit": "ns"
          },
          {
            "name": "32/share_snapshot",
            "value": 4.773706763711175,
            "unit": "ns"
          },
          {
            "name": "33/inbox_submit",
            "value": 281.097136373761,
            "unit": "ns"
          },
          {
            "name": "34/restart_bookkeeping",
            "value": 27.17594323977515,
            "unit": "ns"
          },
          {
            "name": "35/pick_least",
            "value": 3.7374817429568807,
            "unit": "ns"
          },
          {
            "name": "36/resolve_profile",
            "value": 95.43140930126326,
            "unit": "ns"
          },
          {
            "name": "37/enqueue_platform",
            "value": 163.30355652023152,
            "unit": "ns"
          },
          {
            "name": "38/route_session",
            "value": 19.69627898230184,
            "unit": "ns"
          },
          {
            "name": "39/scan_thousand",
            "value": 1600.8120202987361,
            "unit": "ns"
          },
          {
            "name": "4.1/stdio_echo_round_trip",
            "value": 204468.2280027933,
            "unit": "ns"
          },
          {
            "name": "4.2/http_echo_round_trip",
            "value": 416438.06562500005,
            "unit": "ns"
          },
          {
            "name": "4.3/mcp_50_concurrent",
            "value": 4999250.9296875,
            "unit": "ns"
          },
          {
            "name": "4.3/mcp_50_sequential",
            "value": 10314643.6875,
            "unit": "ns"
          },
          {
            "name": "4.4/mcp_list_cold",
            "value": 213263.5567251462,
            "unit": "ns"
          },
          {
            "name": "4.4/mcp_list_hit",
            "value": 264.7101591371108,
            "unit": "ns"
          },
          {
            "name": "40/translate_update",
            "value": 130.83643181880285,
            "unit": "ns"
          },
          {
            "name": "41/claim_with_waiters",
            "value": 5674.442262830482,
            "unit": "ns"
          },
          {
            "name": "42/claim_transaction",
            "value": 326640.609929078,
            "unit": "ns"
          },
          {
            "name": "43/publish_to_agents",
            "value": 47083.466063348416,
            "unit": "ns"
          },
          {
            "name": "44/index_swap",
            "value": 198.52415137817957,
            "unit": "ns"
          },
          {
            "name": "44/skill_propose",
            "value": 73.64427943385758,
            "unit": "ns"
          },
          {
            "name": "45/cancel_inflight",
            "value": 50152.46396103896,
            "unit": "ns"
          },
          {
            "name": "46/event_without_span",
            "value": 17.12761570930533,
            "unit": "ns"
          },
          {
            "name": "46/span_per_event",
            "value": 115.41988742696097,
            "unit": "ns"
          },
          {
            "name": "47/dynamic_tokens",
            "value": 246.3554477158242,
            "unit": "ns"
          },
          {
            "name": "47/load_100",
            "value": 29984.660887772196,
            "unit": "ns"
          },
          {
            "name": "48/append_event",
            "value": 80.66065619733376,
            "unit": "ns"
          },
          {
            "name": "48/saved_set",
            "value": 51.09351062084421,
            "unit": "ns"
          },
          {
            "name": "5.1/schema_tokens_1000",
            "value": 1281744.001724138,
            "unit": "ns"
          },
          {
            "name": "5.1/schema_tokens_10000",
            "value": 42976471.72916667,
            "unit": "ns"
          },
          {
            "name": "5.2/index_build_10000",
            "value": 21635895.375,
            "unit": "ns"
          },
          {
            "name": "5.2/index_search_10000",
            "value": 331229.6272727273,
            "unit": "ns"
          },
          {
            "name": "5.3/full_schema_tokens_10000",
            "value": 24580654.525,
            "unit": "ns"
          },
          {
            "name": "5.3/lazy_visible_prompt_10000",
            "value": 15535.222442680777,
            "unit": "ns"
          },
          {
            "name": "5.4/routed_search_10000",
            "value": 25891.865019883757,
            "unit": "ns"
          },
          {
            "name": "5.4/shrink_schemas_10000",
            "value": 64713585.14444444,
            "unit": "ns"
          },
          {
            "name": "5.5/agent_filter_10000",
            "value": 126316.6212543554,
            "unit": "ns"
          },
          {
            "name": "5.5/cache_cold_10000",
            "value": 26104332.6625,
            "unit": "ns"
          },
          {
            "name": "5.5/cache_hit_10000",
            "value": 13.788368744427881,
            "unit": "ns"
          },
          {
            "name": "6.1/gate_check",
            "value": 3.425452458980789,
            "unit": "ns"
          },
          {
            "name": "6.1/live_tool_body",
            "value": 1701.0915765829004,
            "unit": "ns"
          },
          {
            "name": "6.1/loop_8_reads",
            "value": 2151705.9411764704,
            "unit": "ns"
          },
          {
            "name": "6.1/loop_iteration",
            "value": 332.7708512619912,
            "unit": "ns"
          },
          {
            "name": "6.10/budget_notice",
            "value": 77.53314380020092,
            "unit": "ns"
          },
          {
            "name": "6.10/cancel_select",
            "value": 95.31266173341999,
            "unit": "ns"
          },
          {
            "name": "6.10/canonical_args",
            "value": 1059.679508482454,
            "unit": "ns"
          },
          {
            "name": "6.10/gate_check",
            "value": 7.475095675340888,
            "unit": "ns"
          },
          {
            "name": "6.10/live_tool_body",
            "value": 1809.701305499765,
            "unit": "ns"
          },
          {
            "name": "6.10/loop_8_reads",
            "value": 2183512.025210084,
            "unit": "ns"
          },
          {
            "name": "6.10/loop_iteration",
            "value": 784.8335371762021,
            "unit": "ns"
          },
          {
            "name": "6.10/outcome_label",
            "value": 8.400789351017853,
            "unit": "ns"
          },
          {
            "name": "6.10/stall_observe",
            "value": 139.39367763785066,
            "unit": "ns"
          },
          {
            "name": "6.10/steer_append",
            "value": 59.856172824251416,
            "unit": "ns"
          },
          {
            "name": "6.10/stress_repair",
            "value": 53141.62379923151,
            "unit": "ns"
          },
          {
            "name": "6.10/truncate_output",
            "value": 4620.415498665141,
            "unit": "ns"
          },
          {
            "name": "6.10/validate_repair",
            "value": 271.5370523659629,
            "unit": "ns"
          },
          {
            "name": "6.2/cancel_select",
            "value": 96.6491261166409,
            "unit": "ns"
          },
          {
            "name": "6.2/gate_check",
            "value": 7.481117864369667,
            "unit": "ns"
          },
          {
            "name": "6.2/live_tool_body",
            "value": 1836.8884218993514,
            "unit": "ns"
          },
          {
            "name": "6.2/loop_8_reads",
            "value": 2163097.444117647,
            "unit": "ns"
          },
          {
            "name": "6.2/loop_iteration",
            "value": 495.32223688780823,
            "unit": "ns"
          },
          {
            "name": "6.3/cancel_select",
            "value": 96.4753036829897,
            "unit": "ns"
          },
          {
            "name": "6.3/gate_check",
            "value": 7.478105634802867,
            "unit": "ns"
          },
          {
            "name": "6.3/live_tool_body",
            "value": 1714.423906633327,
            "unit": "ns"
          },
          {
            "name": "6.3/loop_8_reads",
            "value": 2171115.7132352944,
            "unit": "ns"
          },
          {
            "name": "6.3/loop_iteration",
            "value": 551.8476102829327,
            "unit": "ns"
          },
          {
            "name": "6.3/validate_repair",
            "value": 255.06839973856867,
            "unit": "ns"
          },
          {
            "name": "6.4/cancel_select",
            "value": 94.8855982906559,
            "unit": "ns"
          },
          {
            "name": "6.4/gate_check",
            "value": 7.4732134031488835,
            "unit": "ns"
          },
          {
            "name": "6.4/live_tool_body",
            "value": 1772.0039463648498,
            "unit": "ns"
          },
          {
            "name": "6.4/loop_8_reads",
            "value": 2176418.7990196077,
            "unit": "ns"
          },
          {
            "name": "6.4/loop_iteration",
            "value": 569.761198787462,
            "unit": "ns"
          },
          {
            "name": "6.4/stall_observe",
            "value": 136.94160131859755,
            "unit": "ns"
          },
          {
            "name": "6.4/validate_repair",
            "value": 262.51804814282514,
            "unit": "ns"
          },
          {
            "name": "6.5/budget_notice",
            "value": 79.7939802636127,
            "unit": "ns"
          },
          {
            "name": "6.5/cancel_select",
            "value": 95.43673402720555,
            "unit": "ns"
          },
          {
            "name": "6.5/gate_check",
            "value": 7.4738315271861495,
            "unit": "ns"
          },
          {
            "name": "6.5/live_tool_body",
            "value": 1791.283681499055,
            "unit": "ns"
          },
          {
            "name": "6.5/loop_8_reads",
            "value": 2176786.394607843,
            "unit": "ns"
          },
          {
            "name": "6.5/loop_iteration",
            "value": 608.1398198198199,
            "unit": "ns"
          },
          {
            "name": "6.5/stall_observe",
            "value": 138.9334124844563,
            "unit": "ns"
          },
          {
            "name": "6.5/validate_repair",
            "value": 287.6095630645337,
            "unit": "ns"
          },
          {
            "name": "6.6/budget_notice",
            "value": 77.3829403920329,
            "unit": "ns"
          },
          {
            "name": "6.6/cancel_select",
            "value": 96.80500592849137,
            "unit": "ns"
          },
          {
            "name": "6.6/gate_check",
            "value": 7.481873134036713,
            "unit": "ns"
          },
          {
            "name": "6.6/live_tool_body",
            "value": 1715.719151950137,
            "unit": "ns"
          },
          {
            "name": "6.6/loop_8_reads",
            "value": 2177213.9831932774,
            "unit": "ns"
          },
          {
            "name": "6.6/loop_iteration",
            "value": 620.0370123437511,
            "unit": "ns"
          },
          {
            "name": "6.6/stall_observe",
            "value": 141.9752096907643,
            "unit": "ns"
          },
          {
            "name": "6.6/truncate_output",
            "value": 4619.9259588708555,
            "unit": "ns"
          },
          {
            "name": "6.6/validate_repair",
            "value": 263.6861951468295,
            "unit": "ns"
          },
          {
            "name": "6.7/budget_notice",
            "value": 79.33075596657463,
            "unit": "ns"
          },
          {
            "name": "6.7/cancel_select",
            "value": 95.75178364028538,
            "unit": "ns"
          },
          {
            "name": "6.7/gate_check",
            "value": 7.483746271693137,
            "unit": "ns"
          },
          {
            "name": "6.7/live_tool_body",
            "value": 1670.3682146305578,
            "unit": "ns"
          },
          {
            "name": "6.7/loop_8_reads",
            "value": 2176642.7614379087,
            "unit": "ns"
          },
          {
            "name": "6.7/loop_iteration",
            "value": 612.260889975068,
            "unit": "ns"
          },
          {
            "name": "6.7/stall_observe",
            "value": 140.71817687121765,
            "unit": "ns"
          },
          {
            "name": "6.7/steer_append",
            "value": 58.56581532242764,
            "unit": "ns"
          },
          {
            "name": "6.7/truncate_output",
            "value": 4660.691368475054,
            "unit": "ns"
          },
          {
            "name": "6.7/validate_repair",
            "value": 260.4104582172602,
            "unit": "ns"
          },
          {
            "name": "6.8/budget_notice",
            "value": 79.79059805372134,
            "unit": "ns"
          },
          {
            "name": "6.8/cancel_select",
            "value": 95.5670478901362,
            "unit": "ns"
          },
          {
            "name": "6.8/canonical_args",
            "value": 1056.8765340254472,
            "unit": "ns"
          },
          {
            "name": "6.8/gate_check",
            "value": 7.477630577459648,
            "unit": "ns"
          },
          {
            "name": "6.8/live_tool_body",
            "value": 1771.9486966383151,
            "unit": "ns"
          },
          {
            "name": "6.8/loop_8_reads",
            "value": 2176557.9095588233,
            "unit": "ns"
          },
          {
            "name": "6.8/loop_iteration",
            "value": 692.7909895968352,
            "unit": "ns"
          },
          {
            "name": "6.8/stall_observe",
            "value": 140.3984164905924,
            "unit": "ns"
          },
          {
            "name": "6.8/steer_append",
            "value": 58.100470791330366,
            "unit": "ns"
          },
          {
            "name": "6.8/truncate_output",
            "value": 4604.567288351369,
            "unit": "ns"
          },
          {
            "name": "6.8/validate_repair",
            "value": 258.51871643651134,
            "unit": "ns"
          },
          {
            "name": "6.9/budget_notice",
            "value": 78.17498961049998,
            "unit": "ns"
          },
          {
            "name": "6.9/cancel_select",
            "value": 97.14716604696767,
            "unit": "ns"
          },
          {
            "name": "6.9/canonical_args",
            "value": 1050.5151561552018,
            "unit": "ns"
          },
          {
            "name": "6.9/gate_check",
            "value": 7.48138892586998,
            "unit": "ns"
          },
          {
            "name": "6.9/live_tool_body",
            "value": 1775.1112339491615,
            "unit": "ns"
          },
          {
            "name": "6.9/loop_8_reads",
            "value": 2184059.1213235296,
            "unit": "ns"
          },
          {
            "name": "6.9/loop_iteration",
            "value": 740.3084847235075,
            "unit": "ns"
          },
          {
            "name": "6.9/outcome_label",
            "value": 8.399196822502432,
            "unit": "ns"
          },
          {
            "name": "6.9/stall_observe",
            "value": 135.63453788111704,
            "unit": "ns"
          },
          {
            "name": "6.9/steer_append",
            "value": 59.23713178281946,
            "unit": "ns"
          },
          {
            "name": "6.9/truncate_output",
            "value": 4764.0472467214295,
            "unit": "ns"
          },
          {
            "name": "6.9/validate_repair",
            "value": 267.88696050072724,
            "unit": "ns"
          },
          {
            "name": "gw5/echo_through_gate",
            "value": 45.67337197051182,
            "unit": "ns"
          },
          {
            "name": "gw5/eight_concurrent_sleeps",
            "value": 2102969.975,
            "unit": "ns"
          },
          {
            "name": "gw7/catalog_arc_swap",
            "value": 2428.5893730534717,
            "unit": "ns"
          },
          {
            "name": "gw7/catalog_reload_10k",
            "value": 28352907.3125,
            "unit": "ns"
          },
          {
            "name": "gw7/catalog_search_10k",
            "value": 302598.83467741933,
            "unit": "ns"
          },
          {
            "name": "gw8/dynamic_rename_task",
            "value": 5705650.256377552,
            "unit": "ns"
          },
          {
            "name": "gw8/every_tool_rename_task",
            "value": 8404617.157142857,
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
          "id": "8f5e668badb36f6af49c24290f17f1c98c8bf161",
          "message": "Add chapters 31–40: composition through editor integration",
          "timestamp": "2026-10-15T04:44:13Z",
          "url": "https://github.com/agnathan/rust-agent-harness/pull/7/commits/8f5e668badb36f6af49c24290f17f1c98c8bf161"
        },
        "date": 1792210103506,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "1.1/serde_50_message_request",
            "value": 10351.294188161377,
            "unit": "ns"
          },
          {
            "name": "1.2/mock_100_concurrent",
            "value": 78360.39668094218,
            "unit": "ns"
          },
          {
            "name": "1.2/mock_turn_overhead",
            "value": 265.7196722779728,
            "unit": "ns"
          },
          {
            "name": "1.3/build_request_and_parse_response",
            "value": 2915.533360073111,
            "unit": "ns"
          },
          {
            "name": "1.4/parse_response_edges",
            "value": 1136.2344170931583,
            "unit": "ns"
          },
          {
            "name": "10/scripted_turn",
            "value": 7.822107863088384,
            "unit": "ns"
          },
          {
            "name": "10/turn_transition",
            "value": 1.0908440694312131,
            "unit": "ns"
          },
          {
            "name": "11/skip_decision",
            "value": 251.15530203164218,
            "unit": "ns"
          },
          {
            "name": "12/api_request_hook",
            "value": 2.17955119852477,
            "unit": "ns"
          },
          {
            "name": "12/empty_pre_post",
            "value": 10.89929320385962,
            "unit": "ns"
          },
          {
            "name": "12/memory_enqueue",
            "value": 56.217300509983055,
            "unit": "ns"
          },
          {
            "name": "13/checkpoint_1000",
            "value": 50751.35282924885,
            "unit": "ns"
          },
          {
            "name": "13/rewind_1000",
            "value": 78285.81317204301,
            "unit": "ns"
          },
          {
            "name": "13/warm_accept",
            "value": 31029.84596042869,
            "unit": "ns"
          },
          {
            "name": "14/empty_budget",
            "value": 0.9041663288665944,
            "unit": "ns"
          },
          {
            "name": "14/repetition_guard",
            "value": 2564.2669256829427,
            "unit": "ns"
          },
          {
            "name": "15/open_breaker",
            "value": 29.987252688007555,
            "unit": "ns"
          },
          {
            "name": "15/permit_release",
            "value": 59.06291984411344,
            "unit": "ns"
          },
          {
            "name": "16/choose_route",
            "value": 6.22545398208822,
            "unit": "ns"
          },
          {
            "name": "16/rotate_key",
            "value": 3.445979609882426,
            "unit": "ns"
          },
          {
            "name": "17/buffer_join",
            "value": 712.8759639894143,
            "unit": "ns"
          },
          {
            "name": "17/fold_deltas",
            "value": 2833.911290706945,
            "unit": "ns"
          },
          {
            "name": "18/wal_append",
            "value": 137073.59065934067,
            "unit": "ns"
          },
          {
            "name": "19/memory_enqueue",
            "value": 54.07686951249993,
            "unit": "ns"
          },
          {
            "name": "2/chat_turn_overhead",
            "value": 747.5207478497593,
            "unit": "ns"
          },
          {
            "name": "2/inbox_submit",
            "value": 40.698669651355374,
            "unit": "ns"
          },
          {
            "name": "20/discover_10k",
            "value": 59127.155827442446,
            "unit": "ns"
          },
          {
            "name": "21/apply_offered",
            "value": 113.1442361622133,
            "unit": "ns"
          },
          {
            "name": "21/predict_offline",
            "value": 286969.41338582675,
            "unit": "ns"
          },
          {
            "name": "22/load_render_500",
            "value": 5973463.696428571,
            "unit": "ns"
          },
          {
            "name": "23/enqueue_unsolicited",
            "value": 20.215360485985105,
            "unit": "ns"
          },
          {
            "name": "23/scan_one_skill",
            "value": 282.2952365917048,
            "unit": "ns"
          },
          {
            "name": "24/cancel_review",
            "value": 754345.7892156863,
            "unit": "ns"
          },
          {
            "name": "24/spawn_reviewer",
            "value": 85.68467694163135,
            "unit": "ns"
          },
          {
            "name": "25/enqueue_curation",
            "value": 19.963544464070363,
            "unit": "ns"
          },
          {
            "name": "26/arc_pass",
            "value": 4.394759607312957,
            "unit": "ns"
          },
          {
            "name": "26/copy_transcript",
            "value": 7569.800942285041,
            "unit": "ns"
          },
          {
            "name": "27/enqueue_reflection",
            "value": 20.81876933543012,
            "unit": "ns"
          },
          {
            "name": "27/schedule_eight",
            "value": 277971.5297164668,
            "unit": "ns"
          },
          {
            "name": "28/filter_100",
            "value": 4039.3252557260394,
            "unit": "ns"
          },
          {
            "name": "29/schedule_100",
            "value": 6865.821854552252,
            "unit": "ns"
          },
          {
            "name": "3.1/registry_lookup",
            "value": 25.725805726170996,
            "unit": "ns"
          },
          {
            "name": "3.2/available_tools_50",
            "value": 30661.669831932773,
            "unit": "ns"
          },
          {
            "name": "3.3/dispatch_8_reads_concurrent",
            "value": 2137976.2977124183,
            "unit": "ns"
          },
          {
            "name": "3.3/dispatch_8_reads_sequential",
            "value": 16739406.283333333,
            "unit": "ns"
          },
          {
            "name": "30/judge_one_permit",
            "value": 384.970370938399,
            "unit": "ns"
          },
          {
            "name": "31/nested_eight",
            "value": 282714.88365384616,
            "unit": "ns"
          },
          {
            "name": "32/share_snapshot",
            "value": 4.0051346826958705,
            "unit": "ns"
          },
          {
            "name": "33/inbox_submit",
            "value": 256.7476762533829,
            "unit": "ns"
          },
          {
            "name": "34/restart_bookkeeping",
            "value": 27.15533150075916,
            "unit": "ns"
          },
          {
            "name": "35/pick_least",
            "value": 3.7375780964808643,
            "unit": "ns"
          },
          {
            "name": "36/resolve_profile",
            "value": 95.2824050470912,
            "unit": "ns"
          },
          {
            "name": "37/enqueue_platform",
            "value": 159.67122609608728,
            "unit": "ns"
          },
          {
            "name": "38/route_session",
            "value": 19.480008774975992,
            "unit": "ns"
          },
          {
            "name": "39/scan_thousand",
            "value": 1610.3442115290068,
            "unit": "ns"
          },
          {
            "name": "4.1/stdio_echo_round_trip",
            "value": 208312.3180952381,
            "unit": "ns"
          },
          {
            "name": "4.2/http_echo_round_trip",
            "value": 414196.12800324673,
            "unit": "ns"
          },
          {
            "name": "4.3/mcp_50_concurrent",
            "value": 5076350.676339285,
            "unit": "ns"
          },
          {
            "name": "4.3/mcp_50_sequential",
            "value": 10445394.72361111,
            "unit": "ns"
          },
          {
            "name": "4.4/mcp_list_cold",
            "value": 215638.4036348267,
            "unit": "ns"
          },
          {
            "name": "4.4/mcp_list_hit",
            "value": 258.7989651311694,
            "unit": "ns"
          },
          {
            "name": "40/translate_update",
            "value": 128.1188327695723,
            "unit": "ns"
          },
          {
            "name": "5.1/schema_tokens_1000",
            "value": 1270864.9683908047,
            "unit": "ns"
          },
          {
            "name": "5.1/schema_tokens_10000",
            "value": 42060858.36666667,
            "unit": "ns"
          },
          {
            "name": "5.2/index_build_10000",
            "value": 21090707.826388888,
            "unit": "ns"
          },
          {
            "name": "5.2/index_search_10000",
            "value": 332968.745412844,
            "unit": "ns"
          },
          {
            "name": "5.3/full_schema_tokens_10000",
            "value": 21871571.666666664,
            "unit": "ns"
          },
          {
            "name": "5.3/lazy_visible_prompt_10000",
            "value": 15596.98879804934,
            "unit": "ns"
          },
          {
            "name": "5.4/routed_search_10000",
            "value": 25844.222300801506,
            "unit": "ns"
          },
          {
            "name": "5.4/shrink_schemas_10000",
            "value": 62426472.1,
            "unit": "ns"
          },
          {
            "name": "5.5/agent_filter_10000",
            "value": 123996.33347554039,
            "unit": "ns"
          },
          {
            "name": "5.5/cache_cold_10000",
            "value": 25046459.700000003,
            "unit": "ns"
          },
          {
            "name": "5.5/cache_hit_10000",
            "value": 13.797589596919664,
            "unit": "ns"
          },
          {
            "name": "6.1/gate_check",
            "value": 3.4254206200879365,
            "unit": "ns"
          },
          {
            "name": "6.1/live_tool_body",
            "value": 1729.422917984398,
            "unit": "ns"
          },
          {
            "name": "6.1/loop_8_reads",
            "value": 2154655.2585784313,
            "unit": "ns"
          },
          {
            "name": "6.1/loop_iteration",
            "value": 363.5993466560351,
            "unit": "ns"
          },
          {
            "name": "6.10/budget_notice",
            "value": 78.6597433218086,
            "unit": "ns"
          },
          {
            "name": "6.10/cancel_select",
            "value": 95.27372783845857,
            "unit": "ns"
          },
          {
            "name": "6.10/canonical_args",
            "value": 1078.7688947909865,
            "unit": "ns"
          },
          {
            "name": "6.10/gate_check",
            "value": 7.480113070608432,
            "unit": "ns"
          },
          {
            "name": "6.10/live_tool_body",
            "value": 1795.5212163596025,
            "unit": "ns"
          },
          {
            "name": "6.10/loop_8_reads",
            "value": 2183967.217647059,
            "unit": "ns"
          },
          {
            "name": "6.10/loop_iteration",
            "value": 763.4278599359375,
            "unit": "ns"
          },
          {
            "name": "6.10/outcome_label",
            "value": 8.398207703196054,
            "unit": "ns"
          },
          {
            "name": "6.10/stall_observe",
            "value": 137.63050893715018,
            "unit": "ns"
          },
          {
            "name": "6.10/steer_append",
            "value": 59.897154536179826,
            "unit": "ns"
          },
          {
            "name": "6.10/stress_repair",
            "value": 53302.4860375159,
            "unit": "ns"
          },
          {
            "name": "6.10/truncate_output",
            "value": 4546.537755697204,
            "unit": "ns"
          },
          {
            "name": "6.10/validate_repair",
            "value": 276.934575667731,
            "unit": "ns"
          },
          {
            "name": "6.2/cancel_select",
            "value": 96.42892464031272,
            "unit": "ns"
          },
          {
            "name": "6.2/gate_check",
            "value": 7.484955343097051,
            "unit": "ns"
          },
          {
            "name": "6.2/live_tool_body",
            "value": 1766.2931383255209,
            "unit": "ns"
          },
          {
            "name": "6.2/loop_8_reads",
            "value": 2165278.5849673203,
            "unit": "ns"
          },
          {
            "name": "6.2/loop_iteration",
            "value": 492.06349148061105,
            "unit": "ns"
          },
          {
            "name": "6.3/cancel_select",
            "value": 96.4375900925348,
            "unit": "ns"
          },
          {
            "name": "6.3/gate_check",
            "value": 7.476254031977811,
            "unit": "ns"
          },
          {
            "name": "6.3/live_tool_body",
            "value": 1739.304745761564,
            "unit": "ns"
          },
          {
            "name": "6.3/loop_8_reads",
            "value": 2174404.767973856,
            "unit": "ns"
          },
          {
            "name": "6.3/loop_iteration",
            "value": 549.9753774257666,
            "unit": "ns"
          },
          {
            "name": "6.3/validate_repair",
            "value": 261.28945564518204,
            "unit": "ns"
          },
          {
            "name": "6.4/cancel_select",
            "value": 96.26734461617534,
            "unit": "ns"
          },
          {
            "name": "6.4/gate_check",
            "value": 7.476239070323242,
            "unit": "ns"
          },
          {
            "name": "6.4/live_tool_body",
            "value": 1770.6208540226528,
            "unit": "ns"
          },
          {
            "name": "6.4/loop_8_reads",
            "value": 2177862.7705882355,
            "unit": "ns"
          },
          {
            "name": "6.4/loop_iteration",
            "value": 557.1125619986528,
            "unit": "ns"
          },
          {
            "name": "6.4/stall_observe",
            "value": 139.36181476037603,
            "unit": "ns"
          },
          {
            "name": "6.4/validate_repair",
            "value": 262.6672642109524,
            "unit": "ns"
          },
          {
            "name": "6.5/budget_notice",
            "value": 76.9738173005648,
            "unit": "ns"
          },
          {
            "name": "6.5/cancel_select",
            "value": 95.71277113406114,
            "unit": "ns"
          },
          {
            "name": "6.5/gate_check",
            "value": 7.479307754965326,
            "unit": "ns"
          },
          {
            "name": "6.5/live_tool_body",
            "value": 1743.8001704989083,
            "unit": "ns"
          },
          {
            "name": "6.5/loop_8_reads",
            "value": 2177955.75,
            "unit": "ns"
          },
          {
            "name": "6.5/loop_iteration",
            "value": 649.351208536221,
            "unit": "ns"
          },
          {
            "name": "6.5/stall_observe",
            "value": 134.1417577866443,
            "unit": "ns"
          },
          {
            "name": "6.5/validate_repair",
            "value": 287.76598133797245,
            "unit": "ns"
          },
          {
            "name": "6.6/budget_notice",
            "value": 79.01300422148182,
            "unit": "ns"
          },
          {
            "name": "6.6/cancel_select",
            "value": 96.71684557024497,
            "unit": "ns"
          },
          {
            "name": "6.6/gate_check",
            "value": 7.475817580648595,
            "unit": "ns"
          },
          {
            "name": "6.6/live_tool_body",
            "value": 1725.5185722061817,
            "unit": "ns"
          },
          {
            "name": "6.6/loop_8_reads",
            "value": 2180247.0176470587,
            "unit": "ns"
          },
          {
            "name": "6.6/loop_iteration",
            "value": 615.4638380221288,
            "unit": "ns"
          },
          {
            "name": "6.6/stall_observe",
            "value": 140.3341801121283,
            "unit": "ns"
          },
          {
            "name": "6.6/truncate_output",
            "value": 4609.281182065218,
            "unit": "ns"
          },
          {
            "name": "6.6/validate_repair",
            "value": 266.4977562982139,
            "unit": "ns"
          },
          {
            "name": "6.7/budget_notice",
            "value": 77.95505761029827,
            "unit": "ns"
          },
          {
            "name": "6.7/cancel_select",
            "value": 97.58819726835748,
            "unit": "ns"
          },
          {
            "name": "6.7/gate_check",
            "value": 7.4806613054954205,
            "unit": "ns"
          },
          {
            "name": "6.7/live_tool_body",
            "value": 1649.4173470250798,
            "unit": "ns"
          },
          {
            "name": "6.7/loop_8_reads",
            "value": 2181001.06372549,
            "unit": "ns"
          },
          {
            "name": "6.7/loop_iteration",
            "value": 612.1691388337649,
            "unit": "ns"
          },
          {
            "name": "6.7/stall_observe",
            "value": 139.95529426975855,
            "unit": "ns"
          },
          {
            "name": "6.7/steer_append",
            "value": 58.42376318093112,
            "unit": "ns"
          },
          {
            "name": "6.7/truncate_output",
            "value": 4601.336616647428,
            "unit": "ns"
          },
          {
            "name": "6.7/validate_repair",
            "value": 269.12458558535184,
            "unit": "ns"
          },
          {
            "name": "6.8/budget_notice",
            "value": 78.9580508920225,
            "unit": "ns"
          },
          {
            "name": "6.8/cancel_select",
            "value": 96.99104133226325,
            "unit": "ns"
          },
          {
            "name": "6.8/canonical_args",
            "value": 1096.4508770514249,
            "unit": "ns"
          },
          {
            "name": "6.8/gate_check",
            "value": 7.47676776507462,
            "unit": "ns"
          },
          {
            "name": "6.8/live_tool_body",
            "value": 1860.970162316186,
            "unit": "ns"
          },
          {
            "name": "6.8/loop_8_reads",
            "value": 2184723.6270424835,
            "unit": "ns"
          },
          {
            "name": "6.8/loop_iteration",
            "value": 673.7257997595993,
            "unit": "ns"
          },
          {
            "name": "6.8/stall_observe",
            "value": 135.4740127916068,
            "unit": "ns"
          },
          {
            "name": "6.8/steer_append",
            "value": 58.02187239742706,
            "unit": "ns"
          },
          {
            "name": "6.8/truncate_output",
            "value": 4597.646382595262,
            "unit": "ns"
          },
          {
            "name": "6.8/validate_repair",
            "value": 258.66505331765524,
            "unit": "ns"
          },
          {
            "name": "6.9/budget_notice",
            "value": 77.3231233958956,
            "unit": "ns"
          },
          {
            "name": "6.9/cancel_select",
            "value": 96.41892599183502,
            "unit": "ns"
          },
          {
            "name": "6.9/canonical_args",
            "value": 1052.5268122832895,
            "unit": "ns"
          },
          {
            "name": "6.9/gate_check",
            "value": 7.47415971710881,
            "unit": "ns"
          },
          {
            "name": "6.9/live_tool_body",
            "value": 1827.8608172781046,
            "unit": "ns"
          },
          {
            "name": "6.9/loop_8_reads",
            "value": 2183794.8382352944,
            "unit": "ns"
          },
          {
            "name": "6.9/loop_iteration",
            "value": 736.9964268371641,
            "unit": "ns"
          },
          {
            "name": "6.9/outcome_label",
            "value": 8.400417708827213,
            "unit": "ns"
          },
          {
            "name": "6.9/stall_observe",
            "value": 136.99229842598558,
            "unit": "ns"
          },
          {
            "name": "6.9/steer_append",
            "value": 59.22962504348695,
            "unit": "ns"
          },
          {
            "name": "6.9/truncate_output",
            "value": 4548.700963274592,
            "unit": "ns"
          },
          {
            "name": "6.9/validate_repair",
            "value": 268.23483921535626,
            "unit": "ns"
          },
          {
            "name": "gw5/echo_through_gate",
            "value": 45.64298192917978,
            "unit": "ns"
          },
          {
            "name": "gw5/eight_concurrent_sleeps",
            "value": 2127680.902777778,
            "unit": "ns"
          },
          {
            "name": "gw7/catalog_arc_swap",
            "value": 2387.5994495312843,
            "unit": "ns"
          },
          {
            "name": "gw7/catalog_reload_10k",
            "value": 27514321.183333334,
            "unit": "ns"
          },
          {
            "name": "gw7/catalog_search_10k",
            "value": 300726.98155737703,
            "unit": "ns"
          },
          {
            "name": "gw8/dynamic_rename_task",
            "value": 5704036.244444445,
            "unit": "ns"
          },
          {
            "name": "gw8/every_tool_rename_task",
            "value": 8427277.897619048,
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
          "id": "af162cc08e70944a407306486bedfbb7da32aec1",
          "message": "Write chapters 11 through 20",
          "timestamp": "2026-10-15T04:44:13Z",
          "url": "https://github.com/agnathan/rust-agent-harness/pull/5/commits/af162cc08e70944a407306486bedfbb7da32aec1"
        },
        "date": 1792211221895,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "1.1/serde_50_message_request",
            "value": 7925.9442164587435,
            "unit": "ns"
          },
          {
            "name": "1.2/mock_100_concurrent",
            "value": 103847.95379338176,
            "unit": "ns"
          },
          {
            "name": "1.2/mock_turn_overhead",
            "value": 234.38272256367583,
            "unit": "ns"
          },
          {
            "name": "1.3/build_request_and_parse_response",
            "value": 2427.6299516267973,
            "unit": "ns"
          },
          {
            "name": "1.4/parse_response_edges",
            "value": 858.0066237021124,
            "unit": "ns"
          },
          {
            "name": "10/scripted_turn",
            "value": 7.84981479130773,
            "unit": "ns"
          },
          {
            "name": "10/turn_transition",
            "value": 1.1335098797600391,
            "unit": "ns"
          },
          {
            "name": "11/skip_decision",
            "value": 204.209380705775,
            "unit": "ns"
          },
          {
            "name": "12/api_request_hook",
            "value": 1.9691214716785903,
            "unit": "ns"
          },
          {
            "name": "12/empty_pre_post",
            "value": 9.066351943667595,
            "unit": "ns"
          },
          {
            "name": "12/memory_enqueue",
            "value": 46.81172180076476,
            "unit": "ns"
          },
          {
            "name": "13/checkpoint_1000",
            "value": 68112.71388888889,
            "unit": "ns"
          },
          {
            "name": "13/rewind_1000",
            "value": 81136.57024597918,
            "unit": "ns"
          },
          {
            "name": "13/warm_accept",
            "value": 28435.031799431294,
            "unit": "ns"
          },
          {
            "name": "14/empty_budget",
            "value": 0.7694053083663588,
            "unit": "ns"
          },
          {
            "name": "14/repetition_guard",
            "value": 1836.4531188458227,
            "unit": "ns"
          },
          {
            "name": "15/open_breaker",
            "value": 24.642748720157872,
            "unit": "ns"
          },
          {
            "name": "15/permit_release",
            "value": 90.21883686029578,
            "unit": "ns"
          },
          {
            "name": "16/choose_route",
            "value": 4.755697469294647,
            "unit": "ns"
          },
          {
            "name": "16/rotate_key",
            "value": 4.804318703770271,
            "unit": "ns"
          },
          {
            "name": "17/buffer_join",
            "value": 569.2965514914144,
            "unit": "ns"
          },
          {
            "name": "17/fold_deltas",
            "value": 2234.719072096542,
            "unit": "ns"
          },
          {
            "name": "18/wal_append",
            "value": 140376.80591016548,
            "unit": "ns"
          },
          {
            "name": "19/memory_enqueue",
            "value": 43.553411779149926,
            "unit": "ns"
          },
          {
            "name": "2/chat_turn_overhead",
            "value": 648.4497457555376,
            "unit": "ns"
          },
          {
            "name": "2/inbox_submit",
            "value": 35.29209740859767,
            "unit": "ns"
          },
          {
            "name": "20/discover_10k",
            "value": 31502.686228973136,
            "unit": "ns"
          },
          {
            "name": "3.1/registry_lookup",
            "value": 15.788007617495198,
            "unit": "ns"
          },
          {
            "name": "3.2/available_tools_50",
            "value": 24425.075382663177,
            "unit": "ns"
          },
          {
            "name": "3.3/dispatch_8_reads_concurrent",
            "value": 2121046.6424603174,
            "unit": "ns"
          },
          {
            "name": "3.3/dispatch_8_reads_sequential",
            "value": 16589360.555555556,
            "unit": "ns"
          },
          {
            "name": "4.1/stdio_echo_round_trip",
            "value": 158178.2726608187,
            "unit": "ns"
          },
          {
            "name": "4.2/http_echo_round_trip",
            "value": 346497.6945601852,
            "unit": "ns"
          },
          {
            "name": "4.3/mcp_50_concurrent",
            "value": 2969745.2564102565,
            "unit": "ns"
          },
          {
            "name": "4.3/mcp_50_sequential",
            "value": 6151127.28125,
            "unit": "ns"
          },
          {
            "name": "4.4/mcp_list_cold",
            "value": 170692.6808815858,
            "unit": "ns"
          },
          {
            "name": "4.4/mcp_list_hit",
            "value": 193.95101902137566,
            "unit": "ns"
          },
          {
            "name": "5.1/schema_tokens_1000",
            "value": 1002490.3972972974,
            "unit": "ns"
          },
          {
            "name": "5.1/schema_tokens_10000",
            "value": 21378549.875,
            "unit": "ns"
          },
          {
            "name": "5.2/index_build_10000",
            "value": 15886677.452380951,
            "unit": "ns"
          },
          {
            "name": "5.2/index_search_10000",
            "value": 163890.99742599743,
            "unit": "ns"
          },
          {
            "name": "5.3/full_schema_tokens_10000",
            "value": 11069036.241319444,
            "unit": "ns"
          },
          {
            "name": "5.3/lazy_visible_prompt_10000",
            "value": 14263.92127414496,
            "unit": "ns"
          },
          {
            "name": "5.4/routed_search_10000",
            "value": 17786.314248366012,
            "unit": "ns"
          },
          {
            "name": "5.4/shrink_schemas_10000",
            "value": 32820903.2,
            "unit": "ns"
          },
          {
            "name": "5.5/agent_filter_10000",
            "value": 62730.36912677305,
            "unit": "ns"
          },
          {
            "name": "5.5/cache_cold_10000",
            "value": 12435215,
            "unit": "ns"
          },
          {
            "name": "5.5/cache_hit_10000",
            "value": 19.87058052410743,
            "unit": "ns"
          },
          {
            "name": "6.1/gate_check",
            "value": 2.8288425138653683,
            "unit": "ns"
          },
          {
            "name": "6.1/live_tool_body",
            "value": 1915.496420903679,
            "unit": "ns"
          },
          {
            "name": "6.1/loop_8_reads",
            "value": 2131350.7037037034,
            "unit": "ns"
          },
          {
            "name": "6.1/loop_iteration",
            "value": 279.74704115811574,
            "unit": "ns"
          },
          {
            "name": "6.10/budget_notice",
            "value": 63.21033484187974,
            "unit": "ns"
          },
          {
            "name": "6.10/cancel_select",
            "value": 180.69117853177676,
            "unit": "ns"
          },
          {
            "name": "6.10/canonical_args",
            "value": 887.7688224588837,
            "unit": "ns"
          },
          {
            "name": "6.10/gate_check",
            "value": 16.679793839589387,
            "unit": "ns"
          },
          {
            "name": "6.10/live_tool_body",
            "value": 1701.6847936342756,
            "unit": "ns"
          },
          {
            "name": "6.10/loop_8_reads",
            "value": 2151787.8655462186,
            "unit": "ns"
          },
          {
            "name": "6.10/loop_iteration",
            "value": 746.5631515512573,
            "unit": "ns"
          },
          {
            "name": "6.10/outcome_label",
            "value": 6.2262739233692,
            "unit": "ns"
          },
          {
            "name": "6.10/stall_observe",
            "value": 113.27977498600777,
            "unit": "ns"
          },
          {
            "name": "6.10/steer_append",
            "value": 54.88726859413666,
            "unit": "ns"
          },
          {
            "name": "6.10/stress_repair",
            "value": 46370.62265331665,
            "unit": "ns"
          },
          {
            "name": "6.10/truncate_output",
            "value": 3591.988769004414,
            "unit": "ns"
          },
          {
            "name": "6.10/validate_repair",
            "value": 264.73872447292706,
            "unit": "ns"
          },
          {
            "name": "6.2/cancel_select",
            "value": 180.59658961808776,
            "unit": "ns"
          },
          {
            "name": "6.2/gate_check",
            "value": 16.37826698494615,
            "unit": "ns"
          },
          {
            "name": "6.2/live_tool_body",
            "value": 1660.3223036834884,
            "unit": "ns"
          },
          {
            "name": "6.2/loop_8_reads",
            "value": 2135883.5694444445,
            "unit": "ns"
          },
          {
            "name": "6.2/loop_iteration",
            "value": 532.9462327647689,
            "unit": "ns"
          },
          {
            "name": "6.3/cancel_select",
            "value": 182.86968389569182,
            "unit": "ns"
          },
          {
            "name": "6.3/gate_check",
            "value": 17.923995478763214,
            "unit": "ns"
          },
          {
            "name": "6.3/live_tool_body",
            "value": 1730.6908847431107,
            "unit": "ns"
          },
          {
            "name": "6.3/loop_8_reads",
            "value": 2141403.444852941,
            "unit": "ns"
          },
          {
            "name": "6.3/loop_iteration",
            "value": 541.0164083858315,
            "unit": "ns"
          },
          {
            "name": "6.3/validate_repair",
            "value": 251.81677774404827,
            "unit": "ns"
          },
          {
            "name": "6.4/cancel_select",
            "value": 182.71177626023882,
            "unit": "ns"
          },
          {
            "name": "6.4/gate_check",
            "value": 16.36543673842592,
            "unit": "ns"
          },
          {
            "name": "6.4/live_tool_body",
            "value": 1810.9959269087958,
            "unit": "ns"
          },
          {
            "name": "6.4/loop_8_reads",
            "value": 2147211.905882353,
            "unit": "ns"
          },
          {
            "name": "6.4/loop_iteration",
            "value": 542.8052291737098,
            "unit": "ns"
          },
          {
            "name": "6.4/stall_observe",
            "value": 114.14711172171621,
            "unit": "ns"
          },
          {
            "name": "6.4/validate_repair",
            "value": 238.74817020045913,
            "unit": "ns"
          },
          {
            "name": "6.5/budget_notice",
            "value": 63.73865442150654,
            "unit": "ns"
          },
          {
            "name": "6.5/cancel_select",
            "value": 180.30261986700452,
            "unit": "ns"
          },
          {
            "name": "6.5/gate_check",
            "value": 16.315933058731954,
            "unit": "ns"
          },
          {
            "name": "6.5/live_tool_body",
            "value": 1629.8202651347115,
            "unit": "ns"
          },
          {
            "name": "6.5/loop_8_reads",
            "value": 2144127.4754901957,
            "unit": "ns"
          },
          {
            "name": "6.5/loop_iteration",
            "value": 579.8329410224749,
            "unit": "ns"
          },
          {
            "name": "6.5/stall_observe",
            "value": 110.84203779248769,
            "unit": "ns"
          },
          {
            "name": "6.5/validate_repair",
            "value": 253.60593544851326,
            "unit": "ns"
          },
          {
            "name": "6.6/budget_notice",
            "value": 65.47461979536607,
            "unit": "ns"
          },
          {
            "name": "6.6/cancel_select",
            "value": 181.42901967096924,
            "unit": "ns"
          },
          {
            "name": "6.6/gate_check",
            "value": 16.524104647809516,
            "unit": "ns"
          },
          {
            "name": "6.6/live_tool_body",
            "value": 1660.1691663633055,
            "unit": "ns"
          },
          {
            "name": "6.6/loop_8_reads",
            "value": 2146374.0620915033,
            "unit": "ns"
          },
          {
            "name": "6.6/loop_iteration",
            "value": 588.1042907151738,
            "unit": "ns"
          },
          {
            "name": "6.6/stall_observe",
            "value": 118.52862179445094,
            "unit": "ns"
          },
          {
            "name": "6.6/truncate_output",
            "value": 3477.965868737432,
            "unit": "ns"
          },
          {
            "name": "6.6/validate_repair",
            "value": 253.59057395421814,
            "unit": "ns"
          },
          {
            "name": "6.7/budget_notice",
            "value": 61.40702537959112,
            "unit": "ns"
          },
          {
            "name": "6.7/cancel_select",
            "value": 184.04519786561832,
            "unit": "ns"
          },
          {
            "name": "6.7/gate_check",
            "value": 16.354266269868994,
            "unit": "ns"
          },
          {
            "name": "6.7/live_tool_body",
            "value": 1575.6785944679227,
            "unit": "ns"
          },
          {
            "name": "6.7/loop_8_reads",
            "value": 2143954.6176470593,
            "unit": "ns"
          },
          {
            "name": "6.7/loop_iteration",
            "value": 584.8089386016276,
            "unit": "ns"
          },
          {
            "name": "6.7/stall_observe",
            "value": 112.3648201640881,
            "unit": "ns"
          },
          {
            "name": "6.7/steer_append",
            "value": 55.45697449964442,
            "unit": "ns"
          },
          {
            "name": "6.7/truncate_output",
            "value": 3553.5685175720405,
            "unit": "ns"
          },
          {
            "name": "6.7/validate_repair",
            "value": 250.22950555833302,
            "unit": "ns"
          },
          {
            "name": "6.8/budget_notice",
            "value": 61.63072480789137,
            "unit": "ns"
          },
          {
            "name": "6.8/cancel_select",
            "value": 177.933744479092,
            "unit": "ns"
          },
          {
            "name": "6.8/canonical_args",
            "value": 883.9547233042074,
            "unit": "ns"
          },
          {
            "name": "6.8/gate_check",
            "value": 16.400385718662726,
            "unit": "ns"
          },
          {
            "name": "6.8/live_tool_body",
            "value": 1637.9550684808073,
            "unit": "ns"
          },
          {
            "name": "6.8/loop_8_reads",
            "value": 2147081.8680555555,
            "unit": "ns"
          },
          {
            "name": "6.8/loop_iteration",
            "value": 694.9860670770111,
            "unit": "ns"
          },
          {
            "name": "6.8/stall_observe",
            "value": 113.95421482825648,
            "unit": "ns"
          },
          {
            "name": "6.8/steer_append",
            "value": 54.00363418215232,
            "unit": "ns"
          },
          {
            "name": "6.8/truncate_output",
            "value": 3457.3306930534154,
            "unit": "ns"
          },
          {
            "name": "6.8/validate_repair",
            "value": 210.74861243116345,
            "unit": "ns"
          },
          {
            "name": "6.9/budget_notice",
            "value": 63.64776059375944,
            "unit": "ns"
          },
          {
            "name": "6.9/cancel_select",
            "value": 180.24405921332504,
            "unit": "ns"
          },
          {
            "name": "6.9/canonical_args",
            "value": 885.9074006875735,
            "unit": "ns"
          },
          {
            "name": "6.9/gate_check",
            "value": 16.314492308090426,
            "unit": "ns"
          },
          {
            "name": "6.9/live_tool_body",
            "value": 1781.267290370739,
            "unit": "ns"
          },
          {
            "name": "6.9/loop_8_reads",
            "value": 2151510.341911765,
            "unit": "ns"
          },
          {
            "name": "6.9/loop_iteration",
            "value": 752.6178030948099,
            "unit": "ns"
          },
          {
            "name": "6.9/outcome_label",
            "value": 6.234441481891423,
            "unit": "ns"
          },
          {
            "name": "6.9/stall_observe",
            "value": 110.21125445779381,
            "unit": "ns"
          },
          {
            "name": "6.9/steer_append",
            "value": 54.07352705603194,
            "unit": "ns"
          },
          {
            "name": "6.9/truncate_output",
            "value": 3570.086407103825,
            "unit": "ns"
          },
          {
            "name": "6.9/validate_repair",
            "value": 236.14970207611628,
            "unit": "ns"
          },
          {
            "name": "gw5/echo_through_gate",
            "value": 94.93038407703156,
            "unit": "ns"
          },
          {
            "name": "gw5/eight_concurrent_sleeps",
            "value": 2102338.6005291007,
            "unit": "ns"
          },
          {
            "name": "gw7/catalog_arc_swap",
            "value": 2108.6480361194435,
            "unit": "ns"
          },
          {
            "name": "gw7/catalog_reload_10k",
            "value": 23000694.839285716,
            "unit": "ns"
          },
          {
            "name": "gw7/catalog_search_10k",
            "value": 158853.18976608186,
            "unit": "ns"
          },
          {
            "name": "gw8/dynamic_rename_task",
            "value": 4542453.5546875,
            "unit": "ns"
          },
          {
            "name": "gw8/every_tool_rename_task",
            "value": 6572193.764814815,
            "unit": "ns"
          }
        ]
      }
    ]
  }
}