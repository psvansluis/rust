pub struct Player {
    pub health: u32,
    pub mana: Option<u32>,
    pub level: u32,
}

impl Player {
    pub fn revive(&self) -> Option<Player> {
        if self.health == 0 {
            Some(Player {
                health: 100,
                mana: if self.level >= 10 { Some(100) } else { None },
                level: self.level,
            })
        } else {
            None
        }
    }

    pub fn cast_spell(&mut self, mana_cost: u32) -> u32 {
        match self.mana {
            Some(mana) if mana_cost > mana => 0,
            Some(mana) => {
                self.mana = Some(mana - mana_cost);
                mana_cost * 2
            }
            None => {
                self.health -= std::cmp::min(self.health, mana_cost);
                0
            }
        }
    }
}
