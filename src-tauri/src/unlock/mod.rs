use crate::device_detection::DetectedDevice;
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UnlockCapability {
    pub id: String,
    pub title: String,
    pub documented: bool,
    pub available: bool,
    pub description: String,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UnlockReport {
    pub device_id: String,
    pub capabilities: Vec<UnlockCapability>,
    pub policy: String,
}

pub fn inspect_unlock(device: &DetectedDevice) -> UnlockReport {
    let mut capabilities = Vec::new();

    if device.radio_system.as_deref() == Some("ExpressLRS") {
        capabilities.push(UnlockCapability {
            id: "elrs_bind_phrase".into(),
            title: "Bind Phrase (documented)".into(),
            documented: true,
            available: true,
            description: "Official ELRS bind phrase configuration via Configuration/Binding Manager — not a hardware unlock.".into(),
        });
        capabilities.push(UnlockCapability {
            id: "elrs_wifi_update".into(),
            title: "WiFi update mode".into(),
            documented: true,
            available: false,
            description: "Documented on many ELRS targets; not automated in this build yet.".into(),
        });
    }

    capabilities.push(UnlockCapability {
        id: "secure_boot_bypass".into(),
        title: "Secure Boot bypass".into(),
        documented: false,
        available: false,
        description: "Not provided. Aether Flash Studio will never implement signature/Secure Boot bypass.".into(),
    });
    capabilities.push(UnlockCapability {
        id: "regional_rf_bypass".into(),
        title: "Regional RF limit bypass".into(),
        documented: false,
        available: false,
        description: "Not provided. Regional and hardware RF limits are not circumvented.".into(),
    });

    UnlockReport {
        device_id: device.id.clone(),
        capabilities,
        policy: "Only manufacturer-documented activation paths are shown. Software toggles are never claimed as hardware unlocks.".into(),
    }
}

pub fn apply_unlock(device: &DetectedDevice, capability_id: &str) -> Result<String, String> {
    let report = inspect_unlock(device);
    let cap = report
        .capabilities
        .iter()
        .find(|c| c.id == capability_id)
        .ok_or_else(|| "Unknown capability".to_string())?;
    if !cap.documented || !cap.available {
        return Err(format!(
            "Capability '{}' is not available via a documented unlock path",
            cap.title
        ));
    }
    if capability_id == "elrs_bind_phrase" {
        return Ok(
            "Use Binding Manager to set a documented Bind Phrase. No hardware unlock performed."
                .into(),
        );
    }
    Err("No automated unlock action for this capability".into())
}
