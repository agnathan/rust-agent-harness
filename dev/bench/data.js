window.BENCHMARK_DATA = {
  "lastUpdate": 1792186974022,
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
      }
    ]
  }
}