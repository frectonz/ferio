pub mod sections_schema {
    use serde::{Deserialize, Serialize};

    #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct SectionsRoot {
        pub parse: Parse,
    }

    #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Parse {
        pub title: String,
        pub pageid: i64,
        pub sections: Vec<Section>,
        pub showtoc: String,
    }

    #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Section {
        pub toclevel: i64,
        pub level: String,
        pub line: String,
        pub number: String,
        pub index: String,
        pub fromtitle: String,
        pub byteoffset: i64,
        pub anchor: String,
    }
}

pub mod holidays_schema {
    use serde::{Deserialize, Serialize};

    #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct HolidayRoot {
        pub parse: Parse,
    }

    #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Parse {
        pub title: String,
        pub pageid: i64,
        pub text: Text,
    }

    #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Text {
        #[serde(rename = "*")]
        pub field: String,
    }
}
