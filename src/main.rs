#![no_main]
#![no_std]

#[cfg(use_config)]
mod config;

use log::info;
use uefi::{CStr16, Identify, Result};
use uefi::prelude::*;
use uefi::fs::{Error, Path};
use uefi::proto::device_path::text::{AllowShortcuts, DevicePathToText, DisplayOnly};
use uefi::proto::loaded_image;
use uefi::proto::loaded_image::LoadedImage;
use uefi::table::boot::LoadImageSource::FromBuffer;
use uefi::table::boot::SearchType;

#[entry]
fn efi_main(image_handle: Handle, mut system_table: SystemTable<Boot>) -> Status {
    uefi_services::init(&mut system_table).unwrap();
    let boot_services = system_table.boot_services();
    let mut file_system = boot_services.get_image_file_system(image_handle).unwrap();
    let path = Path::new(cstr16!("\\EFI\\Arch\\arch-linux.efi"));
    // let target_image = boot_services.load_image(image_handle, )
    // let out = file_system.read_to_string().unwrap();
    info!("{}", path.to_cstr16());
    match file_system.try_exists(&path) {
        Ok(()) => {
            let target = boot_services.load_image(image_handle, FromBuffer {
                buffer: file_system.read(&path).unwrap().as_slice(),
                file_path: None
            }).unwrap();
            boot_services.start_image(target).unwrap();
        }
        Err(_err) => {
            info!("Not found :(")
        }
    }
    print_image_path(boot_services).unwrap();
    info!("Hello worlds!");
    boot_services.stall(10_000_000);

    // try to load
    Status::SUCCESS
}


fn print_image_path(boot_services: &BootServices) -> Result {
    let loaded_image = boot_services
        .open_protocol_exclusive::<LoadedImage>(boot_services.image_handle())?;

    let device_path_to_text_handle = *boot_services
        .locate_handle_buffer(SearchType::ByProtocol(&DevicePathToText::GUID))?
        .first()
        .expect("DevicePathToText is missing");

    let device_path_to_text = boot_services
        .open_protocol_exclusive::<DevicePathToText>(
            device_path_to_text_handle,
        )?;

    let image_device_path =
        loaded_image.file_path().expect("File path is not set");
    let image_device_path_text = device_path_to_text
        .convert_device_path_to_text(
            boot_services,
            image_device_path,
            DisplayOnly(true),
            AllowShortcuts(false),
        )
        .expect("convert_device_path_to_text failed");

    info!("Image path: {}", &*image_device_path_text);
    Ok(())
}