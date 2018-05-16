/* inventory.rs */
#![deny(missing_docs)]
//! This represents the inventory for interactable objects

// For serializing inventory to json
extern crate serde_json;

// For the ability to sort items into the correct bag
use items::{Effect, Item, ItemType};

// For Serialization/Deserialization
use std::fs::File;

/// Inventory struct
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Inventory {
    /// An objects item bag
    bags: Bag,
    /// An interger that determines the amount of money held
    wallet: i32,
}

// Related Functions
impl Inventory {
    /// Returns an empty Inventory
    pub fn new() -> Inventory {
        Inventory {
            bags: Bag::new(),
            wallet: 0,
        }
    }

    pub fn test_inventory() {
        /// Tests the funcitonality of the inventory system
        //******************************************************************************
        // Start Inventory Test

        println!("\n\nStarting the Inventory tests\n\n");

        let potion = Item::new(
            String::from("Potion"),
            String::from("This item heals you"),
            Effect::Heal(15),
            ItemType::Consumable,
            15,
        );

        let mana_pot = Item::new(
            String::from("Mana Pot"),
            String::from("This item restores your mana"),
            Effect::Buff(15, String::from("Ap")),
            ItemType::Consumable,
            20,
        );

        let sword = Item::new(
            String::from("Sword"),
            String::from("A cheap iron sword"),
            Effect::Damage(25),
            ItemType::Weapon,
            35,
        );

        let sheild = Item::new(
            String::from("Sheild"),
            String::from("A cheap iron sheild"),
            Effect::Buff(10, String::from("Def")),
            ItemType::Armor,
            25,
        );

        let true_or_false = |result: Option<(usize, i32)>| -> bool {
            match result {
                Some((_, _)) => true,
                None => false,
            }
        };

        let mut inv: Inventory = Inventory::new();

        println!("Passed: Created items and inventory");

        // add dumby items to the inventory
        inv.add_item(5, &potion);
        inv.add_item(15, &mana_pot);
        inv.add_item(1, &sword);
        inv.add_item(1, &sheild);
        inv.add_money(100);

        // verify that the search alg works and that the items are sucessfully
        // sorted into the appropriate bag
        assert!(true_or_false(inv.has_item(&potion)), true);
        assert!(true_or_false(inv.has_item(&mana_pot)), true);
        assert!(true_or_false(inv.has_item(&sword)), true);
        assert!(true_or_false(inv.has_item(&sheild)), true);

        println!("Passed: Items added to inventory");

        // add duplicate items to verify that it updates the existing Entry instead
        // of making a new one
        inv.add_item(5, &potion);
        //assert!(inv.bags.consumable_bag.len() == , true);

        println!("Passed: Entered duplicate item and the existing entry was updated");

        // tests to see if the it handles the removal of more money that is gon hand
        inv.rm_money(600);
        assert!(inv.get_wallet() == 0, true);
        println!("Passed: Removed more money than was on hand and the wallet did not go negative");

        // removes more than is currently in inventory to see if the entry is
        // removed as intented, then adds them back and removes the exact qty
        inv.rm_item(16, &mana_pot);
        assert!(inv.bags.consumable_bag.len() == 1, true);
        println!("Passed: Removed an entry from the bags with a qty > on hand");

        inv.add_item(15, &mana_pot);
        assert!(inv.bags.consumable_bag.len() == 2, true);
        inv.rm_item(15, &mana_pot);
        assert!(inv.bags.consumable_bag.len() == 1, true);
        println!("Passed: Added then removed an entry of equal qty on hand from the bags");

        inv.rm_item(5, &potion);
        assert!(true_or_false(inv.has_item(&potion)), true);
        println!("Passed: Removed a partial quantity from inventory and the entry persisted");

        inv.save();
        let inv2 = Inventory::load(String::from("inventory.json"));
        let invstr = serde_json::to_string_pretty(&inv).unwrap();
        let inv2str = serde_json::to_string_pretty(&inv2).unwrap();

        assert!(inv == inv2, true);
        assert!(invstr == inv2str, true);
        println!("Passed: Serialized Inventory to JSON, then back again");

        println!("\n\n** All tests passed for the Inventory!\n\n");

        // end inventory test
        //******************************************************************************
    }
}

// Methods
impl Inventory {
    /// Adds an item to the appropriate bag
    pub fn add_item(&mut self, qty: i32, item: &Item) {
        self.bags.add_item(qty, item);
    }

    /// Determines if a particular item is in the inventory
    pub fn has_item(&self, item: &Item) -> Option<(usize, i32)> {
        match self.bags.find_item(item) {
            Some((location, qty_held)) => Some((location, qty_held)),
            None => None,
        }
    }

    /// Removes an item from inventory
    pub fn rm_item(&mut self, qty: i32, item: &Item) {
        let rm = |loc: usize, qty: i32, bag: &mut Vec<Entry>| {
            let remains = bag[loc].rm_qty(qty);

            if remains <= 0 {
                bag.swap_remove(loc);
            }
        };

        if let Some((location, qty_held)) = self.has_item(item) {
            match item.get_type() {
                ItemType::Armor => {
                    rm(location, qty, &mut self.bags.armor_bag);
                }
                ItemType::Consumable => {
                    rm(location, qty, &mut self.bags.consumable_bag);
                }
                ItemType::Weapon => {
                    rm(location, qty, &mut self.bags.weapon_bag);
                }
            }
        }
    }

    /// Adds money to the wallet
    pub fn add_money(&mut self, qty: i32) {
        self.wallet += qty;
    }

    /// Removes money from the inventory. If the wallet would drop bellow
    /// zero, it will be set to zero, otherwise it removes the quantity from
    /// the current amount
    pub fn rm_money(&mut self, qty: i32) {
        if self.wallet < qty {
            self.wallet = 0;
        } else {
            self.wallet -= qty;
        }
    }

    /// Returns the amount of money in the wallet
    pub fn get_wallet(&self) -> i32 {
        self.wallet
    }

    /// Serialize the inventory to a JSON file. This should be called when the player saves the game
    pub fn save(&self) {
        // Open the file for writing, fail if it doesn't open
        let file = File::create("inventory.json").unwrap();
        // write the generated json to the file
        serde_json::to_writer_pretty(file, self).unwrap();
    }

    /// Deserialize JSON into an Inventory Struct. This should be called when the player loads the game
    pub fn load(file_name: String) -> Inventory {
        // open the file to read from
        let file = File::open(file_name).unwrap();
        // deserialize the object from the file
        let inv: Inventory = serde_json::from_reader(file).unwrap();
        inv
    }
}

/// A struct that contains three vectors for each item type: Armor, Consumable, Weapon
#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct Bag {
    /// Vector that holds armor items
    armor_bag: Vec<Entry>,
    /// Vector that holds consumeable items
    consumable_bag: Vec<Entry>,
    /// Vector that holds weapon items
    weapon_bag: Vec<Entry>,
}

// Functions
impl Bag {
    /// A public function that returns an empty bag
    fn new() -> Bag {
        Bag {
            armor_bag: vec![],
            consumable_bag: vec![],
            weapon_bag: vec![],
        }
    }
}

// Methods
impl Bag {
    /// Defines how to add items together. Used to increase the quantity held
    //fn add

    /// Public function that adds an item to the correct item vector
    fn add_item(&mut self, qty: i32, item: &Item) {
        if let Some((location, _qty_held)) = self.find_item(&item) {
            match item.get_type() {
                ItemType::Armor => {
                    self.armor_bag[location].add_qty(qty);
                }
                ItemType::Consumable => {
                    self.consumable_bag[location].add_qty(qty);
                }
                ItemType::Weapon => {
                    self.weapon_bag[location].add_qty(qty);
                }
            }
        } else {
            match item.get_type() {
                ItemType::Armor => {
                    self.armor_bag.push(Entry::new(qty, item.clone()));
                }
                ItemType::Consumable => {
                    self.consumable_bag.push(Entry::new(qty, item.clone()));
                }
                ItemType::Weapon => {
                    self.weapon_bag.push(Entry::new(qty, item.clone()));
                }
            }
        }
    }

    /// Locates an item location in the appropriate ItemType vector. Return None if it is not found
    fn find_item(&self, item: &Item) -> Option<(usize, i32)> {
        let search = |thing: &Item, bag: &Vec<Entry>| -> Option<(usize, i32)> {
            if !bag.is_empty() {
                for (pos, entry) in bag.iter().enumerate() {
                    if entry.item == *thing {
                        return Some((pos, entry.get_qty()));
                    }
                }
            }
            return None;
        };

        match item.get_type() {
            ItemType::Armor => search(item, &self.armor_bag),
            ItemType::Consumable => search(item, &self.consumable_bag),
            ItemType::Weapon => search(item, &self.weapon_bag),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
/// Represents an item in inventory
struct Entry {
    /// The amount of the item in the bag
    qty: i32,
    /// The item held in the bag
    item: Item,
}

// Related functions
impl Entry {
    /// Creates a new Entry struct
    fn new(qty: i32, item: Item) -> Entry {
        Entry { qty, item }
    }
}
// Methods
impl Entry {
    /// Returns the name of the item in the Entry
    fn get_name(&self) -> String {
        self.item.get_name()
    }

    fn get_item(&self) -> &Item {
        &self.item
    }

    /// Returns the quantity held in the Entry
    fn get_qty(&self) -> i32 {
        self.qty
    }

    /// Adds to the quantity held in the Entry
    fn add_qty(&mut self, qty_to_add: i32) {
        self.qty += qty_to_add;
    }

    /// Reduces the quantity held in the Entry
    fn rm_qty(&mut self, qty_to_drop: i32) -> i32 {
        self.qty -= qty_to_drop;
        self.qty
    }
}
