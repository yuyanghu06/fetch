# Reader acceptance fixtures

Run `python3 readers/codex/tests/fixtures.py /absolute/path/to/fetch-codex-session-reader`
after setup. It creates disposable native-shaped sessions and requests actual
complete conversations. It checks long text, native pagination and inherited fork
history, native read-only index selection/name, and explicit missing-ancestry
errors. It verifies original fixture and index bytes were unchanged. Personal session
contents and Codex settings are never read.

The mixed missing-current-rollout case proves an unrelated valid conversation still lists, the stale thread never falls back to its older file, and source/index bytes remain unchanged.
