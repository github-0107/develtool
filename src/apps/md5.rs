use clap::Parser;
use md5::{Digest, Md5};

#[derive(Debug, Parser)]
pub struct Encrypter {
    /// The original string that needs to be encrypted
    #[arg(short, long)]
    ostr: String,

    /// Whether it is uppercase or not
    #[arg(short, long)]
    upper: bool,

    /// Returns 16 for the MD5 string, default 32 bits
    #[arg(short, long)]
    is16: bool,
}

impl Encrypter {
    pub fn md5(&self) {
        let mut hasher = Md5::new();
        hasher.update(self.ostr.as_bytes());
        
        let result = hasher.finalize();

        if self.is16 {
            self.print_hex(&result[4..12]);
        } else {
            self.print_hex(&result[..]);     
        }
    }

    fn print_hex(&self, rs: &[u8]) {
        for x in rs {
            if self.upper {
                print!("{:X}", x);
            } else {
                print!("{:x}", x);
            }
            
        }
        println!();
    }
}
