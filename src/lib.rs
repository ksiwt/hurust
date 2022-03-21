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
    let head = repo.head().unwrap();
    let oid = head.target().unwrap();
    let commit = repo.find_commit(oid).unwrap();

    let _branch = match repo.branch(branch_name, &commit, false) {
        Ok(branch) => Some(branch),
        Err(ref e) if e.code() == ErrorCode::Exists => {
            println!("branch has already exists: {}", branch_name);
            None
        }
        Err(e) => panic!("Error: failed to open site repository: {}", e),
    };

    println!("successfuly create brach: {}", branch_name);

    Ok(())
}
