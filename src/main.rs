#![no_std]

const BASE62:[u8;62]=*b"0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz";
const U128_MAX_LEN:usize=22;

struct Base62Buf{buf:[u8;U128_MAX_LEN]}

impl Base62Buf{
    const fn new()->Self{
        Self {buf:[b"0"[0];U128_MAX_LEN]}
    }

    #[inline(always)]
    const fn decode_byte(byte:u8)->Result<u128,DecodeError>{
        match byte{
            b'0'..=b'9'=>Ok((byte-b'0') as u128),
            b'A'..=b'Z'=>Ok((byte-b'A'+10) as u128),
            b'a'..=b'z'=>Ok((byte-b'a'+36) as u128),
            _=>Err(DecodeError::InvalidCharacter),
        }
    }

    #[inline]
    pub fn encode(&mut self,mut val:u128)->&str{
        if val==0{return "0"}

        let mut c:usize=U128_MAX_LEN;
        
        while val>0{
            c-=1;
            self.buf[c]=BASE62[(val%62)as usize];
            val/=62;
        }

        unsafe{core::str::from_utf8_unchecked(&self.buf[c..])}
    }

    #[inline]
    pub fn decode(input:&str)->Result<u128,DecodeError>{
        let bytes:&[u8]=input.as_bytes();
        if bytes.is_empty(){
            return Err(DecodeError::Empty);
        }

        let mut result:u128=0;

        for&b in bytes{
            let x:u128=Self::decode_byte(b)?;

            result=result
                .checked_mul(62)
                .and_then(|r:u128|r.checked_add(x))
                .ok_or(DecodeError::Overflow)?
        }

        Ok(result)
    }
}

#[derive(Debug,PartialEq,Eq)]
pub enum DecodeError{
    InvalidCharacter,
    Overflow,
    Empty,
}

fn main(){
}