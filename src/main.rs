extern crate git2;
extern crate webbrowser;

use git2::Repository;

fn open(url: &str) {
    let modified = url.replace(":", "/")
        .replace("git@", "https://")
        .replace(".git", "");

    webbrowser::open(&modified).expect("Could not open browser");
}

fn main() {
    if let Ok(repo) = Repository::open(".") {
        if let Ok(remote) = repo.find_remote("origin") {
            open(remote.url().unwrap())
        } else {
            println!("no origin set")
        }
    } else {
        println!("Not in a git repository")
    }
}
