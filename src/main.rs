use std::fs;
fn main() {
    let data = fs::read_to_string("bootstrap-sm4-cdn-min.js").unwrap();
    let mut customized_prefix_data = fs::read_to_string("customized-cdn_prefix.js").unwrap();
    let mut customized_data = fs::read_to_string("customized-cdn.js").unwrap();
    // remove last ;
    match customized_prefix_data.rfind(";") {
        Some(i) => {
            if i == customized_prefix_data.len() - 1 {
                customized_prefix_data.pop();
            }
        }
        None => {}
    };
    // remove last ;
    match customized_data.rfind(";") {
        Some(i) => {
            if i == customized_data.len() - 1 {
                customized_data.pop();
            }
        }
        None => {}
    }
    //content replacement
    let content = data
        .replace("'${transaction_id}'", "''")
        .replace("'${device_id}'", "'")
        .replace("'${request_token_ttl}'", "''")
        .replace("'${sw_url}'", "''")
        .replace(
            "'${control_param_api}'",
            "'https://api.baidu.com/v1/bootstrap/param?source='",
        )
        .replace("'${source_regulatr_e}'", "/bootstrap.js\\?.*source=/")
        .replace("'${collector_api_array}'", "[]")
        .replace("'${request_token}'", "''")
        .replace("'${source_id}'", "'sw_3d81e4f25a8883b9308380ca68466a56'")
        .replace("'${param_list}'", "[]")
        .replace("'${cookie_name}'", "[]")
        .replace("'${customized_prefix}'", &customized_prefix_data)
        .replace("'${customized}'", &customized_data);
    // output file
    fs::write("bootstrap.js", content).unwrap();
}
