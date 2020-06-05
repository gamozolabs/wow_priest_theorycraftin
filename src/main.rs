type Database = Vec<Vec<Item>>;

const BASE_HEALTH:    i32 = 368;
const BASE_MANA:      i32 = 665;
const BASE_STAMINA:   i32 = 32;
const BASE_INTELLECT: i32 = 57;

const NUM_WORKERS: usize = 4;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
#[repr(usize)]
enum Slot {
    Head,
    Neck,
    Shoulder,
    Back,
    Chest,
    Wrist,
    Hands,
    Waist,
    Legs,
    Feet,
    Ring1,
    Ring2,
    Trinket1,
    Trinket2,
    Weapon,
    OffHand,
    Wand,

    // Enchants
    HeadEnchant,
    LegEnchant,
    ChestEnchant,
    WristEnchant,
    WeaponEnchant,

    // No touchie
    MaxSlot,
}

const BRUTE_FORCE_SLOTS: &[Slot] = &[
    Slot::Head,
    Slot::Neck,
    Slot::Shoulder,
    Slot::Back,
    Slot::Chest,
    Slot::Wrist,
    Slot::Hands,
    Slot::Waist,
    Slot::Legs,
    Slot::Feet,
    Slot::Ring1,
    Slot::Ring2,
    Slot::Weapon,
    Slot::OffHand,
    Slot::Wand,

    /*
    Slot::HeadEnchant,
    Slot::ChestEnchant,
    Slot::WristEnchant,
    Slot::LegEnchant,
    Slot::WeaponEnchant,*/
];

#[derive(Default, Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
struct Item {
    name:      &'static str,
    armor:     i32,
    strength:  i32,
    stamina:   i32,
    intellect: i32,
    spirit:    i32,
    agility:   i32,
    spell:     i32,
    healing:   i32,
    mana:      i32,
    unique:    bool,
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
struct Character<'a> {
    slots:    Vec<Option<&'a Item>>,
    database: Database,
}

impl<'a> Character<'a> {
    fn new() -> Self {
        let mut database = vec![Vec::new(); Slot::MaxSlot as usize];
        database[Slot::Head as usize].extend_from_slice(&[
            Item {
                name:      "Holy Shroud",
                armor:     40,
                spirit:    6,
                healing:   33,
                ..Default::default()
            },
            Item {
                name:      "Elders Hat of the Eagle",
                armor:     37,
                intellect: 9,
                stamina:   9,
                ..Default::default()
            },
            Item {
                name:      "Elders Hat of Intellect",
                armor:     37,
                intellect: 14,
                ..Default::default()
            },
            Item {
                name:      "Elders Hat of Stamina",
                armor:     37,
                intellect: 14,
                ..Default::default()
            },
        ]);
        database[Slot::Neck as usize].extend_from_slice(&[
            Item {
                name:      "Crystal Starfire Medallion",
                intellect: 4,
                stamina:   4,
                spirit:    3,
                ..Default::default()
            },
            Item {
                name:      "River Pride Choker",
                strength:  4,
                stamina:   9,
                ..Default::default()
            },
        ]);
        database[Slot::Shoulder as usize].extend_from_slice(&[
            Item {
                name:      "Magician's Mantle",
                armor:     32,
                intellect: 9,
                spell:     5,
                healing:   5,
                ..Default::default()
            },
            Item {
                name:      "Berylline Pads",
                armor:     39,
                intellect: 10,
                stamina:   5,
                spirit:    6,
                ..Default::default()
            },
        ]);
        database[Slot::Back as usize].extend_from_slice(&[
            Item {
                name:      "Cloak of Rot",
                armor:     22,
                intellect: 7,
                stamina:   -5,
                ..Default::default()
            },
            Item {
                name:      "Archer's Cloak of the Eagle",
                armor:     23,
                intellect: 5,
                stamina:   5,
                ..Default::default()
            },
            Item {
                name:      "Archer's Cloak of Stamina",
                armor:     23,
                stamina:   8,
                ..Default::default()
            },
            Item {
                name:      "Archer's Cloak of Intellect",
                armor:     23,
                intellect: 8,
                ..Default::default()
            },
            Item {
                name:      "Archer's Cloak of Healing",
                armor:     23,
                healing:   18,
                ..Default::default()
            },
        ]);
        database[Slot::Chest as usize].extend_from_slice(&[
            Item {
                name:      "Mechbuilder's Overalls",
                armor:     48,
                intellect: 15,
                stamina:   5,
                ..Default::default()
            },
            Item {
                name:      "Beguiler Robes",
                armor:     50,
                intellect: 12,
                stamina:   7,
                spirit:    8,
                ..Default::default()
            },
        ]);
        database[Slot::Wrist as usize].extend_from_slice(&[
            Item {
                name:      "Mindthrust Bracers",
                armor:     17,
                intellect: 9,
                stamina:   -5,
                ..Default::default()
            },
            Item {
                name:      "Gallan Cuffs",
                armor:     21,
                intellect: 7,
                stamina:   3,
                ..Default::default()
            },
            Item {
                name:      "Vital Bracelets of the Eagle",
                armor:     19,
                intellect: 5,
                stamina:   5,
                ..Default::default()
            },
            Item {
                name:      "Vital Bracelets of Stamina",
                armor:     19,
                stamina:   7,
                ..Default::default()
            },
        ]);
        database[Slot::Hands as usize].extend_from_slice(&[
            Item {
                name:      "Hotshot Pilot's GLoves",
                armor:     31,
                intellect: 8,
                stamina:   5,
                spirit:    5,
                agility:   3,
                ..Default::default()
            },
            Item {
                name:      "Truefaith Gloves",
                armor:     27,
                intellect: 3,
                healing:   15,
                ..Default::default()
            },
            Item {
                name:      "Vital Handwraps of the Eagle",
                armor:     29,
                intellect: 7,
                stamina:   7,
                ..Default::default()
            },
            Item {
                name:      "Vital Handwraps of Healing",
                armor:     29,
                healing:   24,
                ..Default::default()
            },
        ]);
        database[Slot::Waist as usize].extend_from_slice(&[
            Item {
                name:      "Razzeric's Customized Seatbelt",
                armor:     30,
                intellect: 12,
                stamina:   1,
                ..Default::default()
            },
            Item {
                name:      "Conjurer's Cinch of the Eagle",
                armor:     26,
                intellect: 7,
                stamina:   7,
                ..Default::default()
            },
            Item {
                name:      "Conjurer's Cinch of Intellect",
                armor:     26,
                intellect: 11,
                ..Default::default()
            },
            Item {
                name:      "Conjurer's Cinch of Healing",
                armor:     26,
                healing:   24,
                ..Default::default()
            },
        ]);
        database[Slot::Legs as usize].extend_from_slice(&[
            Item {
                name:      "Elder's Pants of the Eagle",
                armor:     40,
                intellect: 9,
                stamina:   9,
                ..Default::default()
            },
            Item {
                name:      "Elder's Pants of Healing",
                armor:     40,
                healing:   31,
                ..Default::default()
            },
        ]);
        database[Slot::Feet as usize].extend_from_slice(&[
            Item {
                name:      "Acidic Walkers",
                armor:     34,
                intellect: 8,
                spirit:    4,
                spell:     5,
                healing:   5,
                ..Default::default()
            },
            Item {
                name:      "Nightsky Boots",
                armor:     32,
                intellect: 4,
                stamina:   8,
                ..Default::default()
            },
        ]);
        database[Slot::Ring1 as usize].extend_from_slice(&[
            Item {
                name:      "Nogg's Gold Ring",
                stamina:   9,
                spirit:    4,
                unique:    true,
                ..Default::default()
            },
            Item {
                name:      "Plains Ring",
                stamina:   8,
                intellect: 3,
                unique:    true,
                ..Default::default()
            },
            Item {
                name:      "Black Widow Band",
                stamina:   -5,
                intellect: 7,
                unique:    true,
                ..Default::default()
            },
        ]);
        database[Slot::Ring2 as usize] =
            database[Slot::Ring1 as usize].clone();
        database[Slot::Weapon as usize].extend_from_slice(&[
            Item {
                name:      "Death Speaker Scepter",
                spirit:    1,
                healing:   11,
                ..Default::default()
            },
            Item {
                name:      "Skullbreaker",
                intellect: 5,
                stamina:   3,
                ..Default::default()
            },
            Item {
                name:      "Crested Scepter",
                intellect: 2,
                stamina:   5,
                ..Default::default()
            },
        ]);
        database[Slot::OffHand as usize].extend_from_slice(&[
            Item {
                name:      "Witch's Finger",
                intellect: 7,
                stamina:   4,
                ..Default::default()
            },
            Item {
                name:    "Orb of Mismantle",
                spirit:  4,
                healing: 9,
                ..Default::default()
            },
        ]);
        database[Slot::Wand as usize].extend_from_slice(&[
            Item {
                name:      "Gravestone Scepter",
                spirit:    1,
                ..Default::default()
            },
        ]);
        database[Slot::HeadEnchant as usize].extend_from_slice(&[
            Item {
                name:      "Lesser Arcanum of Constitution",
                stamina:   10,
                ..Default::default()
            },
            Item {
                name:      "Arcanum of Focus",
                spell:     8,
                healing:   8,
                ..Default::default()
            },
            Item {
                name:      "Lesser Arcanum of Rumination",
                intellect: 10,
                ..Default::default()
            },
        ]);
        database[Slot::LegEnchant as usize] =
            database[Slot::HeadEnchant as usize].clone();
        database[Slot::ChestEnchant as usize].extend_from_slice(&[
            Item {
                name:      "Enchant Chest - Major Health",
                stamina:   10,
                ..Default::default()
            },
            Item {
                name:      "Enchant Chest - Major Mana",
                mana:      100,
                ..Default::default()
            },
            Item {
                name:      "Enchant Chest - Greater Stats",
                intellect: 4,
                spirit:    4,
                agility:   4,
                strength:  4,
                stamina:   4,
                ..Default::default()
            },
        ]);
        database[Slot::WristEnchant as usize].extend_from_slice(&[
            Item {
                name:    "Enchant Bracer - Healing Power",
                healing: 24,
                ..Default::default()
            },
            Item {
                name:    "Enchant Bracer - Superior Stamina",
                stamina: 9,
                ..Default::default()
            },
        ]);
        database[Slot::WeaponEnchant as usize].extend_from_slice(&[
            Item {
                name:      "Enchant Weapon - Mighty Intellect",
                intellect: 22,
                ..Default::default()
            },
            Item {
                name:    "Enchant Weapon - Healing Power",
                healing: 55,
                ..Default::default()
            },
        ]);

        Character {
            slots:    vec![None; Slot::MaxSlot as usize],
            database: database,
        }
    }

    fn healing(&self) -> i32 {
        self.slots.iter().map(|x| x.map(|x| x.healing).unwrap_or(0)).sum()
    }

    fn health(&self) -> i32 {
        let mut stam = BASE_STAMINA;
        for item in self.slots.iter() {
            if let Some(item) = item {
                stam += item.stamina;
            }
        }

        if stam < 20 {
            BASE_HEALTH + stam
        } else {
            BASE_HEALTH + 20 + (stam - 20) * 10
        }
    }

    fn mana(&self) -> i32 {
        let mut int = BASE_INTELLECT;
        let mut mana = BASE_MANA;
        for item in self.slots.iter() {
            if let Some(item) = item {
                int  += item.intellect;
                mana += item.mana;
            }
        }

        // Mana = base mana + 1 mana for int for the first 20 int, then 15 mana
        // per int
        if int < 20 {
            mana + int
        } else {
            mana + 20 + (int - 20) * 15
        }
    }

    fn worker(&'a mut self, start: usize, iters_to_run: usize) {
        let mut iters = 0;

        // Allocate progress indicators which will track which id we're
        // trying for each slot
        let mut bf_prog = vec![0usize; Slot::MaxSlot as usize];
        let mut bf_slot = 0;
        
        let mut best_health  = 0;
        let mut best_mana    = 0;
        let mut best_healing = 0;

        let mut tmp = start;
        for &slot in BRUTE_FORCE_SLOTS {
            bf_prog[slot as usize] =
                tmp % (self.database[slot as usize].len() + 1);
            tmp /= self.database[slot as usize].len() + 1;
        }

        'done: loop {
            let mut number = 0;
            let mut below = 1;
            for &slot in BRUTE_FORCE_SLOTS {
                self.slots[slot as usize] =
                    self.database[slot as usize].get(bf_prog[slot as usize]);
                number += bf_prog[slot as usize] * below;
                below  *= self.database[slot as usize].len() + 1;
            }

            assert!(number >= start);

            let health  = self.health();
            let mana    = self.mana();
            let healing = self.healing();
            best_health = core::cmp::max(health, best_health);
            best_mana = core::cmp::max(mana, best_mana);
            best_healing = core::cmp::max(healing, best_healing);
            iters += 1;

            if iters >= iters_to_run {
                break;
            }

            if (iters & 0xffffff) == 0 {
                print!("Iters {:12.4} M of {:12.4} M\n",
                       iters as f64 / 1_000_000.,
                       iters_to_run as f64 / 1_000_000.);
            }

            let slot = BRUTE_FORCE_SLOTS[bf_slot] as usize;
            if bf_prog[slot] == self.database[slot].len() {
                // Clear the prior status
                for &prev in &BRUTE_FORCE_SLOTS[..bf_slot + 1] {
                    bf_prog[prev as usize] = 0;
                }

                loop {
                    bf_slot += 1;
                    if bf_slot == BRUTE_FORCE_SLOTS.len() {
                        break 'done;
                    }

                    let slot = BRUTE_FORCE_SLOTS[bf_slot] as usize;
                    if bf_prog[slot] < self.database[slot].len() {
                        bf_prog[slot] += 1;
                        break;
                    } else {
                        bf_prog[slot] = 0;
                    }
                }
                bf_slot = 0;
            } else {
                bf_prog[slot] += 1;
            }
        }

        print!("Best health  {:4}\n", best_health);
        print!("Best mana    {:4}\n", best_mana);
        print!("Best healing {:4}\n", best_healing);
    }

    fn brute_force(&'a mut self) {
        let mut total_combos = 1;
        for &slot in BRUTE_FORCE_SLOTS {
            total_combos *= self.database[slot as usize].len() + 1;
        }

        let mut workers = Vec::new();
        let mut tasking = total_combos;
        for wid in 0..NUM_WORKERS {
            let wt = core::cmp::min(tasking, (total_combos + 10000) / NUM_WORKERS);
            print!("Start {}\n", total_combos - tasking);
            workers.push(std::thread::spawn(move || {
                let mut player = Character::new();
                player.worker(total_combos - tasking, wt);
            }));
            
            tasking -= wt;
        }

        assert!(tasking == 0);

        for thr in workers {
            thr.join().unwrap();
        }
    }
}

fn main() {
    let mut player = Character::new();
    player.brute_force();
}

