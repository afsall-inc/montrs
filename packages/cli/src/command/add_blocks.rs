// @generated - embedded MontRS website block registry for `montrs add`.
// Sources are the canonical files in apps/website/app/src/blocks/<fam>/<name>.rs.

macro_rules! embed_block {
    ($fam:literal, $name:literal) => {
        include_str!(concat!(
            "../../../../apps/website/app/src/blocks/",
            $fam,
            "/",
            $name,
            ".rs"
        ))
    };
}

pub const BLOCKS: &[(&str, &str)] = &[
    ("faq01", embed_block!("faq", "faq01")),
    ("faq02", embed_block!("faq", "faq02")),
    ("faq03", embed_block!("faq", "faq03")),
    ("footer_logos", embed_block!("footer", "footer_logos")),
    ("footer01", embed_block!("footer", "footer01")),
    ("footer02", embed_block!("footer", "footer02")),
    ("footer03", embed_block!("footer", "footer03")),
    ("footer04", embed_block!("footer", "footer04")),
    ("footer05", embed_block!("footer", "footer05")),
    ("header01", embed_block!("header", "header01")),
    (
        "integration01",
        embed_block!("integration", "integration01"),
    ),
    (
        "integration02",
        embed_block!("integration", "integration02"),
    ),
    (
        "integration03",
        embed_block!("integration", "integration03"),
    ),
    (
        "integration04",
        embed_block!("integration", "integration04"),
    ),
    (
        "integration05",
        embed_block!("integration", "integration05"),
    ),
    (
        "integration06",
        embed_block!("integration", "integration06"),
    ),
    (
        "integration07",
        embed_block!("integration", "integration07"),
    ),
    ("login01", embed_block!("login", "login01")),
    ("login02", embed_block!("login", "login02")),
    ("login03", embed_block!("login", "login03")),
    ("login04", embed_block!("login", "login04")),
    (
        "sidenav_inset_right",
        embed_block!("sidenav", "sidenav_inset_right"),
    ),
    ("sidenav_routes", embed_block!("sidenav", "sidenav_routes")),
    (
        "sidenav_routes_selector",
        embed_block!("sidenav", "sidenav_routes_selector"),
    ),
    (
        "sidenav_routes_simplified",
        embed_block!("sidenav", "sidenav_routes_simplified"),
    ),
    ("sidenav01", embed_block!("sidenav", "sidenav01")),
    ("sidenav02", embed_block!("sidenav", "sidenav02")),
    ("sidenav03", embed_block!("sidenav", "sidenav03")),
    ("sidenav04", embed_block!("sidenav", "sidenav04")),
    ("sidenav05", embed_block!("sidenav", "sidenav05")),
    ("sidenav06", embed_block!("sidenav", "sidenav06")),
    ("sidenav07", embed_block!("sidenav", "sidenav07")),
    ("sidenav08", embed_block!("sidenav", "sidenav08")),
    ("sidenav09", embed_block!("sidenav", "sidenav09")),
    ("sidenav10", embed_block!("sidenav", "sidenav10")),
    ("sidenav11", embed_block!("sidenav", "sidenav11")),
];
