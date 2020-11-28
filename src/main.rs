use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "dots")]
enum Dots {
    /// Install new system packages
    Install,

    /// Upgrade all or specified system packages
    Upgrade {
        packages: Vec<String>,
    },

    /// Run post-install setup for some packages
    Setup {
        packages: Vec<String>,
    },

    /// Dump installed packages to list
    Dump,

    /// Apply a pre-made patch
    Patch {
        patch_files: Vec<String>,
    },
    
    /// Open a dotfile in an editor
    Open {
        dotfile: String,
    },

    /// Open the repo in the browser
    Web,
}

fn main() {
    let matches = Dots::from_args();

    println!("{:?}", matches);
}
