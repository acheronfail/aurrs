use std::fmt;

use anyhow::Result;

#[derive(Debug)]
pub struct AurPackage {
    pub name: String,
    pub voted: bool,
    pub token: Option<String>,
    pub meta: Option<raur::Package>,
}

impl AurPackage {
    pub fn new(
        name: impl AsRef<str>,
        token: Option<String>,
        voted: bool,
        fetch_meta: bool,
    ) -> Result<AurPackage> {
        Ok(AurPackage {
            name: name.as_ref().to_owned(),
            token: token,
            meta: if fetch_meta {
                Some(raur::info(&[name.as_ref()])?.remove(0))
            } else {
                None
            },
            voted,
        })
    }
}

impl fmt::Display for AurPackage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let w = 16;
        if let Some(m) = self.meta.as_ref() {
            let n = &String::from("<none>");
            let j = ", ";

            writeln!(f, "{:w$}: {}", "Name", m.name, w = w)?;
            writeln!(f, "{:w$}: {}", "Id", m.id, w = w)?;
            writeln!(f, "{:w$}: {}", "Package base id", m.package_base_id, w = w)?;
            writeln!(f, "{:w$}: {}", "Package base", m.package_base, w = w)?;
            writeln!(f, "{:w$}: {}", "Version", m.version, w = w)?;
            let description = m.description.as_ref().unwrap_or(n);
            writeln!(f, "{:w$}: {}", "Description", description, w = w)?;
            let url = m.url.as_ref().unwrap_or(n);
            writeln!(f, "{:w$}: {}", "Url", url, w = w)?;
            writeln!(f, "{:w$}: {}", "Num votes", m.num_votes, w = w)?;
            writeln!(f, "{:w$}: {}", "Popularity", m.popularity, w = w)?;
            // TODO: convert to date string
            let out_of_date = match m.out_of_date {
                Some(num) => num.to_string(),
                None => "No".to_string(),
            };
            writeln!(f, "{:w$}: {}", "Out of date", out_of_date, w = w)?;
            let maintainer = m.maintainer.as_ref().unwrap_or(n);
            writeln!(f, "{:w$}: {}", "Maintainer", maintainer, w = w)?;
            // TODO: convert to date string
            writeln!(f, "{:w$}: {}", "First submitted", m.first_submitted, w = w)?;
            // TODO: convert to date string
            writeln!(f, "{:w$}: {}", "Last modified", m.last_modified, w = w)?;
            writeln!(f, "{:w$}: {}", "Url path", m.url_path, w = w)?;
            writeln!(f, "{:w$}: {}", "Groups", m.groups.join(j), w = w)?;
            writeln!(f, "{:w$}: {}", "Depends", m.depends.join(j), w = w)?;
            writeln!(
                f,
                "{:w$}: {}",
                "Make depends",
                m.make_depends.join(j),
                w = w
            )?;
            writeln!(f, "{:w$}: {}", "Opt depends", m.opt_depends.join(j), w = w)?;
            writeln!(
                f,
                "{:w$}: {}",
                "Check depends",
                m.check_depends.join(j),
                w = w
            )?;
            writeln!(f, "{:w$}: {}", "Conflicts", m.conflicts.join(j), w = w)?;
            writeln!(f, "{:w$}: {}", "Replaces", m.replaces.join(j), w = w)?;
            writeln!(f, "{:w$}: {}", "Provides", m.provides.join(j), w = w)?;
            writeln!(f, "{:w$}: {}", "License", m.license.join(j), w = w)?;
            writeln!(f, "{:w$}: {}", "Keywords", m.keywords.join(j), w = w)?;
        } else {
            writeln!(f, "{:w$}: {}", "Name", self.name, w = w)?;
        }

        writeln!(f, "{:w$}: {}", "Voted", self.voted, w = w)?;

        Ok(())
    }
}
