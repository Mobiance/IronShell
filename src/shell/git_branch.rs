use git2::Repository;

pub fn get_current_git_branch() -> Option<String> {
    let repo = match Repository::discover(".") {
        Ok(repo) => repo,
        Err(_) => return None,
    };

    let head = match repo.head() {
        Ok(head) => head,
        Err(_) => return None,
    };

    let branch = match head.shorthand() {
        Some(branch) => branch,
        None => return None,
    };

    Some(branch.to_string())
}

