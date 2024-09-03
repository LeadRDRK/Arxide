use std::{collections::HashSet, fs::File, io::{BufRead, BufReader, BufWriter}, path::{Path, PathBuf}};

use arclib::MD5Crypt;

use crate::{game::Game, Error};

pub struct Crypt {
    pub type_: CryptType,
    pub key_path: Option<PathBuf>,
    pub game: Game,
    pub file_list: HashSet<String>
}

impl Crypt {
    pub fn process(&self, input_path: &Path, output_path: &Path, encrypt: bool) -> Result<(), Error> {
        let file_helper = FileHelper::new(input_path, output_path)?;
        match self.type_ {
            CryptType::MD5 => {
                let crypt = match &self.key_path {
                    Some(path) => MD5Crypt::new(&std::fs::read(path)?),
                    None => MD5Crypt::new(
                        &self.game.key()
                        .map(|k| k.data())
                        .unwrap_or_else(|| {
                            println!("+ No key file specified, game key not found");
                            std::process::exit(128);
                        })
                    )
                }?;

                // Process file list
                let mut processed_md5_list = HashSet::with_capacity(self.file_list.len());
                if self.file_list.len() > 0 {
                    println!("-- Processing file list");
                    for path in self.file_list.iter() {
                        println!("{}", path);

                        let path_md5 = arclib::utils::md5_string(path);
                        let mut src_path: PathBuf;
                        let mut dest_path: PathBuf;
                        if encrypt {
                            src_path = input_path.join(path);
                            dest_path = output_path.join(&path_md5);
                            dest_path.set_extension("bin");
                        }
                        else {
                            src_path = input_path.join(&path_md5);
                            src_path.set_extension("bin");
                            dest_path = output_path.join(path);
                        }

                        if path.ends_with(".wmv") {
                            println!("+ WMV file, copying without processing");
                            std::fs::create_dir_all(dest_path.parent().unwrap())?;
                            std::fs::copy(src_path, dest_path)?;
                            continue;
                        }

                        let Ok(meta) = std::fs::metadata(&src_path) else {
                            println!("+ File doesn't exist, skipping: {}", src_path.display());
                            continue;
                        };

                        if !meta.is_file() {
                            println!("+ Not a file, skipping: {}", src_path.display());
                            continue;
                        }

                        let mut reader = BufReader::new(File::open(&src_path)?);
                        std::fs::create_dir_all(dest_path.parent().unwrap())?;
                        let mut writer = BufWriter::new(File::create(&dest_path)?);
        
                        println!("-> {}", dest_path.display());
                        if let Err(e) = crypt.apply(&path_md5, &mut reader, &mut writer) {
                            println!("+ Failed to process file: {}", e);
                            continue;
                        }

                        processed_md5_list.insert(path_md5);
                    }
                }

                // Process stray files
                println!("-- Processing stray files");
                for path in file_helper.input_iter()? {
                    println!("{}", path.display());
    
                    let Some(path_md5) = path.file_stem().map(|s| s.to_str()).unwrap_or_default() else {
                        println!("+ Failed to get file name, skipping");
                        continue;
                    };
    
                    if path_md5.len() != 32 || u128::from_str_radix(path_md5, 16).is_err() {
                        println!("+ Invalid file name, skipping: {}", path_md5);
                        continue;
                    }

                    if processed_md5_list.contains(path_md5) {
                        continue;
                    }

                    let output_file = file_helper.create_output_file_from_input_path(&path)?;
                    let mut reader = BufReader::new(File::open(&path)?);
                    let mut writer = BufWriter::new(output_file);
    
                    if let Err(e) = crypt.apply(path_md5, &mut reader, &mut writer) {
                        println!("+ Failed to process file: {}", e);
                    }
                }
            }
            _ => ()
        }

        Ok(())
    }

    pub fn load_file_list(path: &Path) -> Result<HashSet<String>, Error> {
        let reader = BufReader::new(File::open(path)?);
        let mut list = HashSet::new();
        for res in reader.lines() {
            let line = res?;
            if line.is_empty() { continue; }
            list.insert(line);
        }
        Ok(list)
    }
}

#[derive(Default, Eq, PartialEq, Debug)]
pub enum CryptType {
    #[default] None,
    MD5
}

struct FileHelper<'a> {
    input_path: &'a Path,
    output_path: &'a Path,
}

impl<'a> FileHelper<'a> {
    fn new(input_path: &'a Path, output_path: &'a Path) -> Result<FileHelper<'a>, Error> {
        let input_meta = std::fs::metadata(input_path)?; // always check if input exists
        if let Ok(output_meta) = std::fs::metadata(output_path) {
            // if output exists, then make sure it's the same type as the input
            if output_meta.is_file() != input_meta.is_file() {
                return Err(Error::InvalidPath("output and input path type mismatch, \
                        refusing to delete/overwrite".to_owned()));
            }
        }

        Ok(FileHelper {
            input_path,
            output_path
        })
    }

    fn input_iter(&self) -> Result<Box<dyn Iterator<Item = PathBuf>>, Error> {
        let meta = std::fs::metadata(self.input_path)?;
        if meta.is_file() {
            Ok(Box::new(std::iter::once(self.input_path.to_owned())))
        }
        else {
            Ok(Box::new(std::fs::read_dir(self.input_path)?
                .filter(
                    |r| r.as_ref().is_ok_and(
                        |e| e.metadata().is_ok_and(
                            |m| m.is_file()
                        )
                    )
                )
                .map(|r| r.unwrap().path())
            ))
        }
    }

    fn create_output_file_from_input_path(&self, path: &Path) -> Result<File, Error> {
        let output_path: &Path = if path == self.input_path {
            self.output_path
        }
        else {
            let rel_path = path.strip_prefix(self.input_path)?;
            &self.output_path.join(rel_path)
        };
        println!("-> {}", output_path.display());

        std::fs::create_dir_all(output_path.parent().unwrap())?;
        Ok(File::create(&output_path)?)
    }
}