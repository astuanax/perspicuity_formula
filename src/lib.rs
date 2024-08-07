use regex::Regex;

pub struct PisaReadingIndex;

impl PisaReadingIndex {
    pub fn new() -> Self {
        PisaReadingIndex
    }

    pub fn calculate(&self, text: &str) -> f64 {
        let sentence_count = self.count_sentences(text);
        let word_count = self.count_words(text);
        let character_count = self.count_characters(text);

        if word_count == 0 || sentence_count == 0 {
            return 0.0;
        }

        let words_per_sentence = word_count as f64 / sentence_count as f64;
        let characters_per_word = character_count as f64 / word_count as f64;

        // PISA Reading Index formula
        let pisa_index = 100.0 - (10.0 * words_per_sentence) - (0.5 * characters_per_word);

        pisa_index.clamp(0.0, 100.0)
    }

    fn count_sentences(&self, text: &str) -> usize {
        let re = Regex::new(r"[.!?]").unwrap();
        re.find_iter(text).count()
    }

    fn count_words(&self, text: &str) -> usize {
        let re = Regex::new(r"\b\w+\b").unwrap();
        re.find_iter(text).count()
    }

    fn count_characters(&self, text: &str) -> usize {
        text.chars().filter(|c| c.is_alphanumeric()).count()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_english() {
        let pri = PisaReadingIndex::new();
        let text = "This is a test sentence. It is designed to check the PISA Reading Index.";
        let score = pri.calculate(text);
        assert!(score >= 0.0 && score <= 100.0);
    }

    #[test]
    fn test_spanish() {
        let pri = PisaReadingIndex::new();
        let text = "Este es un texto de prueba. Está diseñado para verificar el índice de lectura PISA.";
        let score = pri.calculate(text);
        assert!(score >= 0.0 && score <= 100.0);
    }

    #[test]
    fn test_edge_cases() {
        let pri = PisaReadingIndex::new();
        let text = "";
        let score = pri.calculate(text);
        assert_eq!(score, 0.0);

        let text = "A.";
        let score = pri.calculate(text);
        assert!(score >= 0.0 && score <= 100.0);
    }
}
