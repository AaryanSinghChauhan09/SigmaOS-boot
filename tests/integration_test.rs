use sigmaos_boot::*;

#[test]
fn component_tracks_boot_phases() {
    let mut bt = BootTracker::new();
    bt.start();
    bt.advance(BootPhase::Firmware, 300);
    assert_eq!(bt.total_duration_ms(), 300);
}
