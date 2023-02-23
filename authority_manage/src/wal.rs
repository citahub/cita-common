// Copyright Rivtower Technologies LLC.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use std::fs::{read_dir, DirBuilder, File, OpenOptions};
use std::io;
use std::io::{Read, Seek, Write};
use std::mem::transmute;
use std::path::Path;
use std::str;

#[allow(dead_code)]
#[derive(Debug)]
pub struct Wal {
    fs: File,
    dir: String,
}

impl Wal {
    pub fn create(dir: &str) -> Result<Wal, io::Error> {
        let fss = read_dir(&dir);
        if fss.is_err() {
            DirBuilder::new().recursive(true).create(dir).unwrap();
        }

        let fpath = dir.to_string() + "/authorities_old";
        let big_path = Path::new(&*fpath).to_path_buf();
        let fs = OpenOptions::new()
            .read(true)
            .create(true)
            .write(true)
            .open(big_path)?;
        Ok(Wal {
            fs,
            dir: dir.to_string(),
        })
    }

    pub fn save(&mut self, mtype: u8, msg: &[u8]) -> io::Result<usize> {
        let mlen = msg.len() as u32;
        if mlen == 0 {
            return Ok(0);
        }
        self.fs.set_len(0)?;
        let len_bytes: [u8; 4] = unsafe { transmute(mlen.to_le()) };
        let type_bytes: [u8; 1] = unsafe { transmute(mtype.to_le()) };
        self.fs.seek(io::SeekFrom::End(0))?;
        self.fs.write_all(&len_bytes[..])?;
        self.fs.write_all(&type_bytes[..])?;
        let hlen = self.fs.write(msg)?;
        self.fs.flush()?;
        Ok(hlen)
    }

    pub fn load(&mut self) -> Vec<(u8, Vec<u8>)> {
        let mut vec_buf: Vec<u8> = Vec::new();
        let mut vec_out: Vec<(u8, Vec<u8>)> = Vec::new();

        self.fs.seek(io::SeekFrom::Start(0)).unwrap();
        let res_fsize = self.fs.read_to_end(&mut vec_buf);
        if res_fsize.is_err() {
            return vec_out;
        }
        let fsize = res_fsize.unwrap();
        if fsize <= 5 {
            return vec_out;
        }
        let mut index = 0;
        loop {
            if index + 5 > fsize {
                break;
            }
            let hd: [u8; 4] = [
                vec_buf[index],
                vec_buf[index + 1],
                vec_buf[index + 2],
                vec_buf[index + 3],
            ];
            let tmp: u32 = unsafe { transmute::<[u8; 4], u32>(hd) };
            let bodylen = tmp as usize;
            let mtype = vec_buf[index + 4];
            index += 5;
            if index + bodylen > fsize {
                break;
            }
            vec_out.push((mtype, vec_buf[index..index + bodylen].to_vec()));
            index += bodylen;
        }
        vec_out
    }
}
