# One server target

delete.rs handles DELETE /api/v1/servers/{server_id}. It applies existing
management authorization/target hiding, rejects query/body input and delegates
to shared deletion. Runtime lifecycle, cleanup and repeat/busy behavior belong
to Manager; HTTP errors and no-store behavior retain common API boundaries.
