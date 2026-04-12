use crate::Set;
use std::collections::HashMap;

#[derive(Debug, Clone, Default)]
pub(crate) struct Namer<'a, 'b>
where
    'a: 'b,
{
    iter: NameIterator,
    names: HashMap<&'b Set<'a>, String>,
}

impl<'a, 'b> Namer<'a, 'b> {
    pub fn get(&mut self, item: &'b Set<'a>) -> &str {
        self.names
            .entry(item)
            .or_insert_with(|| self.iter.next().unwrap())
            .as_str()
    }
}

#[derive(Debug, Clone, Copy, Default)]
struct NameIterator {
    sequence_index: u32,
}

impl Iterator for NameIterator {
    type Item = String;
    fn next(&mut self) -> Option<Self::Item> {
        let mut output = vec![];
        let mut div = self.sequence_index;
        loop {
            output.push(char::from_u32('a' as u32 + (div % 26)).unwrap());
            div /= 26;
            if div == 0 {
                break;
            }
        }
        self.sequence_index += 1;
        Some(output.into_iter().rev().collect())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn name_iterator() {
        assert_eq!(
            NameIterator::default().take(12).collect::<Vec<String>>(),
            vec!["a", "b", "c", "d", "e", "f", "g", "h", "i", "j", "k", "l",]
        );
        assert_eq!(
            NameIterator::default()
                .step_by(26)
                .take(12)
                .collect::<Vec<String>>(),
            vec![
                "a", "ba", "ca", "da", "ea", "fa", "ga", "ha", "ia", "ja",
                "ka", "la",
            ]
        );
        assert_eq!(
            NameIterator::default()
                .step_by(26 * 26)
                .take(12)
                .collect::<Vec<String>>(),
            vec![
                "a", "baa", "caa", "daa", "eaa", "faa", "gaa", "haa", "iaa",
                "jaa", "kaa", "laa",
            ]
        );
    }
}
