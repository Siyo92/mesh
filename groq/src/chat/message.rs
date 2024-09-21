use serde::{Deserialize, Serialize};

use crate::models::Model;

/// Chat completion response returned by the model
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatCompletion {
    /// A unique identifier for the chat completion.
    pub id: String,

    /// A list of chat completion choices. Can be more than one if n is greater than 1.
    pub choices: Vec<CompletionChoice>,

    /// The Unix timestamp (in seconds) of when the chat completion was created.
    pub created: u64,

    /// The model used for the chat completion.
    pub model: Model,

    /// The service tier used for processing the request. This field is only included if the **service_tier** parameter is specified in the request.
    pub service_tier: Option<String>,

    /// This fingerprint represents the backend configuration that the model runs with.
    ///
    /// Can be used in conjunction with the seed request parameter to understand when backend changes have been made that might impact determinism
    pub system_fingerprint: Option<String>,

    /// The object type, which is always **chat.completion**.
    pub object: String,

    /// Usage statistics for the completion request.
    pub usage: CompletionUsage,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompletionChoice {
    /// The reason the model stopped generating tokens. This will be stop if the model hit a natural stop point or a provided stop sequence:
    ///     - **length** if the maximum number of tokens specified in the request was reached
    ///     - **content_filter** if content was omitted due to a flag from our content filters
    ///     - **tool_calls** if the model called a tool
    ///     - **function_call** (deprecated) if the model called a function.
    pub finish_reason: FinishReason,

    /// The index of the choice in the list of choices.
    pub index: u64,

    /// A chat completion message generated by the model.
    pub message: ChoiceMessage,

    /// Log probability information for the choice.
    pub logprobs: Option<LogProb>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FinishReason {
    /// The model hit a natural stop or a provided stop sequence.
    Stop,
    /// The model hit the maximum number of tokens.
    Length,
    /// The model hit an internal content filter omitting the content associated.
    ContentFilter,
    /// The model hit a tool call
    ToolCalls,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChoiceMessage {
    /// The content of the message.
    pub content: Option<String>,

    /// The refusal message generated by the model.
    pub refusal: Option<String>,

    /// The tool calls generated by the model, such as function calls.
    pub tool_calls: Option<Vec<ToolCall>>,

    /// The role of the author of this message.
    pub role: Role,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ToolCall {
    pub id: String,
    #[serde(rename = "type")]
    pub kind: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogProb {
    /// A list of message content tokens with log probability information.
    pub content: Option<Vec<LogProbContent>>,

    /// A list of message refusal tokens with log probability information.
    pub refusal: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogProbContent {
    pub token: String,

    /// The log probability of this token, if it is within the top 20 most likely tokens. Otherwise, the value -9999.0 is used to signify that the token is very unlikely.
    pub logprob: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompletionUsage {
    /// Number of tokens in the generated completion.
    pub completion_tokens: u64,

    /// Number of tokens in the prompt.
    pub prompt_tokens: u64,

    /// Total number of tokens used in the request (prompt + completion).
    pub total_tokens: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    /// The message content.
    pub content: String,

    /// The role of the author of this message.
    pub role: Role,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Role {
    Assistant,
    System,
    User,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateChatCompletion {
    /// The model that will complete your prompt e.g. GPT-4o
    pub model: Model,

    /// Input messages.
    pub messages: Vec<Message>,

    /// Number between -2.0 and 2.0. Positive values penalize new tokens based on their existing frequency in the text so far, decreasing the model's likelihood to repeat the same line verbatim.
    ///
    /// Defaults to **0**.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frequency_penalty: Option<f32>,

    /// Modify the likelihood of specified tokens appearing in the completion.
    ///
    /// [Note]: This is not yet supported by any of Groq models.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logit_bias: Option<serde_json::Value>,

    /// Whether to return log probabilities of the output tokens or not. If true, returns the log probabilities of each output token returned in the content of message.
    ///
    /// Defaults to **false**.
    ///
    /// [Note]: This is not yet supported by any of Groq models.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logprobs: Option<bool>,

    /// An integer between 0 and 20 specifying the number of most likely tokens to return at each token position, each with an associated log probability. **logprobs** must be set to **true** if this parameter is used.
    ///
    /// [Note]: This is not yet supported by any of Groq models.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_logprobs: Option<u8>,

    /// The maximum number of tokens that can be generated in the chat completion. The total length of input tokens and generated tokens is limited by the model's context length.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_tokens: Option<u64>,

    /// How many chat completion choices to generate for each input message. Note that you will be charged based on the number of generated tokens across all of the choices.
    ///
    /// Defaults to **1**.
    ///
    /// [Note]: Only **n=1** is suppoted by Groq supported.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub n: Option<u32>,

    /// Number between -2.0 and 2.0. Positive values penalize new tokens based on whether they appear in the text so far, increasing the model's likelihood to talk about new topics.
    ///
    /// Defaults to **0**.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub presence_penalty: Option<f32>,

    /// An object specifying the format that the model must output.
    ///
    /// Setting to `{ "type": "json_object" }` enables JSON mode, which ensures the message the model generates is valid JSON.
    ///
    /// **Important**: when using JSON mode, you must also instruct the model to produce JSON yourself via a system or user message.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_format: Option<ResponseFormat>,

    /// If specified, Groq system will make a best effort to sample deterministically, such that repeated requests with the same seed and parameters should return the same result. Determinism is not guaranteed, and you should refer to the `system_fingerprint` response parameter to monitor changes in the backend.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seed: Option<u64>,

    /// Specifies the latency tier to use for processing the request. This parameter is relevant for customers subscribed to the scale tier service:
    ///
    /// If set to **auto**, and the Project is Scale tier enabled, the system will utilize scale tier credits until they are exhausted.
    ///
    /// If set to **auto**, and the Project is not Scale tier enabled, the request will be processed using the default service tier with a lower uptime SLA and no latency guarentee.
    ///
    /// If set to **default**, the request will be processed using the default service tier with a lower uptime SLA and no latency guarentee.
    ///
    /// When not set, the default behavior is **auto**.
    ///
    /// When this parameter is set, the response body will include the `service_tier` utilized.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_tier: Option<String>,

    /// Up to 4 sequences where the API will stop generating further tokens.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop: Option<String>,

    /// If **true**, partial message deltas will be sent. Tokens will be sent as data-only server-sent events as they become available.
    ///
    /// Defaults to **false**.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream: Option<bool>,

    /// Options for streaming response. Only set this when **stream** is set to **true**.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_options: Option<StreamOptions>,

    /// What sampling temperature to use, between 0 and 2. Higher values like 0.8 will make the output more random, while lower values like 0.2 will make it more focused and deterministic.
    ///
    /// Defaults to **1**.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f32>,

    /// An alternative to sampling with temperature, called nucleus sampling, where the model considers the results of the tokens with top_p probability mass. So 0.1 means only the tokens comprising the top 10% probability mass are considered.
    ///
    /// Defaults to **1**.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_p: Option<f32>,

    /// Whether to enable parallel function calling during tool use.
    ///
    /// Defaults to **true**.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parallel_tool_calls: Option<bool>,

    /// A unique identifier representing your end-user, which can help OpenAI to monitor and detect abuse.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResponseFormat {
    #[serde(rename = "type")]
    pub kind: ResponseKind,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ResponseKind {
    Text,
    JsonObject,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct StreamOptions {
    /// If set, an additional chunk will be streamed before the **"data: DONE"** message. The usage field on this chunk shows the token usage statistics for the entire request, and the choices field will always be an empty array. All other chunks will also include a usage field, but with a null value.
    pub include_usage: Option<bool>,
}

impl CreateChatCompletion {
    pub fn new(model: Model, messages: Vec<Message>) -> Self {
        Self {
            model,
            messages,
            ..Default::default()
        }
    }

    pub fn with_model(mut self, model: Model) -> Self {
        self.model = model;
        self
    }

    pub fn with_messages(mut self, messages: Vec<Message>) -> Self {
        self.messages = messages;
        self
    }

    pub fn with_frequence_penalty(mut self, frequence_penalty: f32) -> Self {
        self.frequency_penalty = Some(frequence_penalty);
        self
    }

    pub fn with_logit_bias(mut self, logit_bias: serde_json::Value) -> Self {
        self.logit_bias = Some(logit_bias);
        self
    }

    pub fn with_logprobs(mut self, logprobs: bool) -> Self {
        self.logprobs = Some(logprobs);
        self
    }

    pub fn with_top_logprobs(mut self, top_logprobs: u8) -> Self {
        self.top_logprobs = Some(top_logprobs);
        self
    }

    pub fn with_max_tokens(mut self, max_tokens: u64) -> Self {
        self.max_tokens = Some(max_tokens);
        self
    }

    pub fn with_n(mut self, n: u32) -> Self {
        self.n = Some(n);
        self
    }

    pub fn with_presence_penalty(mut self, presence_penalty: f32) -> Self {
        self.presence_penalty = Some(presence_penalty);
        self
    }

    pub fn with_response_format(mut self, response_format: ResponseFormat) -> Self {
        self.response_format = Some(response_format);
        self
    }

    pub fn with_seed(mut self, seed: u64) -> Self {
        self.seed = Some(seed);
        self
    }

    pub fn with_service_tier(mut self, service_tier: String) -> Self {
        self.service_tier = Some(service_tier);
        self
    }

    pub fn with_stop(mut self, stop: String) -> Self {
        self.stop = Some(stop);
        self
    }

    pub fn with_stream(mut self, stream: bool) -> Self {
        self.stream = Some(stream);
        self
    }

    pub fn with_stream_options(mut self, stream_options: StreamOptions) -> Self {
        self.stream_options = Some(stream_options);
        self
    }

    pub fn with_temperature(mut self, temperature: f32) -> Self {
        if temperature < 0.0 {
            self.temperature = Some(0.0);
        } else if temperature > 2.0 {
            self.temperature = Some(2.0);
        } else {
            self.temperature = Some(temperature);
        }
        self
    }

    pub fn with_top_p(mut self, top_p: f32) -> Self {
        if top_p < 0.0 {
            self.top_p = Some(0.0);
        } else if top_p > 1.0 {
            self.top_p = Some(1.0);
        } else {
            self.top_p = Some(top_p);
        }
        self
    }

    pub fn with_parallel_tool_calls(mut self, parallel_tool_calls: bool) -> Self {
        self.parallel_tool_calls = Some(parallel_tool_calls);
        self
    }

    pub fn with_user(mut self, user: String) -> Self {
        self.user = Some(user);
        self
    }
}

impl Default for CreateChatCompletion {
    fn default() -> Self {
        Self {
            model: Model::Llama38B,
            max_tokens: Some(1000),
            messages: Vec::new(),
            n: None,
            seed: None,
            temperature: None,
            top_p: None,
            stream: None,
            stream_options: None,
            logit_bias: None,
            parallel_tool_calls: None,
            stop: None,
            user: None,
            logprobs: None,
            response_format: None,
            presence_penalty: None,
            top_logprobs: None,
            frequency_penalty: None,
            service_tier: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn should_set_stream() {
        let request = CreateChatCompletion::default();
        assert_eq!(request.stream, None);

        let request = request.with_stream(true);
        assert_eq!(request.stream, Some(true));

        let request = request.with_stream(false);
        assert_eq!(request.stream, Some(false));
    }

    #[test]
    fn should_set_temperature_between_boundaries() {
        let request = CreateChatCompletion::default();
        assert_eq!(request.temperature, None);

        let request = request.with_temperature(0.9);
        assert_eq!(request.temperature, Some(0.9));

        let request = request.with_temperature(2.9);
        assert_eq!(request.temperature, Some(2.0));

        let request = request.with_temperature(-1.1);
        assert_eq!(request.temperature, Some(0.0));
    }

    #[test]
    fn should_set_top_p_between_boundaries() {
        let request = CreateChatCompletion::default();
        assert_eq!(request.top_p, None);

        let request = request.with_top_p(0.4);
        assert_eq!(request.top_p, Some(0.4));

        let request = request.with_top_p(-0.4);
        assert_eq!(request.top_p, Some(0.0));

        let request = request.with_top_p(1.4);
        assert_eq!(request.top_p, Some(1.0));
    }
}
