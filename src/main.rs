use tonic::{transport::Server, Request, Response, Status};
use user_agent_analyzer::user_agent_analyzer_server::{UserAgentAnalyzer, UserAgentAnalyzerServer};
use user_agent_analyzer::{UserAgentRequest, UserAgentResponse};

pub mod user_agent_analyzer {
    tonic::include_proto!("_");
}

#[derive(Default)]
pub struct MyUserAgentAnalyzer {}

// Implement the UserAgentAnalyzer trait for the MyUserAgentAnalyzer struct.
// The analyze_user_agent method takes a Request with a UserAgentRequest as input
// and returns a Result with a Response containing a UserAgentResponse or a Status.
// The method extracts the user agent string from the request and makes a decision
// based on the content of the user agent string.
// The decision is then returned as a UserAgentResponse in the Response.
// The method returns a "block" decision if the user agent contains "Safari",
// an "allow" decision if the user agent contains "Firefox", and an "unknown"
// decision otherwise.
// The method returns an Ok response with the UserAgentResponse or an error Status.
#[tonic::async_trait]
impl UserAgentAnalyzer for MyUserAgentAnalyzer {
    async fn analyze_user_agent(
        &self,
        request: Request<UserAgentRequest>,
    ) -> Result<Response<UserAgentResponse>, Status> {
        let user_agent = request.into_inner().user_agent;

        let decision = if user_agent.contains("Safari") {
            "block".to_string()
        } else if user_agent.contains("Firefox") {
            "allow".to_string()
        } else {
            "unknown".to_string()
        };

        let response = UserAgentResponse { decision };

        Ok(Response::new(response))
    }
}

// The main function creates a new MyUserAgentAnalyzer instance and starts a gRPC server
// listening on the address "[::1]:
// 50051". The server is configured with the UserAgentAnalyzer service implemented by
// the MyUserAgentAnalyzer struct. The server is started using the serve method, which
// is an asynchronous operation that returns a Result with an empty tuple () or an error.
// The main function is marked as async to allow the use of the tokio runtime.
// The tokio::main attribute is used to run the async main function.
// The main function returns a Result with an empty tuple () or an error.
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    let user_agent_analyzer = MyUserAgentAnalyzer::default();

    Server::builder()
        .add_service(UserAgentAnalyzerServer::new(user_agent_analyzer))
        .serve(addr)
        .await?;

    Ok(())
}

// The tests module contains a test function that tests the analyze_user_agent method
// of the MyUserAgentAnalyzer struct. The test function reads user agent strings from
// a file and sends requests to the analyze_user_agent method with each user agent string.
// The test function then asserts the decision made by the method based on the content
// of the user agent string. The test function uses the assert_eq macro to compare the
// expected decision with the actual decision returned by the method.
// The test function is marked as async to allow the use of the tokio runtime.
#[cfg(test)]
mod tests {
    use crate::user_agent_analyzer::user_agent_analyzer_server::UserAgentAnalyzer;
    use crate::user_agent_analyzer::UserAgentRequest;
    use crate::MyUserAgentAnalyzer;
    use reqwest;
    use tonic::Request;

    #[tokio::test]
    async fn test_analyze_user_agent() {
        let user_agent_analyzer = MyUserAgentAnalyzer::default();

        // Fetch user agent data from the specified URL
        let url = "https://gist.githubusercontent.com/pzb/b4b6f57144aea7827ae4/raw/cf847b76a142955b1410c8bcef3aabe221a63db1/user-agents.txt";
        let response = reqwest::get(url).await.expect("Failed to fetch data");
        let data = response.text().await.expect("Failed to read response text");

        // Split the data by line and iterate through each user agent
        for line in data.lines() {
            let user_agent = line.to_string();

            let request = Request::new(UserAgentRequest {
                user_agent: user_agent.clone(),
            });

            let response = user_agent_analyzer
                .analyze_user_agent(request)
                .await
                .unwrap()
                .into_inner();

            // Make assertions based on the expected decision
            if user_agent.contains("Safari") {
                assert_eq!(response.decision, "block");
            } else if user_agent.contains("Firefox") {
                assert_eq!(response.decision, "allow");
            } else {
                assert_eq!(response.decision, "unknown");
            }
        }
    }
}
