// use bytebuffer::ByteBuffer;

use bytebuffer::*;
use byteorder::{BigEndian, ByteOrder, LittleEndian};
use std::io::{Read, Write};
use time;
// 包头数据
use std::fmt;

#[derive(Debug, Clone)]
pub struct PackHead {
    pub PktSize: u16,
    pub MsgCount: u8,
    pub Filler: String,
    pub SeqNum: u32,
    pub SendTime: u64,
}

impl PackHead {
    pub fn new(head: Vec<u8>) -> PackHead {
        let mut buffer = ByteBuffer::new();
        // buffer.endian = LittleEndian;
        buffer.set_endian(Endian::LittleEndian);
        for mut row in head {
            buffer.write_u8(row);
        }
        PackHead::unpack(buffer)
    }

    // pub fn unpack()
    pub fn unpack(rd_buffer: ByteBuffer) -> PackHead {
        let mut rd_buffer = rd_buffer;
        PackHead {
            PktSize: rd_buffer.read_u16(),
            MsgCount: rd_buffer.read_u8(),
            Filler: String::from_utf8(rd_buffer.read_bytes(1 as usize)).unwrap(),
            SeqNum: rd_buffer.read_u32(),
            SendTime: rd_buffer.read_u64(),
        }
    }

    // pub fn
}

impl fmt::Display for PackHead {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "PktSize {}，MsgCount {}, Filler {}, SeqNum {}, SendTime {}",
            self.PktSize, self.MsgCount, self.Filler, self.SeqNum, self.SendTime
        )
    }
}

#[derive(Debug, Clone)]
pub struct Head {
    pub MsgSize: u16,
    pub MsgType: u16,
}

impl Head {
    pub fn new(head: Vec<u8>) -> Head {
        let mut buffer = ByteBuffer::new();
        // buffer.endian = LittleEndian;
        buffer.set_endian(Endian::LittleEndian);
        buffer.write_u8(head[0]);
        buffer.write_u8(head[1]);
        buffer.write_u8(head[2]);
        buffer.write_u8(head[3]);
        Head::unpack(buffer)
    }
    // set_endian

    pub fn pack(self) -> ByteBuffer {
        let mut wt_buffer = ByteBuffer::new();
        wt_buffer.write_u16(self.MsgSize);
        wt_buffer.write_u16(self.MsgType);
        return wt_buffer;
    }

    pub fn unpack(rd_buffer: ByteBuffer) -> Head {
        let mut rd_buffer = rd_buffer;
        Head {
            MsgSize: rd_buffer.read_u16(),
            MsgType: rd_buffer.read_u16(),
        }
    }
}

// 53

// {'field': 'MsgSize', 'format': 'uint16', 'offset': 0, 'len': 2, 'value': 60},
// {'field': 'MsgType', 'format': 'uint16', 'offset': 2, 'len': 2, 'value': 53},
// {'field': 'SecurityCode', 'format': 'uint32', 'offset': 4, 'len': 4, 'value': 700},
// {'field': 'Filler', 'format': 'string', 'offset': 8, 'len': 3, 'value': ''},
// {'field': 'NoEntries', 'format': 'uint8', 'offset': 11, 'len': 1, 'value': 2},

#[derive(Debug, Clone)]
pub struct LevelPriceFiller {
    pub AggregateQuantity: u64,
    pub Price: i32,
    pub NumberOfOrders: u32,
    pub PriceLevel: u8,
    pub UpdateAction: u8,
    pub Filler: String,
}

impl LevelPriceFiller {
    pub fn unpack(rd_buffer: ByteBuffer, filler: u8) -> Vec<LevelPriceFiller> {
        let mut level_price_filler: Vec<LevelPriceFiller> = Vec::new();
        let mut rd_buffer = rd_buffer;
        // level_price_filler.
        for row in 0..filler {
            level_price_filler.push(
                LevelPriceFiller {
                AggregateQuantity: rd_buffer.read_u64(),
                Price: rd_buffer.read_i32(),
                NumberOfOrders: rd_buffer.read_u32(),
                PriceLevel: rd_buffer.read_u8(),
                UpdateAction: rd_buffer.read_u8(),
                Filler: String::from_utf8(rd_buffer.read_bytes(3 as usize)).unwrap(),
            });
        }
        return level_price_filler;
    }
}


impl fmt::Display for LevelPriceFiller {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Price {} PriceLevel {}, ",
            self.Price, self.PriceLevel
        )
    }
}

// }

//   {'field': 'AggregateQuantity', 'format': 'uint64', 'offset': 12, 'len': 8, 'value': 100},
//         {'field': 'Price', 'format': 'int32s', 'offset': 20, 'len': 4, 'value': [151000, 151500, 152000, 152500, 153000, 153500, 154000, 154500, 155000, 155500]},
//         {'field': 'NumberOfOrders', 'format': 'uint32', 'offset': 24, 'len': 4, 'value': 2},
//         {'field': 'Side', 'format': 'uint16', 'offset': 28, 'len': 2, 'value': 0},
//         {'field': 'PriceLevel', 'format': 'uint8s', 'offset': 30, 'len': 1, 'value': [i for i in range(1, 11)]},
//         {'field': 'UpdateAction', 'format': 'uint8s', 'offset': 31, 'len': 1, 'value': [0, 1, 2, 74, 0, 1, 2, 74, 0, 1]},
//         {'field': 'Filler', 'format': 'string', 'offset': 32, 'len': 4, 'value': 'a'},

#[derive(Debug, Clone)]
pub struct LevelPrice {
    pub MsgSize: u16,
    pub MsgType: u16,
    pub SecurityCode: u32,
    pub Filler: String,
    pub NoEntries: u8,
    pub FillerData: Vec<LevelPriceFiller>,
}

impl LevelPrice {
    pub fn new(nominalprice: Vec<u8>,filler_nums:u8) -> LevelPrice {
        let mut buffer = ByteBuffer::new();

        // buffer.set_endian(LittleEndian);
        for index in nominalprice {
            buffer.write_u8(index);
        }
        LevelPrice::unpack(buffer,filler_nums)
    }

    pub fn unpack(rd_buffer: ByteBuffer,filler_nums:u8) -> LevelPrice {
        let mut rd_buffer = rd_buffer;
        LevelPrice {
            MsgSize: rd_buffer.read_u16(),
            MsgType: rd_buffer.read_u16(),
            SecurityCode: rd_buffer.read_u32(),
            Filler: String::from_utf8(rd_buffer.read_bytes(3 as usize)).unwrap(),
            NoEntries: rd_buffer.read_u8(),
            FillerData: LevelPriceFiller::unpack(rd_buffer, filler_nums),
        }
    }
}

impl fmt::Display for LevelPrice {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // let filler = self.FillerData;
        let mut filler_string = "";
        // for row in filler{
        //     mut filler_string += format!("{}",row.Price);
        //     mut filler_string += format!("{}",row.PriceLevel);

        // }

        write!(
            f,
            "MsgSize {} MsgType {}, Filler {}, SecurityCode {}, NoEntries {} filler other {}",
            self.MsgSize,
            self.MsgType,
            self.Filler,
            self.SecurityCode,
            self.NoEntries,
            self.FillerData[0]
            // &filler_string
        )
    }
}

// msg 62 收盘价
#[derive(Debug, Clone)]
pub struct CloseingPrice {
    MsgSize: u16,
    MsgType: u16,
    SecurityCode: u32,
    ClosingPrice: i32,
    NumberOfTrades: u32,
}

impl CloseingPrice {
    pub fn new(
        MsgSize: u16,
        MsgType: u16,
        SecurityCode: u32,
        ClosingPrice: i32,
        NumberOfTrades: u32,
    ) -> CloseingPrice {
        CloseingPrice {
            MsgSize: MsgSize,
            MsgType: MsgType,
            SecurityCode: SecurityCode,
            ClosingPrice: ClosingPrice,
            NumberOfTrades: NumberOfTrades,
        }
    }

    pub fn pack(self) -> ByteBuffer {
        let mut wt_buffer = ByteBuffer::new();
        wt_buffer.write_u16(self.MsgSize);
        wt_buffer.write_u16(self.MsgType);
        wt_buffer.write_u32(self.SecurityCode);
        wt_buffer.write_i32(self.ClosingPrice);
        wt_buffer.write_u32(self.NumberOfTrades);
        return wt_buffer;
    }

    pub fn unpack(rd_buffer: ByteBuffer) -> CloseingPrice {
        let mut rd_buffer = rd_buffer;
        CloseingPrice {
            MsgSize: rd_buffer.read_u16(),
            MsgType: rd_buffer.read_u16(),
            SecurityCode: rd_buffer.read_u32(),
            ClosingPrice: rd_buffer.read_i32(),
            NumberOfTrades: rd_buffer.read_u32(),
        }
    }
}

// msg 40 现价

#[derive(Debug, Clone)]
pub struct NominalPrice {
    pub MsgSize: u16,
    pub MsgType: u16,
    pub SecurityCode: u32,
    pub NominalPrice: i32,
}

impl NominalPrice {
    pub fn new(nominalprice: Vec<u8>) -> NominalPrice {
        let mut buffer = ByteBuffer::new();

        // buffer.set_endian(LittleEndian);
        for index in nominalprice {
            buffer.write_u8(index);
        }
        NominalPrice::unpack(buffer)
    }

    // pub fn new(MsgSize: u16, MsgType: u16, SecurityCode: u32, NominalPrice: i32) -> NominalPrice {
    //     NominalPrice {
    //         MsgSize: MsgSize,
    //         MsgType: MsgType,
    //         SecurityCode: SecurityCode,
    //         NominalPrice: NominalPrice,
    //     }
    // }

    pub fn pack(self) -> ByteBuffer {
        let mut wt_buffer = ByteBuffer::new();
        wt_buffer.write_u16(self.MsgSize);
        wt_buffer.write_u16(self.MsgType);
        wt_buffer.write_u32(self.SecurityCode);
        wt_buffer.write_i32(self.NominalPrice);
        return wt_buffer;
    }

    pub fn unpack(rd_buffer: ByteBuffer) -> NominalPrice {
        let mut rd_buffer = rd_buffer;
        NominalPrice {
            MsgSize: rd_buffer.read_u16(),
            MsgType: rd_buffer.read_u16(),
            SecurityCode: rd_buffer.read_u32(),
            NominalPrice: rd_buffer.read_i32(),
        }
    }
}

// 11

#[derive(Debug, Clone)]
pub struct NowPrice {
    pub MsgSize: u16,
    pub MsgType: u16,
    pub SecurityCode: u32,
    pub MarketCode: String,
}

impl NowPrice {
    pub fn new(nominalprice: Vec<u8>) -> NowPrice {
        let mut buffer = ByteBuffer::new();

        // buffer.set_endian(LittleEndian);
        for index in nominalprice {
            buffer.write_u8(index);
        }
        NowPrice::unpack(buffer)
    }

    pub fn unpack(rd_buffer: ByteBuffer) -> NowPrice {
        let mut rd_buffer = rd_buffer;
        NowPrice {
            MsgSize: rd_buffer.read_u16(),
            MsgType: rd_buffer.read_u16(),
            SecurityCode: rd_buffer.read_u32(),
            MarketCode: String::from_utf8(rd_buffer.read_bytes(4 as usize)).unwrap(),
        }
    }
}

//  pub fn unpack(rd_buffer: ByteBuffer) -> IndexDefinition {
//     let mut rd_buffer = rd_buffer;
//     IndexDefinition {
//         MsgSize: rd_buffer.read_u16(),
//         MsgType: rd_buffer.read_u16(),
//         IndexCode: String::from_utf8(rd_buffer.read_bytes(11 as usize)).unwrap(),
//         IndexSource: String::from_utf8(rd_buffer.read_bytes(1 as usize)).unwrap(),
//         CurrencyCode: String::from_utf8(rd_buffer.read_bytes(3 as usize)).unwrap(),
//         Filler: String::from_utf8(rd_buffer.read_bytes(1 as usize)).unwrap(),
//     }
// }
// # msg 41 开盘价
// def get_msg_indicative_equilibrium_price(args):
//     data = unpack('<HHLlQ', args)
//     data_dict = {
//         'MsgSize' : data[0],
//         'MsgType' : data[1],
//         'SecurityCode' : data[2],
//         'Price' : data[3],
//         'AggregateQuantity' : data[4]
//     }
//     return data_dict

// # msg 41 开盘价

#[derive(Debug, Clone)]
pub struct IndicativeEquilibriumPrice {
    MsgSize: u16,
    MsgType: u16,
    SecurityCode: u32,
    Price: i32,
    AggregateQuantity: u64,
}

impl IndicativeEquilibriumPrice {
    pub fn new(
        MsgSize: u16,
        MsgType: u16,
        SecurityCode: u32,
        Price: i32,
        AggregateQuantity: u64,
    ) -> IndicativeEquilibriumPrice {
        IndicativeEquilibriumPrice {
            MsgSize: MsgSize,
            MsgType: MsgType,
            SecurityCode: SecurityCode,
            Price: Price,
            AggregateQuantity: AggregateQuantity,
        }
    }

    pub fn pack(self) -> ByteBuffer {
        let mut wt_buffer = ByteBuffer::new();
        wt_buffer.write_u16(self.MsgSize);
        wt_buffer.write_u16(self.MsgType);
        wt_buffer.write_u32(self.SecurityCode);
        wt_buffer.write_i32(self.Price);
        wt_buffer.write_u64(self.AggregateQuantity);
        return wt_buffer;
    }

    pub fn unpack(rd_buffer: ByteBuffer) -> IndicativeEquilibriumPrice {
        let mut rd_buffer = rd_buffer;
        IndicativeEquilibriumPrice {
            MsgSize: rd_buffer.read_u16(),
            MsgType: rd_buffer.read_u16(),
            SecurityCode: rd_buffer.read_u32(),
            Price: rd_buffer.read_i32(),
            AggregateQuantity: rd_buffer.read_u64(),
        }
    }
}

// # msg 43 收盘竞价时参考价格
// def get_msg_reference_price(args):
//     data = unpack('<HHLlll', args)
//     data_dict = {
//         'MsgSize' : data[0],
//         'MsgType' : data[1],
//         'SecurityCode' : data[2],
//         'ReferencePrice' : data[3],
//         'LowerPrice' : data[4],
//         'UpperPrice' : data[5]
//     }
//     return data_dict

#[derive(Debug, Clone)]
pub struct ReferencePrice {
    MsgSize: u16,
    MsgType: u16,
    SecurityCode: u32,
    ReferencePrice: i32,
    LowerPrice: i32,
    UpperPrice: i32,
}

impl ReferencePrice {
    pub fn new(
        MsgSize: u16,
        MsgType: u16,
        SecurityCode: u32,
        ReferencePrice: i32,
        LowerPrice: i32,
        UpperPrice: i32,
    ) -> ReferencePrice {
        ReferencePrice {
            MsgSize: MsgSize,
            MsgType: MsgType,
            SecurityCode: SecurityCode,
            ReferencePrice: ReferencePrice,
            LowerPrice: LowerPrice,
            UpperPrice: UpperPrice,
        }
    }

    pub fn pack(self) -> ByteBuffer {
        let mut wt_buffer = ByteBuffer::new();
        wt_buffer.write_u16(self.MsgSize);
        wt_buffer.write_u16(self.MsgType);
        wt_buffer.write_u32(self.SecurityCode);
        wt_buffer.write_i32(self.ReferencePrice);
        wt_buffer.write_i32(self.LowerPrice);
        wt_buffer.write_i32(self.UpperPrice);
        return wt_buffer;
    }

    pub fn unpack(rd_buffer: ByteBuffer) -> ReferencePrice {
        let mut rd_buffer = rd_buffer;
        ReferencePrice {
            MsgSize: rd_buffer.read_u16(),
            MsgType: rd_buffer.read_u16(),
            SecurityCode: rd_buffer.read_u32(),
            ReferencePrice: rd_buffer.read_i32(),
            LowerPrice: rd_buffer.read_i32(),
            UpperPrice: rd_buffer.read_i32(),
        }
    }
}

// # msg 23 收市競價交易時段價格限制
// def get_msg_VCM_trigger(args):
//     data = unpack('<HHLQQlll')
//     data_dict = {
//         'MsgSize' : data[0],
//         'MsgType' : data[1],
//         'SecurityCode' : data[2],
//         'CoolingOffStartTime': data[3],
//         'CoolingOffEndTime': data[4],
//         'VCMReferencePrice': data[5],
//         'VCMLowerPrice': data[6],
//         'VCMUpperPrice': data[7]
//     }
//     return data_dict

#[derive(Debug, Clone)]
pub struct VCMTrigger {
    MsgSize: u16,
    MsgType: u16,
    SecurityCode: u32,
    CoolingOffStartTime: u64,
    CoolingOffEndTime: u64,
    VCMReferencePrice: i32,
    VCMLowerPrice: i32,
    VCMUpperPrice: i32,
}

impl VCMTrigger {
    pub fn new(
        MsgSize: u16,
        MsgType: u16,
        SecurityCode: u32,
        CoolingOffStartTime: u64,
        CoolingOffEndTime: u64,
        VCMReferencePrice: i32,
        VCMLowerPrice: i32,
        VCMUpperPrice: i32,
    ) -> VCMTrigger {
        VCMTrigger {
            MsgSize: MsgSize,
            MsgType: MsgType,
            SecurityCode: SecurityCode,
            CoolingOffStartTime: CoolingOffStartTime,
            CoolingOffEndTime: CoolingOffEndTime,
            VCMReferencePrice: VCMReferencePrice,
            VCMLowerPrice: VCMLowerPrice,
            VCMUpperPrice: VCMUpperPrice,
        }
    }

    pub fn pack(self) -> ByteBuffer {
        let mut wt_buffer = ByteBuffer::new();
        wt_buffer.write_u16(self.MsgSize);
        wt_buffer.write_u16(self.MsgType);
        wt_buffer.write_u32(self.SecurityCode);
        wt_buffer.write_u64(self.CoolingOffStartTime);
        wt_buffer.write_u64(self.CoolingOffEndTime);
        wt_buffer.write_i32(self.VCMReferencePrice);
        wt_buffer.write_i32(self.VCMLowerPrice);
        wt_buffer.write_i32(self.VCMUpperPrice);
        return wt_buffer;
    }

    pub fn unpack(rd_buffer: ByteBuffer) -> VCMTrigger {
        let mut rd_buffer = rd_buffer;
        VCMTrigger {
            MsgSize: rd_buffer.read_u16(),
            MsgType: rd_buffer.read_u16(),
            SecurityCode: rd_buffer.read_u32(),
            CoolingOffStartTime: rd_buffer.read_u64(),
            CoolingOffEndTime: rd_buffer.read_u64(),
            VCMReferencePrice: rd_buffer.read_i32(),
            VCMLowerPrice: rd_buffer.read_i32(),
            VCMUpperPrice: rd_buffer.read_i32(),
        }
    }
}

// # msg 60 成交汇总
// def get_msg_statistics(args):
//     data = unpack('<HHLQqllllLq', args)
//     data_dict = {
//         'MsgSize' : data[0],
//         'MsgType' : data[1],
//         'SecurityCode' : data[2],
//         'SharesTraded' : data[3],
//         'Turnover' : data[4],

//         'HighPrice' : data[5],
//         'LowPrice' : data[6],
//         'LastPrice' : data[7],
//         'VWAP' : data[8],
//         'ShortSellSharesTraded' : data[9],
//         'ShortSellTurnover' : data[10]
//     }
//     return data_dict

#[derive(Debug, Clone)]
pub struct Statistics {
    MsgSize: u16,
    MsgType: u16,
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

impl Statistics {
    pub fn new(
        MsgSize: u16,
        MsgType: u16,
        SecurityCode: u32,
        SharesTraded: u64,
        Turnover: i64,
        HighPrice: i32,
        LowPrice: i32,
        LastPrice: i32,
        VWAP: i32,
        ShortSellSharesTraded: u32,
        ShortSellTurnover: i64,
    ) -> Statistics {
        Statistics {
            MsgSize: MsgSize,
            MsgType: MsgType,
            SecurityCode: SecurityCode,
            SharesTraded: SharesTraded,
            Turnover: Turnover,
            HighPrice: HighPrice,
            LowPrice: LowPrice,
            LastPrice: LastPrice,
            VWAP: VWAP,
            ShortSellSharesTraded: ShortSellSharesTraded,
            ShortSellTurnover: ShortSellTurnover,
        }
    }

    pub fn pack(self) -> ByteBuffer {
        let mut wt_buffer = ByteBuffer::new();
        wt_buffer.write_u16(self.MsgSize);
        wt_buffer.write_u16(self.MsgType);
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

    pub fn unpack(rd_buffer: ByteBuffer) -> Statistics {
        let mut rd_buffer = rd_buffer;
        Statistics {
            MsgSize: rd_buffer.read_u16(),
            MsgType: rd_buffer.read_u16(),
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

// # msg 61 市场成交量
// def get_msg_market_turnover(args):
//     data = unpack('<HH4s3s1sq', args)
//     data_dict = {
//         'MsgSize' : data[0],
//         'MsgType' : data[1],
//         'MarketCode' : data[2],
//         'CurrencyCode' : data[3],
//         'Filler' : data[4],
//         'Turnover' : data[5]
//     }
//     return data_dict

#[derive(Debug, Clone)]
pub struct MarketTurnover {
    MsgSize: u16,
    MsgType: u16,
    MarketCode: String,
    CurrencyCode: String,
    Filler: String,
    Turnover: i64,
}

impl MarketTurnover {
    pub fn new(
        MsgSize: u16,
        MsgType: u16,
        MarketCode: String,
        CurrencyCode: String,
        Filler: String,
        Turnover: i64,
    ) -> MarketTurnover {
        MarketTurnover {
            MsgSize: MsgSize,
            MsgType: MsgType,
            MarketCode: MarketCode,
            CurrencyCode: CurrencyCode,
            Filler: Filler,
            Turnover: Turnover,
        }
    }

    pub fn pack(self) -> ByteBuffer {
        let mut wt_buffer = ByteBuffer::new();
        wt_buffer.write_u16(self.MsgSize);
        wt_buffer.write_u16(self.MsgType);
        wt_buffer.write_bytes(&self.MarketCode.into_bytes());
        wt_buffer.write_bytes(&self.CurrencyCode.into_bytes());
        wt_buffer.write_bytes(&self.Filler.into_bytes());
        wt_buffer.write_i64(self.Turnover);
        return wt_buffer;
    }

    pub fn unpack(rd_buffer: ByteBuffer) -> MarketTurnover {
        let mut rd_buffer = rd_buffer;
        MarketTurnover {
            MsgSize: rd_buffer.read_u16(),
            MsgType: rd_buffer.read_u16(),
            MarketCode: String::from_utf8(rd_buffer.read_bytes(4 as usize)).unwrap(),
            CurrencyCode: String::from_utf8(rd_buffer.read_bytes(3 as usize)).unwrap(),
            Filler: String::from_utf8(rd_buffer.read_bytes(1 as usize)).unwrap(),
            Turnover: rd_buffer.read_i64(),
        }
    }
}

// # msg 70 指数定义
// def get_msg_index_definition(args):
//     data = unpack('<HH11s1s3s1s', args)
//     data_dict = {
//         'MsgSize' : data[0],
//         'MsgType' : data[1],
//         'IndexCode' : data[2],
//         'IndexSource' : data[3],
//         'CurrencyCode' : data[4],
//         'Filler' : data[5]
//     }
//     return data_dict

#[derive(Debug, Clone)]
pub struct IndexDefinition {
    MsgSize: u16,
    MsgType: u16,
    IndexCode: String,
    IndexSource: String,
    CurrencyCode: String,
    Filler: String,
}

impl IndexDefinition {
    // pub fn new(
    //       MsgSize: u16,
    // MsgType: u16,
    // IndexCode: String,
    // IndexSource: String,
    // CurrencyCode:String,
    // Filler: String,
    // ) -> IndexDefinition {
    //     IndexDefinition {
    //         MsgSize: MsgSize,
    //         MsgType: MsgType,
    //       IndexCode: IndexCode,
    // IndexSource: IndexSource,
    // CurrencyCode:CurrencyCode,
    // Filler: Filler

    pub fn new(indexdefinition: Vec<u8>) -> IndexDefinition {
        let mut buffer = ByteBuffer::new();

        // buffer.set_endian(LittleEndian);
        for index in 0..20 {
            buffer.write_u8(indexdefinition[index]);
        }
        IndexDefinition::unpack(buffer)
    }

    pub fn pack(self) -> ByteBuffer {
        let mut wt_buffer = ByteBuffer::new();
        wt_buffer.write_u16(self.MsgSize);
        wt_buffer.write_u16(self.MsgType);
        wt_buffer.write_bytes(&self.IndexCode.into_bytes());
        wt_buffer.write_bytes(&self.IndexSource.into_bytes());
        wt_buffer.write_bytes(&self.CurrencyCode.into_bytes());
        wt_buffer.write_bytes(&self.Filler.into_bytes());
        return wt_buffer;
    }

    pub fn unpack(rd_buffer: ByteBuffer) -> IndexDefinition {
        let mut rd_buffer = rd_buffer;
        IndexDefinition {
            MsgSize: rd_buffer.read_u16(),
            MsgType: rd_buffer.read_u16(),
            IndexCode: String::from_utf8(rd_buffer.read_bytes(11 as usize)).unwrap(),
            IndexSource: String::from_utf8(rd_buffer.read_bytes(1 as usize)).unwrap(),
            CurrencyCode: String::from_utf8(rd_buffer.read_bytes(3 as usize)).unwrap(),
            Filler: String::from_utf8(rd_buffer.read_bytes(1 as usize)).unwrap(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct AddOddLotOrder {
    pub MsgSize: u16,
    pub MsgType: u16,
    pub SecurityCode: u32,
    pub OrderId: u64,
    pub Price: i32,
    pub Quantity: u32,
    pub BrokerID: u16,
    pub Side: u16,
}

impl AddOddLotOrder {
    pub fn new(head: Vec<u8>) -> AddOddLotOrder {
        //     AddOddLotOrder {

        //          MsgSize: MsgSize,
        // MsgType: MsgType,
        // SecurityCode: SecurityCode,
        // OrderId: OrderId,
        // Price:Price,
        // Quantity: Quantity,
        // BrokerID:BrokerID,
        // Side:Side

        let mut buffer = ByteBuffer::new();

        // buffer.set_endian(LittleEndian);
        for index in 0..28 {
            buffer.write_u8(head[index]);
        }
        AddOddLotOrder::unpack(buffer)
    }

    // pub fn pack(self) -> ByteBuffer {
    //     let mut wt_buffer = ByteBuffer::new();
    //     wt_buffer.write_u16(self.MsgSize);
    //     wt_buffer.write_u16(self.MsgType);
    //     wt_buffer.write_bytes(&self.IndexCode.into_bytes());
    //     wt_buffer.write_bytes(&self.IndexSource.into_bytes());
    //     wt_buffer.write_bytes(&self.CurrencyCode.into_bytes());
    //     wt_buffer.write_bytes(&self.Filler.into_bytes());
    //     return wt_buffer;
    // }

    pub fn unpack(rd_buffer: ByteBuffer) -> AddOddLotOrder {
        let mut rd_buffer = rd_buffer;
        AddOddLotOrder {
            MsgSize: rd_buffer.read_u16(),
            MsgType: rd_buffer.read_u16(),
            SecurityCode: rd_buffer.read_u32(),
            OrderId: rd_buffer.read_u64(),
            Price: rd_buffer.read_i32(),
            Quantity: rd_buffer.read_u32(),
            BrokerID: rd_buffer.read_u16(),
            Side: rd_buffer.read_u16(),
        }
    }
}

// # msg 71 指数数据
// def get_msg_index_data(args):
//     data = unpack('<HH11s1sqqqqqqqqqqql1s3s', args)
//     data_dict = {
//         'MsgSize' : data[0],
//         'MsgType' : data[1],
//         'IndexCode' : data[2],
//         'IndexStatus' : data[3],
//         'IndexTime' : data[4],
//         'IndexValue' : data[5],

//         'NetChgPrevDay' : data[6],
//         'HighValue' : data[7],
//         'LowValue' : data[8],
//         'EASValue' : data[9],
//         'IndexTurnover' : data[10],
//         'OpeningValue' : data[11],

//         'ClosingValue' : data[12],
//         'PreviousSesClose' : data[13],
//         'IndexVolume' : data[14],
//         'NetChgPrevDayPct' : data[15],
//         'Exception' : data[16],
//         'Filler' : data[17]
//     }
//     return data_dict

pub fn save_data(buff_bytes: Vec<u8>, rd_buffer: &mut ByteBuffer) {
    for buff_byte in buff_bytes {
        if (buff_byte != 0) {
            rd_buffer.write_u8(buff_byte);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_Head() {
        let mut price = Head::new([128, 0, 1, 33].to_vec());
        let clone = price.clone();
        let mut wt_buffer = price.pack();
        let unpack = Head::unpack(wt_buffer);
        assert_eq!(clone.MsgSize, unpack.MsgSize)
    }

    #[test]
    fn test_CloseingPrice() {
        let mut price = CloseingPrice::new(16, 62, 700, 474000, 10000);
        let clone = price.clone();
        let mut wt_buffer = price.pack();
        let unpack = CloseingPrice::unpack(wt_buffer);
        assert_eq!(clone.ClosingPrice, unpack.ClosingPrice)
    }
    #[test]
    fn test_NominalPrice() {
        let mut close_price = NominalPrice::new(12, 40, 700, 474000);
        let close_price_clone = close_price.clone();
        let mut wt_buffer = close_price.pack();
        let unpack = NominalPrice::unpack(wt_buffer);
        assert_eq!(close_price_clone.NominalPrice, unpack.NominalPrice)
    }
    #[test]
    fn test_IndicativeEquilibriumPrice() {
        let mut close_price = IndicativeEquilibriumPrice::new(20, 41, 700, 474000, 100000);
        let close_price_clone = close_price.clone();
        let mut wt_buffer = close_price.pack();
        let unpack = IndicativeEquilibriumPrice::unpack(wt_buffer);
        assert_eq!(
            close_price_clone.AggregateQuantity,
            unpack.AggregateQuantity
        )
    }
    #[test]
    fn test_ReferencePrice() {
        let mut price = ReferencePrice::new(20, 43, 700, 474000, 460000, 480000);
        let clone = price.clone();
        let mut wt_buffer = price.pack();
        let unpack = ReferencePrice::unpack(wt_buffer);
        assert_eq!(clone.LowerPrice, unpack.LowerPrice)
    }

    #[test]
    fn test_VCMTrigger() {
        let mut price = VCMTrigger::new(
            36,
            23,
            700,
            10000001011,
            101001020102,
            479000,
            471000,
            485000,
        );

        let clone = price.clone();
        let mut wt_buffer = price.pack();
        let unpack = VCMTrigger::unpack(wt_buffer);
        assert_eq!(clone.VCMLowerPrice, unpack.VCMLowerPrice)
    }

    #[test]
    fn test_Statistics() {
        let mut price = Statistics::new(
            52,
            60,
            700,
            10000001011,
            101001020102,
            479000,
            471000,
            485000,
            471000,
            10000001011,
            10000001011,
        );
        let clone = price.clone();
        let mut wt_buffer = price.pack();
        let unpack = Statistics::unpack(wt_buffer);
        assert_eq!(clone.ShortSellSharesTraded, unpack.ShortSellSharesTraded)
    }
    #[test]
    fn test_MarketTurnover() {
        let mut price = MarketTurnover::new(
            20,
            61,
            "NAS\0".to_string(),
            "HKD".to_string(),
            "a".to_string(),
            12312,
        );
        let clone = price.clone();
        let mut wt_buffer = price.pack();
        let unpack = MarketTurnover::unpack(wt_buffer);
        println!("{}", unpack.MarketCode);
        assert_eq!(clone.MarketCode, unpack.MarketCode)
    }
    #[test]
    fn test_IndexDefinition() {
        let mut price = IndexDefinition::new(
            20,
            70,
            "00001100100".to_string(),
            "C".to_string(),
            "HKD".to_string(),
            "C".to_string(),
        );
        let clone = price.clone();
        let mut wt_buffer = price.pack();
        let unpack = IndexDefinition::unpack(wt_buffer);
        println!("{}", unpack.IndexCode);
        assert_eq!(clone.IndexCode, unpack.IndexCode)
    }

}
