fn main() {
    {
        use std::io::Result;
        use std::fs;
        use std::thread::spawn;

        #[allow(dead_code)]
        fn process_files(filenames: Vec<String>) -> Result<()> {
            for document in filenames {
                let _text = fs::read_to_string(document)?;
                // let results = process(text);
                // save(&document, result);
            }

            Ok(())
        }

        #[allow(dead_code)]
        fn split_vec_into_chunks(items: Vec<String>, count: usize) -> Vec<Vec<String>> {
            let mut result = Vec::new();
            let mut it = items.iter();
            let chunk_count = items.len() / count;

            for _ in 0..count - 1 {
                let mut sub_vec = Vec::new();
                for _ in 0..chunk_count {
                    match it.next() {
                        Some(str) => sub_vec.push(str.clone()),
                        None => (),
                    }
                }

                result.push(sub_vec);
            }

            let mut sub_vec = Vec::new();
            while let Some(str) = it.next() {
                sub_vec.push(str.clone());
            }

            result.push(sub_vec);

            result
        }

        #[allow(dead_code)]
        fn process_files_in_parallel(filenames: Vec<String>) -> Result<()> {
            const NTHREADS: usize = 8;
            let worklists = split_vec_into_chunks(filenames, NTHREADS);

            let mut thread_handles = vec![];
            for worklist in worklists {
                thread_handles.push(spawn(move || process_files(worklist)));
            }

            for handle in thread_handles {
                handle.join().unwrap()?;
            }

            Ok(())
        }

        #[allow(dead_code)]
        struct Glossary { }

        use std::sync::Arc;

        fn process_files_with_glossary(filenames: Vec<String>, _glossary: &Glossary) -> Result<()> {
            for document in filenames {
                let _text = fs::read_to_string(document)?;
                // let results = process(text);
                // save(&document, result);
            }

            Ok(())
        }

        #[allow(dead_code)]
        fn process_files_in_parallel_with_glossary(filenames: Vec<String>,
            glossary: Arc<Glossary>) -> Result<()> {
            
                const NTHREADS: usize = 8;
                let worklists = split_vec_into_chunks(filenames, NTHREADS);
    
                let mut thread_handles = vec![];
                for worklist in worklists {
                    let glossary_for_child = glossary.clone();
                    thread_handles.push(spawn(move || process_files_with_glossary(worklist, &glossary_for_child)));
                }
    
                for handle in thread_handles {
                    handle.join().unwrap()?;
                }
    
                Ok(())
            }
    }
}
