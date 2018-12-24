#![feature(extern_prelude)]

use bytebuffer::*;
use byteorder::{BigEndian, ByteOrder, LittleEndian};
use std::io::{Read, Write};
use time;
// 包头数据
use std::fmt;
use std::marker::Sized;

// pub trait MsgT {
//     // fn new(&self) -> f64;
//     // fn new(msg_data: Vec<u8>) -> Self;
//     fn new(msg_data: Vec<u8>) -> Self where Self: Sized {
//         let mut buffer = ByteBuffer::new();
//         for index in msg_data {
//             buffer.write_u8(index);
//         }
//         Self::unpack(buffer)
//     };

//     fn pack(self) -> ByteBuffer;
    
//     fn unpack(rd_buffer: ByteBuffer) -> Self;
// }


pub trait Msg{
    // fn new(&self) -> f64;
    fn new(msg_data: Vec<u8>) -> Self;
//  fn new(msg_data: Vec<u8>) -> T {
//         let mut buffer = ByteBuffer::new();
//         for index in msg_data {
//             buffer.write_u8(index);
//         }
//         T::unpack(buffer)
//     };
    fn pack(self) -> ByteBuffer;
    fn unpack(rd_buffer: ByteBuffer) -> Self;
}


pub trait FillerData<T>{

    fn unpack(rd_buffer: ByteBuffer,filler_nums: u8) -> Vec<T>;

}

// 60
#[derive(Debug, Clone)]
pub struct Statistics {
    // MsgSize: u16,
    // MsgType: u16,
    SecurityCode: u32,
    SharesTraded: u64,
    Turnover: i64,
    HighPrice: i32,
    LowPrice: i32,
    LastPrice: i32,
    VWAP: i32,
    ShortSellSharesTraded: u32,
    ShortSellTurnover: i64,
}

impl Msg for Statistics {
    fn new(msg_data: Vec<u8>) -> Self {
        let mut buffer = ByteBuffer::new();
        for index in msg_data {
            buffer.write_u8(index);
        }
        Self::unpack(buffer)
    }

    fn pack(self) -> ByteBuffer {
        let mut wt_buffer = ByteBuffer::new();
        // wt_buffer.write_u16(self.MsgSize);
        // wt_buffer.write_u16(self.MsgType);
        wt_buffer.write_u32(self.SecurityCode);
        wt_buffer.write_u64(self.SharesTraded);
        wt_buffer.write_i64(self.Turnover);
        wt_buffer.write_i32(self.HighPrice);
        wt_buffer.write_i32(self.LowPrice);
        wt_buffer.write_i32(self.LastPrice);
        wt_buffer.write_i32(self.VWAP);
        wt_buffer.write_u32(self.ShortSellSharesTraded);
        wt_buffer.write_i64(self.ShortSellTurnover);

        return wt_buffer;
    }

    fn unpack(rd_buffer: ByteBuffer) -> Statistics {
        let mut rd_buffer = rd_buffer;
        Statistics {
            // MsgSize: rd_buffer.read_u16(),
            // MsgType: rd_buffer.read_u16(),
            SecurityCode: rd_buffer.read_u32(),
            SharesTraded: rd_buffer.read_u64(),
            Turnover: rd_buffer.read_i64(),
            HighPrice: rd_buffer.read_i32(),
            LowPrice: rd_buffer.read_i32(),
            LastPrice: rd_buffer.read_i32(),
            VWAP: rd_buffer.read_i32(),
            ShortSellSharesTraded: rd_buffer.read_u32(),
            ShortSellTurnover: rd_buffer.read_i64(),
        }
    }
}

// 54
#[derive(Debug, Clone)]
pub struct BrokerQueue {
    SecurityCode: u32,
    ItemCount: u8,
    Side: u16,
    BQMoreFlag: String,
    Filler_data: Vec<BrokerQueueFiller>,
}

impl Msg for BrokerQueue {

    fn new(msg_data: Vec<u8>) -> Self {
        let mut buffer = ByteBuffer::new();
        for index in msg_data {
            buffer.write_u8(index);
        }
        Self::unpack(buffer)
    }
    
    fn pack(self) -> ByteBuffer {
       ByteBuffer::new()
    }

    fn unpack(rd_buffer: ByteBuffer) -> Self {
        let mut rd_buffer = rd_buffer;
        let SecurityCode =  rd_buffer.read_u32();
        let ItemCount = rd_buffer.read_u8();
        let Side = rd_buffer.read_u16();
        let BQMoreFlag =  match String::from_utf8(rd_buffer.read_bytes(1 as usize)) {
                Ok(msg) => msg,
                Err(err) => String::new(),
            };
        let Filler_data = BrokerQueueFiller::unpack(rd_buffer,ItemCount);
        Self {
            // MsgSize: rd_buffer.read_u16(),
            // MsgType: rd_buffer.read_u16(),
            SecurityCode: SecurityCode,
            ItemCount:ItemCount,
            Side:Side,
            BQMoreFlag: BQMoreFlag,
            Filler_data: Filler_data,
        }
    }
}

// 54 BrokerQueueFiller
#[derive(Debug, Clone)]
pub struct BrokerQueueFiller {
    Item: u16,
    Type: String,
    Filler: String,
}

impl FillerData<Self>  for BrokerQueueFiller {

    fn unpack(rd_buffer: ByteBuffer, filler: u8) -> Vec<Self> {
        let mut filler_data: Vec<Self> = Vec::new();
        let mut rd_buffer = rd_buffer;
        // level_price_filler.
        for row in 0..filler {
            filler_data.push(Self {
                Item: rd_buffer.read_u16(),
                Type: match String::from_utf8(rd_buffer.read_bytes(1 as usize)) {
                    Ok(msg) => msg,
                    Err(err) => String::new(),
                },
                Filler: match String::from_utf8(rd_buffer.read_bytes(1 as usize)) {
                    Ok(msg) => msg,
                    Err(err) => String::new(),
                },
            });
        }
        return filler_data;
    }
}



// # msg 41 开盘价

#[derive(Debug, Clone)]
pub struct IndicativeEquilibriumPrice {
    SecurityCode: u32,
    Price: i32,
    AggregateQuantity: u64,
}


impl Msg for IndicativeEquilibriumPrice {

     fn new(msg_data: Vec<u8>) -> Self {
        let mut buffer = ByteBuffer::new();
        for index in msg_data {
            buffer.write_u8(index);
        }
        Self::unpack(buffer)
    }

    fn pack(self) -> ByteBuffer {
        let mut wt_buffer = ByteBuffer::new();
        wt_buffer.write_u32(self.SecurityCode);
        wt_buffer.write_i32(self.Price);
        wt_buffer.write_u64(self.AggregateQuantity);
        return wt_buffer;
    }

    fn unpack(rd_buffer: ByteBuffer) -> IndicativeEquilibriumPrice {
        let mut rd_buffer = rd_buffer;
        IndicativeEquilibriumPrice {
            SecurityCode: rd_buffer.read_u32(),
            Price: rd_buffer.read_i32(),
            AggregateQuantity: rd_buffer.read_u64(),
        }
    }
}

//msg 43 收盘竞价时参考价格

#[derive(Debug, Clone)]
pub struct ReferencePrice {
    SecurityCode: u32,
    ReferencePrice: i32,
    LowerPrice: i32,
    UpperPrice: i32,
}

impl Msg for ReferencePrice {

    fn new(msg_data: Vec<u8>) -> Self {
        let mut buffer = ByteBuffer::new();
        for index in msg_data {
            buffer.write_u8(index);
        }
        Self::unpack(buffer)
    }

     fn pack(self) -> ByteBuffer {
        let mut wt_buffer = ByteBuffer::new();
        wt_buffer.write_u32(self.SecurityCode);
        wt_buffer.write_i32(self.ReferencePrice);
        wt_buffer.write_i32(self.LowerPrice);
        wt_buffer.write_i32(self.UpperPrice);
        return wt_buffer;
    }

    fn unpack(rd_buffer: ByteBuffer) -> ReferencePrice {
        let mut rd_buffer = rd_buffer;
        ReferencePrice {
            SecurityCode: rd_buffer.read_u32(),
            ReferencePrice: rd_buffer.read_i32(),
            LowerPrice: rd_buffer.read_i32(),
            UpperPrice: rd_buffer.read_i32(),
        }
    }
}


// msg 62 收盘价
#[derive(Debug, Clone)]
pub struct CloseingPrice {
    SecurityCode: u32,
    ClosingPrice: i32,
    NumberOfTrades: u32,
}

impl Msg for CloseingPrice {
    fn new(msg_data: Vec<u8>) -> Self {
        let mut buffer = ByteBuffer::new();
        for index in msg_data {
            buffer.write_u8(index);
        }
        Self::unpack(buffer)
    }

     fn pack(self) -> ByteBuffer {
        let mut wt_buffer = ByteBuffer::new();
        wt_buffer.write_u32(self.SecurityCode);
        wt_buffer.write_i32(self.ClosingPrice);
        wt_buffer.write_u32(self.NumberOfTrades);
        return wt_buffer;
    }

    fn unpack(rd_buffer: ByteBuffer) -> CloseingPrice {
        let mut rd_buffer = rd_buffer;
        CloseingPrice {
            SecurityCode: rd_buffer.read_u32(),
            ClosingPrice: rd_buffer.read_i32(),
            NumberOfTrades: rd_buffer.read_u32(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct OrderImbalance {
    SecurityCode: u32,
    OrderImbalanceDirection: String,
    Filler: String,
    OrderImbalanceQuantity:u64,
    Filler2:String
}

impl Msg for OrderImbalance {
    fn new(msg_data: Vec<u8>) -> Self {
        let mut buffer = ByteBuffer::new();
        for index in msg_data {
            buffer.write_u8(index);
        }
        Self::unpack(buffer)
    }

     fn pack(self) -> ByteBuffer {
        let mut wt_buffer = ByteBuffer::new();
        // wt_buffer.write_u32(self.SecurityCode);
        // wt_buffer.write_i32(self.ClosingPrice);
        // wt_buffer.write_u32(self.NumberOfTrades);
        return wt_buffer;
    }

    fn unpack(rd_buffer: ByteBuffer) -> Self {
        let mut rd_buffer = rd_buffer;
        Self {
                SecurityCode: rd_buffer.read_u32(),
                OrderImbalanceDirection: match String::from_utf8(rd_buffer.read_bytes(1 as usize)) {
                    Ok(msg) => msg,
                    Err(err) => String::new(),
                },
                Filler: match String::from_utf8(rd_buffer.read_bytes(1 as usize)) {
                    Ok(msg) => msg,
                    Err(err) => String::new(),
                },
                OrderImbalanceQuantity:rd_buffer.read_u64(),
                Filler2:match String::from_utf8(rd_buffer.read_bytes(2 as usize)) {
                    Ok(msg) => msg,
                    Err(err) => String::new(),
                },
        }
    }
}

