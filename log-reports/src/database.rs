use deadpool_postgres::Client;
use crate::models::LogReport;
use std::io;

pub async fn get_log_reports(client: &Client) -> Result<Vec<LogReport>, io::Error> {
    let statement = client.prepare("select * from logs").await.unwrap();
    let logs = client
        .query(&statement, &[])
        .await
        .expect("Error getting log reports")
        .iter()
        .map(|row| LogReport::from(row))
        .collect::<Vec<LogReport>>();
    Ok(logs)
}

pub async fn login(client: &Client, username: String, password: String) -> bool {
    let statement = client
        .prepare("select * \
        from users \
        where username = $1 and password = $2"
        )
        .await
        .unwrap();
    let val = client.query(&statement, &[&username, &password])
        .await
        .expect("Error getting user")
        .iter()
        .map(|row| 1)
        .collect::<Vec<i32>>()
        .len();

    match val {
        1 => true,
        _ => false
    }
}

pub async fn get_logs_user(client: &Client, user: String) -> Result<Vec<LogReport>, io::Error> {
    let statement = client.prepare("select * from logs where username = $1").await.unwrap();
    let logs = client
        .query(&statement, &[&user])
        .await
        .expect("Error getting log reports")
        .iter()
        .map(|row| LogReport::from(row))
        .collect::<Vec<LogReport>>();
    Ok(logs)
}

pub async fn get_logs_type(client: &Client, log_type: String) -> Result<Vec<LogReport>, io::Error> {
    let statement = client.prepare("select * from logs where type = $1").await.unwrap();
    let logs = client
        .query(&statement, &[&log_type])
        .await
        .expect("Error getting log reports")
        .iter()
        .map(|row| LogReport::from(row))
        .collect::<Vec<LogReport>>();
    Ok(logs)
}

pub async fn get_logs_severity(client: &Client, severity: String) -> Result<Vec<LogReport>, io::Error> {
    let statement = client.prepare("select * from logs where severity = $1").await.unwrap();
    let logs = client
        .query(&statement, &[&severity])
        .await
        .expect("Error getting log reports")
        .iter()
        .map(|row| LogReport::from(row))
        .collect::<Vec<LogReport>>();
    Ok(logs)
}

pub async fn remove_log(client: &Client, id: i32, user: String) -> Result<LogReport, io::Error> {
    let statement = client.prepare("delete from logs where id = $1 and username = $2 returning id, type, severity, date, username, actuallog").await.unwrap();
    let log = client.query(&statement, &[&id, &user])
        .await
        .expect("Error removing log")
        .iter()
        .map(|row| LogReport::from(row))
        .collect::<Vec<LogReport>>()
        .pop();
    match log {
        None => Ok(LogReport {id: 0, log_type: "".to_string(), severity: "".to_string(), date: "".to_string(), username: "".to_string(), actual_log: "".to_string() }),
        Some(l) => Ok(l)
    }

}

pub async fn add_log(client: &Client, log_type: String, severity: String, date: String, username: String, actual_log: String) -> Result<LogReport, io::Error> {
    let statement = client.prepare("insert into logs (type, severity, date, username, actuallog) values ($1, $2, $3, $4, $5)\
    returning id, type, severity, date, username, actuallog").await.unwrap();
    client.query(&statement, &[&log_type, &severity, &date, &username, &actual_log])
        .await
        .expect("Error inserting log")
        .iter()
        .map(|row| LogReport::from(row))
        .collect::<Vec<LogReport>>()
        .pop()
        .ok_or(io::Error::new(io::ErrorKind::Other, "Error creating log"))

}