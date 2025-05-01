use win32_ecoqos::utils::{get_process_name, list_all_process_id};

fn main() -> windows_result::Result<()> {
    let list_of_process = list_all_process_id::<15>()?;

    let mut count = 0;
    for (process_name, process_id) in list_of_process
        .into_iter()
        .map(|process_id| get_process_name(process_id, Some(256)).map(|s| (s, process_id)))
        .flatten()
        .map(|(s, id)| s.into_string().map(|s| (s, id)))
        .flatten()
        .inspect(|_| count += 1)
    {
        println!("{process_id:6}: {process_name}");
    }

    println!("found {count} processes");

    Ok(())
}
