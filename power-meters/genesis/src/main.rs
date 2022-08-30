use warp::Filter;

use serde::Deserialize;
use serde::Serialize;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RtlamrCapture {
    #[serde(rename = "Time")]
    pub time: String,
    #[serde(rename = "Offset")]
    pub offset: i64,
    #[serde(rename = "Length")]
    pub length: i64,
    #[serde(rename = "Type")]
    pub type_field: String,
    #[serde(rename = "Message")]
    pub message: Message,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Message {
    #[serde(rename = "FrameSync")]
    pub frame_sync: i64,
    #[serde(rename = "ProtocolID")]
    pub protocol_id: i64,
    #[serde(rename = "EndpointType")]
    pub endpoint_type: i64,
    #[serde(rename = "EndpointID")]
    pub endpoint_id: i64,
    #[serde(rename = "Consumption")]
    pub consumption: i64,
    #[serde(rename = "Tamper")]
    pub tamper: i64,
    #[serde(rename = "PacketCRC")]
    pub packet_crc: i64,
}

#[test]
fn test_rtlamr_capture() {
    let json = r#"{"Time":"2022-08-28T01:20:46.21663461+01:00","Offset":0,"Length":0,"Type":"SCM+","Message":{"FrameSync":5795,"ProtocolID":30,"EndpointType":156,"EndpointID":76324406,"Consumption":88604,"Tamper":3080,"PacketCRC":14115}}"#;

    let msg: RtlamrCapture = serde_json::from_str(&json).unwrap();

    assert_eq!(msg.message.endpoint_id, 76324406);
    assert_eq!(msg.message.consumption, 88604);
}

#[tokio::main]
async fn main() {
    // GET /hello/warp => 200 OK with body "Hello, warp!"
    let hello = warp::path!().map(|| format!("Hello, world!"));

    warp::serve(hello)
        .run(([0, 0, 0, 0, 0, 0, 0, 0], 8080))
        .await;
}
