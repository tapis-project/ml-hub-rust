#[cfg(test)]
mod value_objects_test {
    use crate::shared_kernel::value_objects::{
        Tag, TagError, Tags, TagsError, MAX_TAG_LENGTH_BYTES, MAX_TAGS,
    };

    #[test]
    fn accepts_tags_at_the_byte_limit() -> Result<(), Box<dyn std::error::Error>> {
        let value = "é".repeat(MAX_TAG_LENGTH_BYTES / 2);
        let tag = Tag::new(value)?;

        assert_eq!(tag.as_str().len(), MAX_TAG_LENGTH_BYTES);
        Ok(())
    }

    #[test]
    fn rejects_empty_and_overlong_tags() {
        assert!(matches!(Tag::new(String::new()), Err(TagError::Empty)));
        assert!(matches!(Tag::new("é".repeat(33)), Err(TagError::TooLong)));
    }

    #[test]
    fn bounds_the_tag_collection() -> Result<(), Box<dyn std::error::Error>> {
        let tags = Tags::new((0..MAX_TAGS).map(|index| format!("tag-{index}")).collect())?;
        assert_eq!(tags.len(), MAX_TAGS);

        let too_many = Tags::new((0..=MAX_TAGS).map(|index| format!("tag-{index}")).collect());
        assert!(matches!(too_many, Err(TagsError::TooMany)));
        Ok(())
    }
}
