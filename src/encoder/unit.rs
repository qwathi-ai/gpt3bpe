mod tests {
    fn from_vec(graph: Vec<&str>) -> Vec<Vec<u8>> {
        graph
        .iter()
        .map(|char| -> Vec<u8> {
            char.as_bytes().to_vec()
        })
        .collect::<Vec<Vec<u8>>>()
    }

    #[test]
    fn encode() {
        assert_eq!(
            crate::encoder::encode(&from_vec(vec![
                "l", "e", "t", "Ġ", "t", "h", "e", "r", "e", "Ġ", "b", "e", "Ġ", "l", "i", "g",
                "h", "t", "."
            ]))
            .unwrap().concat(),
            vec![1616, 612, 307, 1657, 13]
        );
        // assert_eq!(
        //     crate::encoder::encode(&from_vec(vec![
        //         "i", "n", "d", "i", "v", "i", "s", "i", "b", "l", "e", "Ġ", "v", "a", "l", "u",
        //         "e", "s"
        //     ]))
        //     .unwrap().concat(),
        //     vec![521, 452, 12843, 1988, 82]
        // );
        // assert_eq!(
        //     crate::encoder::encode(&from_vec(vec![
        //         "P", "n", "e", "u", "m", "o", "n", "o", "u", "l", "t", "r", "a", "m", "i", "c",
        //         "r", "o", "s", "c", "o", "p", "i", "c", "s", "i", "l", "i", "c", "o", "v", "o",
        //         "l", "c", "a", "n", "o", "c", "o", "n", "i", "o", "s", "i", "s"
        //     ])).unwrap().concat(),
        //     vec![
        //         47, 25668, 261, 25955, 859, 291, 4951, 22163, 72, 6359, 2403, 66, 709, 349, 5171,
        //         420, 78, 77, 952, 82, 72, 82
        //     ]
        // );
        // assert_eq!(
        //     crate::encoder::encode(&from_vec(vec![
        //         "h", "e", "l", "l", "o", "Ġ", "ð", "Ł", "ĳ", "ĭ", "Ġ", "w", "o", "r", "l", "d",
        //         "Ġ", "ð", "Ł", "Į", "į",
        //     ])).unwrap().concat(),
        //     vec![31373, 50169, 233, 995, 220, 172, 253, 234, 235]
        // );
    }
    #[test]
    fn decode() {
        assert_eq!(
            crate::encoder::decode(&vec![1616, 612, 307, 1657, 13]).unwrap(),
            from_vec(vec!["let", "Ġthere", "Ġbe", "Ġlight", "."])
        );
        assert_eq!(
            crate::encoder::decode(&vec![521, 452, 12843, 1988, 82]).unwrap(),
            from_vec(vec!["ind", "iv", "isible", "Ġvalue", "s"])
        );
        assert_eq!(
            crate::encoder::decode(&vec![
                47, 25668, 261, 25955, 859, 291, 4951, 22163, 72, 6359, 2403, 66, 709, 349, 5171,
                420, 78, 77, 952, 82, 72, 82
            ])
            .unwrap(),
            from_vec(vec![
                "P", "neum", "on", "oult", "ram", "ic", "ros", "cop", "i", "cs", "ili", "c", "ov",
                "ol", "can", "oc", "o", "n", "io", "s", "i", "s"
            ])
        );
        assert_eq!(
            crate::encoder::decode(&vec![31373, 50169, 233, 995, 220, 172, 253, 234, 235]).unwrap(),
            from_vec(vec!["hello", "ĠðŁĳ", "ĭ", "Ġworld", "Ġ", "ð", "Ł", "Į", "į"])
        );
    }
}
