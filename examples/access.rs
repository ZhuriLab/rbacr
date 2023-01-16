use anyhow::Result;
use kube::Client;
use rbacr::Access;

#[tokio::main]
async fn main() -> Result<()> {
    let client = Client::try_default().await?;
    let access = Access::run(client).await?;
    for crbinfo in access.crbinfo_list {
        if crbinfo.cluster_role_info.metadata.name == Some("csr".to_string()) {
            println!("cluster_role_info: {:#?}", crbinfo.cluster_role_info);
            println!("------------------------------------");
            println!(
                "cluster_role_resources: {:#?}",
                crbinfo.cluster_role_resources
            );
            println!("------------------------------------");
            println!("cluster_role_verbs: {:#?}", crbinfo.cluster_role_verbs);
        }
    }

    // for crbinfo in access.crbinfo_list {
    //     if crbinfo.cluster_role_info.metadata.name == Some("csr".to_string()) {
    //         let subjects = crbinfo.cluster_role_binding_info.subjects;
    //         if let Some(subjects) = subjects {
    //             println!("name: {:#?}", subjects[0].name);
    //         }
    //     }
    // }

    Ok(())
}
