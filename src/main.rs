#![warn(clippy::pedantic)]

use crate::cmd::{Args, Commands};
use pickledb::PickleDb;
use std::{fs, path};

mod cmd;

fn init_trash(trash_dir: &path::PathBuf, metadata_path: &path::PathBuf) {
    fs::create_dir(&trash_dir).expect("Couldn't create trash directory. Does it already exist?");

    PickleDb::new(
        metadata_path,
        pickledb::PickleDbDumpPolicy::AutoDump,
        pickledb::SerializationMethod::Bin,
    );
}

fn main() {
    let arg: Args = argh::from_env();
    let trash_dir = match home::home_dir() {
        Some(mut dir) => {
            dir.push(".trash-rs");
            dir
        }
        None => return,
    };
    let mut metadata_path = trash_dir.clone();
    metadata_path.push(".metadata.db");
    match arg.cmd {
        Commands::Init(_) => init_trash(&trash_dir, &metadata_path),
        Commands::List(_) => {
            let db = PickleDb::load(
                metadata_path,
                pickledb::PickleDbDumpPolicy::AutoDump,
                pickledb::SerializationMethod::Bin,
            )
            .expect("Couldn't get database. Is trash initialized?");

            for kv in db.iter() {
                println!(
                    "{}: {}",
                    kv.get_key(),
                    kv.get_value::<path::PathBuf>().unwrap().display()
                );
            }
        }
        Commands::Add(cmd) => {
            let mut db = PickleDb::load(
                metadata_path,
                pickledb::PickleDbDumpPolicy::AutoDump,
                pickledb::SerializationMethod::Bin,
            )
            .expect("Couldn't get database. Is trash initialized?");

            let cmd_path = path::Path::new(&cmd.path);

            if cmd_path.is_dir() && !arg.recursive {
                eprintln!("Recursive flag required to add directories");
                return;
            }

            db.set(
                cmd_path
                    .file_name()
                    .expect("Couldn't get file name")
                    .to_str()
                    .expect("File name is not valid UTF-8"),
                &cmd_path,
            )
            .expect("Couldn't set path in database");

            fs::rename(&cmd_path, &trash_dir.join(&cmd_path.file_name().unwrap()))
                .expect("Couldn't move file");
        }
        Commands::Restore(cmd) => {
            let mut db = PickleDb::load(
                metadata_path,
                pickledb::PickleDbDumpPolicy::AutoDump,
                pickledb::SerializationMethod::Bin,
            )
            .expect("Couldn't get database. Is trash initialized?");

            let file_path = &trash_dir.join(&cmd.name);

            if file_path.is_dir() && !arg.recursive {
                eprintln!("Recursive flag required to restore directories");
                return;
            }

            fs::rename::<&path::PathBuf, path::PathBuf>(
                file_path,
                db.get(&cmd.name).expect("File not in trash"),
            )
            .expect("Couldn't move file");

            db.rem(&cmd.name).unwrap();
        }
        Commands::Delete(cmd) => {
            let mut db = PickleDb::load(
                metadata_path,
                pickledb::PickleDbDumpPolicy::AutoDump,
                pickledb::SerializationMethod::Bin,
            )
            .expect("Couldn't get database. Is trash initialized?");

            let file_path = &trash_dir.join(&cmd.name);

            if !db.exists(&cmd.name) {
                eprintln!("File doesn't exist in database, trying to remove anyway");
            }

            if file_path.is_dir() && !arg.recursive {
                eprintln!("Recursive flag required to delete directories");
                return;
            } else if file_path.is_dir() {
                fs::remove_dir_all(file_path).expect("Directory doesn't exist in trash");
            } else {
                fs::remove_file(file_path).expect("File doesn't exist in trash")
            }

            if db.exists(&cmd.name) {
                db.rem(&cmd.name).unwrap();
            }
        }
        Commands::Empty(_) => {
            fs::remove_dir_all(&trash_dir).expect("Couldn't remove trash directory");
            init_trash(&trash_dir, &metadata_path)
        }
    }
}
