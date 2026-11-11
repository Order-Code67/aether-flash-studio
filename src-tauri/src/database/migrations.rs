pub const MIGRATIONS: &[&str] = &[
    r#"
    CREATE TABLE IF NOT EXISTS schema_migrations (
        version INTEGER PRIMARY KEY NOT NULL,
        applied_at TEXT NOT NULL
    );

    CREATE TABLE IF NOT EXISTS manufacturers (
        id TEXT PRIMARY KEY NOT NULL,
        name TEXT NOT NULL UNIQUE,
        website TEXT,
        notes TEXT,
        created_at TEXT NOT NULL
    );

    CREATE TABLE IF NOT EXISTS devices (
        id TEXT PRIMARY KEY NOT NULL,
        name TEXT NOT NULL,
        manufacturer TEXT,
        device_type TEXT NOT NULL,
        model TEXT,
        mcu TEXT,
        com_port TEXT,
        vid INTEGER,
        pid INTEGER,
        usb_serial TEXT,
        radio_system TEXT,
        frequency_band TEXT,
        firmware_version TEXT,
        bootloader_version TEXT,
        connection_mode TEXT,
        flash_method TEXT,
        status TEXT NOT NULL,
        is_mock INTEGER NOT NULL DEFAULT 0,
        last_seen_at TEXT,
        created_at TEXT NOT NULL,
        updated_at TEXT NOT NULL
    );

    CREATE TABLE IF NOT EXISTS hardware_targets (
        id TEXT PRIMARY KEY NOT NULL,
        manufacturer TEXT NOT NULL,
        radio_system TEXT NOT NULL,
        device_type TEXT NOT NULL,
        target_name TEXT NOT NULL,
        mcu_family TEXT,
        notes TEXT,
        created_at TEXT NOT NULL,
        UNIQUE(manufacturer, radio_system, device_type, target_name)
    );

    CREATE TABLE IF NOT EXISTS firmware_versions (
        id TEXT PRIMARY KEY NOT NULL,
        manufacturer TEXT NOT NULL,
        radio_system TEXT NOT NULL,
        version TEXT NOT NULL,
        channel TEXT NOT NULL,
        source TEXT,
        release_notes TEXT,
        created_at TEXT NOT NULL,
        UNIQUE(manufacturer, radio_system, version, channel)
    );

    CREATE TABLE IF NOT EXISTS firmware_artifacts (
        id TEXT PRIMARY KEY NOT NULL,
        firmware_version_id TEXT NOT NULL,
        hardware_target_id TEXT,
        file_path TEXT NOT NULL,
        sha256 TEXT NOT NULL,
        file_size INTEGER NOT NULL,
        verified INTEGER NOT NULL DEFAULT 0,
        created_at TEXT NOT NULL,
        FOREIGN KEY(firmware_version_id) REFERENCES firmware_versions(id)
    );

    CREATE TABLE IF NOT EXISTS firmware_builds (
        id TEXT PRIMARY KEY NOT NULL,
        source_repo TEXT,
        branch TEXT,
        target TEXT,
        config_json TEXT,
        status TEXT NOT NULL,
        artifact_id TEXT,
        log_path TEXT,
        started_at TEXT NOT NULL,
        finished_at TEXT,
        error_message TEXT
    );

    CREATE TABLE IF NOT EXISTS device_profiles (
        id TEXT PRIMARY KEY NOT NULL,
        device_id TEXT,
        name TEXT NOT NULL,
        manufacturer TEXT,
        model TEXT,
        notes TEXT,
        created_at TEXT NOT NULL,
        updated_at TEXT NOT NULL
    );

    CREATE TABLE IF NOT EXISTS device_configurations (
        id TEXT PRIMARY KEY NOT NULL,
        device_id TEXT,
        profile_id TEXT,
        category TEXT NOT NULL,
        payload_json TEXT NOT NULL,
        firmware_version TEXT,
        created_at TEXT NOT NULL
    );

    CREATE TABLE IF NOT EXISTS bootloader_sessions (
        id TEXT PRIMARY KEY NOT NULL,
        device_id TEXT,
        mode TEXT NOT NULL,
        status TEXT NOT NULL,
        started_at TEXT NOT NULL,
        finished_at TEXT,
        notes TEXT
    );

    CREATE TABLE IF NOT EXISTS flash_sessions (
        id TEXT PRIMARY KEY NOT NULL,
        device_id TEXT,
        artifact_id TEXT,
        adapter TEXT NOT NULL,
        status TEXT NOT NULL,
        progress REAL,
        started_at TEXT NOT NULL,
        finished_at TEXT,
        error_message TEXT,
        verified INTEGER NOT NULL DEFAULT 0
    );

    CREATE TABLE IF NOT EXISTS recovery_sessions (
        id TEXT PRIMARY KEY NOT NULL,
        device_id TEXT,
        scenario TEXT NOT NULL,
        status TEXT NOT NULL,
        started_at TEXT NOT NULL,
        finished_at TEXT,
        notes TEXT
    );

    CREATE TABLE IF NOT EXISTS backups (
        id TEXT PRIMARY KEY NOT NULL,
        device_id TEXT,
        backup_type TEXT NOT NULL,
        file_path TEXT,
        sha256 TEXT,
        model TEXT,
        firmware_version TEXT,
        description TEXT,
        created_at TEXT NOT NULL
    );

    CREATE TABLE IF NOT EXISTS serial_logs (
        id TEXT PRIMARY KEY NOT NULL,
        device_id TEXT,
        com_port TEXT,
        mode TEXT NOT NULL,
        file_path TEXT,
        started_at TEXT NOT NULL,
        finished_at TEXT
    );

    CREATE TABLE IF NOT EXISTS diagnostic_reports (
        id TEXT PRIMARY KEY NOT NULL,
        report_json TEXT NOT NULL,
        created_at TEXT NOT NULL
    );

    CREATE TABLE IF NOT EXISTS application_settings (
        key TEXT PRIMARY KEY NOT NULL,
        value TEXT NOT NULL,
        updated_at TEXT NOT NULL
    );

    CREATE TABLE IF NOT EXISTS operation_history (
        id TEXT PRIMARY KEY NOT NULL,
        operation_type TEXT NOT NULL,
        device_id TEXT,
        status TEXT NOT NULL,
        started_at TEXT NOT NULL,
        finished_at TEXT,
        description TEXT,
        error_message TEXT
    );
    "#,
    r#"
    ALTER TABLE device_profiles ADD COLUMN preferred_com_port TEXT;
    ALTER TABLE device_profiles ADD COLUMN preferred_baud INTEGER;
    "#,
    r#"
    CREATE TABLE IF NOT EXISTS source_downloads (
        id TEXT PRIMARY KEY NOT NULL,
        project TEXT NOT NULL,
        version_or_ref TEXT NOT NULL,
        source_kind TEXT NOT NULL,
        remote_url TEXT NOT NULL,
        local_path TEXT NOT NULL,
        commit_sha TEXT,
        integrity_ok INTEGER NOT NULL DEFAULT 0,
        notes TEXT,
        downloaded_at TEXT NOT NULL,
        updated_at TEXT NOT NULL,
        UNIQUE(project, version_or_ref, local_path)
    );

    CREATE TABLE IF NOT EXISTS flash_step_logs (
        id TEXT PRIMARY KEY NOT NULL,
        session_id TEXT NOT NULL,
        step TEXT NOT NULL,
        status TEXT NOT NULL,
        message TEXT,
        created_at TEXT NOT NULL
    );

    CREATE TABLE IF NOT EXISTS hardware_targets_ext (
        id TEXT PRIMARY KEY NOT NULL,
        target_name TEXT NOT NULL UNIQUE,
        mcu_family TEXT NOT NULL,
        flash_size_bytes INTEGER,
        max_image_bytes INTEGER,
        bootloader_hint TEXT,
        device_type TEXT,
        frequency_band TEXT,
        notes TEXT,
        created_at TEXT NOT NULL
    );
    "#,
];
