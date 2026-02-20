use std::collections::BTreeSet;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Effect {
    Pure,
    IO,
    Random,
    Time,
    ExternalCall(String),
}

#[derive(Debug, Clone, Default)]
pub struct EffectSet {
    effects: BTreeSet<Effect>,
}

impl EffectSet {
    pub fn is_pure(&self) -> bool {
        self.effects.is_empty()
            || (self.effects.len() == 1 && self.effects.contains(&Effect::Pure))
    }

    pub fn insert(&mut self, effect: Effect) {
        if effect != Effect::Pure {
            self.effects.insert(effect);
        }
    }

    pub fn union(&self, other: &Self) -> Self {
        let mut out = self.clone();
        out.effects.extend(other.effects.iter().cloned());
        out
    }

    pub fn to_names(&self) -> Vec<String> {
        self.effects
            .iter()
            .map(|e| match e {
                Effect::Pure => "pure".to_string(),
                Effect::IO => "io".to_string(),
                Effect::Random => "random".to_string(),
                Effect::Time => "time".to_string(),
                Effect::ExternalCall(name) => format!("external:{name}"),
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_is_pure() {
        let set = EffectSet::default();
        assert!(set.is_pure());
        assert!(set.to_names().is_empty());
    }

    #[test]
    fn union_combines_effects() {
        let mut a = EffectSet::default();
        a.insert(Effect::IO);
        let mut b = EffectSet::default();
        b.insert(Effect::Random);
        let u = a.union(&b);
        assert!(!u.is_pure());
        let names = u.to_names();
        assert!(names.contains(&"io".to_string()));
        assert!(names.contains(&"random".to_string()));
    }
}
