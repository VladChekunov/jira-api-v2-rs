/*
 * The Jira Cloud platform REST API
 *
 * Jira Cloud platform REST API documentation
 *
 * The version of the OpenAPI document: 1001.0.0-SNAPSHOT
 * Contact: ecosystem@atlassian.com
 * Generated by: https://openapi-generator.tech
 */


use reqwest;
use serde::{Deserialize, Serialize};
use crate::{apis::ResponseContent, models};
use super::{Error, configuration};


/// struct for typed errors of method [`addon_properties_resource_period_delete_addon_property_delete`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AddonPropertiesResourcePeriodDeleteAddonPropertyDeleteError {
    Status400(models::OperationMessage),
    Status401(models::OperationMessage),
    Status404(models::OperationMessage),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`addon_properties_resource_period_get_addon_properties_get`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AddonPropertiesResourcePeriodGetAddonPropertiesGetError {
    Status401(models::OperationMessage),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`addon_properties_resource_period_get_addon_property_get`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AddonPropertiesResourcePeriodGetAddonPropertyGetError {
    Status400(models::OperationMessage),
    Status401(models::OperationMessage),
    Status404(models::OperationMessage),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`addon_properties_resource_period_put_addon_property_put`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AddonPropertiesResourcePeriodPutAddonPropertyPutError {
    Status400(models::OperationMessage),
    Status401(models::OperationMessage),
    UnknownValue(serde_json::Value),
}


/// Deletes an app's property.  **[Permissions](#permissions) required:** Only a Connect app whose key matches `addonKey` can make this request.
pub async fn addon_properties_resource_period_delete_addon_property_delete(configuration: &configuration::Configuration, addon_key: &str, property_key: &str) -> Result<(), Error<AddonPropertiesResourcePeriodDeleteAddonPropertyDeleteError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_addon_key = addon_key;
    let p_property_key = property_key;

    let uri_str = format!("{}/rest/atlassian-connect/1/addons/{addonKey}/properties/{propertyKey}", configuration.base_path, addonKey=crate::apis::urlencode(p_addon_key), propertyKey=crate::apis::urlencode(p_property_key));
    let mut req_builder = configuration.client.request(reqwest::Method::DELETE, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        Ok(())
    } else {
        let content = resp.text().await?;
        let entity: Option<AddonPropertiesResourcePeriodDeleteAddonPropertyDeleteError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// Gets all the properties of an app.  **[Permissions](#permissions) required:** Only a Connect app whose key matches `addonKey` can make this request.
pub async fn addon_properties_resource_period_get_addon_properties_get(configuration: &configuration::Configuration, addon_key: &str) -> Result<models::PropertyKeys, Error<AddonPropertiesResourcePeriodGetAddonPropertiesGetError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_addon_key = addon_key;

    let uri_str = format!("{}/rest/atlassian-connect/1/addons/{addonKey}/properties", configuration.base_path, addonKey=crate::apis::urlencode(p_addon_key));
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        serde_json::from_str(&content).map_err(Error::from)
    } else {
        let content = resp.text().await?;
        let entity: Option<AddonPropertiesResourcePeriodGetAddonPropertiesGetError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// Returns the key and value of an app's property.  **[Permissions](#permissions) required:** Only a Connect app whose key matches `addonKey` can make this request.
pub async fn addon_properties_resource_period_get_addon_property_get(configuration: &configuration::Configuration, addon_key: &str, property_key: &str) -> Result<models::EntityProperty, Error<AddonPropertiesResourcePeriodGetAddonPropertyGetError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_addon_key = addon_key;
    let p_property_key = property_key;

    let uri_str = format!("{}/rest/atlassian-connect/1/addons/{addonKey}/properties/{propertyKey}", configuration.base_path, addonKey=crate::apis::urlencode(p_addon_key), propertyKey=crate::apis::urlencode(p_property_key));
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        serde_json::from_str(&content).map_err(Error::from)
    } else {
        let content = resp.text().await?;
        let entity: Option<AddonPropertiesResourcePeriodGetAddonPropertyGetError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// Sets the value of an app's property. Use this resource to store custom data for your app.  The value of the request body must be a [valid](http://tools.ietf.org/html/rfc4627), non-empty JSON blob. The maximum length is 32768 characters.  **[Permissions](#permissions) required:** Only a Connect app whose key matches `addonKey` can make this request.
pub async fn addon_properties_resource_period_put_addon_property_put(configuration: &configuration::Configuration, addon_key: &str, property_key: &str, body: Option<serde_json::Value>) -> Result<models::OperationMessage, Error<AddonPropertiesResourcePeriodPutAddonPropertyPutError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_addon_key = addon_key;
    let p_property_key = property_key;
    let p_body = body;

    let uri_str = format!("{}/rest/atlassian-connect/1/addons/{addonKey}/properties/{propertyKey}", configuration.base_path, addonKey=crate::apis::urlencode(p_addon_key), propertyKey=crate::apis::urlencode(p_property_key));
    let mut req_builder = configuration.client.request(reqwest::Method::PUT, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    req_builder = req_builder.json(&p_body);

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        serde_json::from_str(&content).map_err(Error::from)
    } else {
        let content = resp.text().await?;
        let entity: Option<AddonPropertiesResourcePeriodPutAddonPropertyPutError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

