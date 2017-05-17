using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;

namespace strategy
{
    abstract class CashSuper
    {
        public abstract double accpetCash(double money);
    }

    class CashNormal: CashSuper
    {
        public override double accpetCash(double money)
        {
            return money;
        }

    }

    class CashRebate: CashSuper
    {
        private double moneyRebate = 1d;
        public CashRebate(string moneyRebate)
        {
            this.moneyRebate = double.Parse(moneyRebate);
        }

        public override double accpetCash(double money)
        {
            return money * moneyRebate;
        }

    }

    class CashReturn : CashSuper
    {
        private double moneyCondition = 0.0d;
        private double moneyReturn = 0.0d;
        public CashReturn(string moneyCondition, string moneyReturn)
        {
            this.moneyCondition = double.Parse(moneyCondition);
            this.moneyReturn = double.Parse(moneyReturn);
        }

        public override double accpetCash(double money)
        {
            double result = money;
            if (money >= moneyCondition)
            {   //  Math.Floor(money / moneyCondition) 1000/ 300 = 3  301/300 = 1 
                result = money - Math.Floor(money / moneyCondition) * moneyReturn;
            }
            return result;
        }
    }


    class CashContext
    {
        CashSuper cs = null;
        public CashContext(string type)
        {
            switch (type)
            {
                case "正常收费":
                    CashNormal cs0 = new CashNormal();
                    cs = cs0;
                    break;
                case "满300返还100":
                    CashReturn cr = new CashReturn("300", "100");
                    cs = cr;
                    break;
                case "打8折":
                    CashRebate cr1 = new CashRebate("0.8");
                    cs = cr1;
                    break;

            }
        }
        
        public double getResutl(double money)
        {
            return cs.accpetCash(money);
        }
    }
}
