use std::{error::Error, io::Read, io::Write};

use core::{mem, slice};

pub type Result<T> = std::result::Result<T, Box<dyn Error>>;

pub fn read_n_bytes<T: Read>(reader: &mut T, bytes_to_read: u64) -> Result<Vec<u8>> {
    let mut buffer = vec![];
    let res = reader.take(bytes_to_read).read_to_end(&mut buffer);

    match res {
        Ok(bytes) => {
            if bytes as u64 != bytes_to_read {
                return Err("read error".into());
            }
            Ok(buffer)
        }
        Err(e) => Err(e.into()),
    }
}

pub unsafe fn read_raw_struct<R: Read, T: Sized>(reader: &mut R) -> Result<T> {
    let mut struct_raw = mem::MaybeUninit::uninit();
    let struct_raw_slice =
        slice::from_raw_parts_mut(struct_raw.as_mut_ptr() as *mut u8, mem::size_of::<T>());

    reader.read_exact(struct_raw_slice)?;

    return Ok(struct_raw.assume_init());
}

pub unsafe fn write_raw_struct<W: Write, T: Sized>(writer: &mut W, data: &T) -> Result<()> {
    let struct_raw_slice =
        slice::from_raw_parts((data as *const T) as *const u8, mem::size_of::<T>());

    writer.write_all(struct_raw_slice)?;

    Ok(())
}
