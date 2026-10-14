window.BENCHMARK_DATA = {
  "lastUpdate": 1791937631428,
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
      }
    ]
  }
}