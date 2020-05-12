use noodles_gff as gff;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Feature {
    reference_name: String,
    start: u64,
    end: u64,
    strand: gff::Strand,
}

impl Feature {
    pub fn new(reference_name: String, start: u64, end: u64, strand: gff::Strand) -> Self {
        Self {
            reference_name,
            start,
            end,
            strand,
        }
    }

    pub fn reference_name(&self) -> &str {
        &self.reference_name
    }

    pub fn start(&self) -> u64 {
        self.start
    }

    pub fn end(&self) -> u64 {
        self.end
    }

    pub fn end_mut(&mut self) -> &mut u64 {
        &mut self.end
    }

    pub fn strand(&self) -> gff::Strand {
        self.strand
    }

    pub fn len(&self) -> u64 {
        self.end - self.start + 1
    }

    pub fn is_empty(&self) -> bool {
        self.start == self.end
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn build_feature() -> Feature {
        Feature::new(String::from("sq0"), 8, 13, gff::Strand::Forward)
    }

    #[test]
    fn test_reference_name() {
        let feature = build_feature();
        assert_eq!(feature.reference_name(), "sq0");
    }

    #[test]
    fn test_start() {
        let feature = build_feature();
        assert_eq!(feature.start(), 8);
    }

    #[test]
    fn test_end() {
        let feature = build_feature();
        assert_eq!(feature.end(), 13);
    }

    #[test]
    fn test_strand() {
        let feature = build_feature();
        assert_eq!(feature.strand(), gff::Strand::Forward);
    }

    #[test]
    fn test_len() {
        let feature = build_feature();
        assert_eq!(feature.len(), 6);
    }

    #[test]
    fn test_is_empty() {
        let feature = Feature::new(String::from("sq0"), 1, 1, gff::Strand::Forward);
        assert!(feature.is_empty());

        let feature = build_feature();
        assert!(!feature.is_empty());
    }
}
