use le_stream::{FromLeStream, ToLeStream};
use num_derive::FromPrimitive;
use num_traits::FromPrimitive;

/// The physical environment attribute.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, FromPrimitive)]
#[repr(u8)]
pub enum PhysicalEnvironment {
    /// The physical environment is unspecified.
    Unspecified = 0x00,
    /// The physical environment is a mirror or atrium.
    MirrorOrAtrium = 0x01,
    /// The physical environment is a bar.
    Bar = 0x02,
    /// The physical environment is a courtyard.
    Courtyard = 0x03,
    /// The physical environment is a bathroom.
    Bathroom = 0x04,
    /// The physical environment is a bedroom.
    Bedroom = 0x05,
    /// The physical environment is a billiard room.
    BilliardRoom = 0x06,
    /// The physical environment is a utility room.
    UtilityRoom = 0x07,
    /// The physical environment is a cellar.
    Cellar = 0x08,
    /// The physical environment is a storage closet.
    StorageCloset = 0x09,
    /// The physical environment is a theater.
    Theater = 0x0a,
    /// The physical environment is an office.
    Office = 0x0b,
    /// The physical environment is a deck.
    Deck = 0x0c,
    /// The physical environment is a den.
    Den = 0x0d,
    /// The physical environment is a dining room.
    DiningRoom = 0x0e,
    /// The physical environment is an electrical room.
    ElectricalRoom = 0x0f,
    /// The physical environment is an elevator.
    Elevator = 0x10,
    /// The physical environment is an entryway.
    Entry = 0x11,
    /// The physical environment is a family room.
    FamilyRoom = 0x12,
    /// The physical environment is a main floor.
    MainFloor = 0x13,
    /// The physical environment is an upstairs area.
    Upstairs = 0x14,
    /// The physical environment is a downstairs area.
    Downstairs = 0x15,
    /// The physical environment is a basement or lower level.
    BasementOrLowerLevel = 0x16,
    /// The physical environment is a gallery.
    Gallery = 0x17,
    /// The physical environment is a game room.
    GameRoom = 0x18,
    /// The physical environment is a garage.
    Garage = 0x19,
    /// The physical environment is a gym.
    Gym = 0x1a,
    /// The physical environment is a hallway.
    Hallway = 0x1b,
    /// The physical environment is a house.
    House = 0x1c,
    /// The physical environment is a kitchen.
    Kitchen = 0x1d,
    /// The physical environment is a laundry room.
    LaundryRoom = 0x1e,
    /// The physical environment is a library.
    Library = 0x1f,
    /// The physical environment is a master bedroom.
    MasterBedroom = 0x20,
    /// The physical environment is a mud room (a small room for coats and boots).
    MudRoom = 0x21,
    /// The physical environment is a nursery.
    Nursery = 0x22,
    /// The physical environment is a pantry.
    Pantry = 0x23,
    /// Alternate office.
    ///
    /// TODO: What is the semantic difference between this and `Office`?
    OfficeAlt = 0x24,
    /// The physical environment is an outside area.
    Outside = 0x25,
    /// The physical environment is a pool.
    Pool = 0x26,
    /// The physical environment is a porch.
    Porch = 0x27,
    /// The physical environment is a room for sewing.
    SewingRoom = 0x28,
    /// The physical environment is a sitting room.
    SittingRoom = 0x29,
    /// The physical environment is a stairway.
    Stairway = 0x2a,
    /// The physical environment is a yard.
    Yard = 0x2b,
    /// The physical environment is an attic.
    Attic = 0x2c,
    /// The physical environment is a hot tub.
    HotTub = 0x2d,
    /// The physical environment is a living room.
    LivingRoom = 0x2e,
    /// The physical environment is a sauna.
    Sauna = 0x2f,
    /// The physical environment is a shop or workshop.
    ShopOrWorkshop = 0x30,
    /// The physical environment is a guest bedroom.
    GuestBedroom = 0x31,
    /// The physical environment is a guest bathroom.
    GuestBath = 0x32,
    /// The physical environment is a powder room (a small bathroom).
    PowderRoom = 0x33,
    /// The physical environment is a backyard.
    BackYard = 0x34,
    /// The physical environment is a front yard.
    FrontYard = 0x35,
    /// The physical environment is a patio.
    Patio = 0x36,
    /// The physical environment is a driveway.
    Driveway = 0x37,
    /// The physical environment is a sun room (a room with large windows).
    SunRoom = 0x38,
    /// Alternate living room.
    ///
    /// TODO: What is the semantic difference between this and `LivingRoom`?
    LivingRoomAlt = 0x39,
    /// The physical environment is a spa.
    Spa = 0x3a,
    /// The physical environment is a whirlpool.
    Whirlpool = 0x3b,
    /// The physical environment is a shed.
    Shed = 0x3c,
    /// The physical environment is a storage room for equipment.
    EquipmentStorage = 0x3d,
    /// The physical environment is a hobby or craft room.
    HobbyOrCraftRoom = 0x3e,
    /// The physical environment is a fountain.
    Fountain = 0x3f,
    /// The physical environment is a reception room.
    Pond = 0x40,
    /// The physical environment is a breakfast room.
    ReceptionRoom = 0x41,
    /// The physical environment is a nook (a small corner or recess).
    BreakfastRoom = 0x42,
    /// The physical environment is a garden nook.
    Nook = 0x43,
    /// The physical environment is a garden.
    Garden = 0x44,
    /// The physical environment is a balcony.
    Balcony = 0x45,
    /// The physical environment is a panic room (a fortified room for safety).
    PanicRoom = 0x46,
    /// The physical environment is a terrace.
    Terrace = 0x47,
    /// The physical environment is a roof.
    Roof = 0x48,
    /// The physical environment is a toilet.
    Toilet = 0x49,
    /// The physical environment is the main area of a toilet.
    ToiletMain = 0x4a,
    /// The physical environment is an outside toilet.
    OutsideToilet = 0x4b,
    /// The physical environment is a shower room.
    ShowerRoom = 0x4c,
    /// The physical environment is a study or home office.
    Study = 0x4d,
    /// The physical environment is a front garden.
    FrontGarden = 0x4e,
    /// The physical environment is a back garden.
    BackGarden = 0x4f,
    /// The physical environment is a kettle.
    Kettle = 0x50,
    /// The physical environment is a television.
    Television = 0x51,
    /// The physical environment is a stove.
    Stove = 0x52,
    /// The physical environment is a microwave oven.
    Microwave = 0x53,
    /// The physical environment is a toaster.
    Toaster = 0x54,
    /// The physical environment is a vacuum cleaner.
    Vacuum = 0x55,
    /// The physical environment is a kitchen appliance.
    Appliance = 0x56,
    /// The physical environment is a front door.
    FrontDoor = 0x57,
    /// The physical environment is a back door.
    BackDoor = 0x58,
    /// The physical environment is a fridge door.
    FridgeDoor = 0x59,
    /// The physical environment is a medication cabinet door.
    MedicationCabinetDoor = 0x60,
    /// The physical environment is a wardrobe door.
    WardrobeDoor = 0x61,
    /// The physical environment is a front cupboard door.
    FrontCupboardDoor = 0x62,
    /// The physical environment is another door.
    OtherDoor = 0x63,
    /// The physical environment is a waiting room.
    WaitingRoom = 0x64,
    /// The physical environment is a triage room.
    TriageRoom = 0x65,
    /// The physical environment is a doctor's office.
    DoctorsOffice = 0x66,
    /// The physical environment is a private room for patients.
    PatientsPrivateRoom = 0x67,
    /// The physical environment is a consultation room.
    ConsultationRoom = 0x68,
    /// The physical environment is a nurse station.
    NurseStation = 0x69,
    /// The physical environment is a hospital ward.
    Ward = 0x6a,
    /// The physical environment is a corridor in a hospital.
    Corridor = 0x6b,
    /// The physical environment is an operating theater.
    OperatingTheater = 0x6c,
    /// The physical environment is a dental surgery room.
    DentalSurgeryRoom = 0x6d,
    /// The physical environment is a medical imaging room.
    MedicalImagingRoom = 0x6e,
    /// The physical environment is a decontamination room.
    DecontaminationRoom = 0x6f,
    /// The physical environment is unknown.
    Unknown = 0xff,
}

impl FromLeStream for PhysicalEnvironment {
    fn from_le_stream<T>(bytes: T) -> Option<Self>
    where
        T: Iterator<Item = u8>,
    {
        u8::from_le_stream(bytes).and_then(Self::from_u8)
    }
}

impl ToLeStream for PhysicalEnvironment {
    type Iter = <u8 as ToLeStream>::Iter;

    fn to_le_stream(self) -> Self::Iter {
        (self as u8).to_le_stream()
    }
}
