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
例： 85 6".to_string() );

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

        let vec0: Vec<&str> = line.split(" ").collect();
        let mut kakerareru_su : i64 = vec0[0].parse().unwrap();
        let kakeru_su : i64 = vec0[1].parse().unwrap();
        writeln( format!("{}×{}＝☆（＾～＾）！", kakerareru_su, kakeru_su ) );

        let mut vec1 : Vec<i64> = Vec::new();

        loop {
            let number0 = kakerareru_su % 10;
            kakerareru_su = kakerareru_su / 10;

            // 1の位、10の位、…　の順で入る
            vec1.push( number0 * kakeru_su );

            if kakerareru_su==0 {break};
        }


        for number1 in vec1.iter() {
            writeln( format!("・{}", number1 ) );
        }

        let mut vec2 : Vec<i64> = Vec::new();

        // 1の位から足していく
        let mut kuriagari : i64 = -1;
        for number1 in vec1.iter() {
            if kuriagari==-1 {
                let number4_1 = *number1 % 10;
                kuriagari = *number1 / 10;
                writeln( format!("push{}＝{}％10 kuriagari{}", number4_1, *number1, kuriagari ) );
                vec2.push( number4_1 );
            } else {
                let number2 = number1 + kuriagari;
                let number2_1 = number2 % 10;
                vec2.push( number2_1 );
                writeln( format!("push{} {}＝{}＋{}", number2_1, number2, number1, kuriagari ) );
                kuriagari = number2 / 10;
                writeln( format!("kuriagari{}", kuriagari ) );
            }
        }
        if kuriagari != 0 { vec2.push( kuriagari); }
        vec2.reverse();

        write( "合計は ".to_string() );
        for number3 in vec2.iter() {
            write( format!("{}", number3 ) );
        }
        writeln( " だぜ☆（＾～＾）！".to_string() );
        
    }
}
