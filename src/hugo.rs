use std::process::Command;

pub fn create_post(title: &str) {
    Command::new("hugo")
        .arg("new")
        .arg("post/".to_owned() + title)
        .spawn()
        .expect("Error: Failed to run hugo new command");
}

#[test]
fn test_post() {
    assert_eq!(create_post("test"), ());
}
