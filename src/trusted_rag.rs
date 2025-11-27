// src/trusted_rag.rs
pub async fn auto_learn_trusted(question: &str) -> String {
    let sources = vec![
        "https://luatvietnam.vn/api/search?q=",
        "https://thuvienphapluat.vn/van-ban/",
        "https://vietnamnet.vn/api/search?",
        "https://vnexpress.net/rss",
        "https://api.mathpix.com/v3/text", // OCR bài tập
    ];
    
    // Dùng Serper.dev hoặc tự build Vietnamese search index
    let result = vietnamese_search_engine::query(question).await;
    
    // Lưu vào on-chain knowledge shard (IPFS + OrbitDB hoặc tự build)
    let cid = ipfs::add(&result).await;
    format!("Pappap vừa học từ nguồn chính thống Việt Nam!\n\n{result}\n\n→ Đã lưu vĩnh viễn trên chain: {cid}")
}