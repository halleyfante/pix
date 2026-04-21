use std::io::{self, BufRead, Write};

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::language_server::completion::{self, CompletionKind};

#[derive(Deserialize)]
struct Message {
    id: Option<Value>,
    method: Option<String>,
    params: Option<Value>,
}

#[derive(Serialize)]
struct Response {
    jsonrpc: &'static str,
    id: Value,
    result: Value,
}

#[derive(Deserialize)]
struct TextDocumentPositionParams {
    #[serde(rename = "textDocument")]
    text_document: TextDocumentIdentifier,
    position: Position,
}

#[derive(Deserialize)]
struct TextDocumentIdentifier {
    uri: String,
}

#[derive(Deserialize)]
struct Position {
    line: u64,
    character: u64,
}

#[derive(Deserialize)]
struct DidOpenParams {
    #[serde(rename = "textDocument")]
    text_document: TextDocumentItem,
}

#[derive(Deserialize)]
struct TextDocumentItem {
    uri: String,
    text: String,
}

#[derive(Deserialize)]
struct DidChangeParams {
    #[serde(rename = "textDocument")]
    text_document: VersionedTextDocumentIdentifier,
    #[serde(rename = "contentChanges")]
    content_changes: Vec<ContentChange>,
}

#[derive(Deserialize)]
struct VersionedTextDocumentIdentifier {
    uri: String,
}

#[derive(Deserialize)]
struct ContentChange {
    text: String,
}

#[derive(Serialize)]
struct LspCompletionItem {
    label: String,
    kind: u32,
    #[serde(rename = "insertText", skip_serializing_if = "Option::is_none")]
    insert_text: Option<String>,
    #[serde(rename = "insertTextFormat", skip_serializing_if = "Option::is_none")]
    insert_text_format: Option<u32>,
}

struct DocumentStore {
    documents: std::collections::HashMap<String, String>,
}

impl DocumentStore {
    fn new() -> Self {
        Self {
            documents: std::collections::HashMap::new(),
        }
    }

    fn open(&mut self, uri: String, text: String) {
        self.documents.insert(uri, text);
    }

    fn update(&mut self, uri: &str, text: String) {
        self.documents.insert(uri.to_string(), text);
    }

    fn get(&self, uri: &str) -> Option<&String> {
        self.documents.get(uri)
    }
}

fn completion_kind_to_lsp(kind: &CompletionKind) -> u32 {
    match kind {
        CompletionKind::Keyword => 14,
        CompletionKind::Color => 16,
        CompletionKind::Variable => 6,
        CompletionKind::Format => 21,
        CompletionKind::Snippet => 15,
    }
}

pub fn run() {
    let stdin = io::stdin();
    let stdout = io::stdout();
    let mut reader = stdin.lock();
    let mut writer = stdout.lock();
    let mut store = DocumentStore::new();

    while let Some(content) = read_message(&mut reader) {

        let message: Message = match serde_json::from_str(&content) {
            Ok(message) => message,
            Err(_) => continue,
        };

        let method = match &message.method {
            Some(method) => method.as_str(),
            None => continue,
        };

        match method {
            "initialize" => {
                let capabilities = serde_json::json!({
                    "capabilities": {
                        "completionProvider": {
                            "triggerCharacters": [" "]
                        },
                        "textDocumentSync": 1
                    }
                });
                send_response(&mut writer, message.id.unwrap(), capabilities);
            }
            "initialized" => {}
            "shutdown" => {
                send_response(&mut writer, message.id.unwrap(), Value::Null);
            }
            "exit" => break,
            "textDocument/didOpen" => {
                if let Some(params) = message.params
                    && let Ok(params) = serde_json::from_value::<DidOpenParams>(params)
                {
                    store.open(params.text_document.uri, params.text_document.text);
                }
            }
            "textDocument/didChange" => {
                if let Some(params) = message.params
                    && let Ok(params) = serde_json::from_value::<DidChangeParams>(params)
                    && let Some(change) = params.content_changes.into_iter().last()
                {
                    store.update(&params.text_document.uri, change.text);
                }
            }
            "textDocument/completion" => {
                if let Some(params) = message.params
                    && let Ok(params) = serde_json::from_value::<TextDocumentPositionParams>(params)
                {
                        let items = if let Some(source) = store.get(&params.text_document.uri) {
                            completion::complete(
                                source,
                                params.position.line as usize + 1,
                                params.position.character as usize + 1,
                            )
                        } else {
                            Vec::new()
                        };

                        let lsp_items: Vec<LspCompletionItem> = items
                            .iter()
                            .map(|item| {
                                let (insert_text, insert_text_format) = match &item.snippet {
                                    Some(snippet) => (Some(snippet.clone()), Some(2)),
                                    None => (None, None),
                                };
                                LspCompletionItem {
                                    label: item.label.clone(),
                                    kind: completion_kind_to_lsp(&item.kind),
                                    insert_text,
                                    insert_text_format,
                                }
                            })
                            .collect();

                        send_response(
                            &mut writer,
                            message.id.unwrap(),
                            serde_json::to_value(lsp_items).unwrap(),
                        );
                }
            }
            _ => {}
        }
    }
}

fn read_message(reader: &mut impl BufRead) -> Option<String> {
    let mut header = String::new();
    let mut content_length: usize = 0;

    loop {
        header.clear();
        if reader.read_line(&mut header).ok()? == 0 {
            return None;
        }
        let trimmed = header.trim();
        if trimmed.is_empty() {
            break;
        }
        if let Some(length) = trimmed.strip_prefix("Content-Length: ") {
            content_length = length.parse().ok()?;
        }
    }

    if content_length == 0 {
        return None;
    }

    let mut body = vec![0u8; content_length];
    reader.read_exact(&mut body).ok()?;
    String::from_utf8(body).ok()
}

fn send_response(writer: &mut impl Write, id: Value, result: Value) {
    let response = Response {
        jsonrpc: "2.0",
        id,
        result,
    };
    let body = serde_json::to_string(&response).unwrap();
    write!(writer, "Content-Length: {}\r\n\r\n{}", body.len(), body).unwrap();
    writer.flush().unwrap();
}
