use std::path::Path;

use crate::client::{
    self, PublishPackageRequest, PublishPackageUpgradeRequest, PublishPackageVersionRequest,
};
use crate::credentil_store::read_access_token;
use crate::models::{self, InstallFile, Payload, ReadmeFile, UpgradeFile};

pub async fn publish(client: &client::APIClient, path: &Path) -> anyhow::Result<()> {
    let payload = models::Payload::from_path(path)?;
    let access_token = read_access_token().await?;
    let jwt = client.redeem_access_token(access_token).await?;

    let Some(ref readme_file) = payload.readme_file else {
        return Err(anyhow::anyhow!("No `README.md` file found"));
    };

    let request = create_publish_package_request(&payload);
    client.publish_package(&jwt, &request).await?;

    for install_file in &payload.install_files {
        let request = create_publich_package_version_request(
            &payload.metadata.extension_name,
            install_file,
            readme_file,
        );
        client.publish_package_version(&jwt, &request).await?;
    }

    for upgrade_file in &payload.upgrade_files {
        let request =
            create_publich_package_upgrade_request(&payload.metadata.extension_name, upgrade_file);
        client.publish_package_upgrade(&jwt, &request).await?;
    }

    Ok(())
}

fn create_publish_package_request(payload: &Payload) -> PublishPackageRequest {
    PublishPackageRequest {
        package_name: &payload.metadata.extension_name,
        package_description: &payload.metadata.comment,
    }
}

fn create_publich_package_version_request<'a>(
    package_name: &'a str,
    install_file: &'a InstallFile,
    readme_file: &'a ReadmeFile,
) -> PublishPackageVersionRequest<'a> {
    PublishPackageVersionRequest {
        package_name,
        version: &install_file.version,
        version_source: &install_file.body,
        version_description: readme_file.body(),
    }
}

fn create_publich_package_upgrade_request<'a>(
    package_name: &'a str,
    upgrade_file: &'a UpgradeFile,
) -> PublishPackageUpgradeRequest<'a> {
    PublishPackageUpgradeRequest {
        package_name,
        from_version: &upgrade_file.from_version,
        to_version: &upgrade_file.to_version,
        upgrade_source: &upgrade_file.body,
    }
}
