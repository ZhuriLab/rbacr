use anyhow::Result;
use k8s_openapi::{
    apimachinery::pkg::apis::meta::v1::{ObjectMeta, Time},
    chrono::{Duration, Utc},
};
use serde::{Deserialize, Serialize};

// #[derive(Serialize, Deserialize, Debug)]
// pub enum OutputFormat {
//     RoleBinding,
//     ClusterRoleBinding,
// }

#[derive(Serialize, Deserialize, Debug)]
pub enum SubKind {
    User,
    Group,
    ServiceAccount,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OutR {
    pub rb: String,
    pub ns: String,
    pub r: String,
    pub age: ObjectMeta,
    pub subject: String,
    pub sub_kind: SubKind,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OutCR {
    pub crb: String,
    pub r: String,
    pub age: ObjectMeta,
    pub subject: String,
    pub sub_kind: SubKind,
}

impl OutR {
    pub fn new(
        rb: String,
        ns: String,
        r: String,
        age: ObjectMeta,
        subject: String,
        sub_kind: SubKind,
    ) -> Self {
        Self {
            rb,
            ns,
            r,
            age,
            subject,
            sub_kind,
        }
    }
}

impl OutCR {
    pub fn new(
        crb: String,
        r: String,
        age: ObjectMeta,
        subject: String,
        sub_kind: SubKind,
    ) -> Self {
        Self {
            crb,
            r,
            age,
            subject,
            sub_kind,
        }
    }
}

pub fn output_r(result: Vec<OutR>) -> Result<()> {
    let rb_max_name = result.iter().map(|x| x.rb.len() + 2).max().unwrap_or(63);
    let r_max_name = result.iter().map(|x| x.r.len() + 2).max().unwrap_or(63);
    let subj_max_name = result
        .iter()
        .map(|x| x.subject.len() + 2)
        .max()
        .unwrap_or(63);
    println!(
        "{0:<rb_max_name$} {1:<20} {2:<r_max_name$} {3:<20} {4:<subj_max_name$} {5:<20}",
        "ROLEBINDING", "NAMESPACE", "ROLE", "AGE", "SUBJECT", "SUBJECTKIND",
    );
    for inst in result {
        let age = format_creation_since(inst.age.creation_timestamp);
        println!(
            "{0:<rb_max_name$} {1:<20} {2:<r_max_name$} {3:<20} {4:<subj_max_name$} {5:<20}",
            inst.rb,
            inst.ns,
            inst.r,
            age,
            inst.subject,
            match inst.sub_kind {
                SubKind::User => "User",
                SubKind::Group => "Group",
                SubKind::ServiceAccount => "ServiceAccount",
            },
        );
    }
    Ok(())
}

pub fn output_cr(result: Vec<OutCR>) -> Result<()> {
    let max_name = result.iter().map(|x| x.crb.len() + 2).max().unwrap_or(63);
    let subj_max_name = result
        .iter()
        .map(|x| x.subject.len() + 2)
        .max()
        .unwrap_or(63);
    println!(
        "{0:<width$} {1:<width$} {2:<20} {3:<subj_max_name$} {4:<20}",
        "CLUSTERROLEBINDING",
        "ROLE",
        "AGE",
        "SUBJECT",
        "SUBJECTKIND",
        width = max_name
    );
    for inst in result {
        let age = format_creation_since(inst.age.creation_timestamp);
        println!(
            "{0:<width$} {1:<width$} {2:<20} {3:<subj_max_name$} {4:<20}",
            inst.crb,
            inst.r,
            age,
            inst.subject,
            match inst.sub_kind {
                SubKind::User => "User",
                SubKind::Group => "Group",
                SubKind::ServiceAccount => "ServiceAccount",
            },
            width = max_name
        );
    }
    Ok(())
}

fn format_creation_since(time: Option<Time>) -> String {
    format_duration(Utc::now().signed_duration_since(time.unwrap().0))
}

fn format_duration(dur: Duration) -> String {
    match (dur.num_days(), dur.num_hours(), dur.num_minutes()) {
        (days, _, _) if days > 0 => format!("{days}d"),
        (_, hours, _) if hours > 0 => format!("{hours}h"),
        (_, _, mins) => format!("{mins}m"),
    }
}
