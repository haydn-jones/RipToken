#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use crate::{
        encoding::{par_encode_dataset},
        utils::{count_token_occurence, generate_ngrams, read_file},
        vocab::Vocab,
        test_utils::{setup, cleanup},
    };

    #[test]
    fn read_file_test() {
        // assert that file contents is read into a list of the lines
        setup();
        let f = read_file("./data/test_data.txt");
        assert_eq!(f, ["[C][C=][C][H][Br1]", "[C][B=][F][Z][Br2]"]);
        cleanup();
    }

    #[test]
    fn generate_ngrams_test() {
        setup();
        let f = read_file("./data/test_data.txt");
        let mut n_grams = generate_ngrams(&f, 2);

        let two_grams = [
            "[C][C=]", "[C=][C]", "[C][H]", "[H][Br1]", "[C][B=]", "[B=][F]", "[F][Z]", "[Z][Br2]",
        ];
        //assert that the generate two grams has expected amount of 2 grams
        assert_eq!(n_grams.len(), two_grams.len());
        //assert that expected two_grams are in generated list
        for two_gram in two_grams.iter() {
            assert!(n_grams.contains(two_gram));
        }

        //assert that the generated three grams has the expected amount of 3 grams
        n_grams.extend(generate_ngrams(&f, 3));

        let three_grams = [
            "[C][C=][C]",
            "[C=][C][H]",
            "[C][H][Br1]",
            "[C][B=][F]",
            "[B=][F][Z]",
            "[F][Z][Br2]",
        ];

        //assert that the generate n_grams has expected amount of 2 and 3 grams
        assert_eq!(n_grams.len(), two_grams.len() + three_grams.len());
        //assert that expected two_grams are in generated list
        for two_gram in two_grams.iter() {
            assert!(n_grams.contains(two_gram));
        }
        //assert that expected three_grams are in generated list
        for three_gram in three_grams.iter() {
            assert!(n_grams.contains(three_gram));
        }
        cleanup();
    }

    #[test]
    fn count_token_occurence_test() {
        setup();
        let f = read_file("./data/test_data.txt");
        let mut n_grams = generate_ngrams(&f, 2);
        n_grams.extend(generate_ngrams(&f, 3));
        let mut vocab = Vocab::from_data(&f);
        for ngram in n_grams.iter() {
            vocab.insert(&ngram);
        }
        let encoded_selfies = par_encode_dataset(&f, &vocab);
        let counts = count_token_occurence(&encoded_selfies, &vocab);
        let most_common = vec![
            ("[C]", 0),
            ("[C][C=]", 1),
            ("[C][H][Br1]", 1),
            ("[C][B=]", 1),
            ("[F][Z][Br2]", 1),
        ];
        for i in most_common.iter() {
            let key = vocab.get(i.0).unwrap();
            assert!(counts.contains_key(&key) && *counts.get(&key).unwrap() == i.1)
        }
        cleanup()
    }
}
