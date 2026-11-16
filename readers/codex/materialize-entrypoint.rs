    /// Fetch's private-snapshot entrypoint. Exposes existing official projection only.
    /// Never call this against an original Codex home or source rollout.
    pub async fn fetch_materialize_history(
        &self,
        thread_id: ThreadId,
        rollout_path: PathBuf,
    ) -> ThreadStoreResult<()> {
        let lineage = self.resolve_rollout_lineage(thread_id, Some(rollout_path)).await?;
        for segment in lineage.segments() {
            thread_history_materialization::materialize_to_sqlite(
                self, segment.rollout_id(), segment.rollout_path.as_path(),
            ).await?;
        }
        Ok(())
    }

    /// Present the name using the same native name rules as LocalThreadStore reads.
    pub fn fetch_native_thread_name(metadata: &codex_state::ThreadMetadata) -> Option<String> {
        match metadata.history_mode {
            codex_protocol::protocol::ThreadHistoryMode::Legacy => helpers::distinct_thread_metadata_title(metadata),
            codex_protocol::protocol::ThreadHistoryMode::Paginated => helpers::sqlite_thread_name(metadata),
        }
    }

