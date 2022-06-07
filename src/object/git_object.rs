use crate::object::findable::Findable;
use indexmap::IndexMap;

/// A text-based key-value store.
///
/// This format is a simplified version of mail messages, specified in RFC 2822.
/// It begins with a series of key-value pairs separated with a single space. An
/// item may span over multiple lines, subsequent lins start with a space which
/// the parser must drop.
///
/// See: https://www.ietf.org/rfc/rfc2822.txt
///
/// ### Example
/// A commit object (uncompressed, without the headers) looks like this:
/// ```text
/// tree 8d7a53339121fd3a565b6f46eb0df7a20dc608a1
/// parent 390a277f5f3798af70c1895fa54bcaa6ce8e448e
/// author Justin Shaw <realjustinshaw@gmail.com> 1654631458 -0700
/// committer Justin Shaw <realjustinshaw@gmail.com> 1654631458 -0700
/// gpgsig -----BEGIN PGP SIGNATURE-----
///  
///  iQIzBAABCAAdFiEEsjQ114tLOZFScJjMAczt3vehvxQFAmKfrCIACgkQAczt3veh
///  vxRU5g//YtLJ/ej+ZXGCo/LDHoI76gSeTMbqEzzAqFvHo7e2EKLyhFZXBeCO3NkK
///  DJNASGwlc+QUa+rb3e08NfDce1E1z3dXOFniDdYkd+Kmv98gGPkEjeHDSexpWePr
///  kukYu3TImk7Igp2YpMrVLBUdMxH1RyBWWIgMzOI/O4Tk3CJfkK4V2QgQsbkF+Jio
///  xZs2xgKE01RSdjz9qLX1tTph1/9pWarrAz5BPUxGytWq8nkc4HM/enjaPEeMz9gY
///  FE3Ws6/GBwaU6NR6XvCgfVygxMSIdUUBgykSEG02DFZNZjM7l1jzBTKAMljMPnGb
///  MitToLlDK4CS6DsuM0MPpz3dGx+daAQAUbsJCeMIEJoS/ieH5a6L6+Y6Xg1x9ohI
///  4w30/J9U3rcpImJPUtyzejB1CwiQ8CndAlh4C9CAZSC3VU8+C0y7k1fK/oG5CQvb
///  SqIagiRpXKRFdAEsmzDMexNlrbxD9VmL7+Y67vgZVMvR4dDsrGvsNeKZOuGult/c
///  EpSg8KO5QfwNrHWw+h+nHP+YDeaXIkopZzSx4yzSFwkzxtA/qw7GPiCpzGdODo+I
///  8veuTF5mhYLg5iON/Oin+AvQFGSBj1u+FQyStl4oQ80xF+kYpCTMFO1Iclwrr08l
///  ZQQEV5K3DbwSZ1pFWciiJ6FYa8SWvoK4rqImnxamm3U74brgdz4=
///  =Pifs
///  -----END PGP SIGNATURE-----
///
/// update readme
/// ```
///
/// This is logically equivalent to an insertion-order-preserving map that holds
/// the following key value pairs:
/// ```text
/// tree      => 29ff16c..930c147
/// parent    => 2069413..24d49a0
/// author    => Thibault Polge <thibault@thb.lt> 1527025023 +0200
/// committer => Thibault Polge <thibault@thb.lt> 1527025044 +0200
/// gpgsig    => -----BEGIN PGP SIGNATURE----- ... -----END PGP SIGNATURE-----
/// ```

pub struct GitObject {
  pub map: IndexMap<String, String>,
  data: Vec<u8>,
}

impl GitObject {
  pub fn new() -> Self {
    Self {
      map: IndexMap::new(),
      data: Vec::default(),
    }
  }

  pub fn from_bytes(&mut self, raw: &[u8], offset: usize) {
    // Search for the next space and newline.
    let maybe_space = raw.find(b' ', offset);
    let maybe_newln = raw.find(b'\n', offset);

    // If newline occurs first (or there's no space at all), assume blank line.
    match (maybe_space, maybe_newln) {
      (_any, Some(newline)) if newline <= maybe_space.unwrap_or(newline) => {
        assert_eq!(newline, offset);
        let key = String::from("");
        let value = String::from_utf8(raw[offset..].to_vec()).unwrap();
        self.map.entry(key).or_insert(value);
      }
      (None, None) => (), // reached the end of the raw data
      _ => {
        let space = maybe_space.unwrap(); // shouldn't panic
        let key = String::from_utf8(raw[offset..space].to_vec()).unwrap();
        let mut end = offset;
        loop {
          end = raw.find(b'\n', end + 1).unwrap(); // probably won't panic
          if raw[end + 1] != b' ' {
            break;
          }
        }
        let value = String::from_utf8(raw[space + 1..end].to_vec()).unwrap();
        self.map.entry(key).or_insert(value.replace("\n ", "\n"));
        self.from_bytes(raw, end + 1);
      }
    }

    // Walk through the map and build out the file as a string
    let mut result = String::from("");

    // append the fields (key-value pairs)
    for key in self.map.keys() {
      if key == "" {
        continue;
      }
      let value = self.map.get(key).unwrap();
      result.push_str(key);
      result.push_str(" ");
      result.push_str(&value.replace("\n", "\n "));
      result.push_str("\n");
    }

    // append the message (the key of the message is the empty string)
    result.push_str(self.map.get("").unwrap());

    self.data = result.into_bytes();
  }

  pub fn to_bytes(&self) -> &[u8] {
    return self.data.as_slice();
  }
}
