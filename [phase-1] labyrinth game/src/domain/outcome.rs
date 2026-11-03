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

    Won,
    Died(u8),
}
