use std::fs::{File, OpenOptions};
use std::io::{self, prelude::*, SeekFrom};
use std::path::Path;

pub const PAGE_SIZE: usize = 4096;

pub struct DiskManager {
    heap_file: File,
    next_page_id: u64,
}

pub struct PageId(pub u64);

impl PageId {
    pub fn to_u64(self) -> u64 {
        self.0
    }
}

impl DiskManager {
    pub fn new(heap_file: File) -> io::Result<Self> {
        // ヒープファイルのファイルサイズを取得している
        let heap_file_size = heap_file.metadata()?.len();
        // TODO ここがよくわからない
        let next_page_id = heap_file_size / PAGE_SIZE as u64;
        Ok(Self {
            heap_file,
            next_page_id,
        })
    }

    // file pathを指定して開く
    pub fn open(heap_file_path: impl AsRef<Path>) -> io::Result<Self> {
        let heap_file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(heap_file_path)?;
        Self::new(heap_file)
    }

    // 新たなページIDを割り当てる
    pub fn allocate_page(&mut self) -> PageId {
        let page_id = self.next_page_id;
        PageId(page_id)
    }

    // ページのデータを読み出す
    pub fn read_page_data(&mut self, page_id: PageId, data: &mut [u8]) -> io::Result<()> {
        let offset = PAGE_SIZE as u64 * page_id.to_u64();
        // SeekFrom::Start(offest)はファイルの先頭から数えてoffsetバイト目って意味
        self.heap_file.seek(SeekFrom::Start(offset))?;
        self.heap_file.read_exact(data)
    }

    // ページにデータを書き出す
    pub fn write_page_data(&mut self, page_id: PageId, data: &[u8]) -> io::Result<()> {
        let offset = PAGE_SIZE as u64 * page_id.to_u64();
        self.heap_file.seek(SeekFrom::Start(offset))?;
        self.heap_file.write_all(data)
    }
}
