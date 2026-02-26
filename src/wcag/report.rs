use crate::colour::Colour;
use crate::wcag::WcagCompliance;


pub fn analyze(colour1: Colour, colour2: Colour) {
    let ratio = contrast::contrast(colour1.to_rgb8(), colour2.to_rgb8());

    println!("{}", format_results(colour1, colour2, ratio));
}

fn format_results(colour1: Colour, colour2: Colour, ratio: f32) -> String {
    let wcag_compliance_msg = match WcagCompliance::from_ratio(ratio) {
        WcagCompliance::Aaa => "✅ Meets WCAG AAA standards (Enhanced).".to_string(),
        WcagCompliance::Aa  => "✅ Meets WCAG AA standards (Minimum).".to_string(),
        WcagCompliance::Fail => "❌ Does not meet WCAG AA standards.\nNote: Text requires at least 4.5:1 for AA or 7:1 for AAA.".to_string(),
    };

    format!("Colour 1: {colour1}\nColour 2: {colour2}\n---\nContrast ratio: {ratio:.2}:1\n{wcag_compliance_msg}")
}

#[cfg(test)]
mod tests {
    use super::*;
    use rgb::RGB8;

    fn black() -> Colour { Colour { rgb: RGB8::new(0, 0, 0) }}
    fn white() -> Colour { Colour { rgb: RGB8::new(255, 255, 255) }}

    #[test]
    fn test_format_results_aaa() {
        let output = format_results(black(), white(), 21.0);
        assert!(output.contains("Colour 1: RGB(0, 0, 0)"));
        assert!(output.contains("Colour 2: RGB(255, 255, 255)"));
        assert!(output.contains("Contrast ratio: 21.00:1"));
        assert!(output.contains("Meets WCAG AAA standards"));
    }

    #[test]
    fn test_format_results_aa() {
        let output = format_results(black(), white(), 5.0);
        assert!(output.contains("Contrast ratio: 5.00:1"));
        assert!(output.contains("Meets WCAG AA standards"));
    }

    #[test]
    fn test_format_results_fail() {
        let output = format_results(black(), white(), 2.0);
        assert!(output.contains("Contrast ratio: 2.00:1"));
        assert!(output.contains("Does not meet WCAG AA standards"));
        assert!(output.contains("4.5:1"));
    }
}