use std::collections::HashMap;

use async_trait::async_trait;
use aws_sdk_dynamodb as dynamodb;
use aws_sdk_dynamodb::model::{
    AttributeDefinition, AttributeValue, KeySchemaElement, KeyType, ProvisionedThroughput,
    ScalarAttributeType, Select, TableStatus,
};
use derive_new::new;
use domain::{User, UserId, UserRepository};
use dynamodb::model::KeysAndAttributes;

#[derive(new)]
pub struct UserRepositoryImpl();

#[async_trait]
impl UserRepository for UserRepositoryImpl {
    async fn save(&self, user: &User) -> anyhow::Result<()> {
        let table = "StoreBranch";
        let shared_config = aws_config::load_from_env().await;
        let client = dynamodb::Client::new(&shared_config);

        let request = client
            .put_item()
            .table_name(table)
            .item("UserName", AttributeValue::S(user.name().clone().into()));

        request.send().await?;

        Ok(())
    }

    async fn get_by_ids(&self, ids: &[UserId]) -> anyhow::Result<Vec<User>> {
        todo!()
    }
}
