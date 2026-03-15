use crate::error::{AppError, Result};
use crate::github::auth::Auth;
use crate::github::models::{Project, Ticket, TicketDetail};
use reqwest::Client;
use serde::{Deserialize, Serialize};

pub struct GitHubClient {
    client: Client,
    auth: Auth,
    api_url: String,
}

#[derive(Debug, Serialize)]
struct GraphQLRequest {
    query: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    variables: Option<serde_json::Value>,
}

#[derive(Debug, Deserialize)]
struct GraphQLResponse<T> {
    data: Option<T>,
    errors: Option<Vec<GraphQLError>>,
}

#[derive(Debug, Deserialize)]
struct GraphQLError {
    message: String,
}

impl GitHubClient {
    pub fn new(auth: Auth, api_url: Option<String>) -> Result<Self> {
        auth.validate()?;

        let client = Client::builder()
            .user_agent("project-tui/0.1.0")
            .build()
            .map_err(|e| AppError::Network(e))?;

        Ok(Self {
            client,
            auth,
            api_url: api_url.unwrap_or_else(|| "https://api.github.com/graphql".to_string()),
        })
    }

    async fn execute_query<T>(&self, query: &str, variables: Option<serde_json::Value>) -> Result<T>
    where
        T: serde::de::DeserializeOwned,
    {
        let request = GraphQLRequest {
            query: query.to_string(),
            variables,
        };

        let response = self
            .client
            .post(&self.api_url)
            .header("Authorization", format!("Bearer {}", self.auth.token()))
            .json(&request)
            .send()
            .await
            .map_err(|e| AppError::Network(e))?;

        if !response.status().is_success() {
            return Err(AppError::GitHub(format!(
                "API request failed with status: {}",
                response.status()
            )));
        }

        let graphql_response: GraphQLResponse<T> = response
            .json()
            .await
            .map_err(|e| AppError::Network(e))?;

        if let Some(errors) = graphql_response.errors {
            let error_messages: Vec<String> = errors.iter().map(|e| e.message.clone()).collect();
            return Err(AppError::GitHub(format!(
                "GraphQL errors: {}",
                error_messages.join(", ")
            )));
        }

        graphql_response.data.ok_or_else(|| AppError::GitHub("No data in response".to_string()))
    }

    pub async fn fetch_projects(&self) -> Result<Vec<Project>> {
        #[derive(Debug, Deserialize)]
        struct Data {
            viewer: Viewer,
        }

        #[derive(Debug, Deserialize)]
        struct Viewer {
            #[serde(rename = "projectsV2")]
            projects_v2: ProjectsConnection,
        }

        #[derive(Debug, Deserialize)]
        struct ProjectsConnection {
            nodes: Vec<ProjectNode>,
        }

        #[derive(Debug, Deserialize)]
        struct ProjectNode {
            id: String,
            title: String,
            number: i64,
        }

        let query = r#"
            query {
                viewer {
                    projectsV2(first: 100) {
                        nodes {
                            id
                            title
                            number
                        }
                    }
                }
            }
        "#;

        let data: Data = self.execute_query(query, None).await?;

        let projects = data
            .viewer
            .projects_v2
            .nodes
            .into_iter()
            .map(|node| Project {
                id: node.id,
                title: node.title,
                number: node.number,
            })
            .collect();

        Ok(projects)
    }

    pub async fn fetch_project_items(&self, project_id: &str) -> Result<Vec<Ticket>> {
        #[derive(Debug, Deserialize)]
        struct Data {
            node: Option<ProjectNode>,
        }

        #[derive(Debug, Deserialize)]
        struct ProjectNode {
            items: ItemsConnection,
        }

        #[derive(Debug, Deserialize)]
        struct ItemsConnection {
            nodes: Vec<ItemNode>,
        }

        #[derive(Debug, Deserialize)]
        struct ItemNode {
            id: String,
            content: Option<Content>,
            #[serde(rename = "fieldValues")]
            field_values: FieldValuesConnection,
        }

        #[derive(Debug, Deserialize)]
        #[serde(tag = "__typename")]
        enum Content {
            Issue(IssueContent),
            PullRequest(PullRequestContent),
        }

        #[derive(Debug, Deserialize)]
        struct IssueContent {
            id: String,
            title: String,
            number: i64,
            state: String,
            repository: Repository,
        }

        #[derive(Debug, Deserialize)]
        struct PullRequestContent {
            id: String,
            title: String,
            number: i64,
            state: String,
            repository: Repository,
        }

        #[derive(Debug, Deserialize)]
        struct Repository {
            name: String,
        }

        #[derive(Debug, Deserialize)]
        struct FieldValuesConnection {
            nodes: Vec<FieldValue>,
        }

        #[derive(Debug, Deserialize)]
        #[serde(tag = "__typename")]
        enum FieldValue {
            ProjectV2ItemFieldSingleSelectValue {
                name: String,
            },
            #[serde(other)]
            Other,
        }

        let query = r#"
            query($projectId: ID!) {
                node(id: $projectId) {
                    ... on ProjectV2 {
                        items(first: 100) {
                            nodes {
                                id
                                content {
                                    __typename
                                    ... on Issue {
                                        id
                                        title
                                        number
                                        state
                                        repository {
                                            name
                                        }
                                    }
                                    ... on PullRequest {
                                        id
                                        title
                                        number
                                        state
                                        repository {
                                            name
                                        }
                                    }
                                }
                                fieldValues(first: 10) {
                                    nodes {
                                        __typename
                                        ... on ProjectV2ItemFieldSingleSelectValue {
                                            name
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        "#;

        let variables = serde_json::json!({
            "projectId": project_id
        });

        let data: Data = self.execute_query(query, Some(variables)).await?;

        let node = data.node.ok_or_else(|| AppError::GitHub("Project not found".to_string()))?;

        let tickets = node
            .items
            .nodes
            .into_iter()
            .filter_map(|item| {
                let content = item.content?;
                let (id, title, number, state, repository) = match content {
                    Content::Issue(issue) => (
                        issue.id,
                        issue.title,
                        issue.number,
                        issue.state,
                        issue.repository.name,
                    ),
                    Content::PullRequest(pr) => {
                        (pr.id, pr.title, pr.number, pr.state, pr.repository.name)
                    }
                };

                let status = item
                    .field_values
                    .nodes
                    .into_iter()
                    .find_map(|fv| {
                        if let FieldValue::ProjectV2ItemFieldSingleSelectValue { name } = fv {
                            Some(name)
                        } else {
                            None
                        }
                    });

                Some(Ticket {
                    id,
                    title,
                    number,
                    status,
                    repository: Some(repository),
                    state,
                })
            })
            .collect();

        Ok(tickets)
    }

    pub async fn fetch_item_details(&self, item_id: &str) -> Result<TicketDetail> {
        // For now, we'll use a simplified approach
        // In a full implementation, this would fetch more detailed information
        let _ = item_id;
        Err(AppError::GitHub("Not implemented yet".to_string()))
    }
}
