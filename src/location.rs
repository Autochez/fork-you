use crate::RangedU8;
use std::fmt::{self, Debug, Display, Formatter, Write};

/// A block at the Highfield school.
///
/// *See the [`crate`] documentation for more information*
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HighfieldBlock {
    Howard,
    Parker,
    Unwin,
}

impl Display for HighfieldBlock {
    // Format the HighfieldBlock (use that block's identifier)
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        use HighfieldBlock::*;

        match self {
            Howard => formatter.write_char('H'),
            Parker => formatter.write_char('P'),
            Unwin => formatter.write_char('U'),
        }
    }
}

/// A floor of a [`HighfieldBlock`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HighfieldFloor {
    /// The ground floor of a block.
    Ground,

    /// An upper level of a block.
    ///
    /// # Remarks
    ///
    /// The level must be in the range `1..=9` -- as the [`Self::Ground`] option
    /// is offered, there is no need for a level `0`, and the Highfield school
    /// will never have more than `9` levels.
    Level(RangedU8<1, 9>),
}

impl Display for HighfieldFloor {
    // Format the HighfieldFloor
    // Use 'G' for the ground floor and the floor number for others
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Ground => formatter.write_char('G'),
            Self::Level(level) => write!(formatter, "{}", level.get()),
        }
    }
}

/// A room at the Highfield school.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
// non_exhaustive is used for two reasons:
//  1. An exhaustive list of all of Highfield's rooms has not yet been
//     obtained
//  2. New rooms could be created at Highfield.
#[non_exhaustive]
pub enum HighfieldRoom {
    /// The hall at Highfield (in which assemblies can be held).
    Hall,

    /// The sports hall (generally used for P.E.).
    SportsHall,

    /// A classroom at the Highfield school.
    Classroom {
        /// The block in which the room is located.
        block: HighfieldBlock,

        /// The floor on which the room is located.
        floor: HighfieldFloor,

        /// The discriminator of the room.
        ///
        /// This is used to give each room a unique identity -- without a
        /// discriminator, it would be impossible to distinguish between two
        /// rooms on the same floor of the same block.
        ///
        /// *See the [`crate`] documentation for more information*.
        discriminator: RangedU8<1, 99>,
    },
}

impl Display for HighfieldRoom {
    // Format the HighfieldRoom such that it prints its room identifier
    //
    // See the crate level documentation for more information
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        use HighfieldRoom::*;

        match self {
            Hall => formatter.write_str("Hall"),
            SportsHall => formatter.write_str("Sports Hall"),
            Classroom {
                block,
                floor,
                discriminator,
            } => {
                Display::fmt(block, formatter)?;
                Display::fmt(floor, formatter)?;

                // Format the room number such that it is padded to two digits
                //
                // For example:
                // `1` will formatted as `01`
                // `27` will formatted as `27`
                // `108` is outside the range for the RangedU8, and we therefore do not
                // have to worry about it
                write!(formatter, "{:0>2}", discriminator.get())
            }
        }
    }
}

/// A section at the Fearnhill school.
///
/// *See the [`crate`] documentation for more information*.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FearnhillSection {
    Science,
    Business,
    PSHE,
    Languages,
    Technology,
    Mathematics,
    English,
    Music,
    Humanities,
    IT,
}

impl Display for FearnhillSection {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        use FearnhillSection::*;

        match self {
            Science => formatter.write_str("S"),
            Business => formatter.write_str("B"),
            PSHE => formatter.write_str("P"),
            Languages => formatter.write_str("L"),
            Technology => formatter.write_str("T"),
            Mathematics => formatter.write_str("M"),
            English => formatter.write_str("E"),
            Music => formatter.write_str("Mu"),
            Humanities => formatter.write_str("H"),
            IT => formatter.write_str("I"),
        }
    }
}

/// A room at the Fearnhill school.
///
/// *See the [`crate`] documentation for more information*.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
// non_exhaustive is used for two reasons:
//  1. An exhaustive list of all Fearnhill's rooms has yet to be obtained
//  2. Fearnhill may add additional rooms at any time (and, as a result,
//     new variants may need to be added to the enumeration)
#[non_exhaustive]
pub enum FearnhillRoom {
    /// The sports hall at Fearnhill (primarily used for P.E.).
    SportsHall,

    /// The gym at Fearnhill (primarily used for P.E.).
    Gym,

    /// The dance studio at Fearnhill.
    DanceStudio,

    /// The drama studio at Fearnhill.
    DramaStudio,

    /// A classroom at Fearnhill.
    ///
    /// *See the [`crate`] documentation for more information*.
    Classroom {
        /// The section in which the classroom is located.
        section: FearnhillSection,

        /// The discriminator of the classroom.
        ///
        /// This is used to assign each classroom a unique identity (i.e.,
        /// such that two classrooms in the same section have different
        /// identifiers).
        discriminator: RangedU8<1, 99>,
    },
}

impl Display for FearnhillRoom {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        use FearnhillRoom::*;

        match self {
            SportsHall => formatter.write_str("Sports Hall"),
            Gym => formatter.write_str("Gym"),
            DanceStudio => formatter.write_str("Dance Studio"),
            DramaStudio => formatter.write_str("Drama Studio"),
            Classroom {
                section,
                discriminator,
            } => {
                Display::fmt(section, formatter)?;
                Display::fmt(&discriminator.get(), formatter)
            }
        }
    }
}

/// A location of a room (in which a lesson can take place) in either the
/// Highfield school or the Fearnhill school.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Location {
    /// The location of a room at the Highfield school.
    Highfield(HighfieldRoom),

    /// The location of a room at the Fearnhill school.
    Fearnhill(FearnhillRoom),
}

impl Display for Location {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Highfield(room) => Display::fmt(room, formatter),
            Self::Fearnhill(room) => {
                // Prepend "FH " to all Fearnhill rooms for disambiguation
                // For example, both Highfield and Fearnhill have a
                // "Sports Hall" -- to prevent Fearnhill's sports hall from
                // being mistaken as Highfield's, format the identifier as
                // "FH <room identifier>"
                formatter.write_str("FH ")?;
                Display::fmt(room, formatter)
            }
        }
    }
}
