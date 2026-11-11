use super::{AdapterError, AdapterResult, HardwareAdapter};
use crate::device_detection::DetectedDevice;
use serde_json::{json, Value};

pub struct ExpressLrsAdapter;

impl ExpressLrsAdapter {
    pub fn supports_device(device: &DetectedDevice) -> bool {
        let radio = device
            .radio_system
            .as_deref()
            .unwrap_or("")
            .to_lowercase();
        let mfr = device
            .manufacturer
            .as_deref()
            .unwrap_or("")
            .to_lowercase();
        radio.contains("expresslrs") || radio == "elrs" || mfr.contains("expresslrs")
    }
}

impl HardwareAdapter for ExpressLrsAdapter {
    fn id(&self) -> &'static str {
        "expresslrs"
    }

    fn supports(&self, device: &DetectedDevice) -> bool {
        Self::supports_device(device)
    }

    fn read_configuration(&self, device: &DetectedDevice) -> AdapterResult<Value> {
        if !self.supports(device) {
            return Err(AdapterError::NotSupported(
                "Device is not classified as ExpressLRS".into(),
            ));
        }

        Ok(json!({
            "adapter": "expresslrs",
            "deviceId": device.id,
            "readOnlyPlaceholder": true,
            "note": "Live configuration read requires UART/CRSF protocol support (future phase). Categories below are structural placeholders only.",
            "categories": [
                {
                    "id": "binding",
                    "label": "Binding / UID",
                    "fields": [
                        { "key": "bind_phrase", "label": "Bind Phrase", "value": null, "editable": false },
                        { "key": "uid", "label": "UID", "value": null, "editable": false }
                    ]
                },
                {
                    "id": "radio",
                    "label": "Radio / RF",
                    "fields": [
                        { "key": "frequency_band", "label": "Band", "value": device.frequency_band, "editable": false },
                        { "key": "packet_rate", "label": "Packet Rate", "value": null, "editable": false },
                        { "key": "tx_power", "label": "TX Power", "value": null, "editable": false }
                    ]
                },
                {
                    "id": "hardware",
                    "label": "Hardware",
                    "fields": [
                        { "key": "model", "label": "Model", "value": device.model, "editable": false },
                        { "key": "mcu", "label": "MCU", "value": device.mcu, "editable": false },
                        { "key": "firmware_version", "label": "Firmware", "value": device.firmware_version, "editable": false }
                    ]
                },
                {
                    "id": "serial",
                    "label": "Serial / UART",
                    "fields": [
                        { "key": "com_port", "label": "COM Port", "value": device.com_port, "editable": false },
                        { "key": "baud_rate", "label": "Baud", "value": device.baud_rate, "editable": false }
                    ]
                }
            ]
        }))
    }

    fn flash_firmware(
        &self,
        _device: &DetectedDevice,
        _artifact_path: &std::path::Path,
    ) -> AdapterResult<()> {
        Err(AdapterError::NotSupported(
            "ExpressLRS adapter flash is not enabled until flashing engine integration is complete".into(),
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::device_detection::types::{DeviceStatus, DeviceType};

    #[test]
    fn placeholder_config_only_for_elrs() {
        let elrs = DetectedDevice {
            id: "d1".into(),
            name: "ELRS RX".into(),
            manufacturer: Some("ExpressLRS".into()),
            device_type: DeviceType::Rx,
            model: None,
            mcu: None,
            com_port: None,
            baud_rate: None,
            vid: None,
            pid: None,
            usb_serial: None,
            radio_system: Some("ExpressLRS".into()),
            frequency_band: Some("2.4 GHz".into()),
            firmware_version: None,
            bootloader_version: None,
            connection_mode: None,
            flash_method: None,
            bootloader_mode: false,
            status: DeviceStatus::Online,
            is_mock: false,
            notes: None,
            profile_id: None,
        };
        let cfg = ExpressLrsAdapter.read_configuration(&elrs).unwrap();
        assert_eq!(cfg["adapter"], "expresslrs");
        assert!(cfg["categories"].as_array().unwrap().len() >= 3);

        let mut generic = elrs.clone();
        generic.radio_system = None;
        generic.manufacturer = Some("Unknown".into());
        assert!(ExpressLrsAdapter.read_configuration(&generic).is_err());
    }
}
