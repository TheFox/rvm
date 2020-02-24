
#[derive(Debug)]
pub struct Rvm {
}

impl Rvm {
    /// New Rvm
    pub fn new() -> Self {
        println!("-> Rvm::new()");
        Self {}
    }
}

#[cfg(test)]
mod tests_rvm {
    use super::Rvm;

    #[test]
    fn test_rvm1() {
        Rvm::new();
    }
}
