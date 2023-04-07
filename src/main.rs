fn main() {
    use slack_api::{
    chat::{PostMessageRequest, PostMessageResponse},
    dialog::{Dialog, Element, SelectElementOption},
    oauth::{OAuthRequest, OAuthResponse},
    WebClient,
};
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Set up the Slack API client
    let slack_token = env::var("SLACK_TOKEN").expect("SLACK_TOKEN not set in environment variables");
    let slack_client = WebClient::new(slack_token);

    // Set up the Jira API client
    let jira_token = env::var("JIRA_TOKEN").expect("JIRA_TOKEN not set in environment variables");
    let jira_client = reqwest::Client::new();
    let jira_url = "https://your-jira-url.com/rest/api/2/issue/";

    // Set up the Slack dialog
    let dialog = Dialog {
        callback_id: "create-bug".to_string(),
        title: "Submit a bug".to_string(),
        submit_label: "Submit Bug!".to_string(),
        elements: vec![
            Element::Text {
                label: "What is the issue you're having?".to_string(),
                name: "issue_summary".to_string(),
                placeholder: Some("Briefly describe the issue you're having".to_string()),
                value: None,
                hint: Some("Please be as descriptive as possible".to_string()),
                max_length: Some(100),
                min_length: None,
            },
            Element::Text {
                label: "What environment are you on?".to_string(),
                name: "environment".to_string(),
                placeholder: Some("e.g. production, staging, local".to_string()),
                value: None,
                hint: Some("Please specify the environment where the issue is occurring".to_string()),
                max_length: Some(50),
                min_length: None,
            },
            Element::Select {
                label: "What is your account name?".to_string(),
                name: "account_name".to_string(),
                placeholder: Some("Select an account name".to_string()),
                value: None,
                hint: Some("Please select your account name from the dropdown".to_string()),
                options: vec![
                    SelectElementOption {
                        label: "Alice".to_string(),
                        value: "Alice".to_string(),
                    },
                    SelectElementOption {
                        label: "Bob".to_string(),
                        value: "Bob".to_string(),
                    },
                    SelectElementOption {
                        label: "Charlie".to_string(),
                        value: "Charlie".to_string(),
                    },
                ],
            },
        ],
    };

    // Set up the Slack message to be posted with the dialog trigger
    let channel = env::var("SLACK_CHANNEL").expect("SLACK_CHANNEL not set in environment variables");
    let text = "Click the button below to submit a bug";
    let mut attachments = vec![];
    let mut actions = vec![slack_api::BlockElement::Button {
        action_id: "create-bug-dialog".to_string(),
        text: "Submit a bug".to_string(),
        value: None,
        url: None,
    }];
    attachments.push(slack_api::Attachment {
        text: None,
        fallback: None,
        callback_id: None,
        color: None,
        pretext: None,
        author_name: None,
        author_link: None,
        author_icon: None,
        title: None,
        title_link: None,
        fields: None,

    }}}
