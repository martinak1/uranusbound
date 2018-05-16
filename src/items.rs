/* items.rs */
#![deny(missing_docs)]
//! Defines how an item is represented

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
/// Specifies the type of the item
pub enum ItemType {
    /// The armor item type. This represents sheilds, trinkets, and other typical armor pieces
    Armor,
    /// The consumable item type. This represents potions, food, poison, and other consumable items.
    Consumable,
    /// The weapon item type. This represents things that can be used as a weapon in game. This can
    /// include swords, sticks, and anything that we think up.
    Weapon,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
/// Specifies the effect of the item and the modifier. All hold an i32 that
/// represents the number to modify the stats by, except the Buff and Debuff
/// variants, which also hold a String. This string is used for pattern matching
/// when determining which stat it effects
pub enum Effect {
    /// Increases the targets HP
    Heal(i32),
    /// Reduces the targets HP
    Damage(i32),
    /// Increases one of the targets stats. Takes an i32 to represent the amount
    /// of the buff and a String to specify which stat
    Buff(i32, String),
    /// Decreases one of the targets stats. Takes an i32 to represent the amount
    /// of the debuff and a String to specify which stat
    Debuff(i32, String),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
/// A struct that represents all items in the game
pub struct Item {
    /// The name of the item
    name: String,
    /// The way the item is described to the player
    description: String,
    /// The effect the item has on a target
    effect: Effect,
    /// The type of the item
    item_type: ItemType,
    /// What the item is sold for
    value: i32,
}

// Related functions
impl Item {
    /// Creates a new item
    pub fn new(
        name: String,
        description: String,
        effect: Effect,
        item_type: ItemType,
        value: i32,
    ) -> Item {
        Item {
            name,
            description,
            effect,
            item_type,
            value,
        }
    }
}

// Item methods
impl Item {
    /// Returns the name of an item
    pub fn get_name(&self) -> String {
        self.name.clone()
    }
    /// Returns the effect of an item
    pub fn get_effect(&self) -> Effect {
        self.effect.clone()
    }
    /// Returns the description of an item
    pub fn get_description(&self) -> String {
        self.description.clone()
    }
    /// Returns the type of an item
    pub fn get_type(&self) -> ItemType {
        self.item_type.clone()
    }
    /// Returns the value of an item
    pub fn get_value(&self) -> i32 {
        self.value
    }
}
