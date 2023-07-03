pub const SELECT_CATEGORY_IDX: &'static str = "
    SELECT
        category_idx
    FROM
        types
    ORDER BY
        idx ASC
";

pub const SELECT_CATEGORY_NAME: &'static str = "
    SELECT
        name
    FROM
        category_names
    WHERE
        language = ?
    ORDER BY
        idx ASC
";

pub const SELECT_GROUP_IDX: &'static str = "
    SELECT
        group_idx
    FROM
        types
    ORDER BY
        idx ASC
";

pub const SELECT_GROUP_NAME: &'static str = "
    SELECT
        name
    FROM
        group_names
    WHERE
        language = ?
    ORDER BY
        idx ASC
";

pub const SELECT_MARKET_GROUP_IDX: &'static str = "
    SELECT
        market_group_idx
    FROM
        types
    ORDER BY
        idx ASC
";

pub const SELECT_MARKET_GROUP_NAME: &'static str = "
    SELECT
        name
    FROM
        market_group_names
    WHERE
        language = ?
    ORDER BY
        idx ASC
";

pub const SELECT_TYPE_NAME: &'static str = "
    SELECT
        name
    FROM
        type_names
    WHERE
        language = ?
    ORDER BY
        idx ASC
";

pub const SELECT_TYPE_ID: &'static str = "
    SELECT
        type_id
    FROM
        types
    ORDER BY
        idx ASC
";
