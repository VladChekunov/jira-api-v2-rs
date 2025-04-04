# Jira Cloud API v2 client for Rust generated by openapi 

If you are looking for API v3, you can get it [here](https://crates.io/crates/jira). 

# Usage
Firstly you have to make `Configuration` object like that: 
```rust
let config = Configuration {
    base_path: jira_base_url.to_string(),
    user_agent: Some("jira-rust/0.0.1".to_owned()),
    client: client,
    oauth_access_token: Some(jira_personal_access_token),
    basic_auth: None,
    bearer_access_token: None,
    api_key: None,
};
```
Then you have to call any of API methods by putting config there, like this:

```rust
let search_results: SearchResults = search_for_issues_using_jql(
    &config,
    Some(&jql),
    None,
    None,
    None,
    None,
    None,
    None,
    None,
)
.await?;
```

Finally, process your results, like this:
```rust
match search_results.issues {
    Some(issue_list) => {
        for issue in issue_list {
            println!("{:?}", issue);
        }
    }

    None => {
        println!("No Issues found.");
    }
}
```

You can find out more info from [generated docs](https://github.com/VladChekunov/jira-api-v2-rs/blob/main/README.md). 
