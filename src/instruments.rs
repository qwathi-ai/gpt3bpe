
#[cfg(test)]
mod flamegraph {
    #[test]
    #[ignore]
    fn generate_flamegraphs() {
        {
            let grapheme_guard = pprof::ProfilerGuardBuilder::default()
                .frequency(100000)
                .blocklist(&["libc", "libgcc", "pthread", "vdso"])
                .build()
                .unwrap();
            crate::bpe::unit::grapheme::test_grapheme_ascii();
            crate::bpe::unit::grapheme::test_grapheme_empty();
            crate::bpe::unit::grapheme::test_grapheme_with_numbers();
            crate::bpe::unit::grapheme::test_grapheme_special_chars();
            crate::bpe::unit::grapheme::test_grapheme_unicode();
            crate::bpe::unit::grapheme::test_grapheme_mixed();
            crate::bpe::unit::grapheme::test_grapheme_repeated();
            crate::bpe::unit::grapheme::test_grapheme_let_there_be_light();
            crate::bpe::unit::grapheme::test_grapheme_indivisible_values();
            crate::bpe::unit::grapheme::test_grapheme_pneumonoultramicroscopicsilicovolcanoconiosis();
            if let Ok(report) = grapheme_guard.report().build() {
                let file = std::fs::File::create("src/bpe/flamegraph/grapheme.svg").unwrap();
                let mut options = pprof::flamegraph::Options::default();
                options.image_width = Some(2500);
                report.flamegraph_with_options(file, &mut options).unwrap();
                println!("✅ Grapheme: flamegraph saved");
            } else {
                eprintln!("⚠️ Grapheme: Could not build report");
            }
        }

        {
            let tokens_guard = pprof::ProfilerGuardBuilder::default()
                .frequency(10000)
                .blocklist(&["libc", "libgcc", "pthread", "vdso"])
                .build()
                .unwrap();
            crate::bpe::unit::tokens::test_tokens_contraction();
            crate::bpe::unit::tokens::test_tokens_multiple_words();
            crate::bpe::unit::tokens::test_tokens_unicode();
            crate::bpe::unit::tokens::test_tokens_mixed();
            crate::bpe::unit::tokens::test_tokens_static();
            if let Ok(report) = tokens_guard.report().build() {
                let file = std::fs::File::create("src/bpe/flamegraph/tokens.svg").unwrap();
                let mut options = pprof::flamegraph::Options::default();
                options.image_width = Some(2500);
                report.flamegraph_with_options(file, &mut options).unwrap();
                println!("✅ Tokens: flamegraph saved");
            } else {
                eprintln!("⚠️ Tokens: Could not build report");
            }
        }
        {
            let encoder_guard = pprof::ProfilerGuardBuilder::default()
                .frequency(1000)
                .blocklist(&["libc", "libgcc", "pthread", "vdso"])
                .build()
                .unwrap();
            crate::bpe::unit::encoder::test_encode_empty();
            crate::bpe::unit::encoder::test_encode_ascii();
            crate::bpe::unit::encoder::test_encode_unicode();
            crate::bpe::unit::encoder::test_encode_mixed();
            crate::bpe::unit::encoder::test_encode_let_there_be_light();
            crate::bpe::unit::encoder::test_encode_indivisible_values();
            crate::bpe::unit::encoder::test_encode_pneumonoultramicroscopicsilicovolcanoconiosis();
            if let Ok(report) = encoder_guard.report().build() {
                let file = std::fs::File::create("src/bpe/flamegraph/encoder.svg").unwrap();
                let mut options = pprof::flamegraph::Options::default();
                options.image_width = Some(2500);
                report.flamegraph_with_options(file, &mut options).unwrap();
                println!("✅ Encoder: flamegraph saved");
            } else {
                eprintln!("⚠️ Encoder: Could not build report");
            }
        }
        {
            let decoder_guard = pprof::ProfilerGuardBuilder::default()
                .frequency(100000)
                .blocklist(&["libc", "libgcc", "pthread", "vdso"])
                .build()
                .unwrap();
            // super::decoder::test_decode_mixed();
            crate::bpe::unit::decoder::test_decode_let_there_be_light();
            crate::bpe::unit::decoder::test_decode_indivisible_values();
            crate::bpe::unit::decoder::test_decode_pneumonoultramicroscopicsilicovolcanoconiosis();
            if let Ok(report) = decoder_guard.report().build() {
                let file = std::fs::File::create("src/bpe/flamegraph/decoder.svg").unwrap();
                let mut options = pprof::flamegraph::Options::default();
                options.image_width = Some(2500);
                report.flamegraph_with_options(file, &mut options).unwrap();
                println!("✅ Decoder: flamegraph saved");
            } else {
                eprintln!("⚠️ Decoder: Could not build report");
            }
        }

        {
            let grapheme_guard = pprof::ProfilerGuardBuilder::default()
                .frequency(2500)
                .blocklist(&["libc", "libgcc", "pthread", "vdso"])
                .build()
                .unwrap();
            crate::cli::unit::grapheme::test_grapheme_split();
            if let Ok(report) = grapheme_guard.report().build() {
                let file = std::fs::File::create("src/cli/flamegraph/grapheme.svg").unwrap();
                let mut options = pprof::flamegraph::Options::default();
                options.image_width = Some(2500);
                report.flamegraph_with_options(file, &mut options).unwrap();
                println!("✅ Grapheme: flamegraph saved");
            } else {
                eprintln!("⚠️ Grapheme: Could not build report");
            }
        }
        {
            let decoder_guard = pprof::ProfilerGuardBuilder::default()
                .frequency(100000)
                .blocklist(&["libc", "libgcc", "pthread", "vdso"])
                .build()
                .unwrap();
            // super::decoder::test_decode_mixed();
            crate::cli::unit::decoder::test_decode_p50k();
            crate::cli::unit::decoder::test_decode_cl100k();
            crate::cli::unit::decoder::test_decode_empty_input();
            crate::cli::unit::decoder::test_decode_input_with_whitespace();
            crate::cli::unit::decoder::test_decode_invalid_token_is_ignored();
            if let Ok(report) = decoder_guard.report().build() {
                let file = std::fs::File::create("src/cli/flamegraph/decoder.svg").unwrap();
                let mut options = pprof::flamegraph::Options::default();
                options.image_width = Some(2500);
                report.flamegraph_with_options(file, &mut options).unwrap();
                println!("✅ Decoder: flamegraph saved");
            } else {
                eprintln!("⚠️ Decoder: Could not build report");
            }
        }
        {
            let arguments_guard = pprof::ProfilerGuardBuilder::default()
                .frequency(1000)
                .blocklist(&["libc", "libgcc", "pthread", "vdso"])
                .build()
                .unwrap();
            crate::cli::unit::arguments::test_argh_parsing();
            if let Ok(report) = arguments_guard.report().build() {
                let file = std::fs::File::create("src/cli/flamegraph/arguments.svg").unwrap();
                let mut options = pprof::flamegraph::Options::default();
                options.image_width = Some(100000);
                report.flamegraph_with_options(file, &mut options).unwrap();
                println!("✅ Arguments: flamegraph saved");
            } else {
                eprintln!("⚠️ Arguments: Could not build report");
            }
        }

        {
            let padding_guard = pprof::ProfilerGuardBuilder::default()
                .frequency(2500)
                .blocklist(&["libc", "libgcc", "pthread", "vdso"])
                .build()
                .unwrap();
            crate::embeddings::unit::padding::test_padding_empty();
            crate::embeddings::unit::padding::test_padding_smaller();
            crate::embeddings::unit::padding::test_padding_equal();
            crate::embeddings::unit::padding::test_padding_larger();
            if let Ok(report) = padding_guard.report().build() {
                let file = std::fs::File::create("src/embeddings/flamegraph/padding.svg").unwrap();
                let mut options = pprof::flamegraph::Options::default();
                options.image_width = Some(2500);
                report.flamegraph_with_options(file, &mut options).unwrap();
                println!("✅ Padding: flamegraph saved");
            } else {
                eprintln!("⚠️ Padding: Could not build report");
            }
        }
        // {
        //     let insert_guard = pprof::ProfilerGuardBuilder::default()
        //         .frequency(100000)
        //         .blocklist(&["libc", "libgcc", "pthread", "vdso"])
        //         .build()
        //         .unwrap();
        //     crate::embeddings::unit::insert::test_insert_empty_string();
        //     crate::embeddings::unit::insert::test_insert_constraint_violation();
        //     if let Ok(report) = insert_guard.report().build() {
        //         let file = std::fs::File::create("src/embeddings/flamegraph/insert.svg").unwrap();
        //         let mut options = pprof::flamegraph::Options::default();
        //         options.image_width = Some(2500);
        //         report.flamegraph_with_options(file, &mut options).unwrap();
        //         println!("✅ Insert: flamegraph saved");
        //     } else {
        //         eprintln!("⚠️ Insert: Could not build report");
        //     }
        // }
        {
            let search_guard = pprof::ProfilerGuardBuilder::default()
                .frequency(1000)
                .blocklist(&["libc", "libgcc", "pthread", "vdso"])
                .build()
                .unwrap();
            // crate::embeddings::unit::search::test_search_empty_string();
            // crate::embeddings::unit::search::test_search_zero_k();
            crate::embeddings::unit::search::test_search();
            if let Ok(report) = search_guard.report().build() {
                let file = std::fs::File::create("src/embeddings/flamegraph/search.svg").unwrap();
                let mut options = pprof::flamegraph::Options::default();
                options.image_width = Some(100000);
                report.flamegraph_with_options(file, &mut options).unwrap();
                println!("✅ Search: flamegraph saved");
            } else {
                eprintln!("⚠️ Search: Could not build report");
            }
        }
        {
            let nearest_guard = pprof::ProfilerGuardBuilder::default()
                .frequency(1000)
                .blocklist(&["libc", "libgcc", "pthread", "vdso"])
                .build()
                .unwrap();
            // crate::embeddings::unit::nearest::test_nearest_zero_k();
            crate::embeddings::unit::nearest::test_nearest();
            if let Ok(report) = nearest_guard.report().build() {
                let file = std::fs::File::create("src/embeddings/flamegraph/nearest.svg").unwrap();
                let mut options = pprof::flamegraph::Options::default();
                options.image_width = Some(100000);
                report.flamegraph_with_options(file, &mut options).unwrap();
                println!("✅ Search: flamegraph saved");
            } else {
                eprintln!("⚠️ Search: Could not build report");
            }
        }

    }
}
