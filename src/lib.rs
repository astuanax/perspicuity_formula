use regex::Regex;

pub struct PerspicuityFormula;

impl PerspicuityFormula {
    pub fn new() -> Self {
        PerspicuityFormula
    }

    pub fn calculate(&self, text: &str) -> f64 {
        let sentence_count = self.count_sentences(text);
        let word_count = self.count_words(text);
        let syllable_count = self.count_syllables(text);

        if word_count == 0 || sentence_count == 0 {
            return 0.0;
        }

        let words_per_sentence = word_count as f64 / sentence_count as f64;
        let syllables_per_word = syllable_count as f64 / word_count as f64;

        // Perspicuity Formula
        let perspicuity_score = 206.84 - (0.60 * words_per_sentence) - (1.02 * syllables_per_word);

        perspicuity_score.clamp(0.0, 100.0)
    }

    fn count_sentences(&self, text: &str) -> usize {
        let re = Regex::new(r"[.!?]").unwrap();
        re.find_iter(text).count()
    }

    fn count_words(&self, text: &str) -> usize {
        let re = Regex::new(r"\b\w+\b").unwrap();
        re.find_iter(text).count()
    }

    fn count_syllables(&self, text: &str) -> usize {
        let re = Regex::new(r"[aeiouáéíóúüAEIOUÁÉÍÓÚÜ]+").unwrap();
        re.find_iter(text).count()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_spanish() {
        let pf = PerspicuityFormula::new();
        let text = "Este es un texto de prueba. Está diseñado para verificar la fórmula de perspicuidad.";
        let score = pf.calculate(text);
        assert!(score >= 0.0 && score <= 100.0);
    }

    #[test]
    fn test_edge_cases() {
        let pf = PerspicuityFormula::new();
        let text = "";
        let score = pf.calculate(text);
        assert_eq!(score, 0.0);

        let text = "A.";
        let score = pf.calculate(text);
        assert!(score >= 0.0 && score <= 100.0);
    }
}
