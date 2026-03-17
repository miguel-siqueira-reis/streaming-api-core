use sea_query::Iden;

#[derive(Iden)]
pub enum Anime {
    Table,
    Id,
    Name,
}

#[derive(Iden)]
pub enum Shows {
    Table,
    Id,
    Title,
    AlternativeTitles,
    Slug,
    Synopsis,
    CoverImageUrl,
    Status,
    Metadata,
    CreatedAt,
    UpdatedAt,
}

#[derive(Iden)]
pub enum Episodes {
    Table,
    Id,
    ShowId,
    Number,
    Title,
    VideoPath,
    TranscodeStatus,
    Metadata,
    CreatedAt,
    UpdatedAt,
}

#[derive(Iden)]
pub enum Genders {
    Table,
    Id,
    Name,
}

#[derive(Iden)]
pub enum ShowGenders {
    Table,
    Id,
    ShowId,
    GenderId,
}
