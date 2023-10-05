// I took this from egui's source
// It turns Size into actual tangible numbers and I really really wasn't going to go about remaking this myself

use egui_extras::Size;

#[derive(Clone, Default)]
pub struct Sizing {
    pub(crate) sizes: Vec<Size>,
}

impl Sizing {
    pub fn add(&mut self, size: Size) {
        self.sizes.push(size);
    }

    pub fn to_lengths(&self, length: f32, spacing: f32) -> Vec<f32> {
        if self.sizes.is_empty() {
            return vec![];
        }

        let mut remainders = 0;
        let sum_non_remainder = self
            .sizes
            .iter()
            .map(|&size| match size {
                Size::Absolute { initial, .. } => initial,
                Size::Relative { fraction, range } => {
                    assert!(0.0 <= fraction && fraction <= 1.0);
                    range.clamp(length * fraction)
                }
                Size::Remainder { .. } => {
                    remainders += 1;
                    0.0
                }
            })
            .sum::<f32>()
            + spacing * (self.sizes.len() - 1) as f32;

        let avg_remainder_length = if remainders == 0 {
            0.0
        } else {
            let mut remainder_length = length - sum_non_remainder;
            let avg_remainder_length = 0.0f32.max(remainder_length / remainders as f32).floor();
            self.sizes.iter().for_each(|&size| {
                if let Size::Remainder { range } = size {
                    if avg_remainder_length < range.min {
                        remainder_length -= range.min;
                        remainders -= 1;
                    }
                }
            });
            if remainders > 0 {
                0.0f32.max(remainder_length / remainders as f32)
            } else {
                0.0
            }
        };

        self.sizes
            .iter()
            .map(|&size| match size {
                Size::Absolute { initial, .. } => initial,
                Size::Relative { fraction, range } => range.clamp(length * fraction),
                Size::Remainder { range } => range.clamp(avg_remainder_length),
            })
            .collect()
    }
}

impl From<Vec<Size>> for Sizing {
    fn from(sizes: Vec<Size>) -> Self {
        Self { sizes }
    }
}