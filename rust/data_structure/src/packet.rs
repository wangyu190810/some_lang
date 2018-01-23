use bytebuffer::ByteBuffer;
use time;




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
    MsgSize: u16,
    MsgType: u16,
    SecurityCode: u32,
    NominalPrice: i32,
}


impl NominalPrice {
    pub fn new(MsgSize: u16, MsgType: u16, SecurityCode: u32, NominalPrice: i32) -> NominalPrice {
        NominalPrice {
            MsgSize: MsgSize,
            MsgType: MsgType,
            SecurityCode: SecurityCode,
            NominalPrice: NominalPrice,
        }
    }

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
            SharesTraded:rd_buffer.read_u64(),
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


#[cfg(test)]
mod tests {
    use super::*;

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
            485000
            
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
            10000001011
        );
        let clone = price.clone();
        let mut wt_buffer = price.pack();
        let unpack = Statistics::unpack(wt_buffer);
        assert_eq!(clone.ShortSellSharesTraded, unpack.ShortSellSharesTraded)
    }

}
