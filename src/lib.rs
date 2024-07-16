
use std::collections::HashMap;
use std::collections::HashSet;

#[derive(Debug, Default)]
pub struct ExchangeRates<'a> {
    rates: HashMap<&'a str, HashMap<&'a str, f64>>
}

impl<'a> ExchangeRates<'a> {

    pub fn from_rates(rates: HashMap<(&'a str, &'a str), f64>) -> Self {
        let mut res = ExchangeRates { rates: HashMap::new() };
        for ((curr1, curr2), rate) in rates {
            res.add(curr1, curr2, rate);
        }
        res
    }

    pub fn add(&mut self, curr1: &'a str, curr2: &'a str, rate: f64) {
        self.rates.entry(curr1).or_default().insert(curr2, 1.*rate);
        self.rates.entry(curr2).or_default().insert(curr1, 1./rate);
    }

    pub fn get(&mut self, curr1: &str, curr2: &str) -> Option<f64> {
        if curr1 == curr2 {
            Some(1.)
        } else if !self.rates.contains_key(curr1) {
            None
        } else if let Some(rate) = self.rates.get(curr1).and_then(|rates| rates.get(curr2)) {
            Some(1.*rate)
        } else {
            let mut seen = HashSet::new();
            let mut vec = vec![(curr1, 1.)];
            let mut idx = 0;
            while idx < vec.len() {
                let (curr, rate) = vec[idx];
                for (n_curr, n_rate) in self.rates.get(curr).unwrap() {
                    let rate = rate * n_rate;
                    if *n_curr == curr2 {
                        return Some(rate);
                    }
                    if seen.insert(n_curr) {
                        vec.push((n_curr, rate));
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
        let mut e = ExchangeRates::from_rates(
            HashMap::new()
        );
        assert_eq!(e.get("USD", "EUR"), None);
    }

    #[test]
    fn identical() {
        let mut e = ExchangeRates::from_rates(
            HashMap::new()
        );
        assert_eq!(e.get("USD", "USD"), Some(1.));
    }

    #[test]
    fn is_there() {
        let mut e = ExchangeRates::from_rates(
            HashMap::from([
                (("USD", "EUR"), 1.)
            ])
        );
        assert_eq!(e.get("USD", "EUR"), Some(1.));
    }

    #[test]
    fn is_not_the_one_there() {
        let mut e = ExchangeRates::from_rates(
            HashMap::from([
                (("USD", "EUR"), 1.)
            ])
        );
        assert_eq!(e.get("JPY", "DKK"), None);
    }

    #[test]
    fn reverse() {
        let mut e = ExchangeRates::from_rates(
            HashMap::from([
                (("USD", "EUR"), 2.)
            ])
        );
        assert_eq!(e.get("EUR", "USD"), Some(0.5));
    }

    #[test]
    fn chain() {
        let mut e = ExchangeRates::from_rates(
            HashMap::from([
                (("USD", "JPY"), 2.),
                (("JPY", "EUR"), 3.),
            ])
        );
        assert_eq!(e.get("USD", "EUR"), Some(6.));
    }

    #[test]
    fn longer_chain() {
        let mut e = ExchangeRates::from_rates(
            HashMap::from([
                (("START", "M1"), 2.),
                (("M1", "M2"), 2.),
                (("M2", "M3"), 2.),
                (("M3", "M4"), 5.),
                (("M4", "M5"), 2.),
                (("M5", "M6"), 2.),
                (("M6", "END"), 2.),
            ])
        );
        assert_eq!(e.get("START", "END"), Some(5.*64.));
    }
}
