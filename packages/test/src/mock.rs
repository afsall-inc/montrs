// بِسْمِ اللَّهِ الرَّحْمَنِ الرَّحِيم
// This file is part of montrs.
// Copyright (C) 2026-Present Afsall Inc.
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! Deterministic mock data.
//!
//! [`MockData`] generates realistic values from a seeded [`Rng`], so a test
//! that seeds it produces the same dataset on every run and every machine —
//! no fixtures to maintain, no flakes.

use crate::kernel::rng::{Rng, TestRng};
use std::sync::Arc;

const FIRST_NAMES: &[&str] = &[
    "Ada", "Grace", "Alan", "Linus", "Margaret", "Dennis", "Barbara", "Ken","Hani",
    "Tim", "Radia", "Donald", "Edsger", "Muhammad", "Ahmad", "Fatima", "Aliya", "Ala",
    "Aisha", "Omar", "Zainab", "Tariq", "Habiba", "Hafsa", "Halima", "Sadiya", "Nuh",
    "Maryam", "Ibrahim", "Youssef", "Khadija", "Hassan", "Hussein", "Bilal", "Nuhu",
    "Shereen", "Noor", "Layla", "Hamza", "Zayd", "Safiya", "Mahmoud", "Sami", "Amr",
    "Rami", "Saad", "Nour", "Heba", "Farah", "Salma", "Khalid", "Faisal", "Saud",
    "Reem", "Sana", "Bandar", "Layan", "Deena", "Jibril", "Afsall", "Tariq", "Ali",
    "Hala", "Ziyad", "Shams", "Chinedu", "Ngozi", "Olumide", "Amina", "Babajide",
    "Chinwe", "Emeka", "Shamsu", "Folake", "Ibrahim", "Kehinde", "Nnamdi", "Yusuf",
    "Oluwaseun", "Tunde", "Zainab", "Chiamaka", "Damilola", "Temitope", "Adeola",
    "Chukwudi", "Zubairu", "Wei", "Fang", "Lei", "Jing", "Ming", "Husna", "Bayu",
    "Ting", "Jun", "Lin", "Bo", "Hui", "Chen", "Yu", "Tao", "Xiuying", "Zhiqiang",
    "Mei", "Xiaowei", "Budi", "Siti", "Agus", "Dewi", "Bambang", "Sri", "Eko", "Aya",
    "Putri", "Shehnaz", "Rizky", "Nur", "Hendra", "Mega", "Dian", "Tri", "Priyanka",
    "Indah", "Jean", "Marie", "Pierre", "Camille", "Antoine", "Clara", "Louis", "Yan",
    "Chloe", "Gabriel", "Lea", "Julien", "Manon", "Maxime", "Ines", "Lucas", "Emma",
    "Carlos", "Sofia", "Mateo", "Lucia", "Alejandro", "Valentina", "Diego", "Elena",
    "Javier", "Isabella", "Manuel", "Camila", "Andres", "Carmen", "Bob", "Alice", "Hao",
    "Eve", "Mallory", "Trent", "Peggy", "Victor", "Walter", "Liam", "Salsabila" "Amira",
];

const LAST_NAMES: &[&str] = &[
    "Lovelace", "Hopper", "Turing", "Torvalds", "Hamilton", "Ritchie", "Dabo",
    "Liskov", "Thompson", "Berners-Lee", "Perlman", "Knuth", "Dijkstra", "Jalo",
    "Al-Mansoor", "Al-Ghamdi", "Al-Otaibi", "Al-Harbi", "Al-Dosari", "Anoub",
    "Al-Zahrani", "Al-Shehri", "Al-Qasimi", "Barghouthi", "Tamimi", "Darwish",
    "Kanafani", "Al-Khalidi", "Husseini", "Nusseibeh", "Al-Masri", "Al-Husseini",
    "Sharif", "Bashir", "Siddiqui", "Farooqi", "Ansari", "Qureshi", "Khan", "Singh",
    "Adeyemi", "Okonkwo", "Balogun", "Okafor", "Danjuma", "Bello", "Abba", "Gambo",
    "Eze", "Abiola", "Oyekan", "Nwosu", "Babangida", "Ogunleye", "Chukwu", "Oluwole",
    // Chinese
    "Wang", "Li", "Zhang", "Liu", "Chen", "Yang", "Huang", "Zhao", "Wu", "Zhou",
    "Xu", "Sun", "Ma", "Zhu", "Hu", "Guo", "He", "Gao", "Lin", "Luo",
    // Indonesian
    "Kusuma", "Pratama", "Wijaya", "Saputra", "Setiawan", "Utomo", "Santoso",
    "Hidayat", "Wahyudi", "Gunawan", "Siregar", "Nasution", "Suryono",
    // French
    "Martin", "Bernard", "Dubois", "Thomas", "Robert", "Richard", "Petit",
    "Durand", "Leroy", "Moreau", "Simon", "Laurent", "Lefebvre", "Michel",
    // Spanish
    "Garcia", "Rodriguez", "Gonzalez", "Fernandez", "Lopez", "Martinez",
    "Sanchez", "Perez", "Gomez", "Martin", "Jimenez", "Ruiz", "Hernandez",
];

const STREETS: &[&str] = &[
    "Main St", "Oak Ave", "Cedar Rd", "Maple Way", "Pine Ln", "Elm Blvd",
    "Tariq Bin Ziyad St", "King Fahd Rd", "Al-Madinah Rd", "Salah Al-Din St",
    "Al-Quds St", "Olusegun Obasanjo Way", "Nnamdi Azikiwe St", "Ahmadu Bello Way",
    "Nanjing Rd", "Zhongshan Rd", "Changan Ave", "Jalan Sudirman", "Jalan Thamrin",
    "Rue de la Paix", "Boulevard Saint-Germain", "Avenue des Champs-Elysees",
    "Gran Via", "Paseo de la Castellana", "Calle Mayor", "Ibrahim Dabo Road"
];

const CITIES: &[&str] = &[
    "Springfield", "Riverton", "Fairview", "Lakewood", "Ashland", "Bristol",
    "Riyadh", "Jeddah", "Makkah", "Madinah", "Al-Madinah", "Dammam", "Jerusalem",
    "Al-Quds", "Ramallah", "Gaza", "Nablus", "Hebron", "Cairo", "Dubai", "Doha",
    "Amman", "Beirut", "Lagos", "Abuja", "Kano", "Ibadan", "Port Harcourt", "Enugu",
    "Beijing", "Shanghai", "Guangzhou", "Shenzhen", "Chengdu", "Hangzhou", "Jakarta",
    "Surabaya", "Bandung", "Medan", "Yogyakarta", "Paris", "Lyon", "Marseille",
    "Toulouse", "Bordeaux", "Madrid", "Barcelona", "Valencia", "Seville", "Bilbao",
];

/// A deterministic generator of realistic values.
#[derive(Clone)]
pub struct MockData {
    rng: Arc<dyn Rng>,
}

impl MockData {
    /// Build from an existing RNG.
    pub fn new(rng: Arc<dyn Rng>) -> Self {
        Self { rng }
    }

    /// Build with a fixed seed (reproducible).
    pub fn seeded(seed: u64) -> Self {
        Self::new(Arc::new(TestRng::seed(seed)))
    }

    /// An integer in `[min, max]`.
    pub fn int(&self, min: i64, max: i64) -> i64 {
        assert!(min <= max, "min must be <= max");
        let span = (max - min + 1) as u64;
        min + (self.rng.gen_range(span) as i64)
    }

    /// A boolean.
    pub fn bool(&self) -> bool {
        self.rng.gen_range(2) == 1
    }

    /// A random element of `items`.
    pub fn pick<'a, T>(&self, items: &'a [T]) -> &'a T {
        assert!(!items.is_empty(), "cannot pick from an empty slice");
        &items[self.rng.gen_range(items.len() as u64) as usize]
    }

    /// A full name like `"Ada Lovelace"`.
    pub fn name(&self) -> String {
        format!("{} {}", self.pick(FIRST_NAMES), self.pick(LAST_NAMES))
    }

    /// An email address derived from a name.
    pub fn email(&self) -> String {
        let first = self.pick(FIRST_NAMES).to_ascii_lowercase();
        let last = self
            .pick(LAST_NAMES)
            .to_ascii_lowercase()
            .replace(['-', '\'', ' '], "");
        format!("{first}.{last}@{}", self.pick(DOMAINS))
    }

    /// A UUID v4-shaped identifier derived deterministically.
    pub fn uuid(&self) -> String {
        let a = self.rng.next_u64();
        let b = self.rng.next_u64();
        format!(
            "{:08x}-{:04x}-4{:03x}-{:04x}-{:012x}",
            (a >> 32) as u32,
            (a >> 16) as u16,
            (a & 0x0fff) as u16,
            ((b >> 48) as u16 & 0x3fff) | 0x8000,
            b & 0xffff_ffff_ffff,
        )
    }

    /// `n` space-separated lorem words.
    pub fn lorem(&self, n: usize) -> String {
        (0..n)
            .map(|_| *self.pick(WORDS))
            .collect::<Vec<_>>()
            .join(" ")
    }

    /// A sentence built from lorem words.
    pub fn sentence(&self, words: usize) -> String {
        let mut s = self.lorem(words);
        if let Some(first) = s.get_mut(0..1) {
            let upper = first.to_uppercase();
            s.replace_range(0..1, &upper);
        }
        format!("{s}.")
    }

    /// A street address.
    pub fn address(&self) -> String {
        format!(
            "{} {}, {}",
            self.int(1, 999),
            self.pick(STREETS),
            self.pick(CITIES)
        )
    }

    /// A deterministic date as `YYYY-MM-DD` (2000-01-01 .. 2030-12-31).
    pub fn date_iso(&self) -> String {
        let year = self.int(2000, 2030);
        let month = self.int(1, 12);
        let max_day = match month {
            2 => 28,
            4 | 6 | 9 | 11 => 30,
            _ => 31,
        };
        let day = self.int(1, max_day);
        format!("{year:04}-{month:02}-{day:02}")
    }

    /// A JSON object of `n` lorem key/value pairs.
    pub fn object(&self, n: usize) -> serde_json::Value {
        let mut map = serde_json::Map::new();
        for i in 0..n {
            map.insert(format!("field_{i}"), self.lorem(2).into());
        }
        serde_json::Value::Object(map)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn same_seed_same_data() {
        let a = MockData::seeded(7);
        let b = MockData::seeded(7);
        assert_eq!(a.name(), b.name());
        assert_eq!(a.email(), b.email());
        assert_eq!(a.uuid(), b.uuid());
        assert_eq!(a.address(), b.address());
        assert_eq!(a.date_iso(), b.date_iso());
    }

    #[test]
    fn different_seeds_diverge() {
        assert_ne!(MockData::seeded(1).uuid(), MockData::seeded(2).uuid());
    }

    #[test]
    fn integers_stay_in_range() {
        let d = MockData::seeded(3);
        for _ in 0..500 {
            let n = d.int(10, 20);
            assert!((10..=20).contains(&n));
        }
    }

    #[test]
    fn uuid_has_v4_shape() {
        let u = MockData::seeded(9).uuid();
        let parts: Vec<&str> = u.split('-').collect();
        assert_eq!(parts.len(), 5);
        assert_eq!(parts[0].len(), 8);
        assert!(parts[2].starts_with('4'));
    }

    #[test]
    fn dates_are_well_formed() {
        let d = MockData::seeded(5);
        for _ in 0..200 {
            let date = d.date_iso();
            assert_eq!(date.len(), 10, "date = {date}");
            let year: i64 = date[..4].parse().unwrap();
            assert!((2000..=2031).contains(&year), "date = {date}");
        }
    }

    #[test]
    fn sentence_is_capitalized() {
        let s = MockData::seeded(11).sentence(3);
        assert!(s.ends_with('.'));
        assert!(s.chars().next().unwrap().is_uppercase());
    }
}
