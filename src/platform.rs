use std::fs;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Platform {
    Airoha,
    Broadcom,
}

impl Platform {
    pub fn name(self) -> &'static str {
        match self {
            Platform::Airoha => "Airoha AN758x",
            Platform::Broadcom => "Broadcom",
        }
    }

    pub fn expected_kernel(self) -> &'static str {
        match self {
            Platform::Airoha => "5.4.55",
            Platform::Broadcom => "4.19.235",
        }
    }

    pub fn supports_flash(self) -> bool {
        matches!(self, Platform::Airoha)
    }
}

/// Broadcom BCA boards carry `brcm,...` compatibles in their device tree.
pub fn detect() -> Platform {
    let compatible = fs::read("/proc/device-tree/compatible").unwrap_or_default();
    let broadcom = compatible
        .split(|byte| *byte == 0)
        .any(|entry| entry.starts_with(b"brcm,"));
    if broadcom {
        Platform::Broadcom
    } else {
        Platform::Airoha
    }
}
