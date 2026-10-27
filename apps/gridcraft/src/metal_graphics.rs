//! macOS graphics instance configuration, applied before wgpu creates its Metal device.

use eframe::{NativeOptions, egui_wgpu::WgpuSetup, wgpu::InstanceFlags};

/// Turns off wgpu's indirect-call validation for this start.
///
/// That validation compiles a Metal compute pipeline of its own. On macOS 15 the Metal compiler
/// fails on it ("Compiler encountered an internal error"), wgpu reports the device as lost, and
/// GridCraft quits before its first frame. The check guards indirect draws and dispatches, and
/// GridCraft issues none, so skipping it loses nothing the app relies on.
pub fn without_indirect_validation(options: &mut NativeOptions) {
    if let WgpuSetup::CreateNew(create) = &mut options.wgpu_options.wgpu_setup {
        create.instance_descriptor.flags.remove(InstanceFlags::VALIDATION_INDIRECT_CALL);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn skips_indirect_call_validation() {
        // Regression: its compute pipeline fails in the macOS 15 Metal compiler and the device is lost.
        let mut options = NativeOptions::default();
        without_indirect_validation(&mut options);
        let WgpuSetup::CreateNew(create) = options.wgpu_options.wgpu_setup else {
            panic!("default native options must create a wgpu instance");
        };
        assert!(!create.instance_descriptor.flags.contains(InstanceFlags::VALIDATION_INDIRECT_CALL));
    }
}
