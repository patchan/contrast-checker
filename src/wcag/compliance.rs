#[derive(Debug, PartialEq)]
pub enum WcagCompliance {
    Aaa,
    Aa,
    Fail,
}

impl WcagCompliance {
    pub fn from_ratio(ratio: f32) -> Self {
        match ratio {
            r if r >= 7.0 => WcagCompliance::Aaa,
            r if r >= 4.5 => WcagCompliance::Aa,
            _ => WcagCompliance::Fail,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ratio_above_7_is_aaa() {
        assert_eq!(WcagCompliance::from_ratio(7.0), WcagCompliance::Aaa);
    }

    #[test]
    fn test_ratio_well_above_7_is_aaa() {
        assert_eq!(WcagCompliance::from_ratio(21.0), WcagCompliance::Aaa);
    }

    #[test]
    fn test_boundary_between_aa_and_aaa() {
        assert_eq!(WcagCompliance::from_ratio(6.99), WcagCompliance::Aa);
    }

    #[test]
    fn test_ratio_above_4_5_is_aa() {
        assert_eq!(WcagCompliance::from_ratio(4.5), WcagCompliance::Aa);
    }

    #[test]
    fn test_ratio_between_aa_and_aaa_is_aa() {
        assert_eq!(WcagCompliance::from_ratio(5.5), WcagCompliance::Aa);
    }

    #[test]
    fn test_ratio_below_4_5_is_fail() {
        assert_eq!(WcagCompliance::from_ratio(4.49), WcagCompliance::Fail);
    }

    #[test]
    fn test_ratio_of_1_is_fail() {
        assert_eq!(WcagCompliance::from_ratio(1.0), WcagCompliance::Fail);
    }
}