use clap::{ArgGroup, arg};
use std::collections::HashMap;
use std::process::Stdio;
use std::{process::Command, string::FromUtf8Error};
use thiserror::Error;

#[derive(Error, Debug)]
enum BconnectError {
    #[error(transparent)]
    Io(#[from] std::io::Error),

    #[error(transparent)]
    FromUtf8Error(#[from] FromUtf8Error),

    #[error("Device {0} missing from device_map")]
    BadDeviceString(String),
}

fn main() -> Result<(), BconnectError> {
    let device_map = HashMap::from(
        [
            ("buds", "B0:4A:6A:C9:DF:0C"),
            ("jabra", "30:50:75:C7:3D:B7"),
            ("link", "F8:4E:17:75:18:A2"),
            ("pbuds", "FC:91:5D:70:51:41"),
            ("tw4", "80:C3:BA:55:83:B1"),
            ("xm4", "14:3F:A6:E7:9B:4F"),
            ("xm5", "AC:80:0A:27:99:3E"),
        ]
        .map(|(k, v)| (k.to_string(), v.to_string())),
    );

    let matches = clap::Command::new("Bconnect")
        .about("bluetooth connection shortcuts")
        .arg(
            arg!(<DEVICE>)
                .help("Bluetooth device name")
                .required(false)
                .value_parser(device_map.keys().collect::<Vec<_>>()),
        )
        .arg(arg!(-d --disconnect "Disconnect"))
        .group(
            ArgGroup::new("args")
                .args(["DEVICE", "disconnect"])
                .required(true)
                .multiple(false),
        )
        .get_matches();

    let devices_connected = get_devices_connected()?;

    println!("{:#?}", devices_connected);

    let disconnect = matches.get_flag("disconnect");
    if disconnect {
        for device in devices_connected.iter() {
            _ = disconnect_device(device)?;
        }
    }

    if let Some(device) = matches.get_one::<String>("DEVICE") {
        let device = device_map
            .get(device)
            .ok_or(BconnectError::BadDeviceString(device.to_string()))?;
        if devices_connected.contains(device) {
            _ = disconnect_device(device)?;
        }

        _ = connect_device(device)?;
    }

    Ok(())
}

fn get_devices_connected() -> Result<Vec<String>, BconnectError> {
    let device_output = Command::new("bluetoothctl")
        .args(["devices", "Connected"])
        .output()?;

    let device_lines = String::from_utf8(device_output.stdout)?;

    // let device_lines_debug = String::from(
    //     "Device AC:80:0A:27:99:3E WF-1000XM5\nDevice FC:91:5D:70:51:41 Alexander's Pixel Buds Pro 2\n",
    // );

    let devices_connected: Vec<String> = device_lines
        .trim()
        .split("\n")
        .filter(|x| x.len() >= 24)
        .map(|x| x[7..24].to_string())
        .collect();

    Ok(devices_connected)
}

fn disconnect_device(device: &str) -> Result<(), BconnectError> {
    let status = Command::new("bluetoothctl")
        .args(["disconnect", device])
        .stdout(Stdio::inherit())
        .status();

    println!("{}", status?);
    Ok(())
}

fn connect_device(device: &str) -> Result<(), BconnectError> {
    let status = Command::new("bluetoothctl")
        .args(["connect", device])
        .stdout(Stdio::inherit())
        .status();

    println!("{}", status?);
    Ok(())
}
