#[macro_use]
extern crate serde_derive;

use rusoto_core::{Region, RusotoError};
use rusoto_ec2::{DescribeInstancesRequest, Ec2, Ec2Client, Instance, Reservation};

use tokio::prelude::*;
use serde_json;
use chrono::prelude::*;
use chrono::Duration;
use std::fs::{File, rename};
use std::io::prelude::*;
use std::io::Read;
use std::path::Path;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CacheData { // How we'll cache our data
    written_time: DateTime<Utc>,
    instance_data: Vec<Instance>,
}

pub fn write_saved_json(account: &String, cache_dir: &String, region_name: &String, data: &Vec<Instance>) -> io::Result<String> {
    // Interesting: in rust you can concat a &str onto a String.
    // Deref coercecions may be an interesting topic?
    // let pathname = format!("{}/{}_{}_ec2_instances.json", cache_dir, account, region_name);
    let pathname = format!("/tmp/foo.json")
    
    let tmp_pathname = pathname.to_owned() + ".tmp";

    println!("starting, writing to {}", tmp_pathname);

    let mut cache_file_new = File::create(Path::new(&tmp_pathname))?;
    let cache_data = CacheData {
        written_time: Utc::now(),
        instance_data: data.to_owned(),
    };
    let json_bytes = match serde_json::to_string(&cache_data) {
        Err(_) => "{}".to_string(),
        Ok(output) => output
    };
    cache_file_new.write_all(json_bytes.as_bytes())?;
    // cache_file_new.write_all(serde_json::to_string(&data).unwrap().as_bytes());

    println!("Wrote the tmp cache file");

    rename(tmp_pathname, pathname)?;

    println!("Re-named the local cache file after getting new data");

    Ok("Cache written out".to_string())
}


#[tokio::main]
async fn main() {
    let region = Region::UsEast2;
    let client = Ec2Client::new(region.clone());
    let mut ec2_request_input = DescribeInstancesRequest::default();
    let mut res_count = 0;
    let mut inst_count = 0;
    let mut instances = Vec::new();
    match client.describe_instances(ec2_request_input).await {
        Ok(response) => {
            for res in response.reservations.unwrap().into_iter() {
                for inst in res.instances.unwrap().into_iter() {
                    instances.push(inst);
                }
            }
        }
        Err(error) => {
            println!("Error when invoking describe_instances: {:?}", error);
        }
    }

    println!("reservations: {:?} instances: {:?}", res_count, inst_count);
    println!("Hello, world! {:?}", region);
}

// fn get_instances() 
