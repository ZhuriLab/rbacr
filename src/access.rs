use anyhow::Result;
use k8s_openapi::api::{
    core::v1::Namespace,
    rbac::v1::{ClusterRole, ClusterRoleBinding, Role, RoleBinding},
};
use kube::{
    api::{Api, ListParams, ResourceExt},
    Client,
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Access {
    pub ns_list: NsAll,
    pub rbinfo_list: Vec<RBInfo>,
    pub crbinfo_list: Vec<CRBInfo>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NsAll {
    pub ns_all: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RBInfo {
    pub role_binding_info: RoleBinding,
    pub role_info: Role,
    pub role_resources: Option<Vec<String>>,
    pub role_verbs: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CRBInfo {
    pub cluster_role_binding_info: ClusterRoleBinding,
    pub cluster_role_info: ClusterRole,
    pub cluster_role_resources: Option<Vec<String>>,
    pub cluster_role_verbs: Vec<String>,
}

impl Access {
    pub async fn run(client: Client) -> Result<Self> {
        let ns_all = NsAll::run(client.clone()).await?;
        let rbinfo_list = RBInfo::run(client.clone(), ns_all.clone()).await?;
        let crbinfo_list = CRBInfo::run(client.clone()).await?;
        Ok(Self {
            ns_list: ns_all,
            rbinfo_list,
            crbinfo_list,
        })
    }
}

impl NsAll {
    pub async fn run(client: Client) -> Result<Self> {
        let ns_api: Api<Namespace> = Api::all(client);
        let ns_list = ns_api.list(&ListParams::default()).await?;
        let mut ns_all: Vec<String> = Vec::new();
        for ns in ns_list {
            ns_all.push(ns.name_any());
        }
        Ok(Self { ns_all })
    }
}

impl RBInfo {
    pub async fn run(client: Client, ns_all: NsAll) -> Result<Vec<Self>> {
        let mut rbinfo_list: Vec<Self> = Vec::new();
        for ns in ns_all.ns_all {
            let rb_api: Api<RoleBinding> = Api::namespaced(client.clone(), &ns);
            let list_params = ListParams::default();
            let rb_list = rb_api.list(&list_params).await?;
            for rb in rb_list {
                let role_api: Api<Role> = Api::namespaced(client.clone(), &ns);
                let role_name = &rb.role_ref.name;
                let role = match role_api.get(&role_name).await {
                    Ok(role) => role,
                    Err(_) => continue,
                };
                rbinfo_list.push(Self {
                    role_binding_info: rb,
                    role_info: role.clone(),
                    role_resources: role.rules.as_ref().map(|rules| {
                        rules
                            .iter()
                            .map(|rule| rule.resources.as_ref().unwrap().clone())
                            .flatten()
                            .collect()
                    }),
                    role_verbs: role
                        .rules
                        .as_ref()
                        .map(|rules| {
                            rules
                                .iter()
                                .map(|rule| rule.verbs.clone())
                                .flatten()
                                .collect()
                        })
                        .unwrap_or_default(),
                });
            }
        }
        Ok(rbinfo_list)
    }
}

impl CRBInfo {
    pub async fn run(client: Client) -> Result<Vec<Self>> {
        let mut crbinfo_list: Vec<Self> = Vec::new();
        let crb_api: Api<ClusterRoleBinding> = Api::all(client.clone());
        let list_params = ListParams::default();
        let crb_list = crb_api.list(&list_params).await?;
        for crb in crb_list {
            let cr_api: Api<ClusterRole> = Api::all(client.clone());
            let cr_name = &crb.role_ref.name;
            let cr = match cr_api.get(&cr_name).await {
                Ok(cr) => cr,
                Err(_) => continue,
            };
            crbinfo_list.push(Self {
                cluster_role_binding_info: crb,
                cluster_role_info: cr.clone(),
                cluster_role_resources: cr.rules.as_ref().map(|rules| {
                    rules
                        .iter()
                        .flat_map(|rule| rule.resources.clone())
                        .flatten()
                        .collect()
                }),
                cluster_role_verbs: cr
                    .rules
                    .as_ref()
                    .map(|rules| {
                        rules
                            .iter()
                            .map(|rule| rule.verbs.clone())
                            .flatten()
                            .collect()
                    })
                    .unwrap_or_default(),
            });
        }
        Ok(crbinfo_list)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use kube::Client;

    #[tokio::test]
    async fn test_access() {
        let client = Client::try_default().await.unwrap();
        let access = Access::run(client).await;

        assert!(access.is_ok());
        assert_eq!(
            access.unwrap().ns_list.ns_all,
            vec![
                "default",
                "kube-flannel",
                "kube-node-lease",
                "kube-public",
                "kube-system"
            ]
        );
    }
}
