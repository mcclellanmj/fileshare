use iron::headers::{ContentDisposition, DispositionType, DispositionParam, Charset};

pub fn download_file_header(file_name: &str) -> ContentDisposition {
    ContentDisposition {
        disposition: DispositionType::Attachment,
        parameters: vec![DispositionParam::Filename(
                Charset::Us_Ascii,
                None,
                file_name.as_bytes().to_vec()
        )]
    }
}
