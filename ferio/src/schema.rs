pub mod sections_schema {
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct SectionsRoot {
        pub parse: Parse,
    }

    #[derive(Debug, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Parse {
        pub title: String,
        pub pageid: i64,
        pub sections: Vec<Section>,
        pub showtoc: String,
    }

    #[derive(Debug, Serialize, Deserialize)]
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

    #[derive(Default, Debug, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct HolidayRoot {
        pub parse: Parse,
    }

    #[derive(Default, Debug, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Parse {
        pub title: String,
        pub pageid: i64,
        pub text: Text,
    }

    #[derive(Default, Debug, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Text {
        #[serde(rename = "*")]
        pub field: String,
    }
}

pub mod image_schema {
    use serde::{Deserialize, Serialize};
    use std::collections::HashMap;

    #[derive(Default, Debug, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct ImageRoot {
        pub batchcomplete: String,
        pub query: Query,
    }

    #[derive(Default, Debug, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Query {
        pub pages: HashMap<String, Page>,
    }

    #[derive(Default, Debug, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Page {
        pub pageid: i64,
        pub ns: i64,
        pub title: String,
        pub original: Original,
    }

    #[derive(Default, Debug, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Original {
        pub source: String,
        pub width: i64,
        pub height: i64,
    }
}
