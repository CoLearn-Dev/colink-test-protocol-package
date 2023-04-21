use colink::*;

include!("test-servers.in");

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    let mut cls = vec![];
    for server in SERVERS {
        let cl = CoLink::new(server.0, server.1)
            .switch_to_generated_user()
            .await?;
        cls.push(cl);
    }
    println!("create users: done");
    let mut pos = vec![];
    for i in 0..cls.len() {
        let id = cls[i]
            .start_protocol_operator_full_config(
                "vt_test",
                &cls[i].get_user_id()?,
                false, // change this to `true` if you want to reinstall/upgrade the protocol on the server
                StartProtocolOperatorSourceType::Git,
                "",
                "https://github.com/CoLearn-Dev/colink-protocol-variable-transfer-test.git",
                "",
            )
            .await?;
        pos.push(id);
    }
    println!("start protocol: done");
    let participants = vec![
        Participant {
            user_id: cls[0].get_user_id()?,
            role: "initiator".to_string(),
        },
        Participant {
            user_id: cls[1].get_user_id()?,
            role: "receiver".to_string(),
        },
    ];
    let data = "test".as_bytes();
    let task_id = cls[0]
        .run_task("vt_test", data, &participants, true)
        .await?;
    println!("run task: done");
    let res = cls[1]
        .read_or_wait(&format!("tasks:{}:output", task_id))
        .await?;
    println!("{}", String::from_utf8_lossy(&res));
    assert!(res == data);
    let res = cls[1]
        .read_or_wait(&format!("tasks:{}:output_remote_storage", task_id))
        .await?;
    println!("{}", String::from_utf8_lossy(&res));
    assert!(res == data);
    println!("finish");
    for i in 0..cls.len() {
        cls[i].stop_protocol_operator(&pos[i]).await?;
    }
    Ok(())
}
