use super::types::{DeviceStatus, DeviceType};

#[derive(Debug, Clone)]
pub struct ElrsHint {
    pub manufacturer: &'static str,
    pub model: &'static str,
    pub device_type: DeviceType,
    pub mcu: &'static str,
    pub frequency_band: &'static str,
    pub flash_method: &'static str,
    pub bootloader_likely: bool,
}

/// Documented USB VID/PID and product-string hints for ExpressLRS-related hardware.
/// Classification never uses COM port name alone.
pub fn lookup(
    vid: Option<u16>,
    pid: Option<u16>,
    product: Option<&str>,
    manufacturer: Option<&str>,
) -> Option<ElrsHint> {
    let product_l = product.unwrap_or("").to_lowercase();
    let mfr_l = manufacturer.unwrap_or("").to_lowercase();

    if let Some(hint) = by_vid_pid(vid, pid, &product_l) {
        return Some(hint);
    }

    if product_l.contains("expresslrs")
        || product_l.contains("elrs")
        || mfr_l.contains("expresslrs")
        || mfr_l.contains("elrs")
    {
        let device_type = if product_l.contains("tx") || product_l.contains("transmitter") {
            DeviceType::Tx
        } else if product_l.contains("rx") || product_l.contains("receiver") {
            DeviceType::Rx
        } else {
            DeviceType::Generic
        };
        let mcu = if product_l.contains("esp32") {
            "ESP32"
        } else if product_l.contains("8285") || product_l.contains("esp8266") {
            "ESP8285"
        } else {
            "ESP (unconfirmed)"
        };
        let band = if product_l.contains("900") || product_l.contains("868") {
            if product_l.contains("868") {
                "868 MHz"
            } else {
                "900 MHz"
            }
        } else {
            "2.4 GHz"
        };
        return Some(ElrsHint {
            manufacturer: "ExpressLRS",
            model: "ExpressLRS Device",
            device_type,
            mcu,
            frequency_band: band,
            flash_method: "ESP Bootloader / UART",
            bootloader_likely: product_l.contains("boot"),
        });
    }

    // Brand strings commonly paired with ELRS modules
    let brands = [
        "radiomaster",
        "happymodel",
        "betafpv",
        "iflight",
        "matek",
        "axisflying",
        "jumper",
        "namimno",
    ];
    if brands.iter().any(|b| mfr_l.contains(b) || product_l.contains(b))
        && (product_l.contains("rx")
            || product_l.contains("tx")
            || product_l.contains("elrs")
            || product_l.contains("gemini")
            || product_l.contains("erp")
            || product_l.contains("rp1")
            || product_l.contains("rp2")
            || product_l.contains("rp3")
            || product_l.contains("rp4"))
    {
        let device_type = if product_l.contains("tx") {
            DeviceType::Tx
        } else {
            DeviceType::Rx
        };
        return Some(ElrsHint {
            manufacturer: "ExpressLRS",
            model: "ELRS Module",
            device_type,
            mcu: if product_l.contains("esp32") {
                "ESP32"
            } else {
                "ESP8285"
            },
            frequency_band: "2.4 GHz",
            flash_method: "ESP Bootloader / UART",
            bootloader_likely: false,
        });
    }

    None
}

fn by_vid_pid(vid: Option<u16>, pid: Option<u16>, product_l: &str) -> Option<ElrsHint> {
    match (vid, pid) {
        // Espressif native USB
        (Some(0x303a), _) => {
            if product_l.contains("elrs") || product_l.contains("expresslrs") {
                Some(ElrsHint {
                    manufacturer: "ExpressLRS",
                    model: if product_l.is_empty() {
                        "ESP32 ELRS"
                    } else {
                        "ESP32 ELRS"
                    },
                    device_type: if product_l.contains("tx") {
                        DeviceType::Tx
                    } else if product_l.contains("rx") {
                        DeviceType::Rx
                    } else {
                        DeviceType::Generic
                    },
                    mcu: "ESP32",
                    frequency_band: "2.4 GHz",
                    flash_method: "ESP Bootloader / USB CDC",
                    bootloader_likely: product_l.contains("boot") || product_l.contains("serial"),
                })
            } else {
                None
            }
        }
        // Silicon Labs CP210x family — frequent on ELRS UART bridges
        (Some(0x10c4), Some(0xea60 | 0xea61 | 0xea63 | 0xea70 | 0xea71)) => {
            if product_l.contains("elrs")
                || product_l.contains("expresslrs")
                || product_l.contains("rp1")
                || product_l.contains("rp2")
                || product_l.contains("rp3")
                || product_l.contains("rp4")
                || product_l.contains("ep1")
                || product_l.contains("ep2")
                || product_l.contains("tx")
                || product_l.contains("rx")
            {
                Some(ElrsHint {
                    manufacturer: "ExpressLRS",
                    model: if !product_l.is_empty() {
                        "ELRS UART Device"
                    } else {
                        "ELRS CP210x"
                    },
                    device_type: if product_l.contains("tx") {
                        DeviceType::Tx
                    } else if product_l.contains("rx")
                        || product_l.contains("rp")
                        || product_l.contains("ep")
                    {
                        DeviceType::Rx
                    } else {
                        DeviceType::Generic
                    },
                    mcu: "ESP8285/ESP32",
                    frequency_band: "2.4 GHz",
                    flash_method: "UART / USB CDC",
                    bootloader_likely: false,
                })
            } else {
                None
            }
        }
        // WCH CH340/CH9102 often used on DIY ELRS
        (Some(0x1a86), Some(0x7523 | 0x55d4 | 0x55d3)) => {
            if product_l.contains("elrs") || product_l.contains("expresslrs") {
                Some(ElrsHint {
                    manufacturer: "ExpressLRS",
                    model: "ELRS CH340 Bridge",
                    device_type: DeviceType::Generic,
                    mcu: "ESP8285/ESP32",
                    frequency_band: "2.4 GHz",
                    flash_method: "UART",
                    bootloader_likely: product_l.contains("boot"),
                })
            } else {
                None
            }
        }
        _ => None,
    }
}

pub fn to_status(bootloader_likely: bool) -> DeviceStatus {
    if bootloader_likely {
        DeviceStatus::Bootloader
    } else {
        DeviceStatus::Online
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn detects_elrs_product_string() {
        let hint = lookup(Some(0x10c4), Some(0xea60), Some("RadioMaster RP1 ELRS"), None);
        assert!(hint.is_some());
        assert_eq!(hint.unwrap().device_type, DeviceType::Rx);
    }

    #[test]
    fn ignores_plain_cp210x_without_hints() {
        let hint = lookup(Some(0x10c4), Some(0xea60), Some("CP2102 USB to UART Bridge"), Some("Silicon Labs"));
        assert!(hint.is_none());
    }
}
