//! SigmaOS-boot: boot-phase model for system startup.
//!
//! Real implementation: boot phases (firmware/UEFI, bootloader, kernel,
//! init, target), boot-state tracking, error per-phase, and boot
//! metrics. Replaces the Plan placeholder.

use std::fmt;

/// Boot phases in order.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum BootPhase {
    Firmware,
    Bootloader,
    Kernel,
    Init,
    Target,
}

impl BootPhase {
    pub fn name(self) -> &'static str {
        match self {
            Self::Firmware => "firmware",
            Self::Bootloader => "bootloader",
            Self::Kernel => "kernel",
            Self::Init => "init",
            Self::Target => "target",
        }
    }
}

/// Boot state.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BootState {
    Pending(BootPhase),
    Running(BootPhase),
    Failed(BootPhase),
    Success,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BootError {
    PhaseFailed { phase: BootPhase, detail: String },
    UnknownPhase(String),
}

impl fmt::Display for BootError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::PhaseFailed { phase, detail } =>
                write!(f, "boot phase {} failed: {}", phase.name(), detail),
            Self::UnknownPhase(p) => write!(f, "unknown boot phase {:?}", p),
        }
    }
}

impl std::error::Error for BootError {}

/// Boot tracking with metrics.
#[derive(Debug)]
pub struct BootTracker {
    state: BootState,
    phases: Vec<(BootPhase, u64)>, // phase -> duration_ms
}

impl BootTracker {
    pub fn new() -> Self { Self { state: BootState::Pending(BootPhase::Firmware), phases: vec![] } }

    pub fn start(&mut self) { self.state = BootState::Pending(BootPhase::Firmware); }

    pub fn advance(&mut self, phase: BootPhase, duration_ms: u64) {
        self.phases.push((phase, duration_ms));
        self.state = BootState::Running(phase);
    }

    pub fn fail(&mut self, phase: BootPhase, detail: impl Into<String>) {
        self.state = BootState::Failed(phase);
    }

    pub fn success(&mut self) { self.state = BootState::Success; }

    pub fn state(&self) -> BootState { self.state }

    pub fn total_duration_ms(&self) -> u64 { self.phases.iter().map(|(_, d)| d).sum() }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_phase_names_aaa() {
        assert_eq!(BootPhase::Firmware.name(), "firmware");
        assert_eq!(BootPhase::Target.name(), "target");
    }

    #[test]
    fn test_boot_progression_aaa() {
        let mut bt = BootTracker::new();
        bt.start();
        bt.advance(BootPhase::Firmware, 500);
        bt.advance(BootPhase::Bootloader, 1200);
        assert_eq!(bt.state(), BootState::Running(BootPhase::Bootloader));
        assert_eq!(bt.total_duration_ms(), 1700);
    }

    #[test]
    fn test_boot_failure_aaa() {
        let mut bt = BootTracker::new();
        bt.start();
        bt.fail(BootPhase::Kernel, "panic");
        assert_eq!(bt.state(), BootState::Failed(BootPhase::Kernel));
    }
}
