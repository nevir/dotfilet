pub fn execute(resources: Vec<String>, plan: Option<String>, _verbose: bool, _dry_run: bool) {
    match (resources.is_empty(), plan) {
        (true, None) => {
            println!("dotfilet apply: Not yet implemented");
            println!("This command will apply all configuration changes");
        }
        (false, None) => {
            println!("dotfilet apply: Not yet implemented");
            println!(
                "This command will apply changes for: {}",
                resources.join(", ")
            );
        }
        (_, Some(plan_file)) => {
            println!("dotfilet apply: Not yet implemented");
            println!(
                "This command will apply changes from plan file: {}",
                plan_file
            );
        }
    }
}
