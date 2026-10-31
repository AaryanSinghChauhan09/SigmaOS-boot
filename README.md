# SigmaOS-boot

Boot protocol, image, and verified-boot primitives for SigmaOS.

## Verified boot boundary

`SimpleBootStage` does not implement cryptographic signature verification;
verification returns an explicit error. The empty boot chain does not verify,
and `SimpleSecureBoot` does not claim firmware enforcement. Real trust requires
a cryptographic verifier, authenticated measurement storage, and firmware or
bootloader integration. These scaffolding types must not be used to authorize
kernel or initramfs images.

The UEFI secure-boot, ACPI discovery, firmware-capsule, and microcode-update
helpers also lack their required crypto and hardware backends. They now return
unverified/unavailable status instead of accepting transformed bytes or
reporting simulated firmware changes as complete.

## Development

```sh
cargo check --lib
```
