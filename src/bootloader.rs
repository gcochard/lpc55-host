use hidapi::HidApi;
use enum_iterator::IntoEnumIterator;

use crate::protocol::Protocol;
use crate::status::BootloaderError;
use crate::types::*;
use crate::Result;

#[derive(Debug)]
pub struct Bootloader {
    protocol: Protocol,
    // move around; also "new" should scan the device_list iterator
    // to pull out all the info
    pub vid: u16,
    pub pid: u16,
}

impl Bootloader {
    pub fn try_new(vid: u16, pid: u16) -> Result<Self> {
        let api = HidApi::new()?;
        let device = api.open(vid, pid)?;
        let protocol = Protocol::new(device);
        Ok(Self { protocol, vid, pid } )
    }

    // pub fn command(&self, command: Command) {
    // }

    pub fn info(&self) {
        for property in Property::into_enum_iter() {
            println!("\n{:?}", property);
            self.property(property).ok();
        }
    }

    pub fn enroll_puf(&self) {
        // first time i ran this:
        // 03000C00 A0000002 00000000 15000000 00000000 00000000 00000000 00000000 00000000 00000030 FF5F0030 00000020 FF5F0020 00000000 00000000
        // second time i ran this:
        // 03000C00 A0000002 00000000 15000000 00000000 00000000 00000000 00000000 00000000 00000000 00000000 00000000 00000000 00000000 00000000
        self.protocol.call(&Command::KeyProvisioning(KeyProvisioningOperation::Enroll)).expect("success");
        info!("PUF enrolled");
    }

    /// The reason for this wrapper is that the device aborts early if more than 512 bytes are
    /// requested. Unclear why it does this...
    ///
    /// This is a traffic trace (requesting all of PFR in one go), removing the "surplus junk"
    ///
    /// --> 01002000 03000002 00DE0900 000E0000 00000000 00000000 00000000 00000000 00000000
    /// <-- 03000C00 A3010002 00000000 000E0000
    /// <-- 04003800 00000000 02000000 02000000 02000000 00000000 00000000 00000000 00000000 00000000 00000000 00000000 00000000 00000000 00000000
    /// <-- 04003800 00000000 00000000 00000000 00000000 00000000 00000000 00000000 00000000 00000000 00000000 00000000 00000000 00000000 00000000
    /// <-- 04003800 00000000 00000000 00000000 00000000 00000000 00000000 00000000 00000000 00000000 00000000 00000000 00000000 00000000 00000000
    /// <-- 04003800 00000000 00000000 00000000 00000000 00000000 00000000 00000000 00000000 00000000 00000000 00000000 00000000 00000000 00000000
    /// <-- 04003800 00000000 00000000 00000000 00000000 00000000 00000000 00000000 00000000 02000000 00000000 00000000 00000000 00000000 00000000
    /// <-- 04003800 00000000 00000000 00000000 00000000 00000000 00000000 00000000 00000000 00000000 00000000 00000000 00000000 00000000 00000000
    /// <-- 04003800 00000000 00000000 00000000 00000000 00000000 00000000 00000000 00000000 00000000 00000000 00000000 00000000 00000000 00000000
    /// <-- 04003800 00000000 00000000 00000000 00000000 00000000 00000000 00000000 00000000 00000000 00000000 00000000 00000000 00000000 00000000
    /// <-- 04003800 00000000 00000000 00000000 00000000 00000000 00000000 00000000 00000000 ECE6A668 2922E9CC F462A95F DF81E180 E1528642 7C520098
    /// <-- 04000000
    /// <-- 03000C00 A0000002 65000000 03000000
    ///
    /// The error is 101 = 100 + 1 = (supposedly) a flash driver "alignment" error (?!)
    ///
    /// The interesting thing is that at the point where the device aborts, there are 8 bytes
    /// remaining, which it otherwise produces in a final
    ///
    /// <-- 04000800 2C80BA51 B067AF3C
    /// <-- 03000C00 A0000002 00000000 03000000
    ///
    /// TODO: Need to expect errors such as: `Response status = 139 (0x8b) kStatus_FLASH_NmpaUpdateNotAllowed`
    /// This happens with `read-memory $((0x0009_FC70)) 16`, which would be the UUID
    ///
    /// TODO: Need to expect errors such as: `Response status = 10200 (0x27d8) kStatusMemoryRangeInvalid`
    /// This happens with `read-memory $((0x5000_0FFC)) 1`, which would be the DIEID (for chip rev)
    pub fn read_memory(&self, address: usize, length: usize) -> Vec<u8> {
        let mut data = Vec::new();
        let mut remaining = length;
        let mut address = address;
        while remaining > 0 {
            let length = core::cmp::min(remaining, 512);
            data.extend_from_slice(&self.read_memory_at_most_512(address, length));
            remaining -= length;
            address += length;
        }
        data
    }

    pub fn read_memory_at_most_512(&self, address: usize, length: usize) -> Vec<u8> {
        let response = self.protocol.call(&Command::ReadMemory { address, length }).expect("success");
        if let Response::ReadMemory(data) = response {
            data
        } else {
            todo!();
        }
    }

    fn property(&self, property: Property) -> std::result::Result<Vec<u32>, BootloaderError> {
        let response = self.protocol.call(&Command::GetProperty(property)).expect("success");
        if let Response::GetProperty(values) = response {
            Ok(values)
        } else {
            todo!();
        }
    }

    pub fn properties(&self) -> GetProperties<'_> {
        GetProperties { bl: self }
    }

    pub fn all_properties(&self) -> Properties {
        let proxy = self.properties();
        Properties {
            current_version: proxy.current_version().unwrap(),
            target_version: proxy.target_version().unwrap(),
            available_commands: proxy.available_commands().unwrap(),
            available_peripherals: proxy.available_peripherals().unwrap(),
            pfr_keystore_update_option: proxy.pfr_keystore_update_option().unwrap(),
            ram_start_address: proxy.ram_start_address().unwrap(),
            ram_size: proxy.ram_size().unwrap(),
            flash_start_address: proxy.flash_start_address().unwrap(),
            flash_size: proxy.flash_size().unwrap(),
            flash_page_size: proxy.flash_page_size().unwrap(),
            flash_sector_size: proxy.flash_sector_size().unwrap(),
            verify_writes: proxy.verify_writes().unwrap(),
            flash_locked: proxy.flash_locked().unwrap(),
            max_packet_size: proxy.max_packet_size().unwrap(),
            device_uuid: proxy.device_uuid().unwrap(),
            system_uuid: proxy.system_uuid().unwrap(),
            crc_check_status: proxy.crc_check_status().unwrap(),
            reserved_regions: proxy.reserved_regions().unwrap(),
            irq_notification_pin: proxy.irq_notification_pin().unwrap(),
        }
    }
}

pub struct GetProperties<'a> {
    bl: &'a Bootloader
}

impl GetProperties<'_> {
    pub fn current_version(&self) -> std::result::Result<Version, BootloaderError> {
        Ok(Version::from(self.bl.property(Property::CurrentVersion)?[0]))
    }
    pub fn target_version(&self) -> std::result::Result<Version, BootloaderError> {
        Ok(Version::from(self.bl.property(Property::TargetVersion)?[0]))
    }
    pub fn ram_start_address(&self) -> std::result::Result<usize, BootloaderError> {
        Ok(self.bl.property(Property::RamStartAddress)?[0] as _)
    }
    pub fn ram_size(&self) -> std::result::Result<usize, BootloaderError> {
        Ok(self.bl.property(Property::RamSize)?[0] as _)
    }
    pub fn flash_start_address(&self) -> std::result::Result<usize, BootloaderError> {
        Ok(self.bl.property(Property::FlashStartAddress)?[0] as _)
    }
    pub fn flash_size(&self) -> std::result::Result<usize, BootloaderError> {
        Ok(self.bl.property(Property::FlashSize)?[0] as _)
    }
    pub fn flash_page_size(&self) -> std::result::Result<usize, BootloaderError> {
        Ok(self.bl.property(Property::FlashPageSize)?[0] as _)
    }
    pub fn flash_sector_size(&self) -> std::result::Result<usize, BootloaderError> {
        Ok(self.bl.property(Property::FlashSectorSize)?[0] as _)
    }
    pub fn max_packet_size(&self) -> std::result::Result<usize, BootloaderError> {
        Ok(self.bl.property(Property::MaxPacketSize)?[0] as _)
    }
    pub fn available_peripherals(&self) -> std::result::Result<crate::types::AvailablePeripherals, BootloaderError> {
        Ok(AvailablePeripherals::from_bits_truncate(self.bl.property(Property::AvailablePeripherals)?[0]))
    }
    pub fn available_commands(&self) -> std::result::Result<crate::types::AvailableCommands, BootloaderError> {
        Ok(AvailableCommands::from_bits_truncate(self.bl.property(Property::AvailableCommands)?[0]))
    }
    pub fn pfr_keystore_update_option(&self) -> std::result::Result<crate::types::PfrKeystoreUpdateOptions, BootloaderError> {
        let values = self.bl.property(Property::PfrKeystoreUpdateOptions)?;
        assert_eq!(values.len(), 1);
        Ok(PfrKeystoreUpdateOptions::from(values[0]))
    }
    pub fn verify_writes(&self) -> std::result::Result<bool, BootloaderError> {
        Ok(self.bl.property(Property::VerifyWrites)?[0] == 1)
    }
    pub fn flash_locked(&self) -> std::result::Result<bool, BootloaderError> {
        Ok(match self.bl.property(Property::FlashSecurityState)?[0] {
            0x0 | 0x5AA55AA5 => false,
            0x1 | 0xC33CC33C => true,
            _ => panic!(),
        })
    }
    pub fn device_uuid(&self) -> std::result::Result<u128, BootloaderError> {
        let values = self.bl.property(Property::UniqueDeviceIdent)?;
        assert_eq!(values.len(), 4);
        Ok(
            ((values[3] as u128) << 96) +
            ((values[2] as u128) << 64) +
            ((values[1] as u128) << 32) +
            ((values[0] as u128))
        )
    }
    pub fn system_uuid(&self) -> std::result::Result<u64, BootloaderError> {
        let values = self.bl.property(Property::SystemDeviceIdent)?;
        assert_eq!(values.len(), 2);
        Ok(((values[1] as u64) << 32) + (values[0] as u64))
    }

    pub fn crc_check_status(&self) -> std::result::Result<BootloaderError, BootloaderError> {
        Ok(BootloaderError::from(self.bl.property(Property::CrcCheckStatus)?[0]))
    }

    pub fn reserved_regions(&self) -> std::result::Result<Vec<(usize, usize)>, BootloaderError> {
        let values = self.bl.property(Property::ReservedRegions)?;
        assert_eq!(values.len() % 2, 0);
        let mut pairs = Vec::new();
        for pair in values.chunks_exact(2) {
            let left = pair[0];
            let right = pair[1];
            assert!(right >= left);
            if right > left {
                pairs.push((left as usize, right as usize));
            }
        }
        Ok(pairs)
    }

    pub fn irq_notification_pin(&self) -> std::result::Result<crate::types::IrqNotificationPin, BootloaderError> {
        Ok(IrqNotificationPin::from(self.bl.property(Property::IrqNotificationPin)?[0]))
    }
}

#[cfg(test)]
#[test]
fn test_all_properties() {
    let (vid, pid) = (0x1fc9, 0x0021);
    let bootloader = Bootloader::try_new(vid, pid).unwrap();
    insta::assert_debug_snapshot!(bootloader.all_properties());
}
