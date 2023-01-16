use std::collections::HashMap;

use anyhow::Result;
use kube::ResourceExt;
use log::info;
use serde::{Deserialize, Serialize};
use tokio::fs;

use crate::{
    access::{CRBInfo, RBInfo},
    print::{output_cr, output_r, SubKind},
    Access, OutCR, OutR,
};

#[derive(Serialize, Deserialize, Debug)]
pub struct Check {
    pub itemlist: HashMap<String, CheckItem>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CheckItem {
    // pub namespace: String,
    pub kind: Kind,
    pub rules: Option<Vec<Rules>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Kind {
    All,
    Role,
    ClusterRole,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Rules {
    pub resources: Option<Vec<String>>,
    pub verbs: Vec<String>,
}

impl Check {
    pub fn new() -> Self {
        Self {
            itemlist: HashMap::new(),
        }
    }

    pub fn add(&mut self, key: String, item: CheckItem) {
        // let key = match &item.kind {
        //     Kind::All => "all".to_string(),
        //     Kind::Role => "role".to_string(),
        //     Kind::ClusterRole => "clusterrole".to_string(),
        // };
        self.itemlist.insert(key, item);
    }

    pub async fn load_yaml(path: &str) -> Result<Self> {
        let content = fs::read_to_string(path).await?;
        Self::from_yaml(&content)
    }

    pub fn from_yaml(content: &str) -> Result<Self> {
        let mut check = Self::new();
        let check_yaml: Check = serde_yaml::from_str(content)?;
        for (key, item) in check_yaml.itemlist {
            check.add(key, item);
        }
        Ok(check)
    }

    pub fn get_check(&self) -> Result<Vec<String>> {
        let mut check_list = Vec::new();
        for (key, _item) in &self.itemlist {
            check_list.push(key.to_string());
        }
        Ok(check_list)
    }

    pub fn get_check_item(&self, key: &str) -> Result<&CheckItem> {
        let item = self.itemlist.get(key).unwrap();
        Ok(item)
    }

    pub async fn run(&self, access: Access) -> Result<()> {
        for (key, item) in &self.itemlist {
            info!("Start checking: {}", key);
            match item.kind {
                Kind::All => {
                    let mut out_r_list = Vec::new();
                    for rbinfo in &access.rbinfo_list {
                        if Self::check_r(&item.rules, rbinfo).await? {
                            let rb = &rbinfo.role_binding_info;
                            match &rb.subjects {
                                Some(subjects) => {
                                    let (subject, sub_kind) = {
                                        let sub_kind_string = subjects[0].kind.to_string();
                                        let sub_kind = match sub_kind_string.as_str() {
                                            "User" => SubKind::User,
                                            "Group" => SubKind::Group,
                                            "ServiceAccount" => SubKind::ServiceAccount,
                                            _ => {
                                                return Err(anyhow::anyhow!(
                                                    "Unknown subject kind: {}",
                                                    sub_kind_string
                                                ))
                                            }
                                        };
                                        (subjects[0].name.clone(), sub_kind)
                                    };
                                    let out_r = OutR {
                                        rb: rb.name_any(),
                                        ns: rb.namespace().unwrap_or("default".to_string()),
                                        r: rb.role_ref.name.clone(),
                                        age: rb.metadata.clone(),
                                        subject,
                                        sub_kind,
                                    };
                                    out_r_list.push(out_r);
                                }
                                None => {
                                    info!(
                                        "RoleBinding: \"{}\" does not display without subjects",
                                        rb.name_any()
                                    );
                                }
                            }
                        }
                    }
                    match out_r_list.len() {
                        0 => {}
                        _ => {
                            output_r(out_r_list)?;
                        }
                    }

                    let mut out_cr_list = Vec::new();
                    for crbinfo in &access.crbinfo_list {
                        if Self::check_cr(&item.rules, crbinfo).await? {
                            let crb = &crbinfo.cluster_role_binding_info;
                            match &crb.subjects {
                                Some(subjects) => {
                                    let (subject, sub_kind) = {
                                        let sub_kind_string = subjects[0].kind.to_string();
                                        let sub_kind = match sub_kind_string.as_str() {
                                            "User" => SubKind::User,
                                            "Group" => SubKind::Group,
                                            "ServiceAccount" => SubKind::ServiceAccount,
                                            _ => {
                                                return Err(anyhow::anyhow!(
                                                    "Unknown subject kind: {}",
                                                    sub_kind_string
                                                ))
                                            }
                                        };
                                        (subjects[0].name.clone(), sub_kind)
                                    };
                                    let out_cr = OutCR {
                                        crb: crb.name_any(),
                                        r: crb.role_ref.name.clone(),
                                        age: crb.metadata.clone(),
                                        subject,
                                        sub_kind,
                                    };
                                    out_cr_list.push(out_cr);
                                }
                                None => {
                                    info!(
                                        "ClusterRoleBinding: \"{}\" does not display without subjects",
                                        crb.name_any()
                                    );
                                }
                            };
                        }
                    }
                    match out_cr_list.len() {
                        0 => {}
                        _ => {
                            output_cr(out_cr_list)?;
                        }
                    }
                }
                Kind::Role => {
                    let mut out_r_list = Vec::new();
                    for rbinfo in &access.rbinfo_list {
                        if Self::check_r(&item.rules, rbinfo).await? {
                            let rb = &rbinfo.role_binding_info;
                            match &rb.subjects {
                                Some(subjects) => {
                                    let (subject, sub_kind) = {
                                        let sub_kind_string = subjects[0].kind.to_string();
                                        let sub_kind = match sub_kind_string.as_str() {
                                            "User" => SubKind::User,
                                            "Group" => SubKind::Group,
                                            "ServiceAccount" => SubKind::ServiceAccount,
                                            _ => {
                                                return Err(anyhow::anyhow!(
                                                    "Unknown subject kind: {}",
                                                    sub_kind_string
                                                ))
                                            }
                                        };
                                        (subjects[0].name.clone(), sub_kind)
                                    };
                                    let out_r = OutR {
                                        rb: rb.name_any(),
                                        ns: rb.namespace().unwrap_or("default".to_string()),
                                        r: rb.role_ref.name.clone(),
                                        age: rb.metadata.clone(),
                                        subject,
                                        sub_kind,
                                    };
                                    out_r_list.push(out_r);
                                }
                                None => {
                                    info!(
                                        "RoleBinding: \"{}\" does not display without subjects",
                                        rb.name_any()
                                    );
                                }
                            };
                        }
                    }
                    match out_r_list.len() {
                        0 => {}
                        _ => {
                            output_r(out_r_list)?;
                        }
                    }
                }
                Kind::ClusterRole => {
                    let mut out_cr_list = Vec::new();
                    for crbinfo in &access.crbinfo_list {
                        if Self::check_cr(&item.rules, crbinfo).await? {
                            let crb = &crbinfo.cluster_role_binding_info;
                            match &crb.subjects {
                                Some(subjects) => {
                                    let (subject, sub_kind) = {
                                        let sub_kind_string = subjects[0].kind.to_string();
                                        let sub_kind = match sub_kind_string.as_str() {
                                            "User" => SubKind::User,
                                            "Group" => SubKind::Group,
                                            "ServiceAccount" => SubKind::ServiceAccount,
                                            _ => {
                                                return Err(anyhow::anyhow!(
                                                    "Unknown subject kind: {}",
                                                    sub_kind_string
                                                ))
                                            }
                                        };
                                        (subjects[0].name.clone(), sub_kind)
                                    };
                                    let out_cr = OutCR {
                                        crb: crb.name_any(),
                                        r: crb.role_ref.name.clone(),
                                        age: crb.metadata.clone(),
                                        subject,
                                        sub_kind,
                                    };
                                    out_cr_list.push(out_cr);
                                }
                                None => {
                                    info!(
                                        "ClusterRoleBinding: \"{}\" does not display without subjects",
                                        crb.name_any()
                                    );
                                }
                            }
                        }
                    }
                    match out_cr_list.len() {
                        0 => {}
                        _ => {
                            output_cr(out_cr_list)?;
                        }
                    }
                }
            }
        }
        Ok(())
    }

    pub async fn check_r(rules: &Option<Vec<Rules>>, rbinfo: &RBInfo) -> Result<bool> {
        let mut checked: bool = false;
        match rules {
            Some(rules) => {
                for rules in rules {
                    if let Some(resources) = &rules.resources {
                        for resource in resources {
                            if let Some(ref role_resources) = rbinfo.role_resources {
                                if !role_resources.contains(&resource) {
                                    return Ok(checked);
                                } else {
                                    for verb in &rules.verbs {
                                        if !rbinfo.role_verbs.contains(&verb) {
                                            return Ok(checked);
                                        } else {
                                            checked = true
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
            None => {
                return Ok(checked);
            }
        }
        Ok(checked)
    }

    pub async fn check_cr(rules: &Option<Vec<Rules>>, crbinfo: &CRBInfo) -> Result<bool> {
        let mut checked: bool = false;
        match rules {
            Some(rules) => {
                for rules in rules {
                    if let Some(resources) = &rules.resources {
                        for resource in resources {
                            if let Some(ref clusterrole_resources) = crbinfo.cluster_role_resources
                            {
                                if !clusterrole_resources.contains(&resource) {
                                    return Ok(checked);
                                } else {
                                    for verb in &rules.verbs {
                                        if !crbinfo.cluster_role_verbs.contains(&verb) {
                                            return Ok(checked);
                                        } else {
                                            checked = true
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
            None => return Ok(checked),
        }

        Ok(checked)
    }
}

impl CheckItem {
    pub fn new(kind: Kind, rules: Option<Vec<Rules>>) -> Self {
        Self {
            // namespace: namespace,
            kind,
            rules,
        }
    }
}

impl Rules {
    pub fn new(resources: Option<Vec<String>>, verbs: Vec<String>) -> Self {
        Self { resources, verbs }
    }
}

// impl Kind {
//     pub fn new(kind: &str) -> Self {
//         match kind {
//             "all" => Self::All,
//             "role" => Self::Role,
//             "clusterrole" => Self::ClusterRole,
//             _ => Self::All,
//         }
//     }
// }
