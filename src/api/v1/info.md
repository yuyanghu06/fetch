# Versioned application HTTP handlers

This folder holds HTTP adapters for /api/v1. servers/ adds deletion and native folder opening in
URL-shaped modules; existing route families still live in older API files until
the separately requested reorganization. Backend business logic stays in shared
services. No handler owns credential/file cleanup directly.
