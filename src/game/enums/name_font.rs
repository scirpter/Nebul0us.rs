pub enum NameFont {
    DEFAULT,
    xxraytid,
    xtrusion,
    xxon,
    xefus,
    xenophobia,
    xenowort,
    cenobyte,
    nm_hero,
    xmas,
    xlines,
    xerox_malfunction,
    kaushan_script,
    ball,
    larson,
    superhet,
    gettheme,
    dephun2,
    christmas,
    fire,
    beyno,
    kingthings,
}

impl Default for NameFont {
    fn default() -> Self {
        NameFont::DEFAULT
    }
}

impl Copy for NameFont {}

impl Clone for NameFont {
    fn clone(&self) -> Self {
        *self
    }
}
