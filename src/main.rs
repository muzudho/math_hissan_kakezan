use std::fs::File;
use std::io::Write;
use std::path::Path;

use std::io;

pub fn write( s:String ){
    // 書き込み先
    let mut log_file = File::create(Path::new("log.txt")).unwrap();
        print!( "{}", s );
    match log_file.write_all( s.as_bytes() )
    {
        Err(_why) => {},
        Ok(_) => {},
    }
}
pub fn writeln( s:String ){
    // 書き込み先
    let mut log_file = File::create(Path::new("log.txt")).unwrap();
        println!( "{}", s );
    match log_file.write_all( format!("{}\n", s).as_bytes() )
    {
        Err(_why) => {},
        Ok(_) => {},
    }
}

fn main() {

    loop{
        writeln( "さあ、掛け算をしようか☆（＾～＾）！
a×b の a と b に入る 1桁の正の整数 を半角空白1個で区切って入れろだぜ☆（＾～＾）
終わりたいときは quit と打ちこめだぜ☆
例： 3 4".to_string() );

        let mut line : String = String::new();
        io::stdin().read_line(&mut line)
            .ok()// read_lineの返り値オブジェクトResult の okメソッド
            .expect("info Failed to read line");// OKで無かった場合のエラーメッセージ

        // 末尾の改行を除こうぜ☆（＾～＾）
        // trim すると空白も消えるぜ☆（＾～＾）
        let line : String = line.trim().parse().ok().expect("info Failed to parse");

        if line == "quit" {
            break;
        }

        let vec: Vec<&str> = line.split(" ").collect();
        let kakerareru_su : i64 = vec[0].parse().unwrap();
        let kakeru_su : i64 = vec[1].parse().unwrap();

        let kotae = kakerareru_su * kakeru_su;
        let kotae1 = kotae % 10;
        let kotae10 = kotae / 10;

        writeln( format!("{}×{}＝{}{}☆（＾～＾）！", kakerareru_su, kakeru_su, kotae10, kotae1 ) );
    }
}
