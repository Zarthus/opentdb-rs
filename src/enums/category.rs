/// This is a local mapping of the response from the API request at
/// https://opentdb.com/api_category.php.
///
/// By specifying a value here that is not "Any", you will filter questions
/// by that category.
///
/// Author note:
///
/// Static data retrieved from https://opentdb.com/api_category.php
/// We will have to manually update this whenever it changes.
///
/// TODO: Future solutions might include a dynamic category, although the
/// TODO: builder completely allows for custom integers.
#[derive(Copy, Clone, Deserialize, Debug)]
pub enum Category {
    /// Do not filter for any category.
    Any = 0,

    GeneralKnowledge = 9,

    EntertainmentBooks = 10,
    EntertainmentFilm = 11,
    EntertainmentMusic = 12,
    EntertainmentMusicalsAndTheatres = 13,
    EntertainmentTelevision = 14,
    EntertainmentVideoGames = 15,
    EntertainmentBoardGames = 16,
    EntertainmentComics = 29,
    EntertainmentJapaneseAnimeAndManga = 31,
    EntertainmentCartoonAndAnimations = 32,

    ScienceNature = 17,
    ScienceComputers = 18,
    ScienceMathematics = 19,
    ScienceGadgets = 30,

    Mythology = 20,
    Sports = 21,
    Geography = 22,
    History = 23,
    Politics = 24,
    Art = 25,
    Celebrities = 26,
    Animals = 27,
    Vehicles = 28,
}

impl Default for Category {
    fn default() -> Self {
        Category::Any
    }
}
