#![allow(missing_docs, trivial_casts, unused_variables, unused_mut, unused_imports, unused_extern_crates, unused_attributes, non_camel_case_types)]
#![allow(clippy::derive_partial_eq_without_eq, clippy::disallowed_names)]

use async_trait::async_trait;
use futures::Stream;
use std::error::Error;
use std::collections::BTreeSet;
use std::task::{Poll, Context};
use swagger::{ApiError, ContextWrapper};
use serde::{Serialize, Deserialize};
use crate::server::Authorization;


type ServiceError = Box<dyn Error + Send + Sync + 'static>;

pub const BASE_PATH: &str = "";
pub const API_VERSION: &str = "1.0.0";

mod auth;
pub use auth::{AuthenticationApi, Claims};


#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum GetIndexResponse {
    /// 成功レスポンス
    Status200
    (models::GetIndex200Response)
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum ItemsGetResponse {
    /// アイテムリストを返します。
    Status200
    (Vec<models::ItemsGet200ResponseInner>)
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum ItemsPostResponse {
    /// 作成成功
    Status201
    (models::ItemsPost201Response)
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum ItemsIdDeleteResponse {
    /// 削除成功
    Status204
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum ItemsIdGetResponse {
    /// 指定アイテムを返します。
    Status200
    (models::ItemsGet200ResponseInner)
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum ItemsIdPutResponse {
    /// 更新成功
    Status200
    (models::ItemsIdPut200Response)
}

/// API
#[async_trait]
#[allow(clippy::too_many_arguments, clippy::ptr_arg)]
pub trait Api<C: Send + Sync> {
    fn poll_ready(&self, _cx: &mut Context) -> Poll<Result<(), Box<dyn Error + Send + Sync + 'static>>> {
        Poll::Ready(Ok(()))
    }

    /// メインエンドポイント
    async fn get_index(
        &self,
        context: &C) -> Result<GetIndexResponse, ApiError>;

    /// アイテム一覧取得
    async fn items_get(
        &self,
        context: &C) -> Result<ItemsGetResponse, ApiError>;

    /// 新規アイテム作成
    async fn items_post(
        &self,
        items_post_request: models::ItemsPostRequest,
        context: &C) -> Result<ItemsPostResponse, ApiError>;

    /// アイテム削除
    async fn items_id_delete(
        &self,
        id: i32,
        context: &C) -> Result<ItemsIdDeleteResponse, ApiError>;

    /// アイテム取得
    async fn items_id_get(
        &self,
        id: i32,
        context: &C) -> Result<ItemsIdGetResponse, ApiError>;

    /// アイテム更新
    async fn items_id_put(
        &self,
        id: i32,
        items_id_put_request: models::ItemsIdPutRequest,
        context: &C) -> Result<ItemsIdPutResponse, ApiError>;

}

/// API where `Context` isn't passed on every API call
#[async_trait]
#[allow(clippy::too_many_arguments, clippy::ptr_arg)]
pub trait ApiNoContext<C: Send + Sync> {

    fn poll_ready(&self, _cx: &mut Context) -> Poll<Result<(), Box<dyn Error + Send + Sync + 'static>>>;

    fn context(&self) -> &C;

    /// メインエンドポイント
    async fn get_index(
        &self,
        ) -> Result<GetIndexResponse, ApiError>;

    /// アイテム一覧取得
    async fn items_get(
        &self,
        ) -> Result<ItemsGetResponse, ApiError>;

    /// 新規アイテム作成
    async fn items_post(
        &self,
        items_post_request: models::ItemsPostRequest,
        ) -> Result<ItemsPostResponse, ApiError>;

    /// アイテム削除
    async fn items_id_delete(
        &self,
        id: i32,
        ) -> Result<ItemsIdDeleteResponse, ApiError>;

    /// アイテム取得
    async fn items_id_get(
        &self,
        id: i32,
        ) -> Result<ItemsIdGetResponse, ApiError>;

    /// アイテム更新
    async fn items_id_put(
        &self,
        id: i32,
        items_id_put_request: models::ItemsIdPutRequest,
        ) -> Result<ItemsIdPutResponse, ApiError>;

}

/// Trait to extend an API to make it easy to bind it to a context.
pub trait ContextWrapperExt<C: Send + Sync> where Self: Sized
{
    /// Binds this API to a context.
    fn with_context(self, context: C) -> ContextWrapper<Self, C>;
}

impl<T: Api<C> + Send + Sync, C: Clone + Send + Sync> ContextWrapperExt<C> for T {
    fn with_context(self: T, context: C) -> ContextWrapper<T, C> {
         ContextWrapper::<T, C>::new(self, context)
    }
}

#[async_trait]
impl<T: Api<C> + Send + Sync, C: Clone + Send + Sync> ApiNoContext<C> for ContextWrapper<T, C> {
    fn poll_ready(&self, cx: &mut Context) -> Poll<Result<(), ServiceError>> {
        self.api().poll_ready(cx)
    }

    fn context(&self) -> &C {
        ContextWrapper::context(self)
    }

    /// メインエンドポイント
    async fn get_index(
        &self,
        ) -> Result<GetIndexResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().get_index(&context).await
    }

    /// アイテム一覧取得
    async fn items_get(
        &self,
        ) -> Result<ItemsGetResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().items_get(&context).await
    }

    /// 新規アイテム作成
    async fn items_post(
        &self,
        items_post_request: models::ItemsPostRequest,
        ) -> Result<ItemsPostResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().items_post(items_post_request, &context).await
    }

    /// アイテム削除
    async fn items_id_delete(
        &self,
        id: i32,
        ) -> Result<ItemsIdDeleteResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().items_id_delete(id, &context).await
    }

    /// アイテム取得
    async fn items_id_get(
        &self,
        id: i32,
        ) -> Result<ItemsIdGetResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().items_id_get(id, &context).await
    }

    /// アイテム更新
    async fn items_id_put(
        &self,
        id: i32,
        items_id_put_request: models::ItemsIdPutRequest,
        ) -> Result<ItemsIdPutResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().items_id_put(id, items_id_put_request, &context).await
    }

}


#[cfg(feature = "client")]
pub mod client;

// Re-export Client as a top-level name
#[cfg(feature = "client")]
pub use client::Client;

#[cfg(feature = "server")]
pub mod server;

// Re-export router() as a top-level name
#[cfg(feature = "server")]
pub use self::server::Service;

#[cfg(feature = "server")]
pub mod context;

pub mod models;

#[cfg(any(feature = "client", feature = "server"))]
pub(crate) mod header;
