

use std::collections::HashMap;
use std::collections::HashSet;

pub struct ExchangeRates<'a> {
    rates: HashMap<(&'a str, &'a str), f64>
}

impl<'a> ExchangeRates<'a> {

    pub fn new(rates: HashMap<(&'a str, &'a str), f64>) -> Self {
        ExchangeRates { rates }
    }

    pub fn get_all(&self, currency: &str) -> HashMap<&str, f64> {
        let mut res = HashMap::new();
        for (&(curr1, curr2), v) in self.rates.iter() {
            if curr1 == currency {
                res.insert(curr2, 1.*v.to_owned());
            }

            if curr2 == currency {
                res.insert(curr1, 1./v.to_owned());
            }
        }
        res
    }

    pub fn get(&mut self, curr1: &str, curr2: &str) -> Option<f64> {
        if curr1 == curr2 {
            Some(1.)
        }
        else if let Some(rate) = self.rates.get(&(curr1, curr2)) {
            Some(1.*rate)
        }
        else if let Some(reversed_rate) = self.rates.get(&(curr2, curr1)) {
            Some(1./reversed_rate)
        }
        else {
            let mut seen = HashSet::new();
            let mut vec = vec![(curr1, 1.)];
            let mut idx = 0;
            while idx < vec.len() {
                let (curr, rate) = vec[idx];
                for (n_curr, n_rate) in self.get_all(curr) {
                    // self.rates.entry((curr1, n_curr)).or_insert(rate*n_rate):
                    if n_curr == curr2 {
                        return Some(rate * n_rate);
                    }
                    if seen.insert(n_curr) {
                        vec.push((n_curr, rate*n_rate));
                    }
                }
                idx += 1;
            }
            None
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn not_there() {
        let mut e = ExchangeRates::new(
            HashMap::new()
        );
        assert_eq!(e.get("USD", "EUR"), None);
    }

    #[test]
    fn identical() {
        let mut e = ExchangeRates::new(
            HashMap::new()
        );
        assert_eq!(e.get("USD", "USD"), Some(1.));
    }

    #[test]
    fn is_there() {
        let mut e = ExchangeRates::new(
            HashMap::from([
                (("USD", "EUR"), 1.)
            ])
        );
        assert_eq!(e.get("USD", "EUR"), Some(1.));
    }

    #[test]
    fn reverse() {
        let mut e = ExchangeRates::new(
            HashMap::from([
                (("USD", "EUR"), 2.)
            ])
        );
        assert_eq!(e.get("EUR", "USD"), Some(0.5));
    }

    #[test]
    fn chain() {
        let mut e = ExchangeRates::new(
            HashMap::from([
                (("USD", "JPY"), 2.),
                (("JPY", "EUR"), 3.),
            ])
        );
        assert_eq!(e.get("USD", "EUR"), Some(6.));
    }

    #[test]
    fn longer_chain() {
        let mut e = ExchangeRates::new(
            HashMap::from([
                (("START", "M1"), 2.),
                (("M1", "M2"), 2.),
                (("M2", "M3"), 2.),
                (("M3", "M4"), 2.),
                (("M4", "M5"), 2.),
                (("M5", "M6"), 2.),
                (("M6", "END"), 2.),
            ])
        );
        assert_eq!(e.get("USD", "EUR"), Some(128.));
    }
}
