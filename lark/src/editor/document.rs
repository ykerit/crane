use anyhow::{Context, Result};
use encoding_rs as encoding;
use ropey::{Rope, RopeBuilder};
use std::path::{Path, PathBuf};

#[derive(Default, Debug, Clone, Copy)]
pub struct DocumentId(usize);
const BUF_SIZE: usize = 8192;

pub struct Document {
    pub id: DocumentId,
    text: Rope,
    path: Option<PathBuf>,
    encoding: &'static encoding::Encoding,
}

pub fn from_reader<R: std::io::Read>(
    reader: &mut R,
    encoding: Option<&'static encoding::Encoding>,
) -> Result<(Rope, &'static encoding::Encoding)> {
    let mut buf = [0u8; BUF_SIZE];
    let mut buf_utf8 = [0u8; BUF_SIZE];
    let mut rope_builder = RopeBuilder::new();

    let (encoding, mut decoder, mut slice, mut is_empty) = {
        let read = reader.read(&mut buf)?;
        let is_empty = read == 0;
        let encoding = encoding.unwrap_or_else(|| {
            let mut encoding_detector = chardetng::EncodingDetector::new();
            encoding_detector.feed(&buf, is_empty);
            encoding_detector.guess(None, true)
        });
        let decoder = encoding.new_decoder();
        let slice = &buf[..read];
        (encoding, decoder, slice, is_empty)
    };
    let buf_str = unsafe { std::str::from_utf8_unchecked_mut(&mut buf_utf8[..]) };
    let mut total_write = 0usize;
    loop {
        let mut total_read = 0usize;
        loop {
            let (result, read, write, ..) =
                decoder.decode_to_str(&slice[total_read..], &mut buf_str[total_write..], is_empty);
            total_read += read;
            total_write += write;
            match result {
                encoding::CoderResult::InputEmpty => {
                    break;
                }
                encoding::CoderResult::OutputFull => {
                    rope_builder.append(&buf_str[..total_write]);
                    total_write = 0;
                }
            }
        }
        if is_empty {
            rope_builder.append(&buf_str[..total_write]);
            break;
        }
        let read = reader.read(&mut buf)?;
        slice = &buf[..read];
        is_empty = read == 0;
    }
    let rope = rope_builder.finish();
    Ok((rope, encoding))
}

impl Document {
    pub fn from_rope(text: Rope, encoding: Option<&'static encoding::Encoding>) -> Self {
        let encoding = encoding.unwrap_or(encoding::UTF_8);
        Self {
            id: DocumentId::default(),
            text,
            path: None,
            encoding,
        }
    }

    pub fn open<P: AsRef<Path>>(
        path: &P,
        encoding: Option<&'static encoding::Encoding>,
    ) -> Result<Self> {
        let (repo, encoding) = if path.as_ref().exists() {
            let mut file = std::fs::File::open(path.as_ref())
                .with_context(|| format!("unable to open {:?}", path.as_ref()))?;
            from_reader(&mut file, encoding)?
        } else {
            let encoding = encoding.unwrap_or(encoding::UTF_8);
            (Rope::from(""), encoding)
        };

        let mut doc = Self::from_rope(repo, Some(encoding));
        doc.path = Some(path.as_ref().to_path_buf());
        Ok(doc)
    }

    pub fn format(&mut self) {}

    pub fn save(&mut self) {}

    pub fn format_on_save(&mut self) {}
}
