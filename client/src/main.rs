use reqwest::{Client, Method, Response};
use shared::question::Question;
use shared::question::QuestionId;
use shared::question::CreateQuestion;
use shared::question::GetQuestionById;
/// example code
///
/// Create a reqwest client
/// let client = Client::new();
/// Make a GET HTTP request to our backend's /example route
/// let res = client.get("http://localhost:8088/example").await?;
///
/// Get the response from backend's data
/// let body = res.text().await?;
/// Print out that response
/// println!("GET Response: {}", body);
///
/// Same as GET, but makes a POST request with appropriate header
/// let res = client
///     .post("http://localhost:8088/example")
///     .header("Content-Type", "application/json")
///     .body("Example Body")
///     .await?;
///
/// let body = res.text().await?;
/// println!("POST Response: {}", body);
///
/// You'll use these methods along with DELETE to accomplish your task
#[tokio::main]
async fn main() {
    // Create a reqwest client
    let client = Client::new();

    let question = Question {
        id: QuestionId(1),
        title: "Example Title".to_string(),
        content: "Example Content".to_string(),
        tags: Some(vec!["tag1".to_string(), "tag2".to_string()]),
    };

    let create_question = CreateQuestion {
        title: question.title.clone(),
        content: question.content.clone(),
        tags: question.tags.clone(),
    };

    let res = client.post("http://localhost:8088/questions")
        .json(&create_question)
        .send()
        .await;

    let to_delete = GetQuestionById {
        question_id: 2,
    };

   // Your code here!
}
