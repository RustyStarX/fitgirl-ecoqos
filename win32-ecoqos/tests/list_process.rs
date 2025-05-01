use win32_ecoqos::utils::{get_process_name, list_all_process_id, list_process_id};

#[test]
fn test_list_process() -> windows_result::Result<()> {
    let process_ids = list_process_id(65536)?;
    assert!(process_ids.len() > 0);

    Ok(())
}

#[test]
fn test_list_all_process() -> windows_result::Result<()> {
    let process_ids = list_all_process_id::<10>()?;
    assert!(process_ids.len() > 0);

    Ok(())
}

#[test]
fn test_process_name() -> windows_result::Result<()> {
    let process_id = std::process::id();
    let process_name = get_process_name(process_id, Some(256))?;

    assert!(process_name
        .into_string()
        .unwrap()
        .starts_with(&env!("CARGO_PKG_NAME").replace("-", "_")));

    Ok(())
}
