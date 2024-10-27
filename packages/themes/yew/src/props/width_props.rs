use crate::props::prop_def::prop_optional_arbitary_responsive_string;

prop_optional_arbitary_responsive_string!(WidthProp, Some("rt-r-w"), Some(&["--width"]));
prop_optional_arbitary_responsive_string!(MinWidthProp, Some("rt-r-min-w"), Some(&["--min-width"]));
prop_optional_arbitary_responsive_string!(MaxWidthProp, Some("rt-r-max-w"), Some(&["--max-width"]));
