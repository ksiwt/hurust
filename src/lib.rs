use git2::{Error, ErrorCode, Repository};

pub fn run(title: &str) -> Result<(), Error> {
    let repo = Repository::open(".")?;

    create_branch(&repo, title)?;

    checkout_branch(&repo, title)?;

    Ok(())
}

fn create_branch(repo: &Repository, branch_name: &str) -> Result<(), Error> {
    let head = repo.head()?;
    let oid = head.target().unwrap();
    let commit = repo.find_commit(oid)?;

    match repo.branch(branch_name, &commit, false) {
        Ok(_) => (),
        Err(ref e) if e.code() == ErrorCode::Exists => {
            println!("branch has already exists: {}", branch_name);
            return Ok(());
        }
        Err(e) => return Err(e),
    };

    println!("successfuly create brach: {}", branch_name);

    Ok(())
}

fn checkout_branch(repo: &Repository, branch_name: &str) -> Result<(), Error> {
    let obj = repo.revparse_single(&("refs/heads/".to_owned() + branch_name))?;

    repo.checkout_tree(&obj, None)?;

    repo.set_head(&("refs/heads/".to_owned() + branch_name))?;

    println!("successfuly checkout brach: {}", branch_name);

    Ok(())
}
