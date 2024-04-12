use tonic::Request;
use user_agent_analyzer::user_agent_analyzer_client::UserAgentAnalyzerClient;
use user_agent_analyzer::UserAgentRequest;

pub mod user_agent_analyzer {
    tonic::include_proto!("_");
}

// This function sends a request to the gRPC server to analyze the user agent
// and returns the decision made by the server.
//
// The function takes a user agent string as input and returns a Result with
// the decision as a String or an error.
//
pub async fn analyze_user_agent(user_agent: &str) -> Result<String, Box<dyn std::error::Error>> {
    let mut client = UserAgentAnalyzerClient::connect("http://[::1]:50051").await?;

    let request = Request::new(UserAgentRequest {
        user_agent: user_agent.to_string(),
    });

    let response = client.analyze_user_agent(request).await?;

    Ok(response.into_inner().decision)
}

// The main function reads the user agent string from the command line arguments
// and calls the analyze_user_agent function to get the decision.
//
// It then prints the decision to the console.
//
// If the user does not provide a user agent string as an argument, it prints
// a usage message and exits.
//
// The main function is marked as async to allow the use of the tokio runtime.
//
// The tokio::main attribute is used to run the async main function.
//
// The main function returns a Result with an empty tuple () or an error.
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <user_agent>", args[0]);
        return Ok(());
    }

    let user_agent = &args[1];
    let decision = analyze_user_agent(user_agent).await?;
    println!("Decision: {}", decision);

    Ok(())
}
