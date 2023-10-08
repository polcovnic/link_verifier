use super::command_interface::Command;
use reqwest;
use futures::future::join_all;

pub struct VerifyUrlLinksCommand {
    urls: Vec<String>,
}

#[derive(Debug)]
pub struct UrlLinksResult {
    pub valid_urls: Vec<String>,
    pub invalid_urls: Vec<String>,
}

impl VerifyUrlLinksCommand {
    pub fn new(urls: Vec<String>) -> Self {
        VerifyUrlLinksCommand { urls }
    }
}

impl Command for VerifyUrlLinksCommand {
    type Output = UrlLinksResult;


    fn execute(&self) -> UrlLinksResult {
        let mut valid_urls = Vec::new();
        let mut invalid_urls = Vec::new();

        async fn check_one_url(url: &str) -> (String, Result<reqwest::Response, reqwest::Error>) {
            let response = reqwest::get(url).await;
            (url.to_string(), response)
        }

        let futures: Vec<_> = self.urls.iter().map(|url| check_one_url(url)).collect();

        let results: Vec<_> = tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .unwrap()
            .block_on(async {
                join_all(futures).await
            });

        for (url, res) in results {
            match res {
                Ok(response) if response.status().is_success() => {
                    valid_urls.push(url);
                }
                _ => {
                    invalid_urls.push(url);
                }
            }
        }

        UrlLinksResult {
            valid_urls,
            invalid_urls,
        }
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validity_check() {
        let urls = vec![
            "https://www.google.com".to_string(),
            "invalid_url".to_string(),
            "not_a_url_at_all".to_string(),
        ];

        let command = VerifyUrlLinksCommand::new(urls);
        let result = command.execute();

        assert_eq!(result.valid_urls.len(), 1);
        assert_eq!(result.invalid_urls.len(), 2);

        assert!(result.valid_urls.contains(&"https://www.google.com".to_string()));
        assert!(result.invalid_urls.contains(&"invalid_url".to_string()));
        assert!(result.invalid_urls.contains(&"not_a_url_at_all".to_string()));
    }

}
