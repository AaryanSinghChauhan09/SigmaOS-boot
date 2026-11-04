use sigmaos_boot::boot::firmware::{BootLoader, BootParams};
use sigmaos_boot::boot::multiboot2::Multiboot2BootLoader;
use sigmaos_boot::{BootEntry, BootManager};

#[test]
fn test_boot_manager_entries_aaa() {
    // Arrange
    let mut manager = BootManager::new(5);
    let entry = BootEntry {
        id: "sigmaos-main".to_string(),
        title: "SigmaOS Standard Linux (LTS)".to_string(),
        kernel_path: "/boot/vmlinuz-sigma.efi".to_string(),
        initrd_path: Some("/boot/initramfs-sigma.img".to_string()),
        cmdline_params: "root=UUID=sigma_root ro quiet splash".to_string(),
        is_default: true,
        is_recovery: false,
    };

    // Act
    manager.add_entry(entry);

    // Assert
    assert_eq!(manager.entries.len(), 2); // 1 recovery + 1 main
    assert_eq!(manager.default_entry_id, "sigmaos-main");
    assert!(manager.set_default_entry("sigmaos-recovery"));
    assert_eq!(manager.default_entry_id, "sigmaos-recovery");
}

#[test]
fn test_multiboot2_bootloader_actions_aaa() {
    // Arrange
    let bootloader = Multiboot2BootLoader::new();
    let mut params = BootParams::new();
    params.cmdline = "root=/dev/sda1 quiet splash".to_string();

    // Act
    assert!(bootloader.setup_memory(&mut params).is_ok());
    let enter_res = bootloader.enter_kernel(0x200000, &params);

    // Assert
    assert_eq!(params.memory_size, 32 * 1024 * 1024 * 1024);
    assert!(enter_res.is_ok());
}
