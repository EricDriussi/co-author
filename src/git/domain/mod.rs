use std::fmt::Display;

pub struct CommitBody {
    message: String,
    signatures: Vec<String>,
}

impl CommitBody {
    pub fn new(message: &str, signatures: Vec<String>) -> Self {
        Self {
            message: String::from(message),
            signatures,
        }
    }
}

impl Display for CommitBody {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}\n\n{}", self.message, self.signatures.join("\n"))
    }
}

pub trait GitRepo {
    fn commit(&self);
}

#[cfg(test)]
mod test;
