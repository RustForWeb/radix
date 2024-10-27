use crate::props::prop_def::prop_optional_arbitary_responsive_string;

prop_optional_arbitary_responsive_string!(HeightProp, Some("rt-r-h"), Some(&["--height"]));
prop_optional_arbitary_responsive_string!(
    MinHeightProp,
    Some("rt-r-min-h"),
    Some(&["--min-height"])
);
prop_optional_arbitary_responsive_string!(
    MaxHeightProp,
    Some("rt-r-max-h"),
    Some(&["--max-height"])
);
