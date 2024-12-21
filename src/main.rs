

use std::env;
use std::fs::File;
use std::io::Read;



#[derive(Debug)]
enum Pixel {
    RGB(u8, u8, u8),
    BW(u8)
}

#[derive(Debug)]
struct Row<'a> {
    values: Vec<&'a Pixel>,
}

struct Video {
    name: String,
    data: Vec<Pixel>,
    rgb: bool
}


struct VideoPlayer  {
    video: Video,
    current_frame: u64,
    dimensions: [u32; 2]
}


impl Video {
    fn new(name: String, data: &Vec<u8>, rgb: bool) -> Video {
        let mut chunk_size = 1;
        if rgb {
            chunk_size = 3;
        }
        let mut pixel_data: Vec<Pixel> = Vec::new();
        for chunk in data.chunks(chunk_size) {
            if rgb {
                if chunk.len() == 3 {
                    pixel_data.push(Pixel::RGB(chunk[0], chunk[1],chunk[2])); 
                } else {
                    pixel_data.push(Pixel::RGB(chunk[0], chunk[0],chunk[0])); 
                }
            } else {
                pixel_data.push(Pixel::BW(chunk[0])); 
            }
        }
        Video {name, rgb, data: pixel_data}
    }
    fn get_chunk(&self, row_size: usize, start: usize, end: usize) -> Vec<Row> {
        if start > end {
            eprintln!("Start of chunk cannot be greater then end of chunk");
            std::process::exit(1);
        }
        if start > self.data.len() - 1  {
            eprintln!("Way pass the end of the video data");
            eprintln!("index: {}, len: {}", end, self.data.len());
            std::process::exit(1);
        }
        if end > self.data.len() - 1  {
            eprintln!("Way pass the end of the video data");
            eprintln!("index: {}, len: {}", end, self.data.len());
            std::process::exit(1);
        }
        let mut chunk: Vec<Row> = Vec::new();
        let mut iter = self.data[start..end].iter();
        let mut values: Vec<&Pixel>= Vec::new();
        let mut i = 0;
        while let Some(value) = iter.next() {
            i += 1;
            values.push(value);
            if i == row_size {
                chunk.push(Row {values});
                i = 0;
                values = Vec::new();
            }
        }
        chunk
    }
}

impl VideoPlayer {
    fn new(video: Video) -> VideoPlayer {
        VideoPlayer{video, current_frame: 0, dimensions: [15, 10]}
    }
    fn chunk_size(&self) -> u64 {
        (self.dimensions[0] * self.dimensions[1]).into()
    }
    fn get_current_frame(&self) -> Vec<Row> {
        let start= self.chunk_size() * self.current_frame;
        let end = self.chunk_size() * (self.current_frame + 1);
        self.video.get_chunk(self.dimensions[0] as usize, start as usize, end as usize)
    }
    fn next_frame(&mut self) {
        self.current_frame += 1;
    }

}



fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <file_name>", args[0]);
        std::process::exit(1);
    }
    let file_name = &args[1];
    let mut file = File::open(file_name).unwrap_or_else( |_| {
        eprintln!("Cannot read the file: {}", args[1]);
        std::process::exit(1);
    });
    let mut buffer = Vec::new();
    let _ = file.read_to_end(&mut buffer);
    let video = Video::new(file_name.to_string(), &buffer, false);
    let video_player = VideoPlayer::new(video);
    let frame = video_player.get_current_frame();
    println!("Singel Frame: {:?}", frame);
    println!("Frame Len: {}", frame.len());
}
