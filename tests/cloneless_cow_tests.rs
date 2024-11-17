#[cfg(test)]
mod cloneless_cow_tests {
    use cloneless_cow::ClonelessCow;

    #[derive(Clone)]
    struct SomethingClone {
        byte: u8,
    }

    struct SomethingNotClone {
        byte: u8,
    }

    #[test]
    fn test_e5b1_basic_cloneless_owned() {
        let original_byte = 1u8;
        let obj = ClonelessCow::Owned(SomethingNotClone {
            byte: original_byte,
        });
        let referenced_byte = obj.as_ref().byte;
        assert_eq!(original_byte, referenced_byte);
    }

    #[test]
    fn test_o4g6_basic_cloneless_reference() {
        let original_byte = 1u8;

        // sometimes the instance is owned elsewhere
        let owned = SomethingNotClone {
            byte: original_byte,
        };

        let obj = ClonelessCow::Borrowed(&owned);
        let referenced_byte = obj.as_ref().byte;
        assert_eq!(original_byte, referenced_byte);
    }

    #[test]
    fn test_j3f9_basic_clone_owned() {
        let original_byte = 1u8;
        let obj = ClonelessCow::Owned(SomethingClone {
            byte: original_byte,
        });
        let referenced_byte = obj.as_ref().byte;
        assert_eq!(original_byte, referenced_byte);
    }

    #[test]
    fn test_w7x1_basic_cloneless_reference() {
        let original_byte = 1u8;

        // sometimes the instance is owned elsewhere
        let owned = SomethingClone {
            byte: original_byte,
        };

        let obj = ClonelessCow::Borrowed(&owned);
        let referenced_byte = obj.as_ref().byte;
        assert_eq!(original_byte, referenced_byte);
    }
}