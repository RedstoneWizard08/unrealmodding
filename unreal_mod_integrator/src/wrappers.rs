use repak::{Error, PakBuilder, PakReader, Version};
use std::{
    collections::BTreeMap,
    io::{Read, Seek, Write},
};

pub struct WPakReader<R: Read + Seek> {
    pak: PakReader,
    reader: R,
}

impl<R: Read + Seek> WPakReader<R> {
    pub fn new(mut reader: R) -> Result<Self, Error> {
        Ok(Self {
            pak: PakBuilder::new().reader(&mut reader)?,
            reader,
        })
    }

    pub fn version(&self) -> Version {
        self.pak.version()
    }

    pub fn mount_point(&self) -> &str {
        &self.pak.mount_point()
    }

    pub fn encrypted_index(&self) -> bool {
        self.pak.encrypted_index()
    }

    pub fn encryption_guid(&self) -> Option<u128> {
        self.pak.encryption_guid()
    }

    pub fn path_hash_seed(&self) -> Option<u64> {
        self.pak.path_hash_seed()
    }

    pub fn get(&mut self, path: &str) -> Result<Vec<u8>, Error> {
        self.pak.get(path, &mut self.reader)
    }

    pub fn read_file<W: Write>(&mut self, path: &str, writer: &mut W) -> Result<(), Error> {
        self.pak.read_file(path, &mut self.reader, writer)
    }

    pub fn files(&self) -> Vec<String> {
        self.pak.files()
    }
}

pub struct PakMemory {
    version: Version,
    mount_point: String,
    path_hash_seed: Option<u64>,
    data: BTreeMap<String, Vec<u8>>,
}

impl PakMemory {
    pub fn new(version: Version) -> Self {
        Self {
            version,
            data: BTreeMap::new(),
            mount_point: "../../../".into(),
            path_hash_seed: None,
        }
    }

    pub fn set_entry(&mut self, name: String, data: Vec<u8>) {
        self.data.insert(name, data);
    }

    pub fn get_entry(&self, name: impl AsRef<str>) -> Option<&Vec<u8>> {
        self.data.get(name.as_ref())
    }

    pub fn write<W: Write + Seek>(self, writer: &mut W) -> Result<(), Error> {
        let mut pak =
            PakBuilder::new().writer(writer, self.version, self.mount_point, self.path_hash_seed);

        for (path, data) in self.data {
            pak.write_file(&path, true, data)?;
        }

        pak.write_index()?;

        Ok(())
    }
}
