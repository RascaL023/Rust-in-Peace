pub enum Outcome {
    Moved,
    HitWall,
    OutOfMap,

    TookKey,
    OpenedDoor,
    DoorLocked,

    Hurt(u8),
    Healed(u8),

    Won,
    Died,
}
