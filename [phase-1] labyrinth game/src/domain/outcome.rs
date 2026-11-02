pub enum Outcome {
    Moved,
    HitWall,
    OutOfMap,

    TookKey,
    TookPotion,
    OpenedDoor,
    DoorLocked,

    Hurt(u8),
    Healed(u8),

    DontHavePotion,
    InventoryFull,
    ShowInventory([String; 2]),

    Won,
    Died(u8),
}
