use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RelatedContent {
    Figure(String),
}

#[derive(Debug, Deserialize)]
pub struct Section {
    pub content: String,
    pub related: RelatedContent,
}

#[derive(Debug, Deserialize)]
pub struct Chapter {
    pub sections: Vec<Section>,
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    use super::*;

    #[test]
    fn test_deserialize_with_related_figure() {
        static CHAPTER: &str = indoc! {r#"
            [[sections]]
            content = """
            # Hello world!
            """
            related = { figure = "https://picsum.photos/200/300" }
        "#};

        let chapter: Chapter = toml::from_str(CHAPTER).unwrap();

        insta::assert_debug_snapshot!(chapter, @r###"
        Chapter {
            sections: [
                Section {
                    content: "# Hello world!\n",
                    related: Figure(
                        "https://picsum.photos/200/300",
                    ),
                },
            ],
        }
        "###);
    }
}
