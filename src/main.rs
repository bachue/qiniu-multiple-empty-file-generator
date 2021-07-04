use std::path::Path;

use qiniu_upload::Uploader;
use rayon::ThreadPoolBuilder;

fn main() {
    let thread_pool = ThreadPoolBuilder::new().num_threads(10).build().unwrap();
    thread_pool.scope_fifo(|s| {
        let uploader = Uploader::from_env().expect("QINIU env must be set");
        for i in 0..1000000 {
            let uploader = uploader.to_owned();
            s.spawn_fifo(move |_| {
                let path = Path::new("/dev/null");
                if let Err(err) = uploader
                    .upload_path(path)
                    .expect("Failed to upload /dev/null")
                    .object_name(format!("folder/{}", i))
                    .file_name(format!("{}", i))
                    .start()
                {
                    eprintln!("Error: {}: {:?}", i, err);
                } else {
                    println!("Done: {}", i);
                }
            })
        }
    })
}
