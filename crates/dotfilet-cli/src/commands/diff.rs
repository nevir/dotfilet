pub fn execute(resources: Vec<String>, _verbose: bool, _dry_run: bool) {
    if resources.is_empty() {
        println!("dotfilet diff: Not yet implemented");
        println!("This command will show all pending configuration changes");
    } else {
        println!("dotfilet diff: Not yet implemented");
        println!(
            "This command will show pending changes for: {}",
            resources.join(", ")
        );
    }
}
