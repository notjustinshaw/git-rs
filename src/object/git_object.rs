use crate::object::findable::Findable;
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
/// tree 29ff16c9c14e2652b22f8b78bb08a5a07930c147
/// parent 206941306e8a8af65b66eaaaea388a7ae24d49a0
/// author Thibault Polge <thibault@thb.lt> 1527025023 +0200
/// committer Thibault Polge <thibault@thb.lt> 1527025044 +0200
/// gpgsig -----BEGIN PGP SIGNATURE-----
///  
///  iQIzBAABCAAdFiEExwXquOM8bWb4Q2zVGxM2FxoLkGQFAlsEjZQACgkQGxM2FxoL
///  kGQdcBAAqPP+ln4nGDd2gETXjvOpOxLzIMEw4A9gU6CzWzm+oB8mEIKyaH0UFIPh
///  rNUZ1j7/ZGFNeBDtT55LPdPIQw4KKlcf6kC8MPWP3qSu3xHqx12C5zyai2duFZUU
///  wqOt9iCFCscFQYqKs3xsHI+ncQb+PGjVZA8+jPw7nrPIkeSXQV2aZb1E68wa2YIL
///  3eYgTUKz34cB6tAq9YwHnZpyPx8UJCZGkshpJmgtZ3mCbtQaO17LoihnqPn4UOMr
///  V75R/7FjSuPLS8NaZF4wfi52btXMSxO/u7GuoJkzJscP3p4qtwe6Rl9dc1XC8P7k
///  NIbGZ5Yg5cEPcfmhgXFOhQZkD0yxcJqBUcoFpnp2vu5XJl2E5I/quIyVxUXi6O6c
///  /obspcvace4wy8uO0bdVhc4nJ+Rla4InVSJaUaBeiHTW8kReSFYyMmDCzLjGIu1q
///  doU61OM3Zv1ptsLu3gUE6GU27iWYj2RWN3e3HE4Sbd89IFwLXNdSuM0ifDLZk7AQ
///  WBhRhipCCgZhkj9g2NEk7jRVslti1NdN5zoQLaJNqSwO1MtxTmJ15Ksk3QP6kfLB
///  Q52UWybBzpaP9HEd4XnR+HuQ4k2K0ns2KgNImsNvIyFwbpMUyUWLMPimaV1DWUXo
///  5SBjDB/V/W2JBFR+XKHFJeFwYhj7DD/ocsGr4ZMx/lgc8rjIBkI=
///  =lgTX
///  -----END PGP SIGNATURE-----
///
/// Create first draft
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
use indexmap::IndexMap;

pub struct GitObject(IndexMap<String, String>);

impl GitObject {
  pub fn new() -> Self {
    Self(IndexMap::new())
  }

  pub fn from_bytes(&mut self, raw: &[u8], offset: usize) {
    // Search for the next space and newline.
    let maybe_space = raw[offset..].find(b' ');
    let maybe_newln = raw[offset..].find(b'\n');

    // If newline occurs first (or there's no space at all), assume blank line.
    match maybe_newln {
      Some(newline) if newline <= maybe_space.unwrap_or(newline) => {
        assert_eq!(newline, offset);
        let key = String::from("");
        let value = String::from_utf8(raw[offset..].to_vec()).unwrap();
        self.0.entry(key).or_insert(value);
      }
      _ => {
        let space = maybe_space.unwrap(); // can't panic
        let key = String::from_utf8(raw[offset..space].to_vec()).unwrap();
        let mut end = offset;
        loop {
          end = raw[end + 1..].find(b'\n').unwrap(); // probably won't panic
          if raw[end + 1] != b' ' {
            break;
          }
        }
        let value = String::from_utf8(raw[space + 1..end].to_vec()).unwrap();
        self.0.entry(key).or_insert(value.replace("\n ", "\n"));
        self.from_bytes(raw, end + 1);
      }
    }
  }

  pub fn to_bytes(&self) -> String {
    let mut result = String::from("");

    // append the fields (key-value pairs)
    for key in self.0.keys() {
      if key == "" {
        continue;
      }
      let value = self.0.get(key).unwrap();
      result.push_str(key);
      result.push_str(" ");
      result.push_str(&value.replace("\n", "\n "));
      result.push_str("\n");
    }

    // append the message (the key of the message is the empty string)
    result.push_str("\n");
    result.push_str(self.0.get("").unwrap());

    return result;
  }
}
