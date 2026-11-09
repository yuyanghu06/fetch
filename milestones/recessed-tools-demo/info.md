# Recessed server and tools demo

Goal: apply pressed selected navigation and show read-only tool details on raised cards inside a recessed, independently scrolling tools area.

Scope: new Figma app/component pages only. Active navbar selected appearance is updated across all29 new native-window specimens. Installed servers recess and enlarged window are trialed on Tools expanded78:1438 and scrolling demo177:3654; other source references and window sizes remain separate. No Rust/API changes.

Selected behavior and design: active navigation is blue/white/bold but inset with no glow; inactive tabs remain raised. Installed servers and Available tools use recessed warm-gray #EAE6DF. Shared Available tools area177:1914 has a fixed1072×440 viewport and vertical scrolling. Six tool cards form three columns and two rows, with325.333px square cards and24px spacing/padding. Content is722.667px high, providing282.667px actual internal overflow. Cards remain raised. Shared read-only tool value182:3991 replaces gray Text input instances with raised warm-white plain-text details.

Demo data: preserve the original read_file JSON; five added definitions (write_file, list_directory, create_directory, move_file, search_files) are illustrative fixtures, not live discovery. Full JSON remains on each card; visible values reflect the JSON.

Window trial: both tools windows are1248×1160 inside1328×1240 stages. Original page scrolling remains, including navigation. Outer viewport1200×1080 contains1200×1046 page content. Installed servers area1168×906 has24px padding, preserving1120px server cards and1072px tool well. Neighboring Remote console frame moved down on the canvas to avoid overlap; its product layout is unchanged.

Completion criteria: three cards per row; correct read-only values; active nav inset/no glow; actual scroll inside tools reaches the last row without moving the page; full expanded server/footer and last server fit the enlarged window; independent final review accepts.

Status: implementation and independent actual nested-scroll/navigation/read-only checks PASS. Independent final review ACCEPT. [Review](../history/2026-11-08_19-12-00-193541-evaluator-accept-recessed-tools-demo.md). [Evidence](../history/2026-11-08_19-11-34-697766-tester-pass-recessed-tools-scrolling.md).

The earlier whole-page recessed scrolling trial remains cancelled. This user-requested tools/servers section treatment does not restore fixed navigation or that cancelled whole-page scroll design. Shift-click force restart remains documented intent for later application implementation, not an executable Figma modifier action.

Open the demo with `scaling=scale-down` (Fit width and height). At Actual size100% the enlarged artifact can exceed the monitor; that is preview scaling, not clipped design content.
