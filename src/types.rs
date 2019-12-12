use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "basic")]
pub struct Opts {
    #[structopt(short = "k", long, name = "BitBucket Project key")]
    pub bit_bucket_project_key: String,

    #[structopt(short = "p", long)]
    pub git_ssh_password: Option<String>,
    #[structopt(short = "s", long, name = "BitBucket server base url, http://example.bitbucket.mycompany.com")]
    pub bit_bucket_server: String,
    #[structopt(short = "u", long)]
    pub bit_bucket_username: Option<String>,
    #[structopt(short = "w", long)]
    pub bit_bucket_password: Option<String>,
}

impl Opts {
    pub fn bit_bucket_url(&self) -> String {
        let host = self.bit_bucket_server.to_string();
        let key = self.bit_bucket_project_key.to_string();
        return format!("{host}/rest/api/latest/projects/{key}/repos?limit=5000", host = host, key = key);
    }
}

#[derive(Deserialize, Debug)]
pub struct Projects {
    pub values: Vec<Project>,
}

impl Projects {
    pub fn get_clone_links(&self) -> Vec<Repo> {
        let mut links: Vec<Repo> = Vec::new();
        for value in &self.values {
            for cloneLink in &value.links.clone {
                if cloneLink.name.trim() == "ssh" {
                    links.push(Repo { git: cloneLink.href.clone(), name: value.slug.clone() });
                }
            }
        }
        return links;
    }
}

#[derive(Deserialize, Debug)]
pub struct Project {
    pub slug: String,
    pub links: Links,
}

#[derive(Deserialize, Debug)]
pub struct Links {
    pub clone: Vec<CloneLink>,
}

#[derive(Deserialize, Debug)]
pub struct CloneLink {
    pub name: String,
    pub href: String,
}

#[derive(Debug)]
pub struct Repo {
    pub git: String,
    pub name: String,
}