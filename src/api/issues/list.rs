use super::*;
use crate::params;

#[derive(serde::Serialize)]
pub struct ListIssuesBuilder<'octo, 'b, 'c, 'd> {
    #[serde(skip)]
    handler: &'b IssueHandler<'octo>,
    state: Option<params::State>,
    milestone: Option<params::issues::Filter<u64>>,
    assignee: Option<params::issues::Filter<&'c str>>,
    creator: Option<String>,
    mentioned: Option<String>,
    labels: Option<&'d [String]>,
    sort: Option<crate::params::issues::Sort>,
    direction: Option<crate::params::Direction>,
    per_page: Option<u8>,
    page: Option<u32>,
}

impl<'octo, 'b, 'c, 'd> ListIssuesBuilder<'octo, 'b, 'c, 'd> {
    pub(crate) fn new(handler: &'b IssueHandler<'octo>) -> Self {
        Self {
            handler,
            state: None,
            milestone: None,
            assignee: None,
            creator: None,
            mentioned: None,
            labels: None,
            sort: None,
            direction: None,
            per_page: None,
            page: None,
        }
    }

    /// If an integer is passed, it should refer to a milestone by its number
    /// field. If the string `"*"` is passed, issues with any milestone are
    /// accepted. If the string none is passed, issues without milestones
    /// are returned.
    pub fn milestone(mut self, milestone: impl Into<params::issues::Filter<u64>>) -> Self {
        self.milestone = Some(milestone.into());
        self
    }

    /// Filter by assignee, can be the name of a user. Pass in the string
    /// `"none"` for issues with no assigned user, and `"*"` for issues assigned
    /// to any user.
    pub fn assignee(mut self, assignee: impl Into<params::issues::Filter<&'c str>>) -> Self {
        self.assignee = Some(assignee.into());
        self
    }

    /// Filter by the creator of the issue.
    pub fn creator(mut self, creator: impl Into<String>) -> Self {
        self.creator = Some(creator.into());
        self
    }

    /// Filter by the creator of the issue.
    pub fn mentioned(mut self, mentioned: impl Into<String>) -> Self {
        self.mentioned = Some(mentioned.into());
        self
    }

    /// Filter pull requests by `state`.
    pub fn state(mut self, state: crate::params::State) -> Self {
        self.state = Some(state);
        self
    }

    /// Filter issues by label.
    pub fn labels(mut self, labels: &'d (impl AsRef<[String]> + ?Sized)) -> Self {
        self.labels = Some(labels.as_ref());
        self
    }

    /// What to sort results by. Can be either `created`, `updated`,
    /// `popularity` (comment count) or `long-running` (age, filtering by pulls
    /// updated in the last month).
    pub fn sort(mut self, sort: impl Into<crate::params::issues::Sort>) -> Self {
        self.sort = Some(sort.into());
        self
    }

    /// The direction of the sort. Can be either ascending or descending.
    /// Default: descending when sort is `created` or sort is not specified,
    /// otherwise ascending sort.
    pub fn direction(mut self, direction: impl Into<crate::params::Direction>) -> Self {
        self.direction = Some(direction.into());
        self
    }

    /// Results per page (max 100).
    pub fn per_page(mut self, per_page: impl Into<u8>) -> Self {
        self.per_page = Some(per_page.into());
        self
    }

    /// Page number of the results to fetch.
    pub fn page(mut self, page: impl Into<u32>) -> Self {
        self.page = Some(page.into());
        self
    }

    /// Sends the actual request.
    pub async fn send(self) -> crate::Result<crate::Page<crate::models::Issue>> {
        let url = format!(
            "/repos/{owner}/{repo}/issues",
            owner = self.handler.owner,
            repo = self.handler.repo
        );
        self.handler.crab.get(url, Some(&self)).await
    }
}

#[cfg(test)]
mod tests {
    #[tokio::test]
    async fn serialize() {
        let octocrab = crate::Octocrab::default();
        let handler = octocrab.issues("rust-lang", "rust");
        let labels = vec![
            String::from("help wanted"),
            String::from("good first issue"),
        ];
        let list = handler
            .list()
            .state(crate::params::State::Open)
            .milestone(1234)
            .assignee("ferris")
            .creator("octocrab")
            .mentioned("octocat")
            .labels(&labels)
            .sort(crate::params::issues::Sort::Comments)
            .direction(crate::params::Direction::Ascending)
            .per_page(100)
            .page(1u8);

        assert_eq!(
            serde_json::to_value(list).unwrap(),
            serde_json::json!({
                "state": "open",
                "milestone": 1234,
                "assignee": "ferris",
                "creator": "octocrab",
                "mentioned": "octocat",
                "labels": ["help wanted", "good first issue"],
                "sort": "comments",
                "direction": "asc",
                "per_page": 100,
                "page": 1,
            })
        )
    }
}
