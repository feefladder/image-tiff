use std::default;
use futures::io::{AsyncRead, AsyncSeek};
// use ehttp::

#[derive(Debug)]
struct HttpReader {
    url: String,
    pos: u64,
    chunk_size: u64,
    head_cache: Cache,
    tile_caches: Vec<Cache>,
}

impl HttpReader {
    pub async fn new(url: String) -> Self {
        let chunk_size = 30*1024;
        HttpReader {
            url,
            pos: 0,
            chunk_size,
            head_cache: Cache {
                range: (0, chunk_size),
                data: HttpReader::get_range(url, 0, chunk_size)
            },
            tile_caches: Vec::new()
        }
    }

    async fn get_range(url: String, start: u64, end: u64) -> ehttp::Result<> {
        println!("Requesting: {:?} from {:?}", (start, end), url);
        let mut request = ehttp::Request::get(url);
        request.headers.insert("Range".to_string(), format!("bytes={:?}-{:?}",start,end));
        let response = ehttp::fetch_async(request).await?;
        if !res.ok {
            Err(HttpError::HttpStatus(res.status))
        } else {
            Ok(res.bytes.into())
        }
    }
}

#[derive(Debug)]
struct Cache {
    range: (u64, u64),
    data: Vec<u8>,
}

impl AsyncRead for HttpReader {
    fn poll_read(
                self: std::pin::Pin<&mut Self>,
                cx: &mut std::task::Context<'_>,
                buf: &mut [u8],
            ) -> std::task::Poll<std::io::Result<usize>> {
        
    }
}
