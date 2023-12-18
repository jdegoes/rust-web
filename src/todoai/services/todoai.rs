use crate::todoai::models::priority::Priority;
use crate::todoai::models::todo::Todo;
use async_openai::{config::OpenAIConfig, types::CreateCompletionRequestArgs};
use chrono::NaiveDateTime;

#[async_trait::async_trait]
trait TodoAI {
    async fn infer_title(&self, text: String) -> Option<String>;
    async fn infer_deadline(&self, todo: &Todo) -> Option<NaiveDateTime>;
    async fn infer_priority(&self, todo: &Todo) -> Option<Priority>;
    async fn split_into_todos(&self, prompt: String) -> Vec<Todo>;
    async fn classify(&self, todo: &Todo) -> Vec<String>;
}

struct TodoOpenAIImpl {
    // https://github.com/64bit/async-openai/tree/main/examples/assistants
    client: async_openai::Client<OpenAIConfig>,
}

impl TodoOpenAIImpl {
    pub fn new(client: async_openai::Client<OpenAIConfig>) -> Self {
        Self { client }
    }

    async fn send_prompt(&self, prompt: String) -> Option<String> {
        let request = CreateCompletionRequestArgs::default()
            .model("gpt-3.5")
            .prompt(prompt)
            .max_tokens(40_u16)
            .build()
            .unwrap();

        let response = self.client.completions().create(request).await.unwrap();

        response
            .choices
            .first()
            .map(|first| return first.text.clone())
    }
}

#[async_trait::async_trait]
impl TodoAI for TodoOpenAIImpl {
    async fn infer_title(&self, text: String) -> Option<String> {
        let prompt = format!(
            r#"
        You are a part of an Todo application. You are given a description of a task and you need to infer the title of the task.

        Description: "{}"

        Only respond with the title and nothing else.
        "#,
            text
        );

        self.send_prompt(prompt).await
    }

    async fn infer_deadline(&self, todo: &Todo) -> Option<NaiveDateTime> {
        let prompt = format!(
            r#"
        You are a part of an Todo application. You are given a description of a task and you need to infer the deadline of the task.
        Here is today's date: {}
        Here is the description: "{}"
        Respond with a date in the format: YYYY-MM-DD
        "#,
            chrono::Local::now().naive_local().date(),
            todo.description
        );

        self.send_prompt(prompt)
            .await
            .map(|s| {
                NaiveDateTime::parse_from_str(&s, "%Y-%m-%d")
                    .unwrap()
                    .date()
                    .and_hms_opt(0, 0, 0)
            })
            .flatten()
    }

    async fn infer_priority(&self, todo: &Todo) -> Option<Priority> {
        let prompt = format!(
            r#"
        You are a part of an Todo application. You are given a description of a task and you need to infer the priority of the task.
        The options are: Low, Medium, High
        Only respond with the priority and nothing else.
        Here is the description: "{}"
        "#,
            todo.description
        );

        self.send_prompt(prompt).await.map(|s| Priority::from(s))
    }

    async fn split_into_todos(&self, _prompt: String) -> Vec<Todo> {
        vec![]
    }

    async fn classify(&self, _todo: &Todo) -> Vec<String> {
        vec![]
    }
}
