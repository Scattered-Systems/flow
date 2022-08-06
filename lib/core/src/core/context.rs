/*
    Appellation: context <module>
    Creator: FL03 <jo3mccain@icloud.com>
    Description:
        ... Summary ...
*/

pub trait ContextInterface<Cnf> {
    fn builder(&self, settings: Cnf) -> Result<Self, scsys::BoxError>
        where
            Self: Sized;
}

#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        let f = |x: usize, y: usize| x + y;
        assert_eq!(f(4, 2), 6)
    }
}
