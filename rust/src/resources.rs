use std::path::{Path, PathBuf};
use std::fs;
use std::io::{self, Read};
use std::ffi;
use std::ffi::CString;

//
//*************Error Message
//
#[derive(Debug)]
pub enum Error {
   IO(io::Error),
   FileContainsNil,
   FailedToGetExePath
}
impl From<io::Error> for Error {
   fn from(other: io::Error) -> Self {
      Error::IO(other)
   }
}

//
//*************Resources
//
pub struct Resources {
   root_path: PathBuf
}

impl Resources {
   pub fn from_relative_exe_path(
      rel_path:&Path
   ) -> Result<Resources, Error>{
      let exe_file_name = ::std::env::current_exe()
         .map_err(|_| Error::FailedToGetExePath)?;
      let exe_path = exe_file_name.parent()
         .ok_or(Error::FailedToGetExePath)?;

      Ok(Resources {
         root_path: exe_path.join(rel_path)
      })
   }
   pub fn load_cstring(
      &self,
      resource_name: &str
   ) -> Result<CString, Error> {
      let mut file = fs::File::open(
         self.root_path.join(resource_name)
      )?;

      let mut buffer = Vec::with_capacity(
         file.metadata()?.len() as usize + 1
      );
      file.read_to_end(&mut buffer);

      // check for nul byte
      if buffer.iter().find(|i| **i == 0 ).is_some() {
         return Err(Error::FileContainsNil);
      }

      Ok( unsafe{ CString::from_vec_unchecked(buffer) } )
   }
}

fn resource_name_to_path(
   root_dir: &Path,
   location: &str
) -> PathBuf {
   let mut path:PathBuf = root_dir.into();

   for part in location.split("/") {
      // path.join(part)
      path = path.join(part)
   }

   path
}