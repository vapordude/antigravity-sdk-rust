use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ThinkingLevel {
    Minimal,
    Low,
    Medium,
    High,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct GenerationConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thinking_level: Option<ThinkingLevel>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ModelEntry {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_key: Option<String>,
    #[serde(default)]
    pub generation: GenerationConfig,
}

fn default_model_entry() -> ModelEntry {
    ModelEntry {
        name: "gemini-3.5-flash".to_string(),
        api_key: None,
        generation: GenerationConfig::default(),
    }
}

fn default_image_generation_model_entry() -> ModelEntry {
    ModelEntry {
        name: "gemini-3.1-flash-image-preview".to_string(),
        api_key: None,
        generation: GenerationConfig::default(),
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ModelConfig {
    #[serde(default = "default_model_entry")]
    pub default: ModelEntry,
    #[serde(default = "default_image_generation_model_entry")]
    pub image_generation: ModelEntry,
}

impl Default for ModelConfig {
    fn default() -> Self {
        Self {
            default: default_model_entry(),
            image_generation: default_image_generation_model_entry(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct GeminiConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_key: Option<String>,
    #[serde(default)]
    pub vertex: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub project: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[serde(default)]
    pub models: ModelConfig,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SystemInstructionSection {
    pub content: String,
    #[serde(default = "default_section_title")]
    pub title: String,
}

fn default_section_title() -> String {
    "user_system_instructions".to_string()
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CustomSystemInstructions {
    pub text: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct TemplatedSystemInstructions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity: Option<String>,
    #[serde(default)]
    pub sections: Vec<SystemInstructionSection>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SystemInstructions {
    Custom(CustomSystemInstructions),
    Templated(TemplatedSystemInstructions),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BuiltinTools {
    ListDirectory,
    SearchDirectory,
    FindFile,
    ViewFile,
    CreateFile,
    EditFile,
    RunCommand,
    AskQuestion,
    StartSubagent,
    GenerateImage,
    Finish,
}

impl BuiltinTools {
    pub fn read_only() -> Vec<Self> {
        vec![
            Self::ListDirectory,
            Self::SearchDirectory,
            Self::FindFile,
            Self::ViewFile,
            Self::Finish,
        ]
    }

    pub fn nondestructive() -> Vec<Self> {
        vec![
            Self::ListDirectory,
            Self::SearchDirectory,
            Self::FindFile,
            Self::ViewFile,
            Self::CreateFile,
            Self::EditFile,
            Self::AskQuestion,
            Self::StartSubagent,
            Self::GenerateImage,
            Self::Finish,
        ]
    }

    pub fn all_tools() -> Vec<Self> {
        vec![
            Self::ListDirectory,
            Self::SearchDirectory,
            Self::FindFile,
            Self::ViewFile,
            Self::CreateFile,
            Self::EditFile,
            Self::RunCommand,
            Self::AskQuestion,
            Self::StartSubagent,
            Self::GenerateImage,
            Self::Finish,
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_thinking_level_serialization() {
        assert_eq!(
            serde_json::to_string(&ThinkingLevel::High).unwrap(),
            "\"high\""
        );
        assert_eq!(
            serde_json::to_string(&ThinkingLevel::Low).unwrap(),
            "\"low\""
        );
    }

    #[test]
    fn test_generation_config_serialization() {
        let config = GenerationConfig {
            thinking_level: Some(ThinkingLevel::Medium),
        };
        let json = serde_json::to_string(&config).unwrap();
        assert_eq!(json, r#"{"thinking_level":"medium"}"#);

        let config_none = GenerationConfig {
            thinking_level: None,
        };
        let json_none = serde_json::to_string(&config_none).unwrap();
        assert_eq!(json_none, r#"{}"#);
    }

    #[test]
    fn test_model_config_default() {
        let json = r#"{}"#;
        let config: ModelConfig = serde_json::from_str(json).unwrap();
        assert_eq!(config.default.name, "gemini-3.5-flash");
        assert_eq!(
            config.image_generation.name,
            "gemini-3.1-flash-image-preview"
        );
    }

    #[test]
    fn test_system_instructions_untagged() {
        let custom_json = r#"{"text": "You are a helpful assistant."}"#;
        let inst: SystemInstructions = serde_json::from_str(custom_json).unwrap();
        match inst {
            SystemInstructions::Custom(c) => assert_eq!(c.text, "You are a helpful assistant."),
            _ => panic!("Expected Custom"),
        }

        let templated_json =
            r#"{"identity": "Assistant", "sections": [{"content": "Be nice", "title": "Rules"}]}"#;
        let inst2: SystemInstructions = serde_json::from_str(templated_json).unwrap();
        match inst2 {
            SystemInstructions::Templated(t) => {
                assert_eq!(t.identity.unwrap(), "Assistant");
                assert_eq!(t.sections[0].title, "Rules");
            }
            _ => panic!("Expected Templated"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Content {
    pub text: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Step {
    // Basic fields matching python SDK
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ToolResult {}
// Removed unused HashMap import

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ToolCall {
    pub name: String,
    pub args: serde_json::Value,
}
