use windows::Devices::Bluetooth::{
    BluetoothConnectionStatus, BluetoothDevice, BluetoothMajorClass, BluetoothMinorClass,
};

use crate::{log_error, modules::bluetooth::BLUETOOTH_MANAGER, trace_lock};

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct BluetoothDeviceInfo {
    pub id: String,
    pub name: String,
    pub address: u64,
    pub major_class: BluetoothMajor,
    pub minor_main_class: BluetoothMinor,
    pub minor_sub_class: BluetoothMinor,
    pub status: BluetoothState,

    pub inner: BluetoothDevice,
}

impl From<BluetoothDevice> for BluetoothDeviceInfo {
    fn from(bluetooth_device: BluetoothDevice) -> Self {
        let class = bluetooth_device.ClassOfDevice().unwrap();
        let major: BluetoothMajor = class.MajorClass().unwrap().into();
        let minor = class.MinorClass().unwrap();

        Self {
            id: bluetooth_device
                .BluetoothDeviceId()
                .unwrap()
                .Id()
                .unwrap()
                .to_string(),
            name: bluetooth_device.Name().unwrap().to_string(),
            address: bluetooth_device.BluetoothAddress().unwrap(),
            major_class: major.clone(),
            minor_main_class: BluetoothMinor::parse_main(minor.clone(), major.clone()),
            minor_sub_class: BluetoothMinor::parse_sub(minor, major),
            status: bluetooth_device.ConnectionStatus().unwrap().into(),
            inner: bluetooth_device,
        }
    }
}
//Proxy event handlers for device attrivute changed
impl BluetoothDeviceInfo {
    pub(super) fn on_device_attribute_changed(
        sender: &Option<BluetoothDevice>,
        _args: &Option<windows_core::IInspectable>,
    ) -> windows_core::Result<()> {
        if let Some(device) = sender {
            let mut manager = trace_lock!(BLUETOOTH_MANAGER);
            log_error!(manager.update_device(device));
        }

        Ok(())
    }
}

#[derive(Debug, Clone)]
pub enum BluetoothState {
    Connected,
    Disconnected,
    Unkown,
}

impl From<BluetoothConnectionStatus> for BluetoothState {
    fn from(device_state: BluetoothConnectionStatus) -> Self {
        match device_state {
            BluetoothConnectionStatus::Connected => BluetoothState::Connected,
            BluetoothConnectionStatus::Disconnected => BluetoothState::Disconnected,
            _ => BluetoothState::Unkown,
        }
    }
}

#[derive(Debug, Clone)]
pub enum BluetoothMajor {
    Miscellaneous,
    Computer,
    Phone,
    NetworkAccessPoint,
    AudioVideo,
    Peripheral,
    Imaging,
    Wearable,
    Toy,
    Health,
    Unkown,
}

impl From<BluetoothMajorClass> for BluetoothMajor {
    fn from(device_major: BluetoothMajorClass) -> Self {
        match device_major {
            BluetoothMajorClass::Miscellaneous => BluetoothMajor::Miscellaneous,
            BluetoothMajorClass::Computer => BluetoothMajor::Computer,
            BluetoothMajorClass::Phone => BluetoothMajor::Phone,
            BluetoothMajorClass::NetworkAccessPoint => BluetoothMajor::NetworkAccessPoint,
            BluetoothMajorClass::AudioVideo => BluetoothMajor::AudioVideo,
            BluetoothMajorClass::Peripheral => BluetoothMajor::Peripheral,
            BluetoothMajorClass::Imaging => BluetoothMajor::Imaging,
            BluetoothMajorClass::Wearable => BluetoothMajor::Wearable,
            BluetoothMajorClass::Toy => BluetoothMajor::Toy,
            BluetoothMajorClass::Health => BluetoothMajor::Health,
            _ => BluetoothMajor::Unkown,
        }
    }
}

#[derive(Debug, Clone)]
pub enum BluetoothMinor {
    Uncategorized,
    ComputerDesktop,
    ComputerServer,
    ComputerLaptop,
    ComputerHandheld,
    ComputerPalmSize,
    ComputerWearable,
    ComputerTablet,
    PhoneCellular,
    PhoneCordless,
    PhoneSmartPhone,
    PhoneWired,
    PhoneIsdn,
    NetworkFullyAvailable,
    NetworkUsed01To17Percent,
    NetworkUsed17To33Percent,
    NetworkUsed33To50Percent,
    NetworkUsed50To67Percent,
    NetworkUsed67To83Percent,
    NetworkUsed83To99Percent,
    NetworkNoServiceAvailable,
    AudioVideoWearableHeadset,
    AudioVideoHandsFree,
    AudioVideoMicrophone,
    AudioVideoLoudspeaker,
    AudioVideoHeadphones,
    AudioVideoPortableAudio,
    AudioVideoCarAudio,
    AudioVideoSetTopBox,
    AudioVideoHifiAudioDevice,
    AudioVideoVcr,
    AudioVideoVideoCamera,
    AudioVideoCamcorder,
    AudioVideoVideoMonitor,
    AudioVideoVideoDisplayAndLoudspeaker,
    AudioVideoVideoConferencing,
    AudioVideoGamingOrToy,
    PeripheralJoystick,
    PeripheralGamepad,
    PeripheralRemoteControl,
    PeripheralSensing,
    PeripheralDigitizerTablet,
    PeripheralCardReader,
    PeripheralDigitalPen,
    PeripheralHandheldScanner,
    PeripheralHandheldGesture,
    WearableWristwatch,
    WearablePager,
    WearableJacket,
    WearableHelmet,
    WearableGlasses,
    ToyRobot,
    ToyVehicle,
    ToyDoll,
    ToyController,
    ToyGame,
    HealthBloodPressureMonitor,
    HealthThermometer,
    HealthWeighingScale,
    HealthGlucoseMeter,
    HealthPulseOximeter,
    HealthHeartRateMonitor,
    HealthHealthDataDisplay,
    HealthStepCounter,
    HealthBodyCompositionAnalyzer,
    HealthPeakFlowMonitor,
    HealthMedicationMonitor,
    HealthKneeProsthesis,
    HealthAnkleProsthesis,
    HealthGenericHealthManager,
    HealthPersonalMobilityDevice,

    //Added because they were not identified by the windows api developer, but exists
    PeripheralOther,
    PeripheralPointer,
    PeripheralKeyboard,
    PeripheralKeyboardAndPointer,
}

// https://www.ampedrftech.com/datasheets/cod_definition.pdf ->  Major Device Class
impl BluetoothMinor {
    pub fn parse_sub(minor: BluetoothMinorClass, major: BluetoothMajor) -> Self {
        match major {
            BluetoothMajor::Peripheral => {
                let post = BluetoothMinorClass(minor.0 & 0b1111); //lower 4 bits are for other functional items
                match post {
                    BluetoothMinorClass::PeripheralJoystick => BluetoothMinor::PeripheralJoystick,
                    BluetoothMinorClass::PeripheralGamepad => BluetoothMinor::PeripheralGamepad,
                    BluetoothMinorClass::PeripheralRemoteControl => {
                        BluetoothMinor::PeripheralRemoteControl
                    }
                    BluetoothMinorClass::PeripheralSensing => BluetoothMinor::PeripheralSensing,
                    BluetoothMinorClass::PeripheralDigitizerTablet => {
                        BluetoothMinor::PeripheralDigitizerTablet
                    }
                    BluetoothMinorClass::PeripheralCardReader => {
                        BluetoothMinor::PeripheralCardReader
                    }
                    BluetoothMinorClass::PeripheralDigitalPen => {
                        BluetoothMinor::PeripheralDigitalPen
                    }
                    BluetoothMinorClass::PeripheralHandheldScanner => {
                        BluetoothMinor::PeripheralHandheldScanner
                    }
                    BluetoothMinorClass::PeripheralHandheldGesture => {
                        BluetoothMinor::PeripheralHandheldGesture
                    }
                    _ => BluetoothMinor::Uncategorized,
                }
            }
            _ => BluetoothMinor::Uncategorized,
        }
    }
    pub fn parse_main(minor: BluetoothMinorClass, major: BluetoothMajor) -> Self {
        match major {
            BluetoothMajor::Miscellaneous => BluetoothMinor::Uncategorized,
            BluetoothMajor::Computer => match minor {
                BluetoothMinorClass::ComputerDesktop => BluetoothMinor::ComputerDesktop,
                BluetoothMinorClass::ComputerServer => BluetoothMinor::ComputerServer,
                BluetoothMinorClass::ComputerLaptop => BluetoothMinor::ComputerLaptop,
                BluetoothMinorClass::ComputerHandheld => BluetoothMinor::ComputerHandheld,
                BluetoothMinorClass::ComputerPalmSize => BluetoothMinor::ComputerPalmSize,
                BluetoothMinorClass::ComputerWearable => BluetoothMinor::ComputerWearable,
                BluetoothMinorClass::ComputerTablet => BluetoothMinor::ComputerTablet,
                _ => BluetoothMinor::Uncategorized,
            },
            BluetoothMajor::Phone => match minor {
                BluetoothMinorClass::PhoneCellular => BluetoothMinor::PhoneCellular,
                BluetoothMinorClass::PhoneCordless => BluetoothMinor::PhoneCordless,
                BluetoothMinorClass::PhoneSmartPhone => BluetoothMinor::PhoneSmartPhone,
                BluetoothMinorClass::PhoneWired => BluetoothMinor::PhoneWired,
                BluetoothMinorClass::PhoneIsdn => BluetoothMinor::PhoneIsdn,
                _ => BluetoothMinor::Uncategorized,
            },
            BluetoothMajor::NetworkAccessPoint => match minor {
                BluetoothMinorClass::NetworkFullyAvailable => BluetoothMinor::NetworkFullyAvailable,
                BluetoothMinorClass::NetworkUsed01To17Percent => {
                    BluetoothMinor::NetworkUsed01To17Percent
                }
                BluetoothMinorClass::NetworkUsed17To33Percent => {
                    BluetoothMinor::NetworkUsed17To33Percent
                }
                BluetoothMinorClass::NetworkUsed33To50Percent => {
                    BluetoothMinor::NetworkUsed33To50Percent
                }
                BluetoothMinorClass::NetworkUsed50To67Percent => {
                    BluetoothMinor::NetworkUsed50To67Percent
                }
                BluetoothMinorClass::NetworkUsed67To83Percent => {
                    BluetoothMinor::NetworkUsed67To83Percent
                }
                BluetoothMinorClass::NetworkUsed83To99Percent => {
                    BluetoothMinor::NetworkUsed83To99Percent
                }
                BluetoothMinorClass::NetworkNoServiceAvailable => {
                    BluetoothMinor::NetworkNoServiceAvailable
                }
                _ => BluetoothMinor::Uncategorized,
            },
            BluetoothMajor::AudioVideo => match minor {
                BluetoothMinorClass::AudioVideoWearableHeadset => {
                    BluetoothMinor::AudioVideoWearableHeadset
                }
                BluetoothMinorClass::AudioVideoHandsFree => BluetoothMinor::AudioVideoHandsFree,
                BluetoothMinorClass::AudioVideoMicrophone => BluetoothMinor::AudioVideoMicrophone,
                BluetoothMinorClass::AudioVideoLoudspeaker => BluetoothMinor::AudioVideoLoudspeaker,
                BluetoothMinorClass::AudioVideoHeadphones => BluetoothMinor::AudioVideoHeadphones,
                BluetoothMinorClass::AudioVideoPortableAudio => {
                    BluetoothMinor::AudioVideoPortableAudio
                }
                BluetoothMinorClass::AudioVideoCarAudio => BluetoothMinor::AudioVideoCarAudio,
                BluetoothMinorClass::AudioVideoSetTopBox => BluetoothMinor::AudioVideoSetTopBox,
                BluetoothMinorClass::AudioVideoHifiAudioDevice => {
                    BluetoothMinor::AudioVideoHifiAudioDevice
                }
                BluetoothMinorClass::AudioVideoVcr => BluetoothMinor::AudioVideoVcr,
                BluetoothMinorClass::AudioVideoVideoCamera => BluetoothMinor::AudioVideoVideoCamera,
                BluetoothMinorClass::AudioVideoCamcorder => BluetoothMinor::AudioVideoCamcorder,
                BluetoothMinorClass::AudioVideoVideoMonitor => {
                    BluetoothMinor::AudioVideoVideoMonitor
                }
                BluetoothMinorClass::AudioVideoVideoDisplayAndLoudspeaker => {
                    BluetoothMinor::AudioVideoVideoDisplayAndLoudspeaker
                }
                BluetoothMinorClass::AudioVideoVideoConferencing => {
                    BluetoothMinor::AudioVideoVideoConferencing
                }
                BluetoothMinorClass::AudioVideoGamingOrToy => BluetoothMinor::AudioVideoGamingOrToy,
                _ => BluetoothMinor::Uncategorized,
            },
            BluetoothMajor::Peripheral => {
                let pre = minor.0 >> 4 & 0b11; //upper 2 bits are for keyboard and mouse
                match pre {
                    0 => BluetoothMinor::PeripheralOther,
                    1 => BluetoothMinor::PeripheralPointer,
                    2 => BluetoothMinor::PeripheralKeyboard,
                    3 => BluetoothMinor::PeripheralKeyboardAndPointer,
                    _ => BluetoothMinor::Uncategorized,
                }
            }
            BluetoothMajor::Imaging => BluetoothMinor::Uncategorized,
            BluetoothMajor::Wearable => match minor {
                BluetoothMinorClass::WearableWristwatch => BluetoothMinor::WearableWristwatch,
                BluetoothMinorClass::WearablePager => BluetoothMinor::WearablePager,
                BluetoothMinorClass::WearableJacket => BluetoothMinor::WearableJacket,
                BluetoothMinorClass::WearableHelmet => BluetoothMinor::WearableHelmet,
                BluetoothMinorClass::WearableGlasses => BluetoothMinor::WearableGlasses,
                _ => BluetoothMinor::Uncategorized,
            },
            BluetoothMajor::Toy => match minor {
                BluetoothMinorClass::ToyRobot => BluetoothMinor::ToyRobot,
                BluetoothMinorClass::ToyVehicle => BluetoothMinor::ToyVehicle,
                BluetoothMinorClass::ToyDoll => BluetoothMinor::ToyDoll,
                BluetoothMinorClass::ToyController => BluetoothMinor::ToyController,
                BluetoothMinorClass::ToyGame => BluetoothMinor::ToyGame,
                _ => BluetoothMinor::Uncategorized,
            },
            BluetoothMajor::Health => match minor {
                BluetoothMinorClass::HealthBloodPressureMonitor => {
                    BluetoothMinor::HealthBloodPressureMonitor
                }
                BluetoothMinorClass::HealthThermometer => BluetoothMinor::HealthThermometer,
                BluetoothMinorClass::HealthWeighingScale => BluetoothMinor::HealthWeighingScale,
                BluetoothMinorClass::HealthGlucoseMeter => BluetoothMinor::HealthGlucoseMeter,
                BluetoothMinorClass::HealthPulseOximeter => BluetoothMinor::HealthPulseOximeter,
                BluetoothMinorClass::HealthHeartRateMonitor => {
                    BluetoothMinor::HealthHeartRateMonitor
                }
                BluetoothMinorClass::HealthHealthDataDisplay => {
                    BluetoothMinor::HealthHealthDataDisplay
                }
                BluetoothMinorClass::HealthStepCounter => BluetoothMinor::HealthStepCounter,
                BluetoothMinorClass::HealthBodyCompositionAnalyzer => {
                    BluetoothMinor::HealthBodyCompositionAnalyzer
                }
                BluetoothMinorClass::HealthPeakFlowMonitor => BluetoothMinor::HealthPeakFlowMonitor,
                BluetoothMinorClass::HealthMedicationMonitor => {
                    BluetoothMinor::HealthMedicationMonitor
                }
                BluetoothMinorClass::HealthKneeProsthesis => BluetoothMinor::HealthKneeProsthesis,
                BluetoothMinorClass::HealthAnkleProsthesis => BluetoothMinor::HealthAnkleProsthesis,
                BluetoothMinorClass::HealthGenericHealthManager => {
                    BluetoothMinor::HealthGenericHealthManager
                }
                BluetoothMinorClass::HealthPersonalMobilityDevice => {
                    BluetoothMinor::HealthPersonalMobilityDevice
                }
                _ => BluetoothMinor::Uncategorized,
            },
            BluetoothMajor::Unkown => BluetoothMinor::Uncategorized,
        }
    }
}

// // https://www.ampedrftech.com/datasheets/cod_definition.pdf ->  Major Device Class
// #[derive(Debug, Clone)]
// pub enum BluetoothDeviceClass {
//     Miscellaneous = 0b0,
//     Computer = 0b1,
//     Phone = 0b10,
//     NetworkAccessPoint = 0b11,
//     AudioVideo = 0b100,
//     Peripheral = 0b101,
//     Imaging = 0b110,
//     Wearable = 0b111,
//     Toy = 0b1000,
//     Health = 0b1001,
//     Other = 0b11111,
// }

// impl From<u32> for BluetoothDeviceClass {
//     fn from(class: u32) -> Self {
//         let current = class >> 8 & 0b11111; // Remove first 8 bits and only use the upper 5 of them.

//         for value in BluetoothDeviceClass::values() {
//             if current == value.clone() as u32 {
//                 return value.clone();
//             }
//         }

//         BluetoothDeviceClass::Other
//     }
// }

// static BLUETOOTH_DEVICE_CLASS: [BluetoothDeviceClass; 11] = [
//     BluetoothDeviceClass::Miscellaneous,
//     BluetoothDeviceClass::Computer,
//     BluetoothDeviceClass::Phone,
//     BluetoothDeviceClass::NetworkAccessPoint,
//     BluetoothDeviceClass::AudioVideo,
//     BluetoothDeviceClass::Peripheral,
//     BluetoothDeviceClass::Imaging,
//     BluetoothDeviceClass::Wearable,
//     BluetoothDeviceClass::Toy,
//     BluetoothDeviceClass::Health,
//     BluetoothDeviceClass::Other,
// ];

// impl BluetoothDeviceClass {
//     pub fn values() -> &'static [BluetoothDeviceClass] {
//         &BLUETOOTH_DEVICE_CLASS
//     }
// }
