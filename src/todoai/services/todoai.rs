use crate::todoai::models::priority::Priority;
use crate::todoai::models::todo::Todo;
use async_openai::error::OpenAIError;
use async_openai::types::{
    ChatCompletionRequestSystemMessageArgs, CreateChatCompletionRequestArgs, Role,
};
use async_openai::{config::OpenAIConfig, types::ListModelResponse};
use chrono::{NaiveDate, NaiveDateTime};

#[async_trait::async_trait]
pub trait TodoAI: Send + Sync + Clone {
    async fn infer_title(&self, text: String) -> Option<String>;
    async fn infer_deadline(&self, text: String) -> Option<NaiveDateTime>;
    async fn infer_priority(&self, text: String) -> Option<Priority>;
    async fn infer_tags(&self, text: String) -> Option<String>;
    async fn split_into_todos(&self, prompt: String) -> Vec<Todo>;
    async fn classify(&self, todo: &Todo) -> Vec<String>;
}

#[derive(Clone)]
pub struct OpenAITodoAI {
    // https://github.com/64bit/async-openai/tree/main/examples/assistants
    client: async_openai::Client<OpenAIConfig>,
}

impl OpenAITodoAI {
    pub fn new(client: async_openai::Client<OpenAIConfig>) -> Self {
        Self { client }
    }

    async fn get_models(&self) -> ListModelResponse {
        self.client.models().list().await.unwrap()
    }

    async fn send_prompt(&self, prompt: String) -> Result<Option<String>, OpenAIError> {
        let request = CreateChatCompletionRequestArgs::default()
            .model("gpt-3.5-turbo")
            .messages([
                ChatCompletionRequestSystemMessageArgs::default()
                    .content(
                        r#"
                    You are a part of an Todo application. 
                    Your response must be exact so the response can be use as an API. 
                    No extra text otherwise the whole system will crash. 
                    You are responsible for keeping the system running and running well.
                "#,
                    )
                    .role(Role::System)
                    .build()?
                    .into(),
                ChatCompletionRequestSystemMessageArgs::default()
                    .content(prompt.clone())
                    .build()?
                    .into(),
            ])
            .max_tokens(512u16)
            .build()
            .unwrap();

        let response = self.client.chat().create(request).await;

        match response {
            Ok(response) => {
                println!("Prompt:\n{}", prompt);
                println!("\nResponse:\n");
                for choice in &response.choices {
                    println!(
                        "{}: Role: {}  Content: {:?}",
                        choice.index, choice.message.role, choice.message.content
                    );
                }

                Ok(response
                    .choices
                    .first()
                    .map(|first| return first.message.content.clone())
                    .flatten())
            }

            Err(OpenAIError::ApiError(err)) => {
                let code = err.code.clone();
                if code == Some(serde_json::Value::String("model_not_found".to_string())) {
                    let models = self.get_models().await;
                    println!(
                        "Models: {:?}",
                        models
                            .data
                            .into_iter()
                            .map(|d| d.id)
                            .filter(|id| id.starts_with("gpt"))
                            .collect::<Vec<String>>()
                    );
                } else {
                    println!("Error: {}", err.message);
                }

                Err(OpenAIError::ApiError(err))
            }

            Err(e) => {
                println!("Error: {}", e);
                Err(e)
            }
        }
    }

    fn parse_date(s: &String) -> Option<NaiveDateTime> {
        println!("Date received: '{}'", s);
        NaiveDate::parse_from_str(s, "%Y-%m-%d")
            .map(|d| d.and_hms_opt(0, 0, 0))
            .unwrap()
    }
}

// test date parser:
#[tokio::test]
async fn test_parse_date() {
    let date = OpenAITodoAI::parse_date(&"2021-01-01".to_string());
    assert_eq!(date.unwrap().to_string(), "2021-01-01 00:00:00".to_string());
}

#[async_trait::async_trait]
impl TodoAI for OpenAITodoAI {
    async fn infer_title(&self, text: String) -> Option<String> {
        let prompt = format!(
            r#"
        You are given a description of a task and you need to infer the title of the task.

        Description: "{}"

        Only respond with the title and nothing else.
        "#,
            text
        );

        self.send_prompt(prompt).await.unwrap()
    }

    async fn infer_deadline(&self, text: String) -> Option<NaiveDateTime> {
        let prompt = format!(
            r#"
        You are given a description of a task and you need to infer the deadline of the task.
        You are given today's date and must estimate how long it will take to complete the task. Add a few days to this estimation and return the date.
        Here is today's date: {}
        Here is the description: "{}"
        Respond with a date in the format: YYYY-MM-DD
        "#,
            chrono::Local::now().naive_local().date(),
            text
        );

        self.send_prompt(prompt)
            .await
            .unwrap()
            .map(|s| OpenAITodoAI::parse_date(&s))
            .flatten()
    }

    async fn infer_priority(&self, text: String) -> Option<Priority> {
        let prompt = format!(
            r#"
        You are given a description of a task and you need to infer the priority of the task.
        The options are: Low, Medium, High
        Only respond with the priority and nothing else.
        Here is the description: "{}"
        "#,
            text
        );

        self.send_prompt(prompt)
            .await
            .unwrap()
            .map(|s| Priority::from(s))
    }

    async fn infer_tags(&self, text: String) -> Option<String> {
        let prompt = format!(
            r#"
        You are given a description of a task and you need to infer a tag to classify the task.
        Only respond with the tag and nothing else.
        Here is the description: "{}"
        "#,
            text
        );

        self.send_prompt(prompt).await.unwrap()
    }

    async fn split_into_todos(&self, _prompt: String) -> Vec<Todo> {
        vec![]
    }

    async fn classify(&self, _todo: &Todo) -> Vec<String> {
        vec![]
    }
}
