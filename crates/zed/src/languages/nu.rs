use anyhow::{anyhow, Result};
use async_trait::async_trait;
use language::{CodeLabel, Language, LanguageServerName, LspAdapter, LspAdapterDelegate};
use lsp::LanguageServerBinary;
use std::{any::Any, path::PathBuf, sync::Arc};

pub struct NuLanguageServer;

#[async_trait]
impl LspAdapter for NuLanguageServer {
    async fn name(&self) -> LanguageServerName {
        LanguageServerName("nu".into())
    }

    fn short_name(&self) -> &'static str {
        "nu"
    }

    async fn fetch_latest_server_version(
        &self,
        _: &dyn LspAdapterDelegate,
    ) -> Result<Box<dyn 'static + Any + Send>> {
        Ok(Box::new(()))
    }

    async fn fetch_server_binary(
        &self,
        _version: Box<dyn 'static + Send + Any>,
        _container_dir: PathBuf,
        _: &dyn LspAdapterDelegate,
    ) -> Result<LanguageServerBinary> {
        Err(anyhow!(
            "nu v0.87.0 or greater must be installed and available in your $PATH"
        ))
    }

    async fn cached_server_binary(
        &self,
        _: PathBuf,
        _: &dyn LspAdapterDelegate,
    ) -> Option<LanguageServerBinary> {
        Some(LanguageServerBinary {
            path: "nu".into(),
            arguments: vec!["--lsp".into()],
        })
    }

    fn can_be_reinstalled(&self) -> bool {
        false
    }

    async fn installation_test_binary(&self, _: PathBuf) -> Option<LanguageServerBinary> {
        None
    }

    async fn label_for_completion(
        &self,
        completion: &lsp::CompletionItem,
        language: &Arc<Language>,
    ) -> Option<CodeLabel> {
        return Some(CodeLabel {
            runs: language
                .highlight_text(&completion.label.clone().into(), 0..completion.label.len()),
            text: completion.label.clone(),
            filter_range: 0..completion.label.len(),
        });
    }

    async fn label_for_symbol(
        &self,
        name: &str,
        _: lsp::SymbolKind,
        language: &Arc<Language>,
    ) -> Option<CodeLabel> {
        Some(CodeLabel {
            runs: language.highlight_text(&name.into(), 0..name.len()),
            text: name.to_string(),
            filter_range: 0..name.len(),
        })
    }
}
