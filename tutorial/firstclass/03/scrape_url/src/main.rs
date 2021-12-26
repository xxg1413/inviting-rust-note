use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {

    let url = "http://www.rust-lang.org/";
    let output = "rust.md";

    println!("链接地址：{}", url);

    let body = reqwest::blocking::get(url)?.text()?;

    println!("开始转换");

    let md = html2md::parse_html(&body);

    fs::write(output, md.as_bytes())?;

    println!("转换完成，文件保存在：{}", output);

    Ok(())
}