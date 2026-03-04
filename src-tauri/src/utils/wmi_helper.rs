use wmi::{COMLibrary, WMIConnection};
use wmi::Variant;
use std::collections::HashMap;

pub struct WmiQueryResult {
    pub data: Vec<HashMap<String, String>>,
}

pub fn query_wmi(query: &str) -> Result<WmiQueryResult, String> {
    let com = COMLibrary::new().map_err(|e| format!("COM init: {}", e))?;
    let wmi = WMIConnection::new(com).map_err(|e| format!("WMI connect: {}", e))?;

    let results: Vec<HashMap<String, Variant>> = wmi
        .raw_query(query)
        .map_err(|e| format!("WMI query failed: {}", e))?;

    let data = results
        .into_iter()
        .map(|map| {
            map.into_iter()
                .map(|(k, v)| (k, variant_to_string(v)))
                .collect()
        })
        .collect();

    Ok(WmiQueryResult { data })
}

fn variant_to_string(v: Variant) -> String {
    match v {
        Variant::String(s) => s,
        Variant::I1(n) => n.to_string(),
        Variant::I2(n) => n.to_string(),
        Variant::I4(n) => n.to_string(),
        Variant::I8(n) => n.to_string(),
        Variant::UI1(n) => n.to_string(),
        Variant::UI2(n) => n.to_string(),
        Variant::UI4(n) => n.to_string(),
        Variant::UI8(n) => n.to_string(),
        Variant::R4(f) => f.to_string(),
        Variant::R8(f) => f.to_string(),
        Variant::Bool(b) => b.to_string(),
        Variant::Null => String::new(),
        Variant::Empty => String::new(),
        _ => String::new(),
    }
}
