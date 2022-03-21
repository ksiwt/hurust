use git2::{Error, ErrorCode, Repository};

pub fn run(title: &str) -> Result<(), Error> {
    let repo = match Repository::open(".") {
        Ok(repo) => repo,
        Err(e) => return Err(e),
    };

    create_branch(&repo, &title).unwrap_err();

    println!("{:?}", title);
    println!("{:?}", repo.path().to_str());

    return Ok(());
}

fn create_branch(repo: &Repository, branch_name: &str) -> Result<(), Error> {
    let head = match repo.head() {
        Ok(head) => Some(head),
        Err(ref e) if e.code() == ErrorCode::UnbornBranch || e.code() == ErrorCode::NotFound => {
            None
        }
        Err(e) => return Err(e),
    };

    let head = repo.head().unwrap();
    let oid = head.target().unwrap();
    let commit = repo.find_commit(oid).unwrap();

    let branch = match repo.branch(branch_name, &commit, false) {
        Ok(branch) => Some(branch),
        Err(ref e) if e.code() == ErrorCode::Exists => None,
        Err(e) => return Err(e),
    };

    return Ok(());
}
