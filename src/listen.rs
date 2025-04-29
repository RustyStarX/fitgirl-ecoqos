use std::{collections::HashMap, time::Duration};

use futures_lite::StreamExt;
use serde::Deserialize;
use wmi::{COMLibrary, FilterValue, WMIConnection};

use crate::Error;

#[derive(Deserialize, Debug)]
#[serde(rename = "__InstanceCreationEvent")]
#[serde(rename_all = "PascalCase")]
struct NewProcessEvent {
    target_instance: Process,
}

#[derive(Deserialize, Debug)]
#[serde(rename = "Win32_Process")]
#[serde(rename_all = "PascalCase")]
struct Process {
    process_id: u32,
    name: String,
}

pub async fn listen_process_creation() -> Result<(), Error> {
    let com_con = COMLibrary::new()?;
    let wmi_con = WMIConnection::new(com_con)?;

    let mut filters = HashMap::<String, FilterValue>::new();
    filters.insert("TargetInstance".to_owned(), FilterValue::is_a::<Process>()?);

    let mut stream = wmi_con
        .async_filtered_notification::<NewProcessEvent>(&filters, Some(Duration::from_secs(1)))?;

    while let Some(result) = stream.next().await {
        let process = result?.target_instance;
        println!("---------");
        println!("PID:        {}", process.process_id);
        println!("Name:       {}", process.name);
    }

    Ok(())
}
