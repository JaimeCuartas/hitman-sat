use std::collections::HashMap;
use std::collections::hash_map::Entry;

pub struct Hitman {
    cnf: Vec<Vec<i32>>,
    var2id: HashMap<u32, u32>,
    id2var: Vec<u32>,
}

impl Default for Hitman {
    fn default() -> Self {
        Self::new()
    }
}

impl Hitman {
    pub fn new() -> Self {
        Self {
            cnf: Vec::new(),
            var2id: HashMap::new(),
            id2var: Vec::new(),
        }
    }

    pub fn hit(&mut self, clause: &[i32]) {
        let mut clause_ids: Vec<i32> = Vec::with_capacity(clause.len());

        for &element in clause.iter() {
            if element == 0 {
                continue;
            }
            let id = self.get_id(element);
            let lit = if element > 0 { id } else { -id };
            clause_ids.push(lit);
        }
        self.cnf.push(clause_ids);
    }

    fn get_id(&mut self, var_name: i32) -> i32 {
        let abs_var: u32 = var_name.unsigned_abs();
        let id_entry = self.var2id.entry(abs_var);
        let id = match id_entry {
            Entry::Occupied(occupied) => *occupied.get(),
            Entry::Vacant(_) => {
                let new_id =
                    u32::try_from(self.len() + 1).expect("Variable count exceeded u32::MAX limit");
                self.id2var.push(abs_var);
                self.var2id.insert(abs_var, new_id);
                new_id
            }
        };
        id as i32
    }

    fn len(&self) -> usize {
        self.id2var.len()
    }
}

#[cfg(test)] mod tests {
    use super::*;
    #[test]
    fn new_is_emtpy () {
        let hitman = Hitman::new();
        assert_eq!(hitman.len(), 0);
    }
    #[test]
    fn hit_assigns_ids() {
        let mut hitman = Hitman::new();
        hitman.hit(&[28,10]);
        hitman.hit(&[28,11]);
        hitman.hit(&[29,28,30,10]);
        assert_eq!(hitman.var2id.get(&28u32), Some(&1u32));
        assert_eq!(hitman.var2id.get(&10u32), Some(&2u32));
        assert_eq!(hitman.var2id.get(&11u32), Some(&3u32));
        assert_eq!(hitman.var2id.get(&29u32), Some(&4u32));
        assert_eq!(hitman.var2id.get(&30u32), Some(&5u32));
        assert_eq!(hitman.len(), 5);
    }
    #[test]
    fn repeated_element_reuses_id() {
    }
    #[test]
    fn id_round_trip() {
        let mut hitman = Hitman::new();
        hitman.hit(&[28,10]);
        hitman.hit(&[28,11]);
        hitman.hit(&[29,28,30,10]);
        assert_eq!(hitman.id2var, &[28, 10, 11, 29, 30]);
    }
}
