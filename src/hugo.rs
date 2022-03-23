use std::process::Command;

pub fn create_post(title: &str) {
    Command::new("hugo")
        .arg("new")
        .arg("post/".to_owned() + title)
        .spawn()
        .expect("Error: Failed to run hugo new command");
}

#[cfg(test)]
mod test {
    use super::create_post;

    #[test]
    fn test_create_post() {
        create_post("test");
    }
}
